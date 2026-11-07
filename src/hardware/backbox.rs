use frontbox_core::prelude::*;

use crate::hardware::more_tags::Speaker;
use crate::hardware::planes;

hardware_defs! {
  // Around the 5.25" speakers, in the bottom corners of the backbox. Each ring starts at its bottom middle, on the plane's
  // 270 degrees. The direction it runs from there is yet to be tested.
  pub LEFT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("l_speaker")
    .locations(LedLayout::ring(25, Vec3::new(4.0, 4.0, 0.0), 2.625, 270.0f32.to_radians(), LedLayoutDirection::Clockwise).relative_to(&planes::BACKBOX_PANEL))
    .tag(Speaker);

  pub RIGHT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("r_speaker")
    .locations(LedLayout::ring(25, Vec3::new(23.25, 4.0, 0.0), 2.625, 270.0f32.to_radians(), LedLayoutDirection::Clockwise).relative_to(&planes::BACKBOX_PANEL))
    .tag(Speaker);
}
