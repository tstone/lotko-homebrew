use std::sync::LazyLock;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::planes;

hardware_defs! {
  pub LEFT_FLASHER: LedDefinition = LedDefinition::multi("l_flasher")
    .locations(LedLayout::ring(8, Vec3::new(1.145, 23.0, 0.0), 0.375, 0.0, LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .tag(Playfield)
    .tag(Flasher);

  pub CENTER_FLASHER: LedDefinition = LedDefinition::multi("c_flasher")
    .locations(LedLayout::ring(8, Vec3::new(9.56, 5.16, 0.0), 0.375, 0.0, LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .tag(Playfield)
    .tag(Flasher);
}

static ALL_FLASHERS: LazyLock<LedQ> =
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
      &*ALL_FLASHERS,
      colors,
      Cycle::Times(times),
    ));
  }

  pub fn rotate(&mut self, times: u32, colors: ColorSequence) {
    self.effects.push(LedProgram1d::rotating(
      &*ALL_FLASHERS,
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
      // TODO: this shouldn't need a guard
      if self.effects.get(*idx).is_some() {
        self.effects.remove(*idx);
      }
    });
  }
}
