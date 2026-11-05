use frontbox_core::prelude::*;

use crate::metrics;
use crate::systems::game::SporeApplied;

/// Counts the spores applied this game
#[derive(Clone)]
pub struct SporesSystem {
  count: u32,
}

impl SporesSystem {
  pub fn new() -> Self {
    Self { count: 0 }
  }

  pub fn count(&self) -> u32 {
    self.count
  }
}

impl System for SporesSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<SporeApplied>() {
      self.count += 1;
      ctx.metrics.inc(&metrics::SPORES_APPLIED);
    }
  }
}
