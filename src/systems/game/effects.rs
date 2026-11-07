use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

/// Plays effects to their end for whatever started them, which doesn't have to stay around for it, e.g. a mode's
/// takeover or a flasher on a hit
#[derive(Clone)]
pub struct EffectsSystem {
  effects: Vec<LedPlayback>,
}

impl EffectsSystem {
  pub fn new() -> Self {
    Self {
      effects: Vec::new(),
    }
  }

  pub fn play(&mut self, mut effect: LedPlayback) {
    effect.play();
    self.effects.push(effect);
  }
}

impl System for EffectsSystem {
  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.effects.retain_mut(|effect| {
      if effect.is_complete() {
        effect.stop(ctx);
        false
      } else {
        effect.apply(delta, ctx);
        true
      }
    });
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    for effect in self.effects.iter_mut() {
      effect.stop(ctx);
    }
    self.effects.clear();
  }
}
