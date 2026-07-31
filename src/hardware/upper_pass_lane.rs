use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub SPINNER: LedDefinition = LedDefinition::single("upper_pass_spinner")
    .tag(Circle)
    .tag(Lane)
    .tag(Playfield);

  pub ARROW_LED1: LedDefinition = LedDefinition::single("upper_pass_arrow1")
    .tag(SmallArrow)
    .tag(Lane)
    .tag(Playfield);

  pub ARROW_LED2: LedDefinition = LedDefinition::single("upper_pass_arrow2")
    .tag(SmallArrow)
    .tag(Lane)
    .tag(Playfield);
}
