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
  pub const PLUNGE_LANE: &str = "plunge_lane";
  // midfield
  pub const POP_RIGHT: &str = "pop_right";
  pub const DROP_TARGET_RIGHT1: &str = "drop_target_right1";
  pub const DROP_TARGET_RIGHT2: &str = "drop_target_right2";
  pub const DROP_TARGET_RIGHT3: &str = "drop_target_right3";
  // upper playfield
  pub const POP_LEFT: &str = "pop_left";
  pub const DROP_TARGET_LEFT1: &str = "drop_target_left1";
  pub const DROP_TARGET_LEFT2: &str = "drop_target_left2";
  pub const DROP_TARGET_LEFT3: &str = "drop_target_left3";
}

pub mod switch_groups {
  pub const BALL_IN_PLAY: &str = "ball_in_play";
}

pub mod driver_groups {
  pub const PLAYFIELD: &str = "playfield";
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

pub fn io_network() -> IoNetwork {
  let mut io_network = IoNetworkBuilder::new();

  io_network.add_board(
    FastIoBoards::cabinet()
      .with_switch(switches::COIN_DOOR, 11)
      .with_switch(switches::ACTION_BUTTON, 12)
      .with_switch(switches::START_BUTTON, 13)
      .with_switch(switches::LEFT_FLIPPER1, 15)
      .with_switch(switches::LEFT_FLIPPER2, 14)
      .with_switch(switches::DOOR_MENU_GREEN, 17)
      .with_switch(switches::DOOR_MENU_RED_L, 18)
      .with_switch(switches::DOOR_MENU_RED_R, 19)
      .with_switch(switches::DOOR_MENU_BLACK, 20)
      .with_switch(switches::TILT_BOB, 21)
      .with_switch(switches::RIGHT_FLIPPER1, 22)
      .with_switch(switches::RIGHT_FLIPPER2, 23)
      .with_driver( drivers::START_BUTTON, 2)
  );

  io_network.add_board(
    FastIoBoards::io_3208()
      // TODO: left outlane switch not responding, need to investigate
      .with_switch(switches::INLANE_LEFT, 27)
      .with_switch(switches::SLINGSHOT_LEFT, 31)
      .with_switch(switches::FLIPPER_MAIN_LEFT_EOS, 30)
      .with_switch(switches::FLIPPER_MAIN_RIGHT_EOS, 29)
      .with_switch(switches::SLINGSHOT_RIGHT, 28)
      .with_switch(switches::INLANE_RIGHT, 24)
      // TODO: right outlane switch not responding, need to investigate
      .with_switch(switches::TROUGH_POS6, 25)
      .with_switch(switches::TROUGH_POS5, 22)
      .with_switch(switches::TROUGH_POS4, 21)
      .with_switch(switches::TROUGH_POS3, 19)
      .with_switch(switches::TROUGH_POS2, 20)
      .with_switch(switches::TROUGH_POS1, 18)
      .with_switch(switches::PLUNGE_LANE, 16) // TODO: temorary, this needs to be physicall hooked up

      // Flippers
      .with_driver_cfg( drivers::FLIPPER_MAIN_LEFT, 1, FlipperMainDirectMode {
        button_switch: switches::LEFT_FLIPPER1,
        eos_switch: switches::FLIPPER_MAIN_LEFT_EOS,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_MAIN_HOLD_LEFT, 2, FlipperHoldDirectMode {
        button_switch: switches::LEFT_FLIPPER1,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_MAIN_RIGHT, 5, FlipperMainDirectMode {
        button_switch: switches::RIGHT_FLIPPER1,
        eos_switch: switches::FLIPPER_MAIN_RIGHT_EOS,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_MAIN_HOLD_RIGHT, 6, FlipperHoldDirectMode {
        button_switch: switches::RIGHT_FLIPPER1,
        ..Default::default()
      })
      // Plunge
      .with_driver(drivers::TROUGH_EJECT, 3)
      .with_driver(drivers::AUTO_PLUNGER, 4)
      // Slings
      .with_driver_cfg( drivers::SLINGSHOT_LEFT, 0, PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::SLINGSHOT_LEFT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      })
      .with_driver_cfg( drivers::SLINGSHOT_RIGHT, 7, PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::SLINGSHOT_RIGHT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      })
  );

  io_network.add_board(
    FastIoBoards::io_1616()
      .with_switch(switches::POP_LEFT, 1)
      .with_switch(switches::POP_RIGHT, 4)
      .with_switch(switches::DROP_TARGET_RIGHT1, 5)
      .with_switch(switches::DROP_TARGET_RIGHT2, 6)
      .with_switch(switches::DROP_TARGET_RIGHT3, 7)
      .with_driver_cfg( drivers::POP_RIGHT, 0, PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::POP_RIGHT),
        initial_pwm_power: Power::FULL,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_UPPER_LEFT, 1, FlipperMainDirectMode {
        button_switch: switches::LEFT_FLIPPER2,
        eos_switch: switches::FLIPPER_MAIN_LEFT_EOS,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_UPPER_HOLD_LEFT, 2, FlipperHoldDirectMode {
        button_switch: switches::LEFT_FLIPPER2,
        ..Default::default()
      })
      .with_driver_cfg( drivers::DROP_TARGET_RIGHT, 3, PulseMode {
        trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
        initial_pwm_power: Power::FULL,
        ..Default::default()
      })
      .with_driver_cfg( drivers::DROP_TARGET_LEFT, 4, PulseMode {
        trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
        initial_pwm_power: Power::FULL,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_UPPER_RIGHT, 5, FlipperMainDirectMode {
        button_switch: switches::RIGHT_FLIPPER2,
        eos_switch: switches::FLIPPER_MAIN_RIGHT_EOS,
        ..Default::default()
      })
      .with_driver_cfg( drivers::FLIPPER_UPPER_HOLD_RIGHT, 6, FlipperHoldDirectMode {
        button_switch: switches::RIGHT_FLIPPER2,
        ..Default::default()
      })
      .with_driver_cfg(drivers::POP_LEFT, 7, PulseMode {
        initial_pwm_power: Power::FULL,
        ..Default::default()
      })
  );

  io_network.add_switch_group(switch_groups::BALL_IN_PLAY, vec![
    switches::OUTLANE_LEFT,
    switches::INLANE_LEFT,
    switches::INLANE_RIGHT,
    switches::OUTLANE_RIGHT,
    switches::SLINGSHOT_LEFT,
    switches::SLINGSHOT_RIGHT,
    switches::POP_LEFT,
    switches::POP_RIGHT,
  ]);

  io_network.add_driver_group(driver_groups::PLAYFIELD, vec![
    drivers::SLINGSHOT_LEFT,
    drivers::SLINGSHOT_RIGHT,
    drivers::FLIPPER_MAIN_LEFT,
    drivers::FLIPPER_MAIN_HOLD_LEFT,
    drivers::FLIPPER_MAIN_RIGHT,
    drivers::FLIPPER_MAIN_HOLD_RIGHT,
    drivers::TROUGH_EJECT,
    drivers::AUTO_PLUNGER,
    drivers::POP_RIGHT,
    drivers::DROP_TARGET_RIGHT,
    drivers::FLIPPER_UPPER_LEFT,
    drivers::FLIPPER_UPPER_HOLD_LEFT,
    drivers::DROP_TARGET_LEFT,
    drivers::FLIPPER_UPPER_RIGHT,
    drivers::FLIPPER_UPPER_HOLD_RIGHT,
  ]);

  io_network.build()
}
