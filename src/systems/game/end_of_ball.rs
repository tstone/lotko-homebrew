use frontbox_core::prelude::*;
use frontbox_dmd::{InitialsEntered, InitialsEntrySystem};
use frontbox_pinball::{GameEndReason, GameEnding, HighScoreSlot, HighScoresSystem, TurnEnding};
use frontbox_sound::SoundSystem;

use crate::hardware::cabinet;
use crate::systems::sounds::LANE_HIT_COMPLETE;

/// A participant in `TurnEnding` (end-of-ball sound) and `GameEnding` (high-score initials entry, skipped after a
/// slam tilt)
#[derive(Default)]
pub struct EndOfBallSystem {
  /// Players still to enter initials, in player order
  initials_needed: Vec<(HighScoreSlot, &'static str, u32)>,
}

impl EndOfBallSystem {
  pub fn new() -> Self {
    Self {
      initials_needed: Vec::new(),
    }
  }

  fn on_end_of_ball(&self, ctx: &SystemContext) {
    let mut sound = ctx.systems.expect::<SoundSystem>();
    sound.stop_music(Duration::from_millis(500));
    sound.play_sfx(LANE_HIT_COMPLETE);

    // TODO: DMD bonus or something

    ctx.routines.complete_step::<TurnEnding>();
  }

  fn on_game_end(&mut self, scores: &[(&'static str, u32)], ctx: &SystemContext) {
    self.initials_needed = match ctx.systems.get::<HighScoresSystem>() {
      Some(high_scores) => scores
        .iter()
        .filter_map(|(name, score)| {
          high_scores
            .is_high_score(*score)
            .map(|slot| (slot, *name, *score))
        })
        .collect(),
      None => Vec::new(),
    };
    self.get_next_initials(ctx);
  }

  fn get_next_initials(&mut self, ctx: &SystemContext) {
    if let Some(&(_, name, _)) = self.initials_needed.first() {
      ctx.systems.spawn(InitialsEntrySystem::new(
        name,
        cabinet::LEFT_FLIPPER_SWITCH1.q(),
        cabinet::RIGHT_FLIPPER_SWITCH1.q(),
        SwitchQ::any(vec![
          &cabinet::start_button::SWITCH.q(),
          &cabinet::action_button::SWITCH.q(),
        ]),
      ));
    } else {
      ctx.routines.complete_step::<GameEnding>();
    }
  }
}

impl System for EndOfBallSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Continue>() {
      self.on_end_of_ball(ctx);
    } else if let Some(event) = event.downcast_ref::<InitialsEntered>()
      && !self.initials_needed.is_empty()
    {
      let (slot, _, score) = self.initials_needed.remove(0);
      if let Some(mut high_scores) = ctx.systems.get::<HighScoresSystem>() {
        high_scores.set_high_score(slot, event.initials.clone(), score);
      }
      self.get_next_initials(ctx);
    }
  }

  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    if routine.is::<TurnEnding>() {
      // insert a slice delay to let hardware settle and final points come in
      ctx.cues.start(Continue, Duration::from_millis(500).once());
    } else if let Some(GameEnding { scores, reason }) = routine.downcast_ref::<GameEnding>() {
      if *reason == GameEndReason::SlamTilted {
        log::info!("No high score entry after a slam tilt");
        ctx.routines.complete_step::<GameEnding>();
      } else {
        self.on_game_end(scores, ctx);
      }
    }
  }
}

#[derive(serde::Serialize, Event)]
struct Continue;
