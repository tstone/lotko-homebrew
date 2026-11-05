use frontbox_core::prelude::*;
use frontbox_core::tags::Playfield;
use frontbox_pinball::{GameManager, TurnPhase};

/// A system to give a low amount of points just for the ball bouncing around
#[derive(Debug, Clone)]
pub struct BasicPoints;

impl BasicPoints {
  pub fn new() -> Self {
    Self
  }
}

impl System for BasicPoints {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(e) = event.downcast_ref::<SwitchClosed>() {
      if e.switch.has_tag::<Playfield>() {
        ctx
          .systems
          .expect::<GameManager>()
          .add_points(110, ctx.into());
      }
    }
  }
}
