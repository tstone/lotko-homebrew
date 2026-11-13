use frontbox_core::prelude::*;
use frontbox_dmd::*;
use frontbox_gfx::canvas::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManager, TurnEnding};
use frontbox_sound::SoundSystemExt;

use crate::hardware::cabinet::DoubleFlipped;
use crate::systems::dmd::TURN_END_DMD_LAYER;
use crate::systems::sounds;

/// A participant in `TurnEnding`: shows the player's score for the turn that's ending on the DMD. A double flip moves
/// on early.
#[derive(Clone)]
pub struct TurnEndScoreSystem {
  /// The cue for moving off the score, while it's shown
  score_cue: Option<u64>,
}

impl TurnEndScoreSystem {
  pub fn new() -> Self {
    Self { score_cue: None }
  }

  fn on_turn_ending(&mut self, turn_ending: &TurnEnding, ctx: &SystemContext) {
    let score = ctx
      .systems
      .get::<GameManager>()
      .and_then(|game| {
        game
          .game_state()
          .and_then(|state| state.current_player_score())
      })
      .unwrap_or(0);

    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      dmd.insert_layer(
        TURN_END_DMD_LAYER,
        Self::draw(turn_ending.current_player, score).default_position(),
      );
    }
    ctx.play_sfx(sounds::QUEST_COMPLETE);
    self.score_cue = Some(ctx.cues.start(ScoreShown, Duration::from_secs(2).once()));
  }

  /// Clears the score and completes this step of the turn ending
  fn complete(&mut self, ctx: &SystemContext) {
    ctx.cues.cancel(self.score_cue.take());
    self.clear_screen(ctx);
    ctx.routines.complete_step::<TurnEnding>();
  }

  fn draw(player: u8, score: u64) -> Container {
    let mut frame = Container::new(Fill2d::Solid(Rgba::black()));
    frame.add(
      SIGI_BOLD_7PX_FONT
        .center_aligned(
          format!("PLAYER {}", player + 1),
          Rgba::yellow().lighten(0.3),
        )
        .top_offset(4),
    );
    frame.add(
      BOLD_PIXELS_10PX_REGULAR_FONT
        .center_aligned(TextFormatting::number(score), Rgba::white())
        .top_offset(15),
    );
    frame
  }

  fn clear_screen(&self, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      dmd.remove_layer(TURN_END_DMD_LAYER);
    }
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
    if self.score_cue.is_none() {
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
