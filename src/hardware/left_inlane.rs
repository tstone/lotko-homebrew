use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "l_inlane";

hardware_defs! {
  pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Target);

  pub TARGET_LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Target);
}
