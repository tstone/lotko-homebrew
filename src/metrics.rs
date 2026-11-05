//! Lotko's own metrics, alongside the ones `frontbox-pinball` records. See [`frontbox_core::metrics`] for how metrics work.

use std::sync::LazyLock;

use frontbox_core::prelude::*;

use crate::systems::game::{ExclusiveMode, NonExclusiveMode};

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

pub static SPORE_MULTIBALL_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new(
    "Spore Multiball Completed",
    "Times Spore Multiball was completed",
  )
  .key("spore_multiball_completed")
  .category("Modes")
});

pub static WIZARD_COMPLETED: LazyLock<Counter> = LazyLock::new(|| {
  Counter::new("Wizard Completed", "Times the wizard mode was completed")
    .key("wizard_completed")
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

/// The counter for completions of an exclusive mode
pub fn exclusive_mode_completed(mode: &ExclusiveMode) -> &'static Counter {
  match mode {
    ExclusiveMode::HydroCore => &HYDRO_CORE_COMPLETED,
    ExclusiveMode::SkyrailStation => &SKYRAIL_STATION_COMPLETED,
    ExclusiveMode::SolariumAtrium => &SOLARIUM_ATRIUM_COMPLETED,
    ExclusiveMode::MeridianBasins => &MERIDIAN_BASINS_COMPLETED,
    ExclusiveMode::SporeMultiball => &SPORE_MULTIBALL_COMPLETED,
    ExclusiveMode::Wizard => &WIZARD_COMPLETED,
  }
}

/// The counter for completions of a non-exclusive mode
pub fn non_exclusive_mode_completed(mode: &NonExclusiveMode) -> &'static Counter {
  match mode {
    NonExclusiveMode::NimbusPromenade => &NIMBUS_PROMENADE_COMPLETED,
    NonExclusiveMode::ApexTerraces => &APEX_TERRACES_COMPLETED,
  }
}
