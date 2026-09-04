use frontbox::prelude::*;
use frontbox_pin2dmd::{InitialsEntered, InitialsEntrySystem};
use frontbox_sound::SoundSystem;
use frontbox_turn_based::{GameEnded, HighScoreSlot, HighScoresSystem};

use crate::hardware::cabinet;
use crate::{GameManager, PlayerTurnEnding, systems::sounds::LANE_HIT_COMPLETE};

#[derive(Default)]
pub struct EndOfBallSystem {
  initials_needed: Vec<(HighScoreSlot, &'static str, u32)>,
  game_end_event: Option<GameEnded>,
}

impl EndOfBallSystem {
  pub fn new() -> Self {
    Self {
      initials_needed: Vec::new(),
      game_end_event: None,
    }
  }

  fn on_end_of_ball(&self, ctx: &SystemContext) {
    let mut sound = ctx.expect::<SoundSystem>();
    sound.stop_music(Duration::from_millis(500));
    sound.play_sfx(LANE_HIT_COMPLETE);

    // TODO: DMD bonus or something

    ctx.expect::<GameManager>().advance_turn(ctx.into());
  }

  fn on_game_end(&mut self, scores: &Vec<(&'static str, u32)>, ctx: &SystemContext) {
    if let Some(high_scores) = ctx.get::<HighScoresSystem>() {
      self.initials_needed = scores
        .iter()
        .filter_map(|(name, score)| {
          high_scores
            .is_high_score(*score)
            .map(|slot| (slot, *name, *score))
        })
        .collect();
      self.get_next_initials(ctx);
    }
  }

  fn get_next_initials(&mut self, ctx: &SystemContext) {
    if self.initials_needed.len() > 0 {
      ctx.spawn_system(InitialsEntrySystem::new(
        self.initials_needed[0].1,
        cabinet::LEFT_FLIPPER_SWITCH1.q(),
        cabinet::RIGHT_FLIPPER_SWITCH1.q(),
        SwitchQ::any(vec![
          &cabinet::start_button::SWITCH.q(),
          &cabinet::action_button::SWITCH.q(),
        ]),
      ));
    } else if let Some(event) = self.game_end_event.as_ref() {
      ctx.emit(event.clone());
      self.game_end_event = None;
    }
  }
}

impl System for EndOfBallSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.register_interrupt::<GameEnded>(1);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<PlayerTurnEnding>() {
      self.on_end_of_ball(ctx);
    } else if let Some(event) = event.downcast_ref::<InitialsEntered>() {
      if let Some(first) = self.initials_needed.pop()
        && let Some(mut high_scores) = ctx.get::<HighScoresSystem>()
      {
        high_scores.set_high_score(first.0, event.initials.clone(), first.2);
      }
      self.get_next_initials(ctx);
    }
  }

  fn on_interrupt(&mut self, event: &dyn Event, ctx: &SystemContext) -> InterruptResult {
    if let Some(event) = event.downcast_ref::<GameEnded>() {
      self.on_game_end(&event.scores, ctx);
      self.game_end_event = Some(event.clone());
      InterruptResult::Halt
    } else {
      InterruptResult::Continue
    }
  }
}
