use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "r_outlane";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane);

  pub LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane);
}
