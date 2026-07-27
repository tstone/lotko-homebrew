use frontbox::prelude::DriverTriggerMode::*;
use frontbox::prelude::color_sequence::Modification;
use frontbox::prelude::*;
use frontbox::tags::*;

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
        Power::percent(75),
      ),
      secondary_pwm_power: HardwareValue::Fixed(Power::ZERO),
      secondary_pwm_length: HardwareValue::Fixed(Duration::ZERO),
      kick_length: HardwareValue::config(
        "Lower Scoop Eject Time",
        "Duration that the plunger exert full power onto the ball (kick)",
        Duration::from_millis(40),
        Ranges::duration(10, 300),
      ),
      ..Default::default()
    });

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lower_scoop").inverted();

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt1")
    .tag(Bolt)
    .tag(Playfield);

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt2")
    .tag(Bolt)
    .tag(Playfield);
}

#[derive(Clone)]
pub struct LowerScoopSystem {
  eject_effect: LedEffect,
}

impl LowerScoopSystem {
  pub fn new() -> Self {
    let mut eject_effect = LedEffect::rotate(
      LEFT_BOLT.q().or(RIGHT_BOLT.q()),
      ColorSequence::exact(vec![Rgba::white(), Rgba::default()]).modify(Modification::rotated(0.0)),
      Duration::from_millis(332),
      RotationDirection::Clockwise,
    );
    eject_effect.stop();

    Self { eject_effect }
  }

  pub fn eject(&mut self, ctx: &Context) {
    // TODO: play sound
    ctx.cue(EjectLowerScoop, Cue::Once(Duration::from_millis(750)));
    self.eject_effect.play();
  }

  fn complete_eject(&mut self, ctx: &Context) {
    ctx.activate_driver(COIL.name, ActivationMode::Tap);
    self.eject_effect.stop_and_clear(ctx);
  }
}

impl System for LowerScoopSystem {
  fn on_spawn(&mut self, ctx: &Context) {
    // check if there is a ball at startup and eject if so
    if ctx.switches.is_closed(OPTO.name).unwrap_or(false) {
      log::debug!("Lower scoop is occupied. Ejecting.");
      self.complete_eject(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == OPTO.name
    {
      self.eject(ctx);
    } else if event.is::<EjectLowerScoop>() {
      self.complete_eject(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    self.eject_effect.apply(delta, ctx);
  }
}

struct EjectLowerScoop;
