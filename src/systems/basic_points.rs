use frontbox::prelude::*;
use frontbox::tags::{Playfield, SlingShot};
use frontbox_turn_based::GameManager;

/// A system to give a low amount of points just for the ball bouncing around
#[derive(Debug, Clone)]
pub struct BasicPoints;

impl BasicPoints {
  pub fn new() -> Self {
    Self
  }
}

impl System for BasicPoints {
  fn on_event(&mut self, event: &dyn Event, ctx: &Context, systems: &Systems) {
    if let Some(e) = event.downcast_ref::<SwitchClosed>() {
      if e.switch.has_tag::<SlingShot>() {
        systems.expect_mut::<GameManager>().add_points(100, ctx);
      } else if e.switch.has_tag::<Playfield>() {
        systems.expect_mut::<GameManager>().add_points(10, ctx);
      }
    }
  }
}
