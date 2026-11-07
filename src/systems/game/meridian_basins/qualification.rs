use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::left_orbit::LeftOrbitHit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

#[derive(Clone)]
pub struct MeridianBasinsQualifier;

impl ExclusiveModeQualifier for MeridianBasinsQualifier {
  const REQUIRED_HITS: u8 = 3;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT3;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<LeftOrbitHit>() || event.is::<CenterOrbitHit>() || event.is::<RightOrbitHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.systems.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::MeridianBasins,
      Duration::from_millis(300),
      ctx.into(),
    );
    ctx.systems.despawn_self();
  }

  fn attention_effect() -> LedPlayback {
    led_playback!("meridian_basins/qualification.scene")
  }
}

pub type MeridianBasinsQualification = ExclusiveModeQualification<MeridianBasinsQualifier>;
