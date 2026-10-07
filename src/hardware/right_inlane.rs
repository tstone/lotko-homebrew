use frontbox::led::LedChannels::GRB;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

const NAME: &'static str = "r_inlane";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane)
    .location(Vec3::new(15.756, 32.38, 0.0).relative_to(&planes::PLAYFIELD));

  pub ENTRANCE_LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(Lane)
    .location(Vec3::new(15.776, 30.803, 0.0).relative_to(&planes::PLAYFIELD));

  pub LANE_LED1: LedDefinition = LedDefinition::single("r_inlane_lane1")
    .channels(GRB)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(15.666, 35.661, 0.0).relative_to(&planes::PLAYFIELD));

  pub LANE_LED2: LedDefinition = LedDefinition::single("r_inlane_lane2")
    .channels(GRB)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(14.118, 36.718, 0.0).relative_to(&planes::PLAYFIELD));
}
