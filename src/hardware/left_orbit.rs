use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "l_orbit";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Hex)
    .tag(Lane);
}
