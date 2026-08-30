use frontbox::prelude::*;

use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::*;
use crate::systems::sounds;

#[derive(Clone)]
pub struct SkyrailStationStarter;

impl ExclusiveModeStarter for SkyrailStationStarter {
  const START_SND_KEY: &'static str = sounds::HYDRO_CORE_ONLINE;
  const MODE: ExclusiveMode = ExclusiveMode::HydroCore;

  fn mode_color() -> Rgba<u8> {
    *MODE_COLOR
  }

  fn on_start(ctx: &SystemContext) {
    // TODO: ctx.replace_self(HydroCoreMode::new());
  }
}

pub type SkyrailStationStartable = LiftRampStartable<SkyrailStationStarter>;
