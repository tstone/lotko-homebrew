use frontbox::{animation::Curve, prelude::*};
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::{GameManager, TurnState};

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
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
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

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::flash(
      LedQ::any(vec![
        &captive_ball::LEFT_BOLT.q(),
        &captive_ball::RIGHT_BOLT.q(),
      ]),
      ColorSequence::solid(Rgba::yellow()),
      Cycle::Forever,
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::flash(
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
        ColorSequence::solid(Rgba::yellow()),
        Cycle::Times(3),
      ),
      LedProgram1d::rotating(
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
        ColorSequence::fade(Rgba::yellow(), Rgba::yellow()),
        Duration::from_millis(400),
        Curve::Linear,
        Cycle::Times(3),
      ),
    ])
    .stopped()
  }
}

impl System for SporeMultiballStartable {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    let game_manager = ctx.expect::<GameManager>();
    let mode_manager = ctx.expect::<ModeManager>();
    game_manager
      .turn_state()
      .map(|s| *s == TurnState::Active)
      .unwrap_or(false)
      && (mode_manager.current_mode().is_none()
        || *mode_manager.current_mode() == Some(ExclusiveMode::SporeMultiball))
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CaptiveBallHit>() && !self.complete {
      match ctx
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
      ctx.replace_self(SporeMultiballMode::new());
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
  }
}
