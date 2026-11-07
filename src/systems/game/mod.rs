mod apex_terraces;
mod bonus;
mod end_of_ball;
mod effects;
mod extra_ball;
mod game_greeting;
mod hydro_core;
mod meridian_basins;
mod mini_wizard;
mod mode;
mod mode_manager;
mod mode_run;
mod mode_select;
mod nimbus_promenade;
mod playfield_illumination;
mod progression;
mod scoop_qualifier;
mod skill_shots;
mod skyrail_station;
mod solarium_atrium;
mod spore_multiball;
mod spores;
mod tilt_presenter;

pub use apex_terraces::*;
pub use bonus::*;
pub use end_of_ball::*;
pub use effects::*;
pub use extra_ball::*;
pub use game_greeting::*;
pub use hydro_core::*;
pub use meridian_basins::*;
pub use mini_wizard::*;
pub use mode::*;
pub use mode_manager::*;
pub use mode_run::*;
pub use mode_select::*;
pub use nimbus_promenade::*;
pub use playfield_illumination::*;
pub use progression::*;
pub use scoop_qualifier::*;
pub use skill_shots::*;
pub use skyrail_station::*;
pub use solarium_atrium::*;
pub use spore_multiball::*;
pub use spores::*;
pub use tilt_presenter::*;

/// How long a multiball saves drains once its balls are in play
pub const MULTIBALL_BALL_SAVE: std::time::Duration = std::time::Duration::from_secs(12);

/// A spore was applied to the city, which for now means a mode shot was made
#[derive(serde::Serialize, frontbox_core::prelude::Event)]
pub struct SporeApplied;

mod points {
  pub static EXCL_QUAL_HIT: u32 = 50_250;
  pub static EXCL_START: u32 = 350_000;
  pub static EXL_COMPLETION: u32 = 50_000_000;
  pub static EXL_MODE_HIT: u32 = 150_000;
  pub static BONUS_PER_CITY_MODE: u32 = 5_000_000;
  pub static BONUS_PER_SPORE: u32 = 15_000;
  /// Awarded for collecting an extra ball once the player is at the limit
  pub static EXTRA_BALL_AS_POINTS: u32 = 10_000_000;
}
