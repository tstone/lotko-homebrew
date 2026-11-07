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

/// A short flash of `color` on `leds`, fading out
pub fn flash_effect(leds: &LedQ, color: Rgba<u8>) -> LedPlayback {
  led_playback!("shared/flash.scene", leds.clone()).param("color", color)
}

/// `lit` of the `count` LEDs in `leds` in `color`, from the first, e.g. segments of a ring. The scene's shape is around
/// the first LED, and to light a number of them its edge goes between the last of them and the next.
pub fn fill_effect(leds: &LedQ, count: usize, lit: usize, color: Rgba<u8>) -> LedPlayback {
  let width = 2.0 * (lit as f32 - 0.5) / (count as f32 - 1.0).max(1.0);
  led_playback!("shared/fill.scene", leds.clone())
    .param("color", color)
    .param("width", width.max(0.0))
}

/// A double pulse of `color` on `leds`, once every `beat`
pub fn heartbeat_effect(leds: &LedQ, color: Rgba<u8>, beat: Duration) -> LedPlayback {
  led_playback!("shared/heartbeat.scene", leds.clone())
    .param("color", color)
    .over(beat)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::hardware::main_shot::MainShot;

  #[test]
  fn the_scenes_load() {
    let leds = MainShot::LeftOrbit.hex_ring();
    flash_effect(leds, Rgba::green());
    fill_effect(leds, 6, 3, Rgba::green());
    heartbeat_effect(leds, Rgba::green(), Duration::from_secs(1));
    crate::hardware::city_map::mode_progress_effect(Rgba::green(), 0.5);
  }
}
