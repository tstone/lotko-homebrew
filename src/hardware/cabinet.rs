use crate::hardware::planes;
use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub LEFT_FLIPPER_SWITCH1: SwitchDefinition = SwitchDefinition::new("l_flipper1");
  pub LEFT_FLIPPER_SWITCH2: SwitchDefinition = SwitchDefinition::new("l_flipper2");

  pub RIGHT_FLIPPER_SWITCH1: SwitchDefinition = SwitchDefinition::new("r_flipper1");
  pub RIGHT_FLIPPER_SWITCH2: SwitchDefinition = SwitchDefinition::new("r_flipper2");

  pub TILT_BOB_SWITCH: SwitchDefinition = SwitchDefinition::new("tilt");
}

pub mod action_button {
  use super::*;
  pub const NAME: &'static str = "action";

  hardware_defs! {
    pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
      .tag(Cabinet)
      .location(Vec3::new(11.0, 0.0, 0.0).relative_to(&planes::CABINET_FRONT));

    pub LED: LedDefinition = LedDefinition::single(NAME)
      .tag(Cabinet)
      .location(Vec3::new(11.0, 0.0, 0.0).relative_to(&planes::CABINET_FRONT));
  }
}

pub mod start_button {
  use super::*;
  use frontbox_turn_based::GameStartable;

  pub const NAME: &'static str = "start";

  hardware_defs! {
    pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
      .tag(Cabinet)
      .location(Vec3::new(2.25, 2.25, 0.0).relative_to(&planes::CABINET_FRONT));

    pub LAMP_DRIVER: DriverDefinition = GameStartable::lamp_driver(NAME)
      .tag(Cabinet)
      .location(Vec3::new(2.25, 2.25, 0.0).relative_to(&planes::CABINET_FRONT));
  }
}

pub mod coin_door {
  use super::*;
  hardware_defs! {
    pub LEFT_COIN_LED: LedDefinition = LedDefinition::single("l_coin_led")
      .tag(Cabinet);

    pub RIGHT_COIN_LED: LedDefinition = LedDefinition::single("l_coin_led")
      .tag(Cabinet);

    /// The switch that detects if the door is opened or closed
    pub OPEN_SWITCH: SwitchDefinition = SwitchDefinition::new("coin_door_open")
      .tag(Cabinet)
      .location(Vec3::new(5.419, 11.767, 0.0).relative_to(&planes::CABINET_FRONT));

    pub MENU_BLACK_SWITCH: SwitchDefinition = SwitchDefinition::new("menu_black");
    pub MENU_RED_R_SWITCH: SwitchDefinition = SwitchDefinition::new("menu_red_r");
    pub MENU_RED_L_SWITCH: SwitchDefinition = SwitchDefinition::new("menu_red_l");
    pub MENU_GREEN_SWITCH: SwitchDefinition = SwitchDefinition::new("menu_green");
  }
}
