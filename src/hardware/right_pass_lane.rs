use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub UPPER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_upper")
    .tag(Playfield)
    .tag(RightPassLane)
    .tag(DoesNotCancelSkillshot)
    .tag(Lane);

  pub LOWER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_lower")
    .tag(Playfield)
    .tag(RightPassLane)
    .tag(DoesNotCancelSkillshot)
    .tag(Lane);

  pub ARROW_LED: LedDefinition = LedDefinition::single("r_pass_lane_arr")
    .tag(Playfield)
    .tag(Insert)
    .tag(SmallArrow)
    .tag(Lane);
}

#[derive(Tag)]
pub struct RightPassLane;
