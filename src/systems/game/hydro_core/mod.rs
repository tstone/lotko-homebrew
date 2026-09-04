mod mode;
mod qualification;

use std::sync::LazyLock;

use frontbox::prelude::*;
pub use mode::*;
pub use qualification::*;

pub static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::cyan());
