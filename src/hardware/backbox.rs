use frontbox_core::prelude::*;

use crate::hardware::planes;

hardware_defs! {
  // Around the 5.25" speakers, in the bottom corners of the backbox. Where each ring starts and which way it runs is yet
  // to be measured.
  pub LEFT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("l_speaker")
    .locations(LedLayout::ring(25, Vec3::new(4.0, 4.0, 0.0), 2.625, 0.0, LedLayoutDirection::Clockwise).relative_to(&planes::BACKBOX_PANEL));

  pub RIGHT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("r_speaker")
    .locations(LedLayout::ring(25, Vec3::new(23.25, 4.0, 0.0), 2.625, 0.0, LedLayoutDirection::Clockwise).relative_to(&planes::BACKBOX_PANEL));
}
