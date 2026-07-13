use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::cabinet;
use crate::hardware::left_flipper;
use crate::hardware::left_inlane;
use crate::hardware::plunge_lane;
use crate::hardware::right_flipper;
use crate::hardware::right_inlane;
use crate::hardware::slingshots;
use crate::hardware::trough;
use crate::hardware::upper_flipper;

pub fn io_network() -> IoNetwork {
  let mut io_network = IoNetwork::new(vec![
    IoBoards::cabinet()
      // switches
      .wire_switch(11, &cabinet::coin_door::OPEN_SWITCH)
      .wire_switch(12, &cabinet::action_button::SWITCH)
      .wire_switch(13, &cabinet::start_button::SWITCH)
      .wire_switch(14, &cabinet::LEFT_FLIPPER_SWITCH2)
      .wire_switch(15, &cabinet::LEFT_FLIPPER_SWITCH1)
      // 16?
      .wire_switch(17, &cabinet::coin_door::MENU_GREEN_SWITCH)
      .wire_switch(18, &cabinet::coin_door::MENU_RED_L_SWITCH)
      .wire_switch(19, &cabinet::coin_door::MENU_RED_R_SWITCH)
      .wire_switch(20, &cabinet::coin_door::MENU_BLACK_SWITCH)
      .wire_switch(21, &cabinet::TILT_BOB_SWITCH)
      .wire_switch(22, &cabinet::RIGHT_FLIPPER_SWITCH1)
      .wire_switch(23, &cabinet::RIGHT_FLIPPER_SWITCH2)
      // drivers
      .wire_driver(2, &cabinet::start_button::LAMP_DRIVER),
    IoBoards::io_3208()
      // switches
      .wire_switch(18, &trough::SWITCH1)
      .wire_switch(19, &trough::SWITCH3)
      .wire_switch(20, &trough::SWITCH2)
      .wire_switch(21, &trough::SWITCH4)
      .wire_switch(22, &trough::SWITCH5)
      .wire_switch(24, &right_inlane::SWITCH)
      .wire_switch(25, &trough::SWITCH6)
      .wire_switch(27, &left_inlane::TARGET_SWITCH)
      .wire_switch(28, &slingshots::RIGHT_SWITCH)
      .wire_switch(29, &right_flipper::EOS_SWITCH)
      .wire_switch(30, &left_flipper::EOS_SWITCH)
      .wire_switch(31, &slingshots::LEFT_SWITCH)
      // drivers
      .wire_driver(0, &slingshots::LEFT_COIL)
      .wire_driver(1, &left_flipper::MAIN_COIL)
      .wire_driver(2, &left_flipper::HOLD_COIL)
      .wire_driver(3, &trough::COIL)
      .wire_driver(4, &plunge_lane::COIL)
      .wire_driver(5, &right_flipper::MAIN_COIL)
      .wire_driver(6, &right_flipper::HOLD_COIL)
      .wire_driver(7, &slingshots::RIGHT_COIL),
    IoBoards::io_1616()
      // switches
      // TODO
      // 0?
      .wire_driver(1, &upper_flipper::MAIN_COIL)
      .wire_driver(2, &upper_flipper::HOLD_COIL)
      .wire_driver(3, &upper_flipper::HOLD_COIL),
  ]);

  io_network.add_board(
    FastIoBoards::io_1616()
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
            initial_pwm_power: Power::percent(35),
            secondary_pwm_power: Power::percent(92),
            button_switch: switches::RIGHT_FLIPPER2,
            eos_switch: switches::FLIPPER_MAIN_RIGHT_EOS,
            ..Default::default()
          }),
      )
      .with(
        driver(2)
          .named(drivers::FLIPPER_UPPER_HOLD_RIGHT)
          .mode(FlipperHoldDirectMode {
            button_switch: switches::RIGHT_FLIPPER2,
            ..Default::default()
          }),
      )
      .with(driver(3).named(drivers::POP_LEFT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::POP_LEFT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      }))
      .with(driver(4).named(drivers::LIFT_RAMP).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::ACTION_BUTTON),
        initial_pwm_power: Power::percent(100),
        ..Default::default()
      }))
      .with(driver(5).named(drivers::POP_UPPER_RIGHT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::POP_UPPER_RIGHT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      }))
      .with(driver(6).named(drivers::POP_LOWER_RIGHT).mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(switches::POP_LOWER_RIGHT),
        initial_pwm_power: Power::percent(80),
        ..Default::default()
      }))
      .with(driver(7).named(drivers::LEFT_SCOOP).mode(PulseKickMode {
        trigger_mode: DriverTriggerMode::Switch(switches::ACTION_BUTTON),
        initial_pwm_length: Duration::from_millis(7),
        secondary_pwm_length: Duration::from_millis(38),
        ..Default::default()
      })),
  );

  io_network.build()
}
