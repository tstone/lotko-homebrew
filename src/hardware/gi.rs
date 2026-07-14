use frontbox::prelude::*;
use frontbox::tags::*;

// The 3 pin FAST GI LEDs seem to be GRB not RGB
const GI_CHANNELS: LedChannels = LedChannels::GRB;

hardware_defs! {
  pub LEFT_SLING: LedDefinition = LedDefinition::single("l_sling")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub RIGHT_SLING: LedDefinition = LedDefinition::single("r_sling")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_SCOOP_TRIANGLE: LedDefinition = LedDefinition::single("lower_scoop_tri")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_SCOOP_ABOVE: LedDefinition = LedDefinition::single("lower_scoop_above")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LEFT_RAMP: LedDefinition = LedDefinition::single("left_ramp")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub DROP1: LedDefinition = LedDefinition::single("drop1")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub DROP2: LedDefinition = LedDefinition::single("drop2")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub CAPTIVE_BALL: LedDefinition = LedDefinition::single("captive_ball")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_RIGHT_POP: LedDefinition = LedDefinition::single("lr_pop")
    .channels(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);
}
