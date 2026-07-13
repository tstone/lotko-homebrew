use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

pub const NAME: &'static str = "center_orbit";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME);
  pub SPINNER_OPTO: SwitchDefinition = SwitchDefinition::new("center_spinner");

  pub SPINNER_LED: LedDefinition = LedDefinition::single("center_spinner")
    .tag(Circle)
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Hex)
    .tag(Lane);
}
