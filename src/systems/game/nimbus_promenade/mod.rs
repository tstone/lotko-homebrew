mod qualification;

use std::sync::LazyLock;

use frontbox::prelude::*;
pub use qualification::*;

static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::magenta());

mod points {
  pub static QUAL_HIT: u32 = 1_000;
  pub static START: u32 = 50_000;
}
