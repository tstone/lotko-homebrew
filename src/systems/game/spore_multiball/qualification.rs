use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::gi;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox::prelude::*;

#[derive(Clone)]
pub struct SporeMultiballQualifier;

impl ExclusiveModeQualifier for SporeMultiballQualifier {
  const REQUIRED_HITS: u8 = 4;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT3;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<CaptiveBallHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.replace_self(SporeMultiballStartable::new());
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::pulse(
      gi::CAPTIVE_BALL.q().at_z(1),
      Rgba::white(),
      Duration::from_millis(1400),
      Cycle::Forever,
    )
  }
}

pub type SporeMultiballQualification = ExclusiveModeQualification<SporeMultiballQualifier>;
