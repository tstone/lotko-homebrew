use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub SPINNER: LedDefinition = LedDefinition::single("upper_pass_spinner")
    .tag(Circle)
    .tag(Lane)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(5.836, 16.294, 0.0).relative_to(&planes::PLAYFIELD));

  pub ARROW_LED1: LedDefinition = LedDefinition::single("upper_pass_arrow1")
    .tag(SmallArrow)
    .tag(Lane)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(9.548, 16.845, 0.0).relative_to(&planes::PLAYFIELD));

  pub ARROW_LED2: LedDefinition = LedDefinition::single("upper_pass_arrow2")
    .tag(SmallArrow)
    .tag(Lane)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(7.677, 16.61, 0.0).relative_to(&planes::PLAYFIELD));
}
