use crate::hardware::lift_ramp;
use crate::hardware::lift_ramp::LiftRampHit;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

#[derive(Clone)]
pub struct SkyrailStationQualifier;

impl ExclusiveModeQualifier for SkyrailStationQualifier {
  const REQUIRED_HITS: u8 = 2;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT2;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<LiftRampHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.systems.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::SkyrailStation,
      Duration::from_millis(300),
      ctx.into(),
    );
    ctx.systems.despawn_self();
  }

  fn attention_effect() -> LedScene {
    LedScene::new(
      (&*lift_ramp::HEX_CENTER_LED).at_z(1),
      led_scene!("skyrail_station/qualification.ron"),
    )
  }
}

pub type SkyrailStationQualification = ExclusiveModeQualification<SkyrailStationQualifier>;
