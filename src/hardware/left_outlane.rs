use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Circle;

const NAME: &'static str = "l_outlane";

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
