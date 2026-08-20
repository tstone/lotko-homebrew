use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Bolt;

hardware_defs! {
  pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("cap_ball_target")
    .tag(Playfield);

  pub REST_SWITCH: SwitchDefinition = SwitchDefinition::new("cap_ball_rest")
    .tag(Playfield);

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("l_cap_ball_bolt")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield);

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("r_cap_ball_bolt")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield);
}
