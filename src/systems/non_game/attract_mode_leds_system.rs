use std::sync::LazyLock;

use frontbox::animation::*;
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
  program: LedProgram1d,
  prior_pair: (Rgba<u8>, Rgba<u8>),
}

impl AttractModeLedsSystem {
  pub fn new() -> Self {
    let (program, from, to) = Self::rnd_program(Rgba::cyan(), Rgba::magenta());
    Self {
      program,
      prior_pair: (from, to),
    }
  }

  fn rnd_program(from: Rgba<u8>, to: Rgba<u8>) -> (LedProgram1d, Rgba<u8>, Rgba<u8>) {
    let colors = COLORS
      .as_slice()
      .sample(&mut rand::rng(), 2)
      .collect::<Vec<_>>();

    let next_from = colors[0].lighten(rand::random_range(0.0..=0.3));
    let next_to = colors[1].darken(rand::random_range(0.0..=0.3));
    let cycle = Cycle::Times(rand::random_range(1..5));
    let duration = Duration::from_secs(rand::random_range(6..=12));

    let program = LedProgram1d::rotating(
      LedQ::tag::<Playfield>(),
      ColorSequence::fade(from, to).shuffle(rand::random()),
      Duration::from_secs(36),
      Curve::Steps(12),
      cycle,
    )
    // Modulated starting color
    .modulate(
      Tween::new(duration, Curve::Linear, vec![from, next_from], cycle),
      |colors, from| {
        colors.fill.gradient_stops_mut().unwrap()[0] = GradientStop::new(0.0, from);
      },
    )
    // Modulate ending color
    .modulate(
      Tween::new(duration, Curve::Linear, vec![to, next_to], cycle),
      |colors, to| {
        colors.fill.gradient_stops_mut().unwrap()[1] = GradientStop::new(1.0, to);
      },
    );

    (program, *colors[0], *colors[1])
  }
}

impl System for AttractModeLedsSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    // TODO: should an LED system depend on a DMD system? this might need a generalized 'machine state' system
    !ctx.is_game_started()
      && !ctx
        .get::<DmdMenuSystem>()
        .map(|menu| menu.is_active(ctx))
        .unwrap_or(false)
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.program.is_complete() {
      let (prev_from, prev_to) = self.prior_pair;
      let (program, from, to) = Self::rnd_program(prev_from, prev_to);
      self.prior_pair = (from, to);
      self.program = program;
      self.program.apply(Duration::ZERO, ctx);
    } else {
      self.program.apply(delta, ctx);
    }
  }
}
