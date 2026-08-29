mod mode;
mod qualification;

use frontbox::prelude::*;
use std::sync::LazyLock;

pub use mode::*;
pub use qualification::*;

static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::magenta());

mod points {
  pub static QUAL_HIT: u32 = 1_000;
  pub static START: u32 = 50_000;
}
