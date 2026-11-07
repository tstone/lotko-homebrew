use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameStarting, TurnActive, TurnStartReason, TurnStarting};

use crate::systems::{Callout, SayExt};

/// Opens the game, and each player's first turn with a scene as their ball is served. At the root, as no player is up
/// yet when it starts.
pub struct GameGreeting {
  first_turn_effect: LedPlayback,
}

impl GameGreeting {
  pub fn new() -> Self {
    Self {
      first_turn_effect: led_playback!("first_turn.scene").stopped(),
    }
  }
}

impl System for GameGreeting {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<RoutineStarted<GameStarting>>() {
      ctx.say(Callout::GameStart);
    } else if let Some(RoutineCompleted { routine, .. }) =
      event.downcast_ref::<RoutineCompleted<TurnStarting>>()
    {
      // an extra ball keeps the turn it's played on, so only a turn in order can be the first
      if routine.turn == 0 && routine.reason == TurnStartReason::Ordered {
        self.first_turn_effect.reset();
        self.first_turn_effect.play();
      }
    } else if event.is::<TurnActive>() {
      self.first_turn_effect.stop(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.first_turn_effect.apply(delta, ctx);
  }
}
