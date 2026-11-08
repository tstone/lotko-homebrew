mod apex_terraces;
mod arc_streak;
mod bonus;
mod center_spinner;
mod combos;
mod effects;
mod end_of_ball;
mod extra_ball;
mod game_greeting;
mod heartwood_multiball;
mod hydro_core;
mod meridian_basins;
mod mini_wizards;
mod mode;
mod mode_manager;
mod mode_run;
mod mode_select;
mod nimbus_promenade;
mod outlane_save;
mod playfield_illumination;
mod progression;
mod scoop_qualifier;
mod side_quests;
mod skill_shots;
mod skyrail_station;
mod solarium_atrium;
mod spores;
mod tilt_presenter;

pub use apex_terraces::*;
pub use arc_streak::*;
pub use bonus::*;
pub use center_spinner::*;
pub use combos::*;
pub use effects::*;
pub use end_of_ball::*;
pub use extra_ball::*;
pub use game_greeting::*;
pub use heartwood_multiball::*;
pub use hydro_core::*;
pub use meridian_basins::*;
pub use mini_wizards::*;
pub use mode::*;
pub use mode_manager::*;
pub use mode_run::*;
pub use mode_select::*;
pub use nimbus_promenade::*;
pub use outlane_save::*;
pub use playfield_illumination::*;
pub use progression::*;
pub use scoop_qualifier::*;
pub use side_quests::*;
pub use skill_shots::*;
pub use skyrail_station::*;
pub use solarium_atrium::*;
pub use spores::*;
pub use tilt_presenter::*;

/// How long a multiball saves drains once its balls are in play
pub const MULTIBALL_BALL_SAVE: std::time::Duration = std::time::Duration::from_secs(12);

mod points {
  use super::Mode;

  /// A district shot, by the district's step (DESIGN.md, Scoring)
  pub fn mode_hit(mode: Mode) -> u32 {
    match mode.step() {
      1 => 1_500_000,
      2 => 2_250_000,
      4 => 3_500_000,
      _ => 5_000_000,
    }
  }

  /// Completing a district or a mini-wizard, by its step (DESIGN.md, Scoring)
  pub fn mode_completion(mode: Mode) -> u32 {
    match mode.step() {
      1 => 35_000_000,
      2 => 50_000_000,
      3 => 110_000_000,
      4 => 70_000_000,
      5 => 90_000_000,
      _ => 225_000_000,
    }
  }

  pub static EXCL_QUAL_HIT: u32 = 50_250;
  pub static EXCL_START: u32 = 350_000;
  pub static BONUS_PER_CITY_MODE: u32 = 5_000_000;
  pub static BONUS_PER_SPORE: u32 = 15_000;
  /// Awarded for collecting an extra ball once the player is at the limit
  pub static EXTRA_BALL_AS_POINTS: u32 = 10_000_000;
  /// Each spin of the center spinner, doubled by the Apex perk
  pub static CENTER_SPIN: u32 = 10_000;
  pub static SKILL_SHOT: u32 = 500_000;
  /// The drop bank skill shot's consolation for hitting the bank but missing the lit target
  pub static SKILL_SHOT_MISSED: u32 = 50_000;
  pub static SKYRAIL_SCOOP_ENTER: u32 = 500;
  /// Each hit on Apex's lit terrace, below its step's shot as it takes 12
  pub static APEX_HIT: u32 = 2_000_000;
  /// Bursting a terrace, times how many have burst
  pub static APEX_BURST: u32 = 2_000_000;
  /// Each orbit hit raising Floodgates' water
  pub static FLOOD_HIT: u32 = 1_000_000;
  /// A ramp at high water, times how many have been made
  pub static FLOOD_SUPER_JACKPOT: u32 = 20_000_000;
  /// Each shot Blackout puts out
  pub static BLACKOUT_HIT: u32 = 4_000_000;
  pub static BLACKOUT_SUPER_JACKPOT: u32 = 120_000_000;
  /// Each drop target in Derailment, times the bank clears so far plus one
  pub static DERAIL_JACKPOT: u32 = 3_000_000;
  /// Each bank clear in Derailment, times how many there have been
  pub static DERAIL_SUPER_JACKPOT: u32 = 16_000_000;
  /// What Canopy's value builds from, and starts over at after a collect
  pub static CANOPY_BASE: u32 = 25_000_000;
  /// Each spin of either spinner, onto Canopy's value
  pub static CANOPY_SPIN: u32 = 250_000;
  /// Each pop, onto Canopy's value
  pub static CANOPY_POP: u32 = 1_000_000;
  /// Each hit of a Root Lash, onto Canopy's value
  pub static CANOPY_ROOTS: u32 = 5_000_000;
  pub static HEARTWOOD_JACKPOT: u32 = 5_000_000;
  pub static HEARTWOOD_SUPER_JACKPOT: u32 = 25_000_000;
  /// Each captive ball hit toward the multiball
  pub static HEARTWOOD_CHARGE: u32 = 50_000;
  /// A spore burst with no mode running for it to double
  pub static BURST: u32 = 2_500_000;
  pub static SURVEILLANCE_HIT: u32 = 25_000;
  pub static SURVEILLANCE_SET: u32 = 500_000;
  pub static DRONE_DOWN: u32 = 250_000;
  /// Each drop target before the Sentinels are deployed
  pub static DROP_TARGET_UNDEPLOYED: u32 = 10_000;
  /// A bank clear before the Sentinels are deployed
  pub static BANK_CLEAR_UNDEPLOYED: u32 = 100_000;
  /// 5 drones down
  pub static SENTINELS_FIRST_AWARD: u32 = 5_000_000;
  /// 15 drones down, until DESIGN.md settles what it is
  pub static SENTINELS_THIRD_AWARD: u32 = 15_000_000;
  pub static ROOT_SEGMENT: u32 = 25_000;
  pub static ROOTS_SPREAD: u32 = 1_000_000;
  /// A 2 shot route
  pub static COMBO: u32 = 1_000_000;
  /// A 3 shot route
  pub static COMBO_LONG: u32 = 2_500_000;
  pub static GRIDLOCK: u32 = 25_000_000;
  /// ArcStreak reflex shots: the first, the second, and each after
  pub static ARC_STREAK: [u32; 3] = [2_000_000, 3_000_000, 5_000_000];
}
