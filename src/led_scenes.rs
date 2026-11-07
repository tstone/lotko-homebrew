use frontbox_gfx::scene::{Scene, file};

/// A scene from `assets/leds`, built into the game
macro_rules! led_scene {
  ($path:literal) => {
    $crate::led_scenes::load(
      $path,
      include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/assets/leds/",
        $path
      )),
    )
  };
}

pub fn load(path: &str, source: &str) -> Scene {
  file::load(source).unwrap_or_else(|e| panic!("{path}: {e}"))
}
