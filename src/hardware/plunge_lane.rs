use frontbox::prelude::*;
use frontbox::tags::*;
use frontbox_pinball::{AutoPlungerSystem, PlungeLaneSystem};

use crate::hardware::planes;

hardware_defs! {
  pub COIL: DriverDefinition = AutoPlungerSystem::coil_definition("plunge_coil")
    .location(Vec3::new(19.222, 41.884, -1.5).relative_to(&planes::PLAYFIELD));

  pub SWITCH: SwitchDefinition = PlungeLaneSystem::switch_definition("plunge_lane_sw")
    .location(Vec3::new(19.185, 40.442, 0.0).relative_to(&planes::PLAYFIELD));

  pub LED_STRIP: LedDefinition = LedDefinition::multi("plunge")
    .locations(LedLayout::strip(4, Vec3::new(19.21, 35.706, 0.0), 270.0f32.to_radians(), 0.25).relative_to(&planes::PLAYFIELD))
    .tag(Insert)
    .tag(Playfield);
}
