use frontbox::prelude::*;
use frontbox::provided::{AutoPlungerSystem, PlungeLaneSystem};
use frontbox::tags::*;

use crate::hardware::planes;

hardware_defs! {
  pub COIL: DriverDefinition = AutoPlungerSystem::coil_definition("plunge_coil");

  pub SWITCH: SwitchDefinition = PlungeLaneSystem::switch_definition("plunge_lane_sw")
    .location(Vec3::new(19.185, 40.442, 0.0).relative_to(&planes::PLAYFIELD));

  pub LED_STRIP: LedDefinition = LedDefinition::multi("plunge", 4)
    .tag(Insert)
    .tag(Playfield);
}
