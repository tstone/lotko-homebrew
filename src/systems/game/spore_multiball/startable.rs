use frontbox::{animation::Curve, prelude::*};

use crate::{
  hardware::{
    arc_ramp, captive_ball, center_orbit, dome_ramp, flashers, left_orbit, lift_ramp, right_orbit,
  },
  systems::game::SporeMultiballMode,
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
      ColorSequence::solid(Rgba::orange()),
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
        ColorSequence::solid(Rgba::orange()),
        Cycle::Times(3),
      ),
      LedProgram1d::rotating(
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
        ColorSequence::fade(Rgba::yellow(), Rgba::orange()),
        Duration::from_millis(400),
        Curve::Linear,
        Cycle::Times(3),
      ),
    ])
    .stopped()
  }
}

impl System for SporeMultiballStartable {
  fn on_event(&mut self, event: &dyn Event, _ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == captive_ball::TARGET_SWITCH.name
      && !self.complete
    {
      self.complete = true;
      self.hit_effect.play();
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
