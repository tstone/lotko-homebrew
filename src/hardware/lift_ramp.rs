use frontbox::prelude::ActivationMode::VirtualSwitchOn;
use frontbox::prelude::DeactivationMode::VirtualSwitchOff;
use frontbox::prelude::DriverTriggerMode::VirtualSwitchTrue;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::ScoopBallEntered;
use crate::hardware::more_tags::*;

hardware_defs! {
  pub RAMP_COIL: DriverDefinition = DriverDefinition::new("lift_ramp")
    .tag(Playfield)
    .mode(PulseHoldCancelMode {
      trigger_mode: DriverTriggerDualMode::VirtualFlip_FlopSwitchTrue(SCOOP_OPTO.name),
      initial_pwm_length: HardwareValue::config(
        "Lift Ramp Kick Duration",
        "Amount of time to initially kick the lift ramp open",
        Duration::from_millis(15),
        Ranges::duration(5, 100)
      ),
      initial_pwm_power: HardwareValue::config(
        "Lift Ramp Kick Power",
        "Power to use when initially kicking the lift ramp open",
        Power::FULL,
        Ranges::full_power()
      ),
      secondary_pwm_power: HardwareValue::config(
        "Lift Ramp Hold Power",
        "Amount of power to keep ramp held up",
        Power::percent(15),
        Ranges::full_power()
      ),
      ..Default::default()
    });

  pub EJECT_COIL: DriverDefinition = DriverDefinition::new("lift_ramp_eject")
    .tag(Playfield)
    .mode(PulseKickMode {
      trigger_mode: VirtualSwitchTrue,
      initial_pwm_length: HardwareValue::config(
        "Rear Scoop Touch Time",
        "Duration by which the eject plunger is brought into contact with the ball, before full eject",
        Duration::from_millis(7),
        Ranges::duration(0, 100),
      ),
      initial_pwm_power: HardwareValue::fixed(
        Power::percent(75),
      ),
      secondary_pwm_power: HardwareValue::Fixed(Power::ZERO),
      secondary_pwm_length: HardwareValue::Fixed(Duration::ZERO),
      kick_length: HardwareValue::config(
        "Rear Scoop Eject Time",
        "Duration that the plunger exert full power onto the ball (kick)",
        Duration::from_millis(35),
        Ranges::duration(10, 300),
      ),
      ..Default::default()
    });

  pub SCOOP_OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_scoop_opto")
    .inverted()
    .debounce_close(Duration::from_millis(100))
    .tag(Playfield);

  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_opto")
    .inverted()
    .tag(Playfield);

  pub BOLT_LED: LedDefinition = LedDefinition::single("lift_ramp_bolt")
    .tag(Playfield)
    .tag(Bolt)
    .tag(Lane);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("lift_ramp_lane", 7)
    .tag(Playfield)
    .tag(Hex)
    .tag(Lane);
}

pub fn hex_center_led_q() -> HardwareQuery {
  HEX_LEDS.child(6).unwrap().q()
}

pub fn hex_line_leds_q() -> HardwareQuery {
  Q::names(vec![
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
  ])
}

pub fn hex_circle_leds_q() -> HardwareQuery {
  // TODO: verify order
  Q::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
}

pub const SCOOP_NAME: &'static str = "lift_ramp";

#[derive(Clone)]
pub struct LiftRampSystem {
  ramp_lifted: bool,
  close_cue_id: Option<u64>,
  ball_present: bool,
}

impl LiftRampSystem {
  pub fn new() -> Self {
    Self {
      ramp_lifted: false,
      close_cue_id: None,
      ball_present: false,
    }
  }

  pub fn is_lifted(&self) -> bool {
    self.ramp_lifted
  }

  pub fn lift_up(&mut self, ctx: &Context, max_duration: Duration) {
    if !self.ramp_lifted {
      log::debug!("Lifting ramp up.");
      self.ramp_lifted = true;
      ctx.activate_driver(RAMP_COIL.name, VirtualSwitchOn);
      ctx.emit(LiftRampUp);
      self.close_cue_id = Some(ctx.cue(CloseRamp, Cue::Once(max_duration)));
    }
  }

  pub fn lift_down(&mut self, ctx: &Context) {
    if self.ramp_lifted {
      log::debug!("Lifting ramp down.");
      self.ramp_lifted = false;
      ctx.deactivate_driver(RAMP_COIL.name, VirtualSwitchOff);
      ctx.emit(LiftRampDown);
      if let Some(cue_id) = self.close_cue_id.take() {
        ctx.cancel_cue(cue_id);
        self.close_cue_id = None;
      }
    }
  }

  pub fn eject(&mut self, ctx: &Context) {
    ctx.activate_driver(EJECT_COIL.name, ActivationMode::Tap);
  }
}

impl System for LiftRampSystem {
  fn on_spawn(&mut self, ctx: &Context) {
    self.ball_present = ctx.switches.is_closed(SCOOP_OPTO.name).unwrap_or(false);
    if self.ball_present {
      self.eject(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SCOOP_OPTO.name
    {
      ctx.emit(ScoopBallEntered(SCOOP_NAME));
    } else if event.is::<CloseRamp>() {
      self.lift_down(ctx);
    }
  }
}

struct CloseRamp;

pub struct LiftRampUp;
pub struct LiftRampDown;
