mod mode;
mod qualification;
mod startable;

use std::sync::LazyLock;

use frontbox::prelude::*;
pub use mode::*;
pub use qualification::*;
pub use startable::*;

static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::cyan());

mod points {
  pub static QUAL_HIT: u32 = 25_000;
  pub static START: u32 = 250_000;
  pub static COMBO_BASE: u32 = 100_000;
  pub static COMPLETION: u32 = 50_000_000;
}
