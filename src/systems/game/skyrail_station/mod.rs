mod mode;

use std::sync::LazyLock;

use frontbox_lighting::prelude::*;
pub use mode::*;

pub static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::blue());
