use std::path::PathBuf;

use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;

use crate::hardware::city_map;
use crate::systems::{Callout, sounds};
use crate::systems::game::{
  ApexTerracesMode, CalderaGeneratorMode, CargoQuayMode, HalcyonVentsMode, GroundswellMode, HeartwoodMultiballMode,
  HelixVaultsMode, HydroCoreMode, MeridianBasinsMode, NimbusPromenadeMode, SkyrailStationMode, SolariumAtriumMode,
};

const MUSIC_FOLDER: &str = "/userdata/home/armsom/music";

/// The Helix Vaults' growth medium, a neon yellow-green
const HELIX_VAULTS_COLOR: Rgba<u8> = Rgba([170, 255, 0, 255]);

/// Meridian Basins' grow lights, a yellow between Helix Vaults' yellow-green and Apex's gold
const MERIDIAN_BASINS_COLOR: Rgba<u8> = Rgba([255, 220, 0, 255]);

/// The Caldera Generator's embers, a deeper orange than Cargo Quay's
const CALDERA_GENERATOR_COLOR: Rgba<u8> = Rgba([255, 75, 0, 255]);

/// Halcyon Vents' clean air, a pale sky blue
const HALCYON_VENTS_COLOR: Rgba<u8> = Rgba([120, 200, 255, 255]);

/// A stop on the route up the city (DESIGN.md, Primary progression): a district or a mini-wizard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Mode {
  MeridianBasins,
  CargoQuay,
  HydroCore,
  NimbusPromenade,
  SkyrailStation,
  ApexTerraces,
  SolariumAtrium,
  HelixVaults,
  CalderaGenerator,
  Groundswell,
  HalcyonVents,
}

impl Mode {
  pub const DISTRICTS: [Mode; 7] = [
    Self::MeridianBasins,
    Self::CargoQuay,
    Self::HydroCore,
    Self::NimbusPromenade,
    Self::SkyrailStation,
    Self::ApexTerraces,
    Self::SolariumAtrium,
  ];

