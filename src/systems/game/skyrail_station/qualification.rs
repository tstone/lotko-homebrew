use crate::hardware::lift_ramp;
use crate::hardware::lift_ramp::LiftRampHit;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox::animation::*;
use frontbox::prelude::*;

#[derive(Clone)]
pub struct SkyrailStationQualifier;

impl ExclusiveModeQualifier for SkyrailStationQualifier {
  const REQUIRED_HITS: u8 = 2;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT2;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<LiftRampHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::SkyrailStation,
      Duration::from_millis(300),
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      (&*lift_ramp::HEX_CENTER_LED).at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }
}

pub type SkyrailStationQualification = ExclusiveModeQualification<SkyrailStationQualifier>;
