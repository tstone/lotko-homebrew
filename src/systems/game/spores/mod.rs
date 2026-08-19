mod pink_spore;
mod spore;

pub use pink_spore::*;
pub use spore::*;

use crate::systems::game::CityShot;

static SPORE_COUNT: u8 = 6;
static SPORE_UNIT: f32 = 1.0 / SPORE_COUNT as f32;
