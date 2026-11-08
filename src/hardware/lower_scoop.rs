use std::collections::HashSet;
use std::sync::LazyLock;

use frontbox_core::prelude::*;
use frontbox_core::tags::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, PlayerTilted, TurnEnding};
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp::ArcRampSubwayHit;
use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lower_scoop")
    .tag(Playfield)
    .mode(
      DriverMode::pulse_kick()
        .trigger_mode(DriverTriggerMode::VirtualSwitchTrue)
        .initial_pwm_length(HardwareValue::config(
          "Lower Scoop Touch Time",
          "Duration by which the eject plunger is brought into contact with the ball, before full eject",
          Duration::from_millis(7),
          Ranges::duration(0, 100),
        ))
        .initial_pwm_power(HardwareValue::fixed(Power::THREE_QUARTERS))
        .secondary_pwm_power(HardwareValue::Fixed(Power::ZERO))
        .secondary_pwm_length(HardwareValue::Fixed(Duration::ZERO))
        .kick_length(HardwareValue::config(
          "Lower Scoop Eject Time",
          "Duration that the plunger exert full power onto the ball (kick)",
          Duration::from_millis(35),
          Ranges::duration(10, 300),
        ))
        .build(),
    )
    .location(Vec3::new(1.675, 25.596, -2.0).relative_to(&planes::PLAYFIELD));

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lower_scoop")
    .inverted()
    .debounce(Duration::from_millis(50))
    .tag(Playfield)
    .location(Vec3::new(1.675, 25.596, -1.5).relative_to(&planes::PLAYFIELD));

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt_l")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(2.393, 28.11, 0.0).relative_to(&planes::PLAYFIELD));

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt_r")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(3.339, 27.357, 0.0).relative_to(&planes::PLAYFIELD));
}

pub static BOLTS_Q: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    LEFT_BOLT.names()[0].clone(),
    RIGHT_BOLT.names()[0].clone(),
  ])
});

// -- System --

pub const LOWER_SCOOP_EJECT: &'static str = "lower_scoop_eject";

/// Ejects a ball that lands in the scoop, unless something is holding the ball there. Anything that needs the ball to
/// wait (the mode select menu, a mode's intro, the extra ball animation) calls [`Self::hold`], and the ball ejects once
/// the last of them calls [`Self::release`]. A tilt or the end of a turn clears every hold.
#[derive(Clone)]
pub struct LowerScoopSystem {
  eject_program: LedPlayback,
  /// Plays each time the coil fires
  after_eject_program: LedPlayback,
  subway_entry: bool,
  handle: SystemHandle,
  /// Who's holding the ball, by `ctx.id()`
  holds: HashSet<u64>,
  /// There's a ball that hasn't been ejected yet
  ball_waiting: bool,
  eject_pending: bool,
  eject_cue: Option<u64>,
}

impl LowerScoopSystem {
  pub fn new() -> Self {
    let eject_program = led_playback!("lower_scoop/eject.scene").stopped();

    Self {
      eject_program,
      after_eject_program: led_playback!("lower_scoop/after_eject.scene").stopped(),
      subway_entry: false,
      holds: HashSet::new(),
      ball_waiting: false,
      eject_pending: false,
      eject_cue: None,
      handle: SystemHandle::default(),
    }
  }

  /// Keep a ball in the scoop, keyed by the requester's `ctx.id()`. This also catches one that's about to eject.
  pub fn hold(&mut self, requester: u64, ctx: &ServiceContext) {
    self.holds.insert(requester);
    if self.eject_pending {
      let ctx = &ctx.for_system(self.handle);
      if let Some(cue) = self.eject_cue.take() {
        ctx.cues.cancel(cue);
      }
      self.eject_program.stop(ctx);
      self.eject_pending = false;
    }
  }

  /// Take back a [`Self::hold`]. The ball ejects once nothing is holding it.
  pub fn release(&mut self, requester: u64, ctx: &ServiceContext) {
    self.holds.remove(&requester);
    // left to the next event, so that a hold being taken over from this one can get in first
    ctx.for_system(self.handle).emit(BeginEject);
  }

  fn begin_eject(&mut self, ctx: &SystemContext) {
    if self.ball_waiting && !self.eject_pending && self.holds.is_empty() {
      ctx.play_sfx(LOWER_SCOOP_EJECT);
      self.eject_cue = Some(
        ctx
          .cues
          .start(EjectLowerScoop, Cue::Once(Duration::from_millis(750))),
      );
      self.eject_program.reset();
      self.eject_program.play();
      self.eject_pending = true;
    }
  }

  fn complete_eject(&mut self, ctx: &SystemContext) {
    ctx.drivers.activate(COIL.name, ActivationMode::Tap);
    log::info!("Stopping LEDs after complete eject");
    self.eject_program.stop(ctx);
    self.after_eject_program.reset();
    self.after_eject_program.play();
    self.eject_pending = false;
    self.eject_cue = None;
    if self.ball_waiting {
      self.ball_waiting = false;
      ctx.emit(LowerScoopEjected);
    }
    ctx.systems.expect::<MachineSystem>().refresh_switch_state();
    ctx
      .cues
      .start(CheckVacated, Cue::Once(Duration::from_millis(250)));
  }

  fn clear_holds(&mut self, ctx: &SystemContext) {
    self.holds.clear();
    self.begin_eject(ctx);
  }

  fn on_ball_enter(&mut self, ctx: &SystemContext) {
    log::info!("Ball entered lower scoop");

    let event = LowerScoopBallEnter {
      subway_entry: self.subway_entry,
    };

    // If the player gets the ball into the scoop without using the subway, it gives points
    if !self.subway_entry {
      ctx.add_points(500);
    }

    self.subway_entry = false;
    self.ball_waiting = true;

    // whatever wants to hold the ball hears about it first, and the eject starts if nothing did
    ctx.emit(event);
    ctx.emit(BeginEject);
  }
}

impl System for LowerScoopSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();

    // check if there is a ball at startup and eject if so
    if ctx.switches.is_closed(OPTO.name).unwrap_or(false) {
      log::debug!("Lower scoop is occupied. Ejecting.");
      self.complete_eject(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<ArcRampSubwayHit>() {
      self.subway_entry = true;
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == OPTO.name
    {
      self.on_ball_enter(ctx);
    } else if event.is::<BeginEject>() {
      self.begin_eject(ctx);
    } else if event.is::<EjectLowerScoop>() {
      self.complete_eject(ctx);
    } else if event.is::<CheckVacated>()
      && self.holds.is_empty()
      && ctx.switches.is_closed(OPTO.name).unwrap_or(false)
    {
      self.complete_eject(ctx);
    } else if event.is::<RoutineStarted<TurnEnding>>() || event.is::<PlayerTilted>() {
      self.clear_holds(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.eject_program.apply(delta, ctx);
    self.after_eject_program.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
pub struct LowerScoopBallEnter {
  pub subway_entry: bool,
}

/// The ball has been kicked out of the scoop
#[derive(serde::Serialize, Event)]
pub struct LowerScoopEjected;

#[derive(serde::Serialize, Event)]
struct BeginEject;

#[derive(serde::Serialize, Event)]
struct EjectLowerScoop;

#[derive(serde::Serialize, Event)]
struct CheckVacated;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    LowerScoopSystem::new();
  }
}
