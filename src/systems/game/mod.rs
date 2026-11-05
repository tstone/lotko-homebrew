mod apex_terraces;
mod bonus;
mod end_of_ball;
mod exclusive_mode;
mod exclusive_mode_qualification;
mod extra_ball;
mod hydro_core;
mod left_scoop_startable;
mod lift_ramp_startable;
mod meridian_basins;
mod mode_manager;
mod nimbus_promenade;
mod playfield_illumination;
mod skill_shots;
mod skyrail_station;
mod solarium_atrium;
mod spore_multiball;
mod spores;
mod tilt_presenter;

pub use apex_terraces::*;
pub use bonus::*;
pub use end_of_ball::*;
pub use exclusive_mode::*;
pub use exclusive_mode_qualification::*;
pub use extra_ball::*;
pub use hydro_core::*;
pub use left_scoop_startable::*;
pub use lift_ramp_startable::*;
pub use meridian_basins::*;
pub use mode_manager::*;
pub use nimbus_promenade::*;
pub use playfield_illumination::*;
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
