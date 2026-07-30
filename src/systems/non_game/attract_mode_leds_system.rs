use std::sync::LazyLock;

use frontbox::animation::*;
use frontbox::prelude::color_sequence::Fill1d;
use frontbox::prelude::tags::*;
use frontbox::prelude::*;
use frontbox_pin2dmd::menu::DmdMenuSystem;
use frontbox_turn_based::*;
use rand::seq::IndexedRandom;

static COLORS: LazyLock<Vec<Rgba<u8>>> = LazyLock::new(|| {
  vec![
    Rgba::cyan(),
    Rgba::blue(),
    Rgba::turquoise(),
    Rgba::magenta(),
    Rgba::purple(),
    Rgba::yellow(),
    Rgba::white().darken(0.25),
  ]
});

pub struct AttractModeLedsSystem {
  effect: LedEffect,
  prior_pair: (Rgba<u8>, Rgba<u8>),
}

impl AttractModeLedsSystem {
  pub fn new() -> Self {
    let (effect, from, to) = Self::rnd_effect(Rgba::cyan(), Rgba::magenta());
    Self {
      effect,
      prior_pair: (from, to),
    }
  }

  fn rnd_effect(from: Rgba<u8>, to: Rgba<u8>) -> (LedEffect, Rgba<u8>, Rgba<u8>) {
    let colors = COLORS
      .as_slice()
      .sample(&mut rand::rng(), 2)
      .collect::<Vec<_>>();

    let mut next = ColorSequence::fade(
      colors[0].lighten(rand::random_range(0.0..=0.3)),
      colors[1].darken(rand::random_range(0.0..=0.3)),
    );

    for _ in 0..5 {
      if rand::random_bool(0.25) {
        next = next.overwrite(
          Fill1d::Pattern {
            pattern: vec![Rgba::default()],
            cycle: Cycle::Times(rand::random_range(1..5)),
          },
          color_sequence::Fill1dArea::Full,
        )
      }
    }

    let effect = LedEffect::cycle(
      Q::tag::<Playfield>(),
      Duration::from_secs(rand::random_range(6..=12)),
      Curve::SmoothRandom,
      Cycle::Once,
      vec![ColorSequence::fade(from, to), next],
    )
    .shuffled(rand::random())
    .rotating(
      Duration::from_secs(rand::random_range(20..=32)),
      Curve::Linear,
    );

    (effect, *colors[0], *colors[1])
  }
}

impl System for AttractModeLedsSystem {
  fn is_active(&self, ctx: &Context) -> bool {
    // TODO: should an LED system depend on a DMD system? this might need a generalized 'machine state' system
    !ctx.is_game_started()
      && !ctx
        .systems
        .get::<DmdMenuSystem>()
        .map(|menu| menu.is_active(ctx))
        .unwrap_or(false)
  }

  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    if self.effect.is_complete() {
      let (prev_from, prev_to) = self.prior_pair;
      let (effect, from, to) = Self::rnd_effect(prev_from, prev_to);
      self.prior_pair = (from, to);
      self.effect = effect;
      self.effect.apply(Duration::ZERO, ctx);
    } else {
      self.effect.apply(delta, ctx);
    }
  }
}
