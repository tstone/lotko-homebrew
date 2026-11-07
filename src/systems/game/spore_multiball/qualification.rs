use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::captive_ball::CaptiveBallHit;
use crate::systems::game::{self, ModeManager, SporeMultiballStartable};
use crate::systems::sounds;

const REQUIRED_HITS: u8 = 4;

/// Captive ball hits toward the multiball, until Heartwood's charge replaces this (PLAN.md, Part 5)
#[derive(Clone)]
pub struct SporeMultiballQualification {
  hits: u8,
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
}

impl SporeMultiballQualification {
  pub fn new() -> Self {
    Self {
      hits: 0,
      attention_effect: led_playback!("spore_multiball/qualification.scene"),
      hit_effect: led_playback!("exclusive_qualification_hit.scene").stopped(),
    }
  }

  fn on_hit(&mut self, ctx: &SystemContext) {
    self.hits += 1;
    ctx.add_points(game::points::EXCL_QUAL_HIT);
    self.hit_effect.reset();
    self.hit_effect.play();
    if self.hits == REQUIRED_HITS {
      ctx.play_sfx(sounds::LANE_HIT3);
    }
  }
}

impl System for SporeMultiballQualification {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
      && ctx
        .systems
        .expect::<ModeManager>()
        .current_multiball()
        .is_none()
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    if self.hits >= REQUIRED_HITS && self.hit_effect.is_complete() {
      self.attention_effect.stop(ctx);
      ctx.systems.replace_self(SporeMultiballStartable::new());
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CaptiveBallHit>() {
      self.on_hit(ctx);
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    ctx.deactivate_led_declarations();
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
  }
}
