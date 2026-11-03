use frontbox::prelude::*;
use frontbox_canvas::*;
use frontbox_pin2dmd::*;
use frontbox_pinball::{GameManager, TurnEnding};
use frontbox_sound::SoundSystemExt;

use crate::systems::dmd::TURN_END_DMD_LAYER;
use crate::systems::sounds;

/// A participant in `TurnEnding`: shows the player's score for the turn that's ending on the DMD
#[derive(Clone)]
pub struct TurnEndScoreSystem;

impl TurnEndScoreSystem {
  pub fn new() -> Self {
    Self
  }

  fn on_turn_ending(&self, turn_ending: &TurnEnding, ctx: &SystemContext) {
    let score = ctx
      .get::<GameManager>()
      .and_then(|game| {
        game
          .game_state()
          .and_then(|state| state.current_player_score())
      })
      .unwrap_or(0);

    if let Some(mut dmd) = ctx.get::<DmdSystem>() {
      dmd.insert_layer(
        TURN_END_DMD_LAYER,
        Self::draw(turn_ending.current_player, score).default_position(),
      );
    }
    ctx.play_sfx(sounds::QUEST_COMPLETE);
    ctx.cue(ScoreShown, Duration::from_secs(2).once());
  }

  fn draw(player: u8, score: u32) -> Container {
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
    if let Some(mut dmd) = ctx.get::<DmdSystem>() {
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
    if event.is::<ScoreShown>() {
      self.clear_screen(ctx);
      ctx.complete_routine_step::<TurnEnding>();
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.clear_screen(ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct ScoreShown;
