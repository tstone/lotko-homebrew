mod heartwood;
mod root_tunnels;
mod sentinels;
mod surveillance;

pub use heartwood::*;
pub use root_tunnels::*;
pub use sentinels::*;
pub use surveillance::*;

use frontbox_core::prelude::*;

/// Sentinels lit a lock, for Groundswell to take up once it exists (PLAN.md, Part 4)
#[derive(serde::Serialize, Event)]
pub struct LockLit;

/// A root tunnel was collected. The current district counts it as one hit of progress, if it has one.
#[derive(serde::Serialize, Event)]
pub struct TunnelCollected;
