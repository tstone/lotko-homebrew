use std::path::PathBuf;

use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;

use crate::hardware::city_map;
use crate::systems::{Callout, sounds};
use crate::systems::game::{
  ApexTerracesMode, BlackoutMode, CanopyMode, DerailmentMode, HeartwoodMultiballMode,
  HelixVaultsMode, HydroCoreMode, MeridianBasinsMode, NimbusPromenadeMode, SkyrailStationMode, SolariumAtriumMode,
};

const MUSIC_FOLDER: &str = "/userdata/home/armsom/music";

/// The Helix Vaults' growth medium, a neon yellow-green
const HELIX_VAULTS_COLOR: Rgba<u8> = Rgba([170, 255, 0, 255]);

/// A stop on the route up the city (DESIGN.md, Primary progression): a district or a mini-wizard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Mode {
  MeridianBasins,
  HydroCore,
  NimbusPromenade,
  SkyrailStation,
  ApexTerraces,
  SolariumAtrium,
  HelixVaults,
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
      Self::HelixVaults => "HELIX VAULTS",
      Self::Blackout => "BLACKOUT",
      Self::Derailment => "DERAILMENT",
      Self::Canopy => "CANOPY",
    }
  }

  /// The mode's screen in the mode select menu, from the content folder
  pub fn menu_scene(&self) -> &'static str {
    match self {
      Self::MeridianBasins => "dmd/mode_select/meridian_basins.scene",
      Self::HydroCore => "dmd/mode_select/hydro_core.scene",
      Self::NimbusPromenade => "dmd/mode_select/nimbus_promenade.scene",
      Self::SkyrailStation => "dmd/mode_select/skyrail_station.scene",
      Self::ApexTerraces => "dmd/mode_select/apex_terraces.scene",
      Self::SolariumAtrium => "dmd/mode_select/solarium_atrium.scene",
      Self::HelixVaults => "dmd/mode_select/helix_vaults.scene",
      Self::Blackout => "dmd/mode_select/blackout.scene",
      Self::Derailment => "dmd/mode_select/derailment.scene",
      Self::Canopy => "dmd/mode_select/canopy.scene",
    }
  }

  pub fn is_mini_wizard(&self) -> bool {
    matches!(
      self,
      Self::HelixVaults | Self::Blackout | Self::Derailment | Self::Canopy
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

  /// Tier 1 is the lower city, up to its mini-wizard. Tier 2 is the upper city.
  pub fn tier(&self) -> u8 {
    match self {
      Self::MeridianBasins
      | Self::HydroCore
      | Self::NimbusPromenade
      | Self::HelixVaults
      | Self::Blackout => 1,
      Self::SkyrailStation
      | Self::ApexTerraces
      | Self::SolariumAtrium
      | Self::Derailment
      | Self::Canopy => 2,
    }
  }

  /// The step on the route it's on (DESIGN.md, Primary progression), from 1, which sets what it's worth
  pub fn step(&self) -> u8 {
    match self {
      Self::MeridianBasins | Self::HydroCore => 1,
      Self::NimbusPromenade => 2,
      Self::HelixVaults | Self::Blackout => 3,
      Self::SkyrailStation | Self::ApexTerraces => 4,
      Self::SolariumAtrium => 5,
      Self::Derailment | Self::Canopy => 6,
    }
  }

  /// The district that leads to a mini-wizard
  pub fn district(&self) -> Mode {
    match self {
      Self::HelixVaults => Self::MeridianBasins,
      Self::Blackout => Self::HydroCore,
      Self::Derailment => Self::SkyrailStation,
      Self::Canopy => Self::ApexTerraces,
      district => *district,
    }
  }

  pub fn color(&self) -> Rgba<u8> {
    if *self == Self::HelixVaults {
      return HELIX_VAULTS_COLOR;
    }
    match self.district() {
      Self::HydroCore => Rgba::cyan(),
      Self::SkyrailStation => Rgba::blue(),
      Self::SolariumAtrium => Rgba::purple(),
      Self::MeridianBasins => Rgba::green(),
      Self::NimbusPromenade => Rgba::magenta(),
      _ => Rgba::gold(),
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
      Self::SkyrailStation => "deadmau5-arguru2k19.mp3",
      Self::SolariumAtrium => "fehrplay-arcadia.mp3",
      Self::MeridianBasins => "glitch-mob-cant-kill-us.mp3",
      Self::NimbusPromenade => "wice-5omething.mp3",
      Self::Blackout => "65daysofstatic-crash-tactics.mp3",
      Self::Derailment => "colyn-cyclone.mp3",
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
      Self::HydroCore => &[Callout::CoolantCoreDropping],
      Self::NimbusPromenade => &[Callout::PromenadeClosed, Callout::WalkwaysWillBeSealed],
      Self::SkyrailStation => &[Callout::SkyrailServiceSuspended, Callout::ClearThePlatform],
      Self::ApexTerraces => &[Callout::ResidentsEvacuateNow],
      Self::SolariumAtrium => &[Callout::AtriumIntegrityVerified],
      Self::HelixVaults => &[Callout::ContainmentNominal],
      Self::Blackout => &[Callout::OakDarkness],
      Self::Derailment => &[Callout::TrainDerailmentSubverting, Callout::OakFall],
      Self::Canopy => &[Callout::OakHigher],
    }
  }

  /// What's said as the mode is completed: the Steward's line, then the Oaks' for the districts that have one
  pub fn completion_lines(&self) -> &'static [Callout] {
    match self {
      Self::MeridianBasins => &[Callout::MeridianBasinsUninhabitable, Callout::OakOurs],
      Self::HydroCore => &[Callout::CoreOffline, Callout::OakDrink],
      Self::NimbusPromenade => &[
        Callout::AccessToUpperCityRestricted,
        Callout::OakTheWayIsOpen,
      ],
      Self::SkyrailStation => &[Callout::AllLinesAreNowClosed],
      Self::ApexTerraces => &[Callout::ApexTerracesOffline, Callout::OakNothingIsYours],
      Self::SolariumAtrium => &[Callout::TheScienceWillNotBeLost, Callout::OakOpenSky],
      Self::HelixVaults => &[Callout::OakOpenTheGates],
      Self::Blackout | Self::Derailment | Self::Canopy => &[],
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
      Self::NimbusPromenade => ctx.systems.spawn(NimbusPromenadeMode::new()),
      Self::ApexTerraces => ctx.systems.spawn(ApexTerracesMode::new()),
      Self::HelixVaults => ctx.systems.spawn(HelixVaultsMode::new()),
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
  // Comes with the Groundswell lock (design/multiballs.md)
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
      Self::Groundswell => Some(music_file("inzo-im-dreaming.mp3")),
    }
  }

  /// Whether the multiball's track replaces a running mode's. When it doesn't, it plays only when no mode is running.
  pub fn music_plays_over_modes(&self) -> bool {
    match self {
      Self::Heartwood => false,
      Self::Groundswell => true,
    }
  }

  /// Starts the multiball's system as a child of the caller, for those that have one so far. It despawns itself when
  /// the multiball ends.
  pub fn spawn(&self, ctx: &SystemContext) {
    match self {
      Self::Heartwood => {
        ctx.systems.spawn(HeartwoodMultiballMode::new());
      }
      Self::Groundswell => {}
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
  /// Spread roots grow twice as fast
  DoubleRoots,
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
      Self::DoubleRoots => Mode::MeridianBasins,
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
    assert!(!Multiball::Groundswell.stacks_with(Mode::HelixVaults));
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
