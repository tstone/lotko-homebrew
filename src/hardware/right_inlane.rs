use frontbox::LedChannels::GRB;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "r_inlane";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane);

  pub ENTRANCE_LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane);

  pub LANE_LED1: LedDefinition = LedDefinition::single("r_inlane_lane1")
    .channels(GRB)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LANE_LED2: LedDefinition = LedDefinition::single("r_inlane_lane2")
    .channels(GRB)
    .tag(Playfield)
    .tag(GeneralIllumination);
}
