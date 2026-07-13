use crate::hardware::cabinet;
use frontbox::prelude::*;

hardware_defs! {
  pub EOS_SWITCH: SwitchDefinition = SwitchDefinition::new("u_flipper_eos");

  pub MAIN_COIL: DriverDefinition = DriverDefinition::new("u_flipper_main")
  .mode(FlipperMainDirectMode {
      button_switch: cabinet::RIGHT_FLIPPER_SWITCH2.name,
      eos_switch: EOS_SWITCH.name,
      ..Default::default()
    });

  pub HOLD_COIL: DriverDefinition = DriverDefinition::new("u_flipper_hold")
    .mode(FlipperHoldDirectMode {
      button_switch: cabinet::RIGHT_FLIPPER_SWITCH2.name,
      ..Default::default()
    });
}
