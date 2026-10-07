use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub UPPER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_upper")
    .tag(Playfield)
    .tag(RightPassLane)
    .tag(DoesNotCancelSkillshot)
    .tag(Lane)
    .location(Vec3::new(19.109, 25.405, 0.0).relative_to(&planes::PLAYFIELD));

  pub LOWER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_lower")
    .tag(Playfield)
    .tag(RightPassLane)
    .tag(DoesNotCancelSkillshot)
    .tag(Lane)
    .location(Vec3::new(19.11, 30.29, 0.0).relative_to(&planes::PLAYFIELD));

  pub ARROW_LED: LedDefinition = LedDefinition::single("r_pass_lane_arr")
    .tag(Playfield)
    .tag(Insert)
    .tag(SmallArrow)
    .tag(Lane);
}

#[derive(Tag)]
pub struct RightPassLane;
