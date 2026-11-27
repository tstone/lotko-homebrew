use frontbox_core::prelude::*;
use frontbox_pinball::{GameEndReason, GameEnding, GameManagerExt, TurnEndReason, TurnEnding};
use frontbox_sound::SoundSystem;

use crate::systems::game::ModeManager;
use crate::systems::sounds::LANE_HIT_COMPLETE;
use crate::systems::{Callout, SayExt};

/// Districts completed by the player whose game ends that count as high overgrowth, which the Oaks answer (start value)
const HIGH_OVERGROWTH: usize = 3;

/// A participant in `TurnEnding` (end-of-ball sound) and `GameEnding` (the Oaks' parting words)
#[derive(Default)]
pub struct EndOfBallSystem;

impl EndOfBallSystem {
  pub fn new() -> Self {
    Self
  }

  fn on_end_of_ball(&self, ctx: &SystemContext) {
    let mut sound = ctx.systems.expect::<SoundSystem>();
    sound.stop_music(Duration::from_millis(500));
    sound.play_sfx(LANE_HIT_COMPLETE);

    // TODO: DMD bonus or something

    ctx.routines.complete_step::<TurnEnding>();
  }
}

impl System for EndOfBallSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Continue>() {
      self.on_end_of_ball(ctx);
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
    } else if let Some(GameEnding { reason, .. }) = routine.downcast_ref::<GameEnding>() {
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
      ctx.routines.complete_step::<GameEnding>();
    }
  }
}

#[derive(serde::Serialize, Event)]
struct Continue;
