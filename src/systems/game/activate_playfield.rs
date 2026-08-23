use crate::hardware::*;
use frontbox_turn_based::ActivatePlayfieldSystem;

pub fn activate_playfield() -> ActivatePlayfieldSystem {
  ActivatePlayfieldSystem::new()
    // slings
    .driver(slingshots::LEFT_COIL.name, slingshots::LEFT_SWITCH.name)
    .driver(slingshots::RIGHT_COIL.name, slingshots::RIGHT_SWITCH.name)
    // flippers
    .driver(
      left_flipper::MAIN_COIL.name,
      cabinet::LEFT_FLIPPER_SWITCH1.name,
    )
    .driver(
      left_flipper::HOLD_COIL.name,
      cabinet::LEFT_FLIPPER_SWITCH1.name,
    )
    .driver(
      right_flipper::MAIN_COIL.name,
      cabinet::RIGHT_FLIPPER_SWITCH1.name,
    )
    .driver(
      right_flipper::HOLD_COIL.name,
      cabinet::RIGHT_FLIPPER_SWITCH1.name,
    )
    .driver(
      upper_flipper::MAIN_COIL.name,
      cabinet::RIGHT_FLIPPER_SWITCH2.name,
    )
    .driver(
      upper_flipper::HOLD_COIL.name,
      cabinet::RIGHT_FLIPPER_SWITCH2.name,
    )
    // pops
    .driver(
      pop_cluster::lower_right::COIL.name,
      pop_cluster::lower_right::SPOON_SWITCH.name,
    )
    .driver(
      pop_cluster::upper_right::COIL.name,
      pop_cluster::upper_right::SPOON_SWITCH.name,
    )
    .driver(
      pop_cluster::left::COIL.name,
      pop_cluster::left::SPOON_SWITCH.name,
    )
    .driver(lower_scoop::COIL.name, lower_scoop::OPTO.name)
}
