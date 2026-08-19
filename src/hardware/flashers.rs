use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub LEFT_FLASHER: LedDefinition = LedDefinition::strip("l_flasher", 8)
    .tag(Playfield)
    .tag(Flasher);

  pub CENTER_FLASHER: LedDefinition = LedDefinition::strip("c_flasher", 8)
    .tag(Playfield)
    .tag(Flasher);
}
