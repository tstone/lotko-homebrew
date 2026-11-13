use frontbox_core::prelude::DriverTriggerMode::VirtualSwitchTrue;
use frontbox_core::prelude::*;
use frontbox_core::tags::*;
use frontbox_pinball::{BallManagementExt, BallSearching};

use crate::hardware::planes;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lift_ramp_eject")
    .tag(Playfield)
    .mode(
      DriverMode::pulse_kick()
        .trigger_mode(VirtualSwitchTrue)
        .initial_pwm_length(HardwareValue::config(
          "Rear Scoop Touch Time",
          "Duration by which the eject plunger is brought into contact with the ball, before full eject",
          Duration::from_millis(2),
          Ranges::duration(0, 100),
        ))
        .initial_pwm_power(HardwareValue::fixed(Power::HALF))
        .secondary_pwm_power(HardwareValue::Fixed(Power::ZERO))
        .secondary_pwm_length(HardwareValue::Fixed(Duration::ZERO))
        .kick_length(HardwareValue::config(
          "Rear Scoop Eject Time",
          "Duration that the plunger exert full power onto the ball (kick)",
          Duration::from_millis(12),
          Ranges::duration(10, 100),
        ))
        .build(),
    )
    .location(Vec3::new(17.424, 1.428, -2.5).relative_to(&planes::PLAYFIELD));

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_scoop_opto")
    .inverted()
    .debounce_close(Duration::from_millis(10))
    .tag(Playfield)
    .location(Vec3::new(17.424, 1.428, -1.5).relative_to(&planes::PLAYFIELD));
}

/// Ejects any ball that lands in the scoop behind the lift ramp, at any time (in a turn, after a tilt, at boot), and
/// kicks again if the ball is still there afterwards. Kicks during a ball search too.
#[derive(Clone)]
pub struct RearScoopSystem {
  handle: SystemHandle,
}

impl RearScoopSystem {
  pub fn new() -> Self {
    Self {
      handle: SystemHandle::default(),
    }
  }

  pub fn eject(&mut self, ctx: &ServiceContext) {
    let ctx = ctx.for_system(self.handle);
    ctx.drivers.activate(COIL.name, ActivationMode::Tap);
    ctx
      .cues
      .start(VerifyVacancy, Duration::from_millis(1200).once());
  }

  fn is_occupied(&self, ctx: &SystemContext) -> bool {
    ctx.switches.is_closed(OPTO.name).unwrap_or(false)
  }
}

impl System for RearScoopSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();

    if self.is_occupied(ctx) {
      log::debug!("Rear scoop is occupied. Ejecting.");
      ctx.set_balls_held(1);
      self.eject(ctx.into());
    }
  }

  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    if routine.is::<BallSearching>() {
      ctx.drivers.activate(COIL.name, ActivationMode::Tap);
      ctx.routines.complete_step::<BallSearching>();
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == OPTO.name
    {
      log::info!("Ball entered rear scoop");
      ctx.set_balls_held(1);
      ctx.emit(RearScoopBallEnter);
      self.eject(ctx.into());
    } else if let Some(event) = event.downcast_ref::<SwitchOpened>()
      && event.switch.name == OPTO.name
    {
      ctx.set_balls_held(0);
    } else if event.is::<VerifyVacancy>() && self.is_occupied(ctx) {
      self.eject(ctx.into());
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct RearScoopBallEnter;

#[derive(serde::Serialize, Event)]
struct VerifyVacancy;
