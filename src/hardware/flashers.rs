use std::sync::LazyLock;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub LEFT_FLASHER: LedDefinition = LedDefinition::strip("l_flasher", 8)
    .tag(Playfield)
    .tag(Flasher);

  pub CENTER_FLASHER: LedDefinition = LedDefinition::strip("c_flasher", 8)
    .tag(Playfield)
    .tag(Flasher);
}

static all_flashers: LazyLock<LedQ> =
  LazyLock::new(|| LedQ::any(vec![&LEFT_FLASHER.q(), &CENTER_FLASHER.q()]));

#[derive(Clone)]
pub struct FlashersSystem {
  effects: Vec<LedProgram1d>,
}

impl FlashersSystem {
  pub fn new() -> Self {
    Self {
      effects: Vec::new(),
    }
  }

  pub fn flash(&mut self, times: u32, colors: ColorSequence) {
    self.effects.push(LedProgram1d::flash(
      &*all_flashers,
      colors,
      Cycle::Times(times),
    ));
  }

  pub fn rotate(&mut self, times: u32, colors: ColorSequence) {
    self.effects.push(LedProgram1d::rotating(
      &*all_flashers,
      colors,
      Duration::from_millis(600),
      Curve::Linear,
      Cycle::Times(times),
    ))
  }
}

impl System for FlashersSystem {
  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    let mut effects_to_remove = Vec::new();

    for (idx, effect) in self.effects.iter_mut().enumerate() {
      if effect.is_complete() {
        effect.stop(ctx);
        effects_to_remove.push(idx);
      } else {
        effect.apply(delta, ctx);
      }
    }

    effects_to_remove.iter().for_each(|idx| {
      self.effects.remove(*idx);
    });
  }
}
