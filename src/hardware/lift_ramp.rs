use std::collections::HashSet;
use std::sync::LazyLock;

use frontbox_core::prelude::ActivationMode::VirtualSwitchOn;
use frontbox_core::prelude::DeactivationMode::VirtualSwitchOff;
use frontbox_core::prelude::DriverTriggerMode::VirtualSwitchTrue;
use frontbox_core::prelude::*;
use frontbox_core::tags::*;
use frontbox_pinball::{BallSearching, TurnPhase};

use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub RAMP_COIL: DriverDefinition = DriverDefinition::new("lift_ramp")
    .tag(Playfield)
    .mode(
      DriverMode::pulse_hold()
        .trigger_mode(VirtualSwitchTrue)
        .initial_pwm_length(HardwareValue::config(
          "Lift Ramp Kick Duration",
          "Amount of time to initially kick the lift ramp open",
          Duration::from_millis(22),
          Ranges::duration(5, 100)
        ))
        .initial_pwm_power(HardwareValue::config(
          "Lift Ramp Kick Power",
          "Power to use when initially kicking the lift ramp open",
          Power::FULL,
          Ranges::full_power()
        ))
        .secondary_pwm_power(HardwareValue::fixed(Power::QUARTER))
        .rest(HardwareValue::Fixed(Duration::from_millis(255)))
        .build(),
    )
    .location(Vec3::new(14.211, 5.726, -1.0).relative_to(&planes::PLAYFIELD));

  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_opto")
    .inverted()
    .debounce(Duration::from_millis(2))
    .tag(Playfield)
    .location(Vec3::new(12.945, 2.357, 2.0).relative_to(&planes::PLAYFIELD));

  pub BOLT_LED: LedDefinition = LedDefinition::single("lift_ramp_bolt")
    .tag(Playfield)
    .tag(Insert)
    .tag(Bolt)
    .tag(Lane)
    .location(Vec3::new(14.118, 10.827, 0.0).relative_to(&planes::PLAYFIELD));

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("lift_ramp_lane")
    .locations(LedLayout::ring(6, Vec3::new(13.5, 14.068, 0.0), 0.5, 42.0f32.to_radians(), LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .locations([Vec3::new(13.5, 14.068, 0.0).relative_to(&planes::PLAYFIELD)])
    .tag(Playfield)
    .tag(Insert)
    .tag(Hex)
    .tag(Lane);
}

pub static HEX_CENTER_LED: LazyLock<LedQ> = LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

pub static HEX_LINE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
  ])
});

pub static HEX_CIRCLE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
  ])
});

/// How long a ball search keeps the ramp up, for a ball stuck under it to roll out
const SEARCH_LIFT_TIME: Duration = Duration::from_secs(3);

/// Lifts the ramp while anything asks for it during an active turn, and reports ramp hits during one. During a ball
/// search it lifts the ramp if it's down, and lowers it again a few seconds later unless it's wanted up by then.
#[derive(Clone)]
pub struct LiftRampSystem {
  /// Who wants the ramp up, by `ctx.id()`. It stays up while anyone does.
  requests: HashSet<u64>,
  ramp_lifted: bool,
  close_cue_id: Option<u64>,
  handle: SystemHandle,
}

impl LiftRampSystem {
  pub fn new() -> Self {
    Self {
      requests: HashSet::new(),
      ramp_lifted: false,
      close_cue_id: None,
      handle: SystemHandle::default(),
    }
  }

  pub fn is_lifted(&self) -> bool {
    self.ramp_lifted
  }

  /// Ask for the ramp to be up, keyed by the requester's `ctx.id()`
  pub fn request_up(&mut self, requester: u64, ctx: &ServiceContext) {
    self.requests.insert(requester);
    self.lift_up(ctx);
  }

  /// Take back a [`Self::request_up`]. The ramp comes down once nobody is asking.
  pub fn release(&mut self, requester: u64, ctx: &ServiceContext) {
    self.requests.remove(&requester);
    if self.requests.is_empty() {
      self.lift_down(ctx);
    }
  }

  fn lift_up(&mut self, ctx: &ServiceContext) {
    if !self.ramp_lifted {
      let ctx = ctx.for_system(self.handle);
      log::info!("Lifting ramp up.");
      ctx.drivers.activate(RAMP_COIL.name, VirtualSwitchOn);
      self.ramp_lifted = true;
      ctx.emit(LiftRampUp);
    }
  }

  fn lift_down(&mut self, ctx: &ServiceContext) {
    if self.ramp_lifted {
      let ctx = ctx.for_system(self.handle);
      log::info!("Lifting ramp down.");
      self.ramp_lifted = false;
      ctx.drivers.deactivate(RAMP_COIL.name, VirtualSwitchOff);
      ctx.emit(LiftRampDown);
      if let Some(cue_id) = self.close_cue_id {
        ctx.cues.cancel(cue_id);
        self.close_cue_id = None;
      }
    }
  }

  fn search_lift(&mut self, ctx: &SystemContext) {
    if !self.ramp_lifted {
      self.lift_up(ctx.into());
      ctx.cues.start(SearchLowerDue, SEARCH_LIFT_TIME.once());
    }
    ctx.routines.complete_step::<BallSearching>();
  }

  /// Someone asked for the ramp, and the turn is active
  fn is_wanted_up(&self, ctx: &SystemContext) -> bool {
    !self.requests.is_empty() && ctx.states.is(TurnPhase::Active)
  }
}

impl System for LiftRampSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
  }

  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    if routine.is::<BallSearching>() {
      self.search_lift(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CloseRamp>() {
      log::info!("Lift ramp: Max time expired");
      self.lift_down(ctx.into());
    } else if let Some(changed) = event.downcast_ref::<StateChanged<TurnPhase>>() {
      // The ramp is only up during an active turn. Whoever still wants it up gets it back on the next.
      if changed.to == TurnPhase::Active {
        if !self.requests.is_empty() {
          self.lift_up(ctx.into());
        }
      } else if changed.from == TurnPhase::Active {
        self.lift_down(ctx.into());
      }
    } else if event.is::<SearchLowerDue>() && !self.is_wanted_up(ctx) {
      self.lift_down(ctx.into());
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == RAMP_OPTO.name
      && ctx.states.is(TurnPhase::Active)
    {
      log::info!("Lift ramp hit");
      ctx.emit(LiftRampHit);
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.lift_down(ctx.into());
  }
}

#[derive(serde::Serialize, Event)]
struct CloseRamp;

#[derive(serde::Serialize, Event)]
struct SearchLowerDue;

#[derive(serde::Serialize, Event)]
pub struct LiftRampHit;

#[derive(serde::Serialize, Event)]
pub struct LiftRampUp;

#[derive(serde::Serialize, Event)]
pub struct LiftRampDown;
