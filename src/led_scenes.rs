use frontbox_core::prelude::LightQ;
use frontbox_gfx::scene::file::FormatError;
use frontbox_scene::{Completes, PlayableScene, Scene};

/// A scene from `assets/leds`, built into the game, on the LEDs it says, or for one whose layers are all on the same
/// LEDs, on `$leds`. A bad one panics, saying which.
macro_rules! scene {
  ($path:literal) => {
    $crate::led_scenes::scene($path, $crate::led_scenes::load!($path), None)
  };
  ($path:literal, $leds:expr) => {
    $crate::led_scenes::scene($path, $crate::led_scenes::load!($path), Some($leds))
  };
}

/// [`scene!`] for a scene that plays through and completes, to hand off with `play_scene`. One that doesn't panics,
/// saying which.
macro_rules! completing_scene {
  ($path:literal) => {
    $crate::led_scenes::completing($path, $crate::led_scenes::load!($path), None)
  };
  ($path:literal, $leds:expr) => {
    $crate::led_scenes::completing($path, $crate::led_scenes::load!($path), Some($leds))
  };
}

macro_rules! load {
  ($path:literal) => {
    frontbox_scene::Scene::load(include_str!(concat!(
      env!("CARGO_MANIFEST_DIR"),
      "/src/assets/leds/",
      $path
    )))
  };
}

pub(crate) use load;

pub fn scene(
  path: &str,
  loaded: Result<Scene, FormatError>,
  leds: Option<LightQ>,
) -> PlayableScene {
  let scene = PlayableScene::new(loaded.unwrap_or_else(|e| panic!("{path}: {e}")));
  match leds {
    Some(leds) => scene
      .on_lights(leds)
      .unwrap_or_else(|e| panic!("{path}: {e}")),
    None => scene,
  }
}

pub fn completing(
  path: &str,
  loaded: Result<Scene, FormatError>,
  leds: Option<LightQ>,
) -> PlayableScene<Completes> {
  let scene = PlayableScene::completing(loaded.unwrap_or_else(|e| panic!("{path}: {e}")))
    .unwrap_or_else(|e| panic!("{path}: {e}"));
  match leds {
    Some(leds) => scene
      .on_lights(leds)
      .unwrap_or_else(|e| panic!("{path}: {e}")),
    None => scene,
  }
}
