mod heartwood;
mod sentinels;
mod spread_roots;
mod surveillance;

pub use heartwood::*;
pub use sentinels::*;
pub use spread_roots::*;
pub use surveillance::*;

use frontbox_core::prelude::*;

/// Sentinels lit a lock, for Groundswell to take up once it exists (PLAN.md, Part 4)
#[derive(serde::Serialize, Event)]
pub struct LockLit;

/// The second camera went dark, so the Steward sends out its drones and Sentinels starts
#[derive(serde::Serialize, Event)]
pub struct SentinelsDeployed;

/// Root Lash was struck. The current mode counts it as `hits` shots, if it has any.
#[derive(serde::Serialize, Event)]
pub struct RootLashed {
  pub hits: u8,
}
