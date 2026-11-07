use frontbox_core::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};

use crate::hardware::center_orbit::CenterSpinnerHit;
use crate::systems::game::{ModeManager, Perk, points};

/// The center spinner's base value for each spin, doubled once Apex is completed
#[derive(Clone)]
pub struct CenterSpinnerScoring;

impl CenterSpinnerScoring {
  pub fn new() -> Self {
    Self
  }
}

impl System for CenterSpinnerScoring {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CenterSpinnerHit>() {
      let multiplier = if ctx
        .systems
        .expect::<ModeManager>()
        .has_perk(Perk::DoubleCenterSpinner)
      {
        2
      } else {
        1
      };
      ctx.add_points(points::CENTER_SPIN * multiplier);
    }
  }
}
