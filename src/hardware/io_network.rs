use frontbox::prelude::*;

use crate::hardware::cabinet;
use crate::hardware::left_flipper;
use crate::hardware::left_inlane;
use crate::hardware::lift_ramp;
use crate::hardware::lower_scoop;
use crate::hardware::plunge_lane;
use crate::hardware::pop_cluster;
use crate::hardware::right_flipper;
use crate::hardware::right_inlane;
use crate::hardware::slingshots;
use crate::hardware::trough;
use crate::hardware::upper_flipper;

pub fn io_network() -> IoNetwork {
  IoNetwork::new(vec![
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
      .wire_switch(0, &upper_flipper::EOS_SWITCH) // temporary
      .wire_switch(1, &plunge_lane::SWITCH) // also temp for testing
      // drivers
      .wire_driver(0, &lift_ramp::EJECT_COIL)
      .wire_driver(1, &upper_flipper::HOLD_COIL)
      .wire_driver(2, &upper_flipper::MAIN_COIL)
      .wire_driver(3, &pop_cluster::left::COIL)
      .wire_driver(4, &lift_ramp::RAMP_COIL)
      .wire_driver(5, &pop_cluster::upper_right::COIL)
      .wire_driver(6, &pop_cluster::lower_right::COIL)
      .wire_driver(7, &lower_scoop::COIL),
  ])
}
