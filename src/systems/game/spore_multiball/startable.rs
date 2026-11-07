use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::TurnPhase;
use frontbox_sound::SoundSystemExt;

use crate::{
  hardware::{
    arc_ramp,
    captive_ball::{self, CaptiveBallHit},
    center_orbit, dome_ramp, flashers, left_orbit, lift_ramp, right_orbit,
  },
  systems::{
    game::{ExclusiveMode, ModeManager, SporeMultiballMode},
    sounds,
  },
};

#[derive(Clone)]
pub struct SporeMultiballStartable {
  attention_effect: LedScene,
  hit_effect: LedScene,
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

  fn attention_effect() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &captive_ball::LEFT_BOLT.q(),
        &captive_ball::RIGHT_BOLT.q(),
      ]),
      led_scene!("spore_multiball/startable_attention.ron"),
    )
  }

  fn hit_effect() -> LedScene {
    LedScene::unbound(led_scene!("spore_multiball/startable_hit.ron"))
      .on(
        "rings",
        LedQ::any(vec![
          &left_orbit::HEX_CIRCLE_LEDS,
          &dome_ramp::HEX_CIRCLE_LEDS,
          &arc_ramp::HEX_CIRCLE_LEDS,
          &captive_ball::LEFT_BOLT.q(),
          &captive_ball::RIGHT_BOLT.q(),
          &center_orbit::HEX_CIRCLE_LEDS,
          &lift_ramp::HEX_CIRCLE_LEDS,
          &right_orbit::HEX_CIRCLE_LEDS,
        ]),
      )
      .unwrap()
      .on(
        "flashers",
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
      )
      .unwrap()
      .stopped()
  }
}

impl System for SporeMultiballStartable {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    let mode_manager = ctx.systems.expect::<ModeManager>();
    ctx.states.is(TurnPhase::Active)
      && (mode_manager.current_mode().is_none()
        || *mode_manager.current_mode() == Some(ExclusiveMode::SporeMultiball))
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CaptiveBallHit>() && !self.complete {
      match ctx
        .systems
        .expect::<ModeManager>()
        .take_exclusive(ExclusiveMode::SporeMultiball, ctx.into())
      {
        Ok(_) => {
          self.complete = true;
          self.hit_effect.play();
          ctx.play_sfx(sounds::HIT_ORGANIC3);
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
