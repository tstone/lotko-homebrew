use std::path::PathBuf;

use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

use crate::hardware::city_map;
use crate::systems::Callout;
use crate::systems::game::{
  ApexTerracesMode, BlackoutMode, CanopyMode, DerailmentMode, FloodgatesMode, HydroCoreMode,
  MeridianBasinsMode, NimbusPromenadeMode, SkyrailStationMode, SolariumAtriumMode,
};

const MUSIC_FOLDER: &str = "/userdata/home/armsom/music";

/// A stop on the route up the city (DESIGN.md, Primary progression): a district or a mini-wizard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Mode {
  MeridianBasins,
  HydroCore,
  NimbusPromenade,
  SkyrailStation,
  ApexTerraces,
  SolariumAtrium,
  Floodgates,
  Blackout,
  Derailment,
  Canopy,
}

impl Mode {
  pub const DISTRICTS: [Mode; 6] = [
    Self::MeridianBasins,
    Self::HydroCore,
    Self::NimbusPromenade,
    Self::SkyrailStation,
    Self::ApexTerraces,
    Self::SolariumAtrium,
  ];

  pub fn name(&self) -> &'static str {
    match self {
      Self::MeridianBasins => "MERIDIAN BASINS",
      Self::HydroCore => "HYDRO CORE",
      Self::NimbusPromenade => "NIMBUS PROMENADE",
      Self::SkyrailStation => "SKYRAIL STATION",
      Self::ApexTerraces => "APEX TERRACES",
      Self::SolariumAtrium => "SOLARIUM ATRIUM",
      Self::Floodgates => "FLOODGATES",
      Self::Blackout => "BLACKOUT",
      Self::Derailment => "DERAILMENT",
      Self::Canopy => "CANOPY",
    }
  }

  pub fn is_mini_wizard(&self) -> bool {
    matches!(
      self,
      Self::Floodgates | Self::Blackout | Self::Derailment | Self::Canopy
    )
  }

  /// Tier 1 is the lower city, up to its mini-wizard. Tier 2 is the upper city.
  pub fn tier(&self) -> u8 {
    match self {
      Self::MeridianBasins
      | Self::HydroCore
      | Self::NimbusPromenade
      | Self::Floodgates
      | Self::Blackout => 1,
      Self::SkyrailStation
      | Self::ApexTerraces
      | Self::SolariumAtrium
      | Self::Derailment
      | Self::Canopy => 2,
    }
  }

  /// The district that leads to a mini-wizard
  pub fn district(&self) -> Mode {
    match self {
      Self::Floodgates => Self::MeridianBasins,
      Self::Blackout => Self::HydroCore,
      Self::Derailment => Self::SkyrailStation,
      Self::Canopy => Self::ApexTerraces,
      district => *district,
    }
  }

  pub fn color(&self) -> Rgba<u8> {
    match self.district() {
      Self::HydroCore => Rgba::cyan(),
      Self::SkyrailStation => Rgba::blue(),
      Self::SolariumAtrium => Rgba::purple(),
      Self::MeridianBasins => Rgba::orange(),
      Self::NimbusPromenade => Rgba::magenta(),
      _ => Rgba::gold(),
    }
  }

  /// The mode's insert on the city map. Mini-wizards don't have one, as the map's six are the districts.
  pub fn map_led(&self) -> Option<LedQ> {
    match self {
      Self::HydroCore => Some(city_map::HYDRO_CORE.q()),
      Self::SkyrailStation => Some(city_map::SKYRAIL_STATION.q()),
      Self::SolariumAtrium => Some(city_map::SOLARIUM_ATRIUMS.q()),
      Self::MeridianBasins => Some(city_map::MERIDIAN_BASINS.q()),
      Self::NimbusPromenade => Some(city_map::NIMBUS_PROMENADE.q()),
      Self::ApexTerraces => Some(city_map::APEX_TERRACES.q()),
      _ => None,
    }
  }

  /// The mode's own track, for those that have one so far
  pub fn music(&self) -> Option<PathBuf> {
    let file = match self {
      Self::HydroCore => "inzo-wonder.mp3",
      Self::SkyrailStation => "deadmau5-arguru2k19.mp3",
      Self::SolariumAtrium => "fehrplay-arcadia.mp3",
      Self::MeridianBasins => "glitch-mob-cant-kill-us.mp3",
      Self::NimbusPromenade => "wice-5omething.mp3",
      _ => return None,
    };
    Some(music_file(file))
  }

  /// What's said as the mode starts, in order. The ball waits in the scoop for the last of them. A mini-wizard is the
  /// forest's, so the Oaks have their say.
  pub fn start_lines(&self) -> &'static [Callout] {
    match self {
      Self::MeridianBasins => &[Callout::RootIntrusion],
      Self::HydroCore => &[Callout::CoolantCoreDropping],
      Self::NimbusPromenade => &[Callout::PromenadeClosed, Callout::WalkwaysWillBeSealed],
      Self::SkyrailStation => &[Callout::SkyrailServiceSuspended, Callout::ClearThePlatform],
      Self::ApexTerraces => &[Callout::ResidentsEvacuateNow],
      Self::SolariumAtrium => &[Callout::AtriumIntegrityVerified],
      Self::Floodgates => &[Callout::OakOpenTheGates, Callout::FloodBarriersEngaged],
      Self::Blackout => &[Callout::OakDarkness],
      Self::Derailment => &[Callout::TrainDerailmentSubverting, Callout::OakFall],
      Self::Canopy => &[Callout::OakHigher],
    }
  }

  /// What's said as the mode is completed: the Steward's line, then the Oaks' for the districts that have one
  pub fn completion_lines(&self) -> &'static [Callout] {
    match self {
      Self::MeridianBasins => &[Callout::BasinsReclassified, Callout::OakOurs],
      Self::HydroCore => &[Callout::CoreOffline, Callout::OakDrink],
      Self::NimbusPromenade => &[
        Callout::AccessToUpperCityRestricted,
        Callout::OakTheWayIsOpen,
      ],
      Self::SkyrailStation => &[Callout::AllLinesAreNowClosed],
      Self::ApexTerraces => &[Callout::ApexTerracesOffline, Callout::OakNothingIsYours],
      Self::SolariumAtrium => &[Callout::TheScienceWillNotBeLost, Callout::OakOpenSky],
      Self::Floodgates | Self::Blackout | Self::Derailment | Self::Canopy => &[],
    }
  }

  /// Starts the mode's system, which stays around until the mode is completed so that the mode can be resumed
  pub fn spawn(&self, ctx: &SystemContext) {
    match self {
      Self::HydroCore => ctx.systems.spawn(HydroCoreMode::new()),
      Self::SkyrailStation => ctx.systems.spawn(SkyrailStationMode::new()),
      Self::SolariumAtrium => ctx.systems.spawn(SolariumAtriumMode::new()),
      Self::MeridianBasins => ctx.systems.spawn(MeridianBasinsMode::new()),
      Self::NimbusPromenade => ctx.systems.spawn(NimbusPromenadeMode::new()),
      Self::ApexTerraces => ctx.systems.spawn(ApexTerracesMode::new()),
      Self::Floodgates => ctx.systems.spawn(FloodgatesMode::new()),
      Self::Blackout => ctx.systems.spawn(BlackoutMode::new()),
      Self::Derailment => ctx.systems.spawn(DerailmentMode::new()),
      Self::Canopy => ctx.systems.spawn(CanopyMode::new()),
    }
  }
}

