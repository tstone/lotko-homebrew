use frontbox::prelude::*;
use frontbox_sound::SoundSystem;

use crate::{GameManager, PlayerTurnEnding, systems::sounds::LANE_HIT_COMPLETE};

#[derive(Default)]
pub struct EndOfBallSystem;

impl EndOfBallSystem {
  pub fn new() -> Self {
    Self
  }

  fn on_end_of_ball(&self, ctx: &SystemContext) {
    let mut sound = ctx.expect::<SoundSystem>();
    sound.stop_music(Duration::from_millis(500));
    sound.play_sfx(LANE_HIT_COMPLETE);

    ctx.expect::<GameManager>().advance_turn(ctx.into());
  }
}

impl System for EndOfBallSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<PlayerTurnEnding>() {
      self.on_end_of_ball(ctx);
    }
  }
}
