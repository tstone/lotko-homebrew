use std::time::Duration;

use frontbox::prelude::*;


pub mod switches {
  // Cabinet
  pub const START_BUTTON: &str = "start_button";
  pub const ACTION_BUTTON: &str = "action_button";
  pub const TILT_BOB: &str = "tilt_bob";
  pub const COIN_DOOR: &str = "coin_door";
  pub const LEFT_FLIPPER1: &str = "left_flipper1";
  pub const LEFT_FLIPPER2: &str = "left_flipper2";
  pub const RIGHT_FLIPPER1: &str = "right_flipper1";
  pub const RIGHT_FLIPPER2: &str = "right_flipper2";
  pub const DOOR_MENU_BLACK: &str = "door_menu_black";
  pub const DOOR_MENU_RED_R: &str = "door_menu_red_r";
  pub const DOOR_MENU_RED_L: &str = "door_menu_red_l";
  pub const DOOR_MENU_GREEN: &str = "door_menu_green";
  // Lower Thirds
  pub const OUTLANE_LEFT: &str = "outlane_left";
  pub const OUTLANE_RIGHT: &str = "outlane_right";
  pub const INLANE_LEFT: &str = "inlane_left";
  pub const INLANE_RIGHT: &str = "inlane_right";
  pub const SLINGSHOT_LEFT: &str = "slingshot_left";
  pub const SLINGSHOT_RIGHT: &str = "slingshot_right";
  pub const FLIPPER_MAIN_LEFT_EOS: &str = "main_left_flipper_eos";
  pub const FLIPPER_MAIN_RIGHT_EOS: &str = "main_right_flipper_eos";
  pub const TROUGH_POS1: &str = "trough_pos1";
  pub const TROUGH_POS2: &str = "trough_pos2";
  pub const TROUGH_POS3: &str = "trough_pos3";
  pub const TROUGH_POS4: &str = "trough_pos4";
  pub const TROUGH_POS5: &str = "trough_pos5";
  pub const TROUGH_POS6: &str = "trough_pos6";
}

pub mod drivers {
  // lower thirds
  pub const START_BUTTON: &str = "start_button";
  pub const SLINGSHOT_LEFT: &str = "slingshot_left";
  pub const SLINGSHOT_RIGHT: &str = "slingshot_right";
  pub const FLIPPER_MAIN_LEFT: &str = "main_left_flipper";
  pub const FLIPPER_MAIN_RIGHT: &str = "main_right_flipper";
  pub const FLIPPER_MAIN_HOLD_LEFT: &str = "main_left_flipper_hold";
  pub const FLIPPER_MAIN_HOLD_RIGHT: &str = "main_right_flipper_hold";
  pub const TROUGH_EJECT: &str = "trough_eject";
  pub const AUTO_PLUNGER: &str = "auto_plunger";
  // midfield
  pub const POP_RIGHT: &str = "pop_right";
  pub const DROP_TARGET_RIGHT: &str = "drop_target_right";
  pub const FLIPPER_UPPER_LEFT: &str = "flipper_upper_left";
  pub const FLIPPER_UPPER_HOLD_LEFT: &str = "flipper_upper_hold_left";
  // upper playfield
  pub const POP_LEFT: &str = "pop_left";
  pub const DROP_TARGET_LEFT: &str = "drop_target_left";
  pub const FLIPPER_UPPER_RIGHT: &str = "flipper_upper_right";
  pub const FLIPPER_UPPER_HOLD_RIGHT: &str = "flipper_upper_hold_right";
}

pub fn io_network() -> IoNetworkSpec {
  let mut io_network = IoNetworkSpec::new();

  io_network.add_board(
    FastIoBoards::cabinet()
      .with_switch(11, switches::COIN_DOOR)
      .with_switch(12, switches::ACTION_BUTTON)
      .with_switch(13, switches::START_BUTTON)
      .with_switch(15, switches::LEFT_FLIPPER1)
      .with_switch(14, switches::LEFT_FLIPPER2)
      .with_switch(17, switches::DOOR_MENU_GREEN)
      .with_switch(18, switches::DOOR_MENU_RED_L)
      .with_switch(19, switches::DOOR_MENU_RED_R)
      .with_switch(20, switches::DOOR_MENU_BLACK)
      .with_switch(21, switches::TILT_BOB)
      .with_switch(22, switches::RIGHT_FLIPPER1)
      .with_switch(23, switches::RIGHT_FLIPPER2)
      .with_driver_cfg(2, drivers::START_BUTTON, PulseHoldMode {
        initial_pwm_length: Duration::ZERO,
        secondary_pwm_power: Power::FULL,
        ..Default::default()
      })
  );

  io_network.add_board(
    FastIoBoards::io_3208()
      // TODO: left outlane switch not responding, need to investigate
      .with_switch(27, switches::INLANE_LEFT)
      .with_switch(31, switches::SLINGSHOT_LEFT)
      .with_switch(30, switches::FLIPPER_MAIN_LEFT_EOS)
      .with_switch(29, switches::FLIPPER_MAIN_RIGHT_EOS)
      .with_switch(28, switches::SLINGSHOT_RIGHT)
      .with_switch(24, switches::INLANE_RIGHT)
      // TODO: right outlane switch not responding, need to investigate
      .with_switch(25, switches::TROUGH_POS6)
      .with_switch(22, switches::TROUGH_POS5)
      .with_switch(21, switches::TROUGH_POS4)
      .with_switch(19, switches::TROUGH_POS3)
      .with_switch(20, switches::TROUGH_POS2)
      .with_switch(18, switches::TROUGH_POS1)

      .with_driver_cfg(0, drivers::SLINGSHOT_LEFT, PulseMode {
        initial_pwm_power: Power::percent(80),
        switch: Some(51), // TODO: this should be switches::SLINGSHOT_LEFT
        ..Default::default()
      })
  );

  io_network
}
