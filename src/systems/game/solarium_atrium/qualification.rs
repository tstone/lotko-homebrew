use crate::hardware::dome_ramp;
use crate::hardware::dome_ramp::DomeRampHit;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox::animation::*;
use frontbox::prelude::*;

#[derive(Clone)]
pub struct SolariumAtriumQualifier;

impl ExclusiveModeQualifier for SolariumAtriumQualifier {
  const REQUIRED_HITS: u8 = 2;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT3;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<DomeRampHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::SolariumAtrium,
      Duration::from_millis(300),
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      (&*dome_ramp::HEX_CENTER_LED).at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }
}

pub type SolariumAtriumQualification = ExclusiveModeQualification<SolariumAtriumQualifier>;
