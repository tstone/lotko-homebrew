use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub LEFT_SLING: LedDefinition = LedDefinition::single("l_sling")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub RIGHT_SLING: LedDefinition = LedDefinition::single("r_sling")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_SCOOP_TRIANGLE: LedDefinition = LedDefinition::single("lower_scoop_tri")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_SCOOP_ABOVE: LedDefinition = LedDefinition::single("lower_scoop_above")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LEFT_RAMP: LedDefinition = LedDefinition::single("left_ramp")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub DROP1: LedDefinition = LedDefinition::single("drop1")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub DROP2: LedDefinition = LedDefinition::single("drop2")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub CAPTIVE_BALL: LedDefinition = LedDefinition::single("captive_ball")
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_RIGHT_POP: LedDefinition = LedDefinition::single("lr_pop")
    .tag(Playfield)
    .tag(GeneralIllumination);
}
