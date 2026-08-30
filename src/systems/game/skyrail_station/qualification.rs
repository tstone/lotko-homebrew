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
    // ctx.replace_self(SkyrailStationStartable::new(Duration::from_millis(2500)));
    // TODO
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      (&*lift_ramp::HEX_CENTER_LED).at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      (&*lift_ramp::HEX_CIRCLE_LEDS).at_z(1),
      ColorSequence::fade(Rgba::white(), Rgba::default()),
      Duration::from_millis(500),
      Curve::Linear,
      Cycle::Once,
    )
    .stopped()
  }
}

pub type SkyrailStationQualification = ExclusiveModeQualification<SkyrailStationQualifier>;
