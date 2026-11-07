use crate::hardware::main_shot::{MainShot, MainShotHit};
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
    event.downcast_ref::<MainShotHit>().is_some_and(|hit| {
      matches!(
        hit.shot,
        MainShot::LeftOrbit | MainShot::CenterOrbit | MainShot::RightOrbit
      )
    })
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
