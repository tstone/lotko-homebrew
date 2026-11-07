use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::gi;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

#[derive(Clone)]
pub struct SporeMultiballQualifier;

impl ExclusiveModeQualifier for SporeMultiballQualifier {
  const REQUIRED_HITS: u8 = 4;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT3;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<CaptiveBallHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.systems.replace_self(SporeMultiballStartable::new());
  }

  fn attention_effect() -> LedScene {
    LedScene::new(
      gi::CAPTIVE_BALL.q().at_z(1),
      led_scene!("spore_multiball/qualification.ron"),
    )
  }
}

pub type SporeMultiballQualification = ExclusiveModeQualification<SporeMultiballQualifier>;
