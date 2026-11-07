use frontbox_core::prelude::*;
use frontbox_core::tags::*;

use crate::hardware::planes;

hardware_defs! {
  pub LEFT_FLASHER: LedDefinition = LedDefinition::multi("l_flasher")
    .locations(LedLayout::ring(8, Vec3::new(1.145, 23.0, 1.25), 0.375, 0.0, LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .tag(Playfield)
    .tag(Flasher);

  pub CENTER_FLASHER: LedDefinition = LedDefinition::multi("c_flasher")
    .locations(LedLayout::ring(8, Vec3::new(9.56, 5.16, 1.25), 0.375, 0.0, LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .tag(Playfield)
    .tag(Flasher);
}
