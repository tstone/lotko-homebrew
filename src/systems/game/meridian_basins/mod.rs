mod mode;

use std::sync::LazyLock;

use frontbox_lighting::prelude::*;
pub use mode::*;

use crate::systems::game::Mode;

/// The basins' grow lights, on the shots that can be hit and the arc ramp's progress
pub static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Mode::MeridianBasins.color());
