use crate::hardware::cabinet;
use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub EOS_SWITCH: SwitchDefinition = SwitchDefinition::new("u_flipper_eos");

  pub MAIN_COIL: DriverDefinition = DriverDefinition::new("u_flipper_main")
    .mode(
      DriverMode::flipper_main_direct(cabinet::RIGHT_FLIPPER_SWITCH2.name, EOS_SWITCH.name)
        .initial_pwm_power(HardwareValue::config("Initial Power", "", Power::HALF, Ranges::full_power()).key("Upper Flipper Initial Power"))
        .secondary_pwm_power(HardwareValue::config("Secondary Power", "", Power::FULL, Ranges::full_power()).key("Upper Flipper Secondary Power"))
        .build(),
    )
    .tag(Playfield);

  pub HOLD_COIL: DriverDefinition = DriverDefinition::new("u_flipper_hold")
    .mode(DriverMode::flipper_hold_direct(cabinet::RIGHT_FLIPPER_SWITCH2.name).build())
    .tag(Playfield);
}
