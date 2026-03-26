use frontbox::prelude::*;
use frontbox::tags::*;

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
      .with(switch(11).named(switches::COIN_DOOR).tagged(CoinDoor))
      .with(
        switch(12)
          .named(switches::ACTION_BUTTON)
          .tagged(ActionButton),
      )
      .with(switch(13).named(switches::START_BUTTON).tagged(StartButton))
      .with(switch(15).named(switches::LEFT_FLIPPER1))
      .with(switch(14).named(switches::LEFT_FLIPPER2))
      .with(switch(17).named(switches::DOOR_MENU_GREEN))
      .with(switch(18).named(switches::DOOR_MENU_RED_L))
      .with(switch(19).named(switches::DOOR_MENU_RED_R))
      .with(switch(20).named(switches::DOOR_MENU_BLACK))
      .with(switch(21).named(switches::TILT_BOB).tagged(Tilt))
      .with(switch(22).named(switches::RIGHT_FLIPPER1))
      .with(switch(23).named(switches::RIGHT_FLIPPER2))
      .with(driver(2).named(drivers::START_BUTTON).tagged(StartButton)),
  );

  io_network.add_board(
    FastIoBoards::io_3208()
      // TODO: left outlane switch not responding, need to investigate
      .with(switch(27).named(switches::INLANE_LEFT).tagged(Playfield))
      .with(switch(31).named(switches::SLINGSHOT_LEFT).tagged(Playfield))
      .with(
        switch(30)
          .named(switches::FLIPPER_MAIN_LEFT_EOS)
          .tagged(Playfield),
      )
      .with(
        switch(29)
          .named(switches::FLIPPER_MAIN_RIGHT_EOS)
          .tagged(Playfield),
      )
      .with(
        switch(28)
          .named(switches::SLINGSHOT_RIGHT)
          .tagged(Playfield),
      )
      .with(switch(24).named(switches::INLANE_RIGHT).tagged(Playfield))
      // TODO: right outlane switch not responding, need to investigate
      .with(switch(25).named(switches::TROUGH_POS6))
      .with(switch(22).named(switches::TROUGH_POS5))
      .with(switch(21).named(switches::TROUGH_POS4))
      .with(switch(19).named(switches::TROUGH_POS3))
      .with(switch(20).named(switches::TROUGH_POS2))
      .with(switch(18).named(switches::TROUGH_POS1))
      .with(
        switch(16)
          .named(switches::PLUNGE_LANE)
          .tagged(AutoPlungerSwitch),
      ) // TODO: temporary, this needs to be physically hooked up
      // Flippers
      .with(
        driver(1)
          .named(drivers::FLIPPER_MAIN_LEFT)
          .mode(FlipperMainDirectMode {
            button_switch: switches::LEFT_FLIPPER1,
            eos_switch: switches::FLIPPER_MAIN_LEFT_EOS,
            ..Default::default()
          }),
      )
      .with(
        driver(2)
          .named(drivers::FLIPPER_MAIN_HOLD_LEFT)
          .mode(FlipperHoldDirectMode {
            button_switch: switches::LEFT_FLIPPER1,
            ..Default::default()
          }),
      )
      .with(
        driver(2)
          .named(drivers::FLIPPER_MAIN_HOLD_LEFT)
          .mode(FlipperHoldDirectMode {
            button_switch: switches::LEFT_FLIPPER1,
            ..Default::default()
          }),
      )
      .with(
        driver(5)
          .named(drivers::FLIPPER_MAIN_RIGHT)
          .mode(FlipperMainDirectMode {
            button_switch: switches::RIGHT_FLIPPER1,
            eos_switch: switches::FLIPPER_MAIN_RIGHT_EOS,
            ..Default::default()
          }),
      )
      .with(
        driver(6)
          .named(drivers::FLIPPER_MAIN_HOLD_RIGHT)
          .mode(FlipperHoldDirectMode {
            button_switch: switches::RIGHT_FLIPPER1,
            ..Default::default()
          }),
      )
      .with(driver(3).named(drivers::TROUGH_EJECT).tagged(TroughCoil))
      .with(
        driver(4)
          .named(drivers::AUTO_PLUNGER)
          .tagged(AutoPlungerCoil),
      )
      // Slings
      .with(driver(0).named(drivers::SLINGSHOT_LEFT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::SLINGSHOT_LEFT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      }))
      .with(driver(7).named(drivers::SLINGSHOT_RIGHT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::SLINGSHOT_RIGHT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      })),
  );

  io_network.add_board(
    FastIoBoards::io_1616()
      .with(switch(1).named(switches::POP_LEFT).tagged(Playfield))
      .with(switch(4).named(switches::POP_RIGHT).tagged(Playfield))
      .with(
        switch(5)
          .named(switches::DROP_TARGET_RIGHT1)
          .tagged(Playfield),
      )
      .with(
        switch(6)
          .named(switches::DROP_TARGET_RIGHT2)
          .tagged(Playfield),
      )
      .with(
        switch(7)
          .named(switches::DROP_TARGET_RIGHT3)
          .tagged(Playfield),
      )
      .with(driver(0).named(drivers::POP_RIGHT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::POP_RIGHT),
        initial_pwm_power: Power::FULL,
        ..Default::default()
      }))
      .with(
        driver(1)
          .named(drivers::FLIPPER_UPPER_LEFT)
          .mode(FlipperMainDirectMode {
            button_switch: switches::LEFT_FLIPPER2,
            eos_switch: switches::FLIPPER_MAIN_LEFT_EOS,
            ..Default::default()
          }),
      )
      .with(
        driver(2)
          .named(drivers::FLIPPER_UPPER_HOLD_LEFT)
          .mode(FlipperHoldDirectMode {
            button_switch: switches::LEFT_FLIPPER2,
            ..Default::default()
          }),
      )
      .with(driver(3).named(drivers::DROP_TARGET_RIGHT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
        initial_pwm_power: Power::FULL,
        ..Default::default()
      }))
      .with(driver(4).named(drivers::DROP_TARGET_LEFT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
        initial_pwm_power: Power::FULL,
        ..Default::default()
      }))
      .with(
        driver(5)
          .named(drivers::FLIPPER_UPPER_RIGHT)
          .mode(FlipperMainDirectMode {
            button_switch: switches::RIGHT_FLIPPER2,
            eos_switch: switches::FLIPPER_MAIN_RIGHT_EOS,
            ..Default::default()
          }),
      )
      .with(
        driver(6)
          .named(drivers::FLIPPER_UPPER_HOLD_RIGHT)
          .mode(FlipperHoldDirectMode {
            button_switch: switches::RIGHT_FLIPPER2,
            ..Default::default()
          }),
      )
      .with(driver(7).named(drivers::POP_LEFT).mode(PulseMode {
        initial_pwm_power: Power::FULL,
        ..Default::default()
      })),
  );

  io_network.build()
}
