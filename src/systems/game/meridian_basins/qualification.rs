use crate::hardware::center_orbit;
use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::left_orbit;
use crate::hardware::left_orbit::LeftOrbitHit;
use crate::hardware::right_orbit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::systems::game::*;
use crate::systems::sounds;
use frontbox::prelude::*;

#[derive(Clone)]
pub struct MeridianBasinsQualifier;

impl ExclusiveModeQualifier for MeridianBasinsQualifier {
  const REQUIRED_HITS: u8 = 2;
  const HIT_SND_KEY: &'static str = sounds::LANE_HIT3;

  fn is_qualifying_shot(event: &dyn Event) -> bool {
    event.is::<LeftOrbitHit>() || event.is::<CenterOrbitHit>() || event.is::<RightOrbitHit>()
  }

  fn on_qualified(ctx: &SystemContext) {
    ctx.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::MeridianBasins,
      Duration::from_millis(300),
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      LedQ::any(vec![
        &*left_orbit::HEX_CENTER_LED,
        &*center_orbit::HEX_CENTER_LED,
        &*right_orbit::HEX_CENTER_LED,
      ])
      .at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }
}

pub type MeridianBasinsQualification = ExclusiveModeQualification<MeridianBasinsQualifier>;
