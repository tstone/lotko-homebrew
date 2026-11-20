use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;

/// A short flash of `color` on `leds`, fading out
pub fn flash_scene(leds: &LightQ, color: Rgba<u8>) -> PlayableScene<Completes> {
  completing_scene!("shared/flash.scene", leds.clone()).variable("color", color)
}

/// `lit` of the `count` LEDs in `leds` in `color`, from the first, e.g. segments of a ring. The scene's shape is around
/// the first LED, and to light a number of them its edge goes between the last of them and the next.
pub fn fill_scene(leds: &LightQ, count: usize, lit: usize, color: Rgba<u8>) -> PlayableScene {
  let width = 2.0 * (lit as f32 - 0.5) / (count as f32 - 1.0).max(1.0);
  scene!("shared/fill.scene", leds.clone())
    .variable("color", color)
    .variable("width", width.max(0.0))
}

/// A slow pulse of `color` on `leds`, at the mode attention z, e.g. the shot a mode wants
pub fn glow_scene(leds: &LightQ, color: Rgba<u8>) -> PlayableScene {
  scene!("shared/glow.scene", leds.clone()).variable("color", color)
}

/// A double pulse of `color` on `leds`, once every `beat`
pub fn heartbeat_scene(leds: &LightQ, color: Rgba<u8>, beat: Duration) -> PlayableScene {
  scene!("shared/heartbeat.scene", leds.clone())
    .variable("color", color)
    .over(beat)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::hardware::main_shot::MainShot;

  #[test]
  fn the_scenes_load() {
    let leds = MainShot::LeftOrbit.hex_ring();
    flash_scene(leds, Rgba::green());
    fill_scene(leds, 6, 3, Rgba::green());
    glow_scene(leds, Rgba::green());
    heartbeat_scene(leds, Rgba::green(), Duration::from_secs(1));
    crate::hardware::city_map::mode_progress_scene(Rgba::green(), 0.5);
  }
}
