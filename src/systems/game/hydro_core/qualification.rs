use frontbox::animation::*;
use frontbox::prelude::*;

use crate::hardware::arc_ramp;
use crate::hardware::arc_ramp::ArcRampHit;
use crate::systems::game::*;
use crate::systems::sounds;

#[derive(Clone)]
pub struct HydroCoreQualifier;

impl ExclusiveModeQualifier for HydroCoreQualifier {
  const REQUIRED_HITS: u8 = 2;
  const HIT_SND_KEY: &'static str = sounds::HYDRO_CORE_ONLINE;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<ArcRampHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::HydroCore,
      Duration::from_millis(2500),
      ctx.into(),
    );
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      (&*arc_ramp::HEX_CENTER_LED).at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }}

pub type HydroCoreQualification = ExclusiveModeQualification<HydroCoreQualifier>;
