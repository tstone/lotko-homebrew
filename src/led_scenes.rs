use frontbox_core::prelude::LedQ;
use frontbox_gfx::scene::file::FormatError;
use frontbox_led::{LedPlayback, LedScene};

/// A scene from `assets/leds`, built into the game. A bad one panics, saying which.
macro_rules! led_scene {
  ($path:literal) => {
    $crate::led_scenes::scene(
      $path,
      frontbox_led::LedScene::load(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/assets/leds/",
        $path
      ))),
    )
  };
}

/// Plays a scene from `assets/leds` on the LEDs it says, or for a scene with a single target, on `$leds`
macro_rules! led_playback {
  ($path:literal) => {
    $crate::led_scenes::playback($path, led_scene!($path), None)
  };
  ($path:literal, $leds:expr) => {
    $crate::led_scenes::playback($path, led_scene!($path), Some($leds))
  };
}

pub fn scene(path: &str, loaded: Result<LedScene, FormatError>) -> LedScene {
  loaded.unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn playback(path: &str, scene: LedScene, leds: Option<LedQ>) -> LedPlayback {
  let playback = match leds {
    Some(leds) => LedPlayback::with_leds(scene, leds),
    None => LedPlayback::new(scene),
  };
  playback.unwrap_or_else(|e| panic!("{path}: {e}"))
}
