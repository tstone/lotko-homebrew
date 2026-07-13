use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Hex;

const NAME: &'static str = "l_ramp";

hardware_defs! {

  pub OPTO: SwitchDefinition = SwitchDefinition::new(NAME);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Hex)
    .tag(Lane);
}
