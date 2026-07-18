use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("drop")
    .tag(Playfield)
    .mode(PulseMode {
      trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
      initial_pwm_length: HardwareValue::config(
        "Drop Target Reset Duration",
        "Amount of time fire the coil to reset the bank",
        Duration::from_millis(35),
        Ranges::duration(5, 100)
      ),
      initial_pwm_power: HardwareValue::config(
        "Drop Target Return Duration",
        "Power to use when resetting the bank",
        Power::FULL,
        Ranges::full_power()
      ),
      ..Default::default()
    });

  pub TARGET1: SwitchDefinition = SwitchDefinition::new("drop_target1");
  pub TARGET2: SwitchDefinition = SwitchDefinition::new("drop_target2");
  pub TARGET3: SwitchDefinition = SwitchDefinition::new("drop_target3");
  pub PADDLE_SWITCH: SwitchDefinition = SwitchDefinition::new("drop_paddle");

  pub TARGET1_LEDS: LedDefinition = LedDefinition::strip("target1", 4);
  pub TARGET2_LEDS: LedDefinition = LedDefinition::strip("target2", 4);
  pub TARGET3_LEDS: LedDefinition = LedDefinition::strip("target3", 4);

  pub PADDLE_LED: LedDefinition = LedDefinition::single("paddle");
}
