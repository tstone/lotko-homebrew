use crate::hardware::dome_ramp::DomeRampHit;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

#[derive(Clone)]
pub struct SolariumAtriumQualifier;

impl ExclusiveModeQualifier for SolariumAtriumQualifier {
  const REQUIRED_HITS: u8 = 2;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT3;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<DomeRampHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.systems.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::SolariumAtrium,
      Duration::from_millis(300),
      ctx.into(),
    );
    ctx.systems.despawn_self();
  }

  fn attention_effect() -> LedPlayback {
    led_playback!("solarium_atrium/qualification.scene")
  }
}

pub type SolariumAtriumQualification = ExclusiveModeQualification<SolariumAtriumQualifier>;
