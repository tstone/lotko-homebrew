use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub UPPER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_upper")
    .tag(Playfield)
    .tag(Lane);

  pub LOWER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_lower")
    .tag(Playfield)
    .tag(Lane);

  pub ARROW_LED: LedDefinition = LedDefinition::single("r_pass_lane_arr")
    .tag(Playfield)
    .tag(Lane);
}
