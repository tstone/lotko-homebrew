use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

const NAME: &'static str = "l_inlane";

hardware_defs! {
  pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Target)
    .location(Vec3::new(2.233, 32.135, 0.0).relative_to(&planes::PLAYFIELD));

  pub TARGET_LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(Target)
    .location(Vec3::new(3.088, 31.606, 0.0).relative_to(&planes::PLAYFIELD));
}
