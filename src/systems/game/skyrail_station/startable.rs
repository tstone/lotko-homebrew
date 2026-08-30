use frontbox::prelude::*;

use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::*;
use crate::systems::sounds;

#[derive(Clone)]
pub struct SkyrailStationStarter;

impl ExclusiveModeStarter for SkyrailStationStarter {
  const START_SND_KEY: &'static str = sounds::ARP_HIT1;
  const MODE: ExclusiveMode = ExclusiveMode::SkyrailStation;

  fn mode_color() -> Rgba<u8> {
    *MODE_COLOR
  }

  fn on_start(ctx: &SystemContext) {
    ctx.replace_self(SkyrailStationMode::new());
  }
}

pub type SkyrailStationStartable = LiftRampStartable<SkyrailStationStarter>;
