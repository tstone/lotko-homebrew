use frontbox::animation::*;
use frontbox::prelude::*;

use crate::hardware::arc_ramp;
use crate::hardware::lower_scoop;
use crate::hardware::lower_scoop::LowerScoopBallEnter;
use crate::systems::game::hydro_core::MODE_COLOR;
use crate::systems::game::*;
use crate::systems::sounds;

#[derive(Clone)]
pub struct HydroCoreStarter;

impl ExclusiveModeStarter for HydroCoreStarter {
  const START_SND_KEY: &'static str = sounds::HYDRO_CORE_ONLINE;
  const MODE: ExclusiveMode = ExclusiveMode::HydroCore;

  fn is_startable_event(event: &dyn Event) -> bool {
    event.is::<LowerScoopBallEnter>()
  }

  fn on_start(ctx: &SystemContext) {
    ctx.replace_self(HydroCoreMode::new());
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::flash(
          &*lower_scoop::BOLTS_Q,
          ColorSequence::solid(*MODE_COLOR),
          Cycle::Forever,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::flash(
          &*arc_ramp::HEX_CENTER_LED,
          ColorSequence::solid(*MODE_COLOR),
          Cycle::Forever,
        ),
      )
      .at(Duration::ZERO, arc_ramp::into_subway_program(*MODE_COLOR))
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::tween(
      LedQ::tag::<tags::Playfield>().at_z(-1),
      Duration::from_millis(750),
      Curve::EaseIn,
      Cycle::Once,
      vec![
        ColorSequence::solid(*MODE_COLOR),
        ColorSequence::solid(Rgba::default()),
      ],
    )
    .stopped()
  }
}

pub type HydroCoreStartable = LeftScoopStartable<HydroCoreStarter>;
