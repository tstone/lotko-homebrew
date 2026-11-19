use frontbox_core::prelude::*;
use frontbox_gfx::scene::Value;
use frontbox_pinball::{GameManager, TurnEnding};
use frontbox_sound::SoundSystemExt;

use crate::hardware::cabinet::DoubleFlipped;
use crate::systems::dmd::{DmdScreen, TURN_END_DMD_LAYER};
use crate::systems::sounds;

/// A participant in `TurnEnding`: shows the player's score for the turn that's ending on the DMD. A double flip moves
/// on early.
#[derive(Clone)]
pub struct TurnEndScoreSystem {
  /// The timer for moving off the score, while it's shown
  score_timer: Option<u64>,
  screen: DmdScreen,
}

impl TurnEndScoreSystem {
  pub fn new() -> Self {
    Self { score_timer: None, screen: DmdScreen::new("dmd/turn_end_score.scene", TURN_END_DMD_LAYER) }
  }

  fn on_turn_ending(&mut self, turn_ending: &TurnEnding, ctx: &SystemContext) {
    let score = ctx
      .systems
      .get::<GameManager>()
      .and_then(|game| {
        game
          .session()
          .and_then(|state| state.current_player_score())
      })
      .unwrap_or(0);

    let label = format!("PLAYER {}", turn_ending.current_player + 1);
    self.screen.show([("label", label.into()), ("score", Value::Int(score as i64))], ctx);
    ctx.play_sfx(sounds::QUEST_COMPLETE);
    self.score_timer = Some(ctx.timers.start(ScoreShown, Duration::from_secs(2).once()));
  }

  /// Clears the score and completes this step of the turn ending
  fn complete(&mut self, ctx: &SystemContext) {
    ctx.timers.cancel(self.score_timer.take());
    self.clear_screen(ctx);
    ctx.routines.complete_step::<TurnEnding>();
  }

  fn clear_screen(&mut self, ctx: &SystemContext) {
    self.screen.clear(ctx);
  }
}

impl System for TurnEndScoreSystem {
  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    if let Some(turn_ending) = routine.downcast_ref::<TurnEnding>() {
      self.on_turn_ending(turn_ending, ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    // the score isn't up until this step starts, so a double flip that skipped the bonus doesn't skip it too
    if self.score_timer.is_none() {
      return;
    }

    if event.is::<ScoreShown>() {
      self.complete(ctx);
    } else if event.is::<DoubleFlipped>() {
      log::info!("Turn end score: skipped");
      self.complete(ctx);
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.clear_screen(ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct ScoreShown;
