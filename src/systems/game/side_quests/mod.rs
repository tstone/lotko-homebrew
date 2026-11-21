mod heartwood;
mod sentinels;
mod spread_roots;
mod surveillance;

pub use heartwood::*;
pub use sentinels::*;
pub use spread_roots::*;
pub use surveillance::*;

use frontbox_core::prelude::*;

/// Sentinels lit a lock. Nothing takes it up now that Groundswell is lit by spreading roots (TODO.md, decisions).
#[derive(serde::Serialize, Event)]
pub struct LockLit;

/// The cameras went a minute without one being blinded, so the Steward sent out its drones and the Sentinels took over.
/// The `ModeManager` slot is already taken when this is heard.
#[derive(serde::Serialize, Event)]
pub struct SentinelsDispatched;

/// Root Lash was struck. The current mode counts it as `hits` shots, if it has any.
#[derive(serde::Serialize, Event)]
pub struct RootLashed {
  pub hits: u8,
}
