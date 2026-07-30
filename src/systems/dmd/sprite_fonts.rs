use std::{path::PathBuf, sync::LazyLock};

use frontbox_pin2dmd::*;

pub static BOLD_10PX: LazyLock<SpriteSheetFont> = LazyLock::new(|| {
  SpriteSheetFontBuilder::new()
    .path(local_asset("bold_pixels.png"))
    .sheet_layout(4, 16)
    .default_char_width(9)
    .custom_char_width(',', 3)
    .build()
});

fn local_asset(path: &str) -> PathBuf {
  PathBuf::from(format!(
    "{}/src/assets/{}",
    env!("CARGO_MANIFEST_DIR"),
    path
  ))
}
