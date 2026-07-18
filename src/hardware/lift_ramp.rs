use frontbox::prelude::DriverTriggerMode::VirtualSwitchTrue;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub RAMP_COIL: DriverDefinition = DriverDefinition::new("lift_ramp")
    .tag(Playfield)
    .mode(PulseHoldCancelMode {
      trigger_mode: DriverTriggerDualMode::VirtualFlip_FlopSwitchTrue(SCOOP_OPTO.name),
      initial_pwm_length: HardwareValue::config(
        "Lift Ramp Kick Duration",
        "Amount of time to initially kick the lift ramp open",
        Duration::from_millis(35),
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
        Power::percent(20),
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

  pub SCOOP_OPTO: SwitchDefinition = SwitchDefinition::new("eject_opto");
  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_opto");

  pub BOLT_LED: LedDefinition = LedDefinition::single("lift_ramp_bolt")
    .tag(Playfield)
    .tag(Bolt)
    .tag(Lane);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("lift_ramp_lane", 7)
    .tag(Playfield)
    .tag(Hex)
    .tag(Lane);
}
