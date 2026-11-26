use frontbox_core::prelude::*;
use frontbox_display::{InitialsEntered, InitialsEntrySystem};
use frontbox_pinball::{
  GameEndReason, GameEnding, GameManagerExt, HighScoreSlot, HighScoresSystem, TurnEndReason,
  TurnEnding,
};
use frontbox_sound::SoundSystem;

use crate::hardware::cabinet;
use crate::systems::game::ModeManager;
use crate::systems::sounds::LANE_HIT_COMPLETE;
use crate::systems::{Callout, SayExt};

/// Districts completed by the player whose game ends that count as high overgrowth, which the Oaks answer (start value)
const HIGH_OVERGROWTH: usize = 3;

/// A participant in `TurnEnding` (end-of-ball sound) and `GameEnding` (high-score initials entry, skipped after a
/// slam tilt)
#[derive(Default)]
pub struct EndOfBallSystem {
  /// Players still to enter initials, in player order
  /// Each with the player's index (0 for player 1)
  initials_needed: Vec<(HighScoreSlot, usize, u64)>,
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

  fn on_game_end(&mut self, scores: &[u64], ctx: &SystemContext) {
    self.initials_needed = match ctx.systems.get::<HighScoresSystem>() {
      Some(high_scores) => scores
        .iter()
        .enumerate()
        .filter_map(|(player, score)| {
          high_scores
            .is_high_score(*score)
            .map(|slot| (slot, player, *score))
        })
        .collect(),
      None => Vec::new(),
    };
    self.get_next_initials(ctx);
  }

  fn get_next_initials(&mut self, ctx: &SystemContext) {
    if let Some(&(_, player, _)) = self.initials_needed.first() {
      ctx.systems.spawn(InitialsEntrySystem::new(
        format!("Player {}", player + 1),
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
    if let Some(turn) = routine.downcast_ref::<TurnEnding>() {
      if turn.reason == TurnEndReason::Drained {
        ctx.say(Callout::GrowthContained);
      }
      // insert a slice delay to let hardware settle and final points come in
      ctx
        .timers
        .start(Continue, Duration::from_millis(500).once());
    } else if let Some(GameEnding { scores, reason }) = routine.downcast_ref::<GameEnding>() {
      // the forest promises to be back once it has taken enough of the city, as the last player to play left it. Their
      // `ModeManager` is in their group, which this system (at the top level) has to name to reach.
      let last_player_group = ctx
        .game_manager()
        .and_then(|game| game.player_group(game.session()?.current_player()));
      let overgrown = last_player_group
        .and_then(|group| ctx.systems.get_in::<ModeManager>(group))
        .is_some_and(|modes| modes.districts_completed() >= HIGH_OVERGROWTH);
      if overgrown && *reason != GameEndReason::SlamTilted {
        ctx.say(Callout::OakSoon);
      }

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
