//! Lotko's own metrics, alongside the ones `frontbox-pinball` records. See [`frontbox_core::metrics`] for how metrics work.

use std::sync::LazyLock;

use frontbox_core::prelude::*;

use crate::systems::game::Mode;

pub static SPORES_APPLIED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Spores Applied", "Spores applied to the city (mode shots)")
    .key("spores_applied")
    .category("Modes")
});

pub static MODES_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Modes Completed", "City modes completed, of any kind")
    .key("modes_completed")
    .category("Modes")
});

pub static HYDRO_CORE_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Hydro Core Completed", "Times Hydro Core was completed")
    .key("hydro_core_completed")
    .category("Modes")
});

pub static SKYRAIL_STATION_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Skyrail Station Completed",
    "Times Skyrail Station was completed",
  )
  .key("skyrail_station_completed")
  .category("Modes")
});

pub static SOLARIUM_ATRIUM_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Solarium Atrium Completed",
    "Times Solarium Atrium was completed",
  )
  .key("solarium_atrium_completed")
  .category("Modes")
});

pub static MERIDIAN_BASINS_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Meridian Basins Completed",
    "Times Meridian Basins was completed",
  )
  .key("meridian_basins_completed")
  .category("Modes")
});

pub static NIMBUS_PROMENADE_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Nimbus Promenade Completed",
    "Times Nimbus Promenade was completed",
  )
  .key("nimbus_promenade_completed")
  .category("Modes")
});

pub static APEX_TERRACES_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Apex Terraces Completed",
    "Times Apex Terraces was completed",
  )
  .key("apex_terraces_completed")
  .category("Modes")
});

pub static BONUS_SKIPPED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Bonus Skipped",
    "End-of-ball bonuses skipped with a double flip",
  )
  .key("bonus_skipped")
  .category("Bonus")
});

pub static HELIX_VAULTS_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Helix Vaults Completed", "Times Helix Vaults was completed")
    .key("floodgates_completed")
    .category("Modes")
});

pub static CALDERA_GENERATOR_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Caldera Generator Completed", "Times Caldera Generator was completed")
    .key("caldera_generator_completed")
    .category("Modes")
});

pub static CARGO_QUAY_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Cargo Quay Completed", "Times Cargo Quay was completed")
    .key("cargo_quay_completed")
    .category("Modes")
});

pub static GROUNDSWELL_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Groundswell Completed", "Times Groundswell was completed")
    .key("groundswell_completed")
    .category("Modes")
});

pub static HALCYON_VENTS_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Halcyon Vents Completed", "Times Halcyon Vents was completed")
    .key("halcyon_vents_completed")
    .category("Modes")
});

/// The counter for completions of a mode
pub fn mode_completed(mode: &Mode) -> &'static Counter {
  match mode {
    Mode::HydroCore => &HYDRO_CORE_COMPLETED,
    Mode::SkyrailStation => &SKYRAIL_STATION_COMPLETED,
    Mode::SolariumAtrium => &SOLARIUM_ATRIUM_COMPLETED,
    Mode::MeridianBasins => &MERIDIAN_BASINS_COMPLETED,
    Mode::CargoQuay => &CARGO_QUAY_COMPLETED,
    Mode::NimbusPromenade => &NIMBUS_PROMENADE_COMPLETED,
    Mode::ApexTerraces => &APEX_TERRACES_COMPLETED,
    Mode::HelixVaults => &HELIX_VAULTS_COMPLETED,
    Mode::CalderaGenerator => &CALDERA_GENERATOR_COMPLETED,
    Mode::Groundswell => &GROUNDSWELL_COMPLETED,
    Mode::HalcyonVents => &HALCYON_VENTS_COMPLETED,
  }
}
