use frontbox_core::prelude::*;
use frontbox_pinball::tags::*;

use crate::hardware::planes;

// The 3 pin FAST GI LEDs seem to be GRB not RGB
const GI_CHANNELS: ChannelOrder = ChannelOrder::GRB;

hardware_defs! {
  pub LEFT_SLING: LedDefinition = LedDefinition::single("gi_l_sling")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub RIGHT_SLING: LedDefinition = LedDefinition::single("gi_r_sling")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub LOWER_SCOOP_TRIANGLE: LedDefinition = LedDefinition::single("gi_lower_scoop_tri")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(1.12, 27.875, 0.0).relative_to(&planes::PLAYFIELD));

  pub LOWER_SCOOP_ABOVE: LedDefinition = LedDefinition::single("gi_lower_scoop_above")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(1.035, 21.817, 0.0).relative_to(&planes::PLAYFIELD));

  pub LEFT_RAMP: LedDefinition = LedDefinition::single("gi_left_ramp")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub DROP1: LedDefinition = LedDefinition::single("gi_drop1")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub DROP2: LedDefinition = LedDefinition::single("gi_drop2")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination);

  pub CAPTIVE_BALL: LedDefinition = LedDefinition::single("gi_captive_ball")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(9.178, 5.883, 0.0).relative_to(&planes::PLAYFIELD));

  pub LOWER_RIGHT_POP: LedDefinition = LedDefinition::single("gi_lr_pop")
    .channel_order(GI_CHANNELS)
    .tag(Playfield)
    .tag(GeneralIllumination)
    .location(Vec3::new(17.147, 25.768, 0.0).relative_to(&planes::PLAYFIELD));
}

/// The GI that goes the Steward's red while the Sentinels are out, every other one around the playfield. The rest stay
/// white. `LEFT_RAMP` isn't wired, so it's in neither half.
pub fn sentinels_half() -> LightQ {
  LightQ::any(vec![
    &LEFT_SLING.q(),
    &LOWER_SCOOP_ABOVE.q(),
    &DROP1.q(),
    &CAPTIVE_BALL.q(),
  ])
}
