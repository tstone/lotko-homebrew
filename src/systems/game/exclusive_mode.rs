use frontbox::prelude::*;

use crate::systems::game::{hydro_core, meridian_basins, skyrail_station, solarium_atrium};
use crate::systems::sound_loader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ExclusiveMode {
  SolariumAtrium,
  HydroCore,
  SkyrailStation,
  MeridianBasins,
  SporeMultiball,
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
      Self::HydroCore => Rgba::cyan(),
      Self::SkyrailStation => Rgba::blue(),
      Self::SolariumAtrium => Rgba::purple(),
      Self::MeridianBasins => Rgba::orange(),
      Self::SporeMultiball => Rgba::yellow(),
      Self::Wizard => Rgba::red(),
    }
  }

  pub fn start(&self, ctx: &SystemContext) {
    match self {
      Self::HydroCore => ctx.spawn_system(hydro_core::HydroCoreMode::new()),
      Self::SkyrailStation => ctx.spawn_system(skyrail_station::SkyrailStationMode::new()),
      Self::SolariumAtrium => ctx.spawn_system(solarium_atrium::SolariumAtriumMode::new()),
      Self::MeridianBasins => ctx.spawn_system(meridian_basins::MeridianBasinsMode::new()),
      Self::SporeMultiball => todo!(),
      Self::Wizard => todo!(),
    }
  }
}
