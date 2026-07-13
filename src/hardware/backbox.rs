use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub LEFT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("l_speaker", 25);
  pub RIGHT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("r_speaker", 25);
}