  pub fn name(&self) -> &'static str {
    match self {
      Self::MeridianBasins => "MERIDIAN BASINS",
      Self::CargoQuay => "CARGO QUAY",
      Self::HydroCore => "HYDRO CORE",
      Self::NimbusPromenade => "NIMBUS PROMENADE",
      Self::SkyrailStation => "SKYRAIL STATION",
      Self::ApexTerraces => "APEX TERRACES",
      Self::SolariumAtrium => "SOLARIUM ATRIUM",
      Self::HelixVaults => "HELIX VAULTS",
      Self::CalderaGenerator => "CALDERA GENERATOR",
      Self::Groundswell => "GROUNDSWELL",
      Self::HalcyonVents => "HALCYON VENTS",
    }
  }

  /// The mode's screen in the mode select menu, from the content folder
  pub fn menu_scene(&self) -> &'static str {
    match self {
      Self::MeridianBasins => "dmd/mode_select/meridian_basins.scene",
      Self::CargoQuay => "dmd/mode_select/cargo_quay.scene",
      Self::HydroCore => "dmd/mode_select/hydro_core.scene",
      Self::NimbusPromenade => "dmd/mode_select/nimbus_promenade.scene",
      Self::SkyrailStation => "dmd/mode_select/skyrail_station.scene",
      Self::ApexTerraces => "dmd/mode_select/apex_terraces.scene",
      Self::SolariumAtrium => "dmd/mode_select/solarium_atrium.scene",
      Self::HelixVaults => "dmd/mode_select/helix_vaults.scene",
      Self::CalderaGenerator => "dmd/mode_select/caldera_generator.scene",
      Self::Groundswell => "dmd/mode_select/groundswell.scene",
      Self::HalcyonVents => "dmd/mode_select/halcyon_vents.scene",
    }
  }

  /// Lit by play rather than reached on the route: Groundswell, from spreading roots. It needs no qualifying, closes
  /// nothing on the route, and can be earned again once it's completed.
  pub fn is_earned(&self) -> bool {
    *self == Self::Groundswell
  }

  pub fn is_mini_wizard(&self) -> bool {
    matches!(
      self,
      Self::HelixVaults | Self::CalderaGenerator | Self::Groundswell | Self::HalcyonVents
    )
  }

  /// A mini-wizard ends when play is back to one ball, except one that carries on until it's completed or the turn ends
  pub fn ends_with_multiball(&self) -> bool {
    *self != Self::HelixVaults
  }

  /// Uses the drop targets or puts more balls in play, so the Sentinels wait for it to end: every mini-wizard
  /// (multiball)
  pub fn uses_drops_or_balls(&self) -> bool {
    self.is_mini_wizard()
  }


  /// The step on the route it's on (DESIGN.md, Primary progression), from 1, which sets what it's worth
  pub fn step(&self) -> u8 {
    match self {
      Self::MeridianBasins | Self::CargoQuay => 1,
      Self::NimbusPromenade => 2,
      Self::HydroCore | Self::HelixVaults | Self::SkyrailStation => 3,
      // earned from spreading roots rather than on the route, and worth what tier 3 is
      Self::ApexTerraces | Self::HalcyonVents | Self::Groundswell => 4,
      Self::CalderaGenerator => 5,
      Self::SolariumAtrium => 6,
    }
  }

  pub fn color(&self) -> Rgba<u8> {
    match self {
      Self::MeridianBasins => MERIDIAN_BASINS_COLOR,
      Self::CargoQuay => Rgba::orange(),
      Self::NimbusPromenade => Rgba::magenta(),
      Self::HydroCore => Rgba::cyan(),
      Self::CalderaGenerator => CALDERA_GENERATOR_COLOR,
      Self::HelixVaults => HELIX_VAULTS_COLOR,
      Self::SkyrailStation => Rgba::blue(),
      Self::Groundswell => Rgba::bronze(),
      Self::ApexTerraces => Rgba::gold(),
      Self::HalcyonVents => HALCYON_VENTS_COLOR,
      Self::SolariumAtrium => Rgba::purple(),
    }
  }

  /// The mode's insert on the city map. Mini-wizards don't have one, as the map's six are the districts.
  pub fn map_led(&self) -> Option<LightQ> {
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
      Self::CargoQuay => "charlie-gearon-modular-idm-01.mp3",
      Self::HalcyonVents => "STRLGHT-rave.mp3",
      Self::SkyrailStation => "deadmau5-arguru2k19.mp3",
      Self::SolariumAtrium => "fehrplay-arcadia.mp3",
      Self::MeridianBasins => "glitch-mob-cant-kill-us.mp3",
      Self::NimbusPromenade => "wice-5omething.mp3",
      Self::CalderaGenerator => "65daysofstatic-crash-tactics.mp3",
      Self::Groundswell => "inzo-im-dreaming.mp3",
      Self::HelixVaults => "substan-syntax.mp3",
      _ => return None,
    };
    Some(music_file(file))
  }

  /// The sound that plays with the mode's intro, for those that have one so far
  pub fn start_sound(&self) -> Option<&'static str> {
    match self {
      Self::MeridianBasins => Some(sounds::PRESSURE_RELEASE_LIGHT),
      _ => None,
    }
  }

  /// What's said as the mode starts, in order. The ball waits in the scoop for the last of them. A mini-wizard is the
  /// forest's, so the Oaks have their say.
  pub fn start_lines(&self) -> &'static [Callout] {
    match self {
      Self::MeridianBasins => &[Callout::CropContaminationDetected],
      // stand-in until it has its own (TODO.md)
      Self::CargoQuay => &[Callout::BiologicalContaminantIdentified],
      Self::HydroCore => &[Callout::CoolantCoreDropping],
      Self::NimbusPromenade => &[Callout::PromenadeClosed, Callout::WalkwaysWillBeSealed],
      Self::SkyrailStation => &[Callout::SkyrailServiceSuspended, Callout::ClearThePlatform],
      Self::ApexTerraces => &[Callout::ResidentsEvacuateNow],
      Self::SolariumAtrium => &[Callout::AtriumIntegrityVerified],
      Self::HelixVaults => &[Callout::ContainmentNominal],
      Self::CalderaGenerator => &[Callout::OakDarkness],
      Self::Groundswell => &[Callout::SeismicEventDetected, Callout::OakRise],
      Self::HalcyonVents => &[Callout::OakHigher],
    }
  }

  /// What's said as the mode is completed: the Steward's line, then the Oaks' for the districts that have one
  pub fn completion_lines(&self) -> &'static [Callout] {
    match self {
      Self::MeridianBasins => &[Callout::MeridianBasinsUninhabitable, Callout::OakOurs],
      // the Steward's line is still to record (TODO.md)
      Self::CargoQuay => &[Callout::OakFall],
      Self::HydroCore => &[Callout::CoreOffline, Callout::OakDrink],
      Self::NimbusPromenade => &[
        Callout::AccessToUpperCityRestricted,
        Callout::OakTheWayIsOpen,
      ],
      Self::SkyrailStation => &[Callout::AllLinesAreNowClosed],
      Self::ApexTerraces => &[Callout::ApexTerracesOffline, Callout::OakNothingIsYours],
      Self::SolariumAtrium => &[Callout::TheScienceWillNotBeLost, Callout::OakOpenSky],
      Self::HelixVaults => &[Callout::OakOpenTheGates],
      Self::Groundswell => &[Callout::SeismicDamageIdentified],
      Self::CalderaGenerator | Self::HalcyonVents => &[],
    }
  }

  /// Starts the mode's system as a child of the caller, returning its id. It stays around until the mode is completed
  /// so that the mode can be resumed.
  pub fn spawn(&self, ctx: &SystemContext) -> u64 {
    match self {
      Self::HydroCore => ctx.systems.spawn(HydroCoreMode::new()),
      Self::SkyrailStation => ctx.systems.spawn(SkyrailStationMode::new()),
      Self::SolariumAtrium => ctx.systems.spawn(SolariumAtriumMode::new()),
      Self::MeridianBasins => ctx.systems.spawn(MeridianBasinsMode::new()),
      Self::CargoQuay => ctx.systems.spawn(CargoQuayMode::new()),
      Self::NimbusPromenade => ctx.systems.spawn(NimbusPromenadeMode::new()),
      Self::ApexTerraces => ctx.systems.spawn(ApexTerracesMode::new()),
      Self::HelixVaults => ctx.systems.spawn(HelixVaultsMode::new()),
      Self::CalderaGenerator => ctx.systems.spawn(CalderaGeneratorMode::new()),
      Self::Groundswell => ctx.systems.spawn(GroundswellMode::new()),
      Self::HalcyonVents => ctx.systems.spawn(HalcyonVentsMode::new()),
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
}

impl Multiball {
  pub fn color(&self) -> Rgba<u8> {
    match self {
      Self::Heartwood => Rgba::lime(),
    }
  }

  pub fn music(&self) -> Option<PathBuf> {
    match self {
      Self::Heartwood => Some(music_file("rezmau5-infraliminal.mp3")),
    }
  }

  /// Whether the multiball's track replaces a running mode's. When it doesn't, it plays only when no mode is running.
  pub fn music_plays_over_modes(&self) -> bool {
    match self {
      Self::Heartwood => false,
    }
  }

  /// Starts the multiball's system as a child of the caller, for those that have one so far. It despawns itself when
  /// the multiball ends.
  pub fn spawn(&self, ctx: &SystemContext) {
    match self {
      Self::Heartwood => {
        ctx.systems.spawn(HeartwoodMultiballMode::new());
      }
    }
  }

  /// Heartwood stacks with any mode. The other way around, a mode can start under a multiball by the same rule.
  pub fn stacks_with(&self, _mode: Mode) -> bool {
    match self {
      Self::Heartwood => true,
    }
  }
}

/// What completing a district gives the player for the rest of the game (DESIGN.md, Perks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Perk {
  /// Spread roots grow twice as fast
  DoubleRoots,
  /// +5s on mode timers
  LongerTimers,
  /// Pops charge twice as fast
  FasterPopCharge,
  /// Every mode on the route gets one more try
  ExtraTry,
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
      Self::DoubleRoots => Mode::MeridianBasins,
      Self::LongerTimers => Mode::HydroCore,
      Self::FasterPopCharge => Mode::CargoQuay,
      Self::ExtraTry => Mode::NimbusPromenade,
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
  fn heartwood_stacks_with_any_mode() {
    assert!(Multiball::Heartwood.stacks_with(Mode::HydroCore));
    assert!(Multiball::Heartwood.stacks_with(Mode::HalcyonVents));
  }

  #[test]
  fn districts_have_inserts_except_cargo_quay() {
    // Cargo Quay has no insert until the map is replanned
    assert!(
      Mode::DISTRICTS
        .iter()
        .filter(|mode| **mode != Mode::CargoQuay)
        .all(|mode| mode.map_led().is_some())
    );
    assert!(Mode::CalderaGenerator.map_led().is_none());
  }
}
