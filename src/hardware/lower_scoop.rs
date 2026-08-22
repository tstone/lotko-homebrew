use std::sync::LazyLock;

use frontbox::animation::Curve;
use frontbox::prelude::DriverTriggerMode::*;
use frontbox::prelude::*;
use frontbox::tags::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::GameManagementExt;

use crate::hardware::arc_ramp::ArcRampSubwayHit;
use crate::hardware::more_tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lower_scoop")
    .tag(Playfield)
    .mode(PulseKickMode {
      trigger_mode: VirtualSwitchTrue,
      initial_pwm_length: HardwareValue::config(
        "Lower Scoop Touch Time",
        "Duration by which the eject plunger is brought into contact with the ball, before full eject",
        Duration::from_millis(7),
        Ranges::duration(0, 100),
      ),
      initial_pwm_power: HardwareValue::fixed(
        Power::THREE_QUARTERS,
      ),
      secondary_pwm_power: HardwareValue::Fixed(Power::ZERO),
      secondary_pwm_length: HardwareValue::Fixed(Duration::ZERO),
      kick_length: HardwareValue::config(
        "Lower Scoop Eject Time",
        "Duration that the plunger exert full power onto the ball (kick)",
        Duration::from_millis(35),
        Ranges::duration(10, 300),
      ),
      ..Default::default()
    });

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lower_scoop")
    .inverted()
    .debounce_close(Duration::from_millis(100))
    .tag(Playfield);

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt1")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield);

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt2")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield);
}

pub static BOLTS_Q: LazyLock<HardwareQuery> = LazyLock::new(|| {
  Q::names(vec![
    LEFT_BOLT.names()[0].clone(),
    RIGHT_BOLT.names()[0].clone(),
  ])
});

// -- System --

pub const LOWER_SCOOP_EJECT_SND: &'static str = "lower_scoop_eject";

#[derive(Clone)]
pub struct LowerScoopSystem {
  eject_program: LedProgram1d,
  subway_entry: bool,
  handle: SystemHandle,
}

impl LowerScoopSystem {
  pub fn new() -> Self {
    let eject_program = LedProgram1d::tween(
      &*BOLTS_Q,
      Duration::from_millis(750 / 4),
      Curve::Steps(2),
      Cycle::Times(4),
      vec![
        ColorSequence::exact(vec![Rgba::white(), Rgba::default()]),
        ColorSequence::exact(vec![Rgba::default(), Rgba::white()]),
      ],
    );

    Self {
      eject_program,
      subway_entry: false,
      handle: SystemHandle::default(),
    }
  }

  pub fn eject(&mut self, ctx: &ServiceContext) {
    let ctx = ctx.for_system(self.handle);
    ctx.play_sfx(LOWER_SCOOP_EJECT_SND);
    ctx.cue(EjectLowerScoop, Cue::Once(Duration::from_millis(750)));
    self.eject_program.reset();
    self.eject_program.play();
  }

  fn complete_eject(&mut self, ctx: &SystemContext) {
    ctx.activate_driver(COIL.name, ActivationMode::Tap);
    log::info!("Stopping LEDs after complete eject");
    self.eject_program.stop(ctx);
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
    self.eject(ctx.into());
    ctx.emit(event);
  }
}

impl System for LowerScoopSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = *ctx.current_handle();

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
    } else if event.is::<EjectLowerScoop>() {
      self.complete_eject(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.eject_program.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
pub struct LowerScoopBallEnter {
  pub subway_entry: bool,
}

#[derive(serde::Serialize, Event)]
struct EjectLowerScoop;
