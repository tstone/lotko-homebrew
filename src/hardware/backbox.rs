use frontbox::prelude::*;

hardware_defs! {
  pub LEFT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("l_speaker").count(25);
  pub RIGHT_SPEAKER_LEDS: LedDefinition = LedDefinition::multi("r_speaker").count(25);
}
