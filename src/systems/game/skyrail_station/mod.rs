mod qualification;
mod startable;

use std::sync::LazyLock;

use frontbox::prelude::*;
pub use qualification::*;
pub use startable::*;

static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::blue());
