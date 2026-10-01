use frontbox::led::LedChannels::GRB;
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
    .tag(Insert)
    .tag(Circle)
    .tag(Lane)
    .location(Vec3::new(16.8, 33.771, 15.477));

  pub LANE_LED1: LedDefinition = LedDefinition::single("r_inlane_lane1")
    .channels(GRB)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(16.557, 38.719, 16.041));

  pub LANE_LED2: LedDefinition = LedDefinition::single("r_inlane_lane2")
    .channels(GRB)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(14.988, 39.845, 16.169));
}
