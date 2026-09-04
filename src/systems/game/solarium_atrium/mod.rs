use std::sync::LazyLock;

use frontbox::prelude::*;

mod mode;
mod qualification;

pub use mode::*;
pub use qualification::*;

pub static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::purple());
