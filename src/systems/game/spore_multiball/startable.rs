use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::TurnPhase;
use frontbox_sound::SoundSystemExt;

use crate::{
  hardware::captive_ball::CaptiveBallHit,
  systems::{
    game::{ModeManager, Multiball, SporeMultiballMode},
    sounds,
  },
};

#[derive(Clone)]
pub struct SporeMultiballStartable {
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  complete: bool,
}

impl SporeMultiballStartable {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(),
      complete: false,
    }
  }

  fn attention_effect() -> LedPlayback {
    led_playback!("spore_multiball/startable_attention.scene")
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("spore_multiball/startable_hit.scene").stopped()
  }
}

impl System for SporeMultiballStartable {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    let mode_manager = ctx.systems.expect::<ModeManager>();
    ctx.states.is(TurnPhase::Active)
      && (mode_manager.current_multiball().is_none()
        || mode_manager.current_multiball() == Some(Multiball::Heartwood))
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CaptiveBallHit>() && !self.complete {
      match ctx
        .systems
        .expect::<ModeManager>()
        .take_multiball(Multiball::Heartwood, ctx.into())
      {
        Ok(_) => {
          self.complete = true;
          self.hit_effect.play();
          ctx.play_sfx(sounds::BRAAM_BIO2);
        }
        _ => {}
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.complete && self.hit_effect.is_complete() {
      ctx.systems.replace_self(SporeMultiballMode::new());
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
  }
}
