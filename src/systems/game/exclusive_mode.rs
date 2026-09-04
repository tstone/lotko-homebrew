use frontbox::prelude::{Rgba, SystemContext};

use crate::systems::game::{hydro_core, skyrail_station, solarium_atrium};
use crate::systems::sound_loader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExclusiveMode {
  SolariumAtrium,
  HydroCore,
  SkyrailStation,
  MeridianBasins,
  SporeCountMultiball,
  Wizard,
}

impl ExclusiveMode {
  pub fn start_sound(&self) -> &'static str {
    match self {
      Self::HydroCore => sound_loader::sounds::HYDRO_CORE_ONLINE,
      _ => sound_loader::sounds::ARP_HIT1, // TODO: improve
    }
  }

  pub fn color(&self) -> Rgba<u8> {
    match self {
      Self::HydroCore => *hydro_core::MODE_COLOR,
      Self::SkyrailStation => *skyrail_station::MODE_COLOR,
      Self::SolariumAtrium => *solarium_atrium::MODE_COLOR,
      _ => todo!(),
    }
  }

  pub fn start(&self, ctx: &SystemContext) {
    match self {
      Self::HydroCore => ctx.spawn_system(hydro_core::HydroCoreMode::new()),
      Self::SkyrailStation => ctx.spawn_system(skyrail_station::SkyrailStationMode::new()),
      Self::SolariumAtrium => ctx.spawn_system(solarium_atrium::SolariumAtriumMode::new()),
      _ => todo!(),
    }
  }
}