/// How far a player has got with a mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ModeState {
  /// Not reached yet, or closed by choosing the other road
  Unplayed,
  /// Can be chosen
  Available,
  /// Started. If it's not running, it can be resumed.
  Played,
  Completed,
}

/// Runs alongside a mode, in a slot of its own
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Multiball {
  Heartwood,
  // Comes with the Groundswell lock (PLAN.md, Part 4)
  #[allow(dead_code)]
  Groundswell,
}

impl Multiball {
  pub fn color(&self) -> Rgba<u8> {
    match self {
      Self::Heartwood => Rgba::lime(),
      Self::Groundswell => Rgba::green(),
    }
  }

  pub fn music(&self) -> Option<PathBuf> {
    match self {
      Self::Heartwood => Some(music_file("rezmau5-infraliminal.mp3")),
      Self::Groundswell => None,
    }
  }

  /// Groundswell stacks with districts but not mini-wizards. Heartwood stacks with any mode. The other way around, a
  /// mode can start under a multiball by the same rule.
  pub fn stacks_with(&self, mode: Mode) -> bool {
    match self {
      Self::Heartwood => true,
      Self::Groundswell => !mode.is_mini_wizard(),
    }
  }
}

/// What completing a district gives the player for the rest of the game (DESIGN.md, Perks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Perk {
  /// Root tunnel hits count double
  DoubleTunnels,
  /// +5s on mode timers
  LongerTimers,
  /// Pops charge twice as fast
  FasterPopCharge,
  /// One drop bank clear lights a lock
  QuickLocks,
  /// The center spinner scores double
  DoubleCenterSpinner,
  /// Captive ball hits charge the heart twice as fast
  FasterHeartCharge,
}

impl Perk {
  /// The district that gives it
  pub fn district(&self) -> Mode {
    match self {
      Self::DoubleTunnels => Mode::MeridianBasins,
      Self::LongerTimers => Mode::HydroCore,
      Self::FasterPopCharge => Mode::NimbusPromenade,
      Self::QuickLocks => Mode::SkyrailStation,
      Self::DoubleCenterSpinner => Mode::ApexTerraces,
      Self::FasterHeartCharge => Mode::SolariumAtrium,
    }
  }
}

pub fn music_file(file: &str) -> PathBuf {
  PathBuf::from(MUSIC_FOLDER).join(file)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn districts_have_lines_for_their_start_and_completion() {
    for mode in Mode::DISTRICTS {
      assert!(!mode.start_lines().is_empty());
      assert!(!mode.completion_lines().is_empty());
    }
  }

  #[test]
  fn groundswell_stacks_with_districts_but_not_mini_wizards() {
    assert!(Multiball::Groundswell.stacks_with(Mode::SkyrailStation));
    assert!(!Multiball::Groundswell.stacks_with(Mode::Floodgates));
  }

  #[test]
  fn heartwood_stacks_with_any_mode() {
    assert!(Multiball::Heartwood.stacks_with(Mode::HydroCore));
    assert!(Multiball::Heartwood.stacks_with(Mode::Canopy));
  }

  #[test]
  fn districts_have_inserts_and_mini_wizards_share_their_districts_color() {
    assert!(Mode::DISTRICTS.iter().all(|mode| mode.map_led().is_some()));
    assert!(Mode::Blackout.map_led().is_none());
    assert_eq!(Mode::Blackout.color(), Mode::HydroCore.color());
    assert_eq!(Mode::Canopy.color(), Rgba::gold());
  }
}
