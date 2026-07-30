use frontbox::animation::*;
use frontbox::prelude::tags::*;
use frontbox::prelude::*;
use frontbox_pin2dmd::menu::DmdMenuSystem;
use frontbox_turn_based::GameManagementExt;

pub struct AttractModeSystem {
  effect: LedEffect,
}

impl AttractModeSystem {
  pub fn new() -> Self {
    Self {
      effect: LedEffect::cycle(
        Q::tag::<Playfield>(),
        Duration::from_secs(12),
        Curve::Linear,
        vec![
          ColorSequence::fade(Rgba::cyan(), Rgba::magenta()),
          ColorSequence::fade(Rgba::magenta(), Rgba::yellow()),
          ColorSequence::fade(Rgba::yellow(), Rgba::cyan()),
        ],
      )
      .shuffled(0),
    }
  }
}

impl System for AttractModeSystem {
  fn is_active(&self, ctx: &Context) -> bool {
    // Either the game isn't running or the menu isn't active
    !ctx.is_game_started()
      && !ctx
        .systems
        .get::<DmdMenuSystem>()
        .map(|menu| menu.is_active(ctx))
        .unwrap_or(false)
  }

  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    self.effect.apply(delta, ctx);
  }
}
