use frontbox_core::prelude::*;
use frontbox_pinball::GameStarting;

use crate::systems::{Callout, SayExt};

/// Opens the game. At the root, as no player is up yet when it starts.
pub struct GameGreeting;

impl GameGreeting {
  pub fn new() -> Self {
    Self
  }
}

impl System for GameGreeting {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<RoutineStarted<GameStarting>>() {
      ctx.say(Callout::GameStart);
    }
  }
}
