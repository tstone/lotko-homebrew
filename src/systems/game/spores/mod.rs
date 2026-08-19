mod pink_spore;
mod spore;

pub use pink_spore::*;
pub use spore::*;

static SPORE_COUNT: u8 = 6;
static SPORE_UNIT: f32 = 1.0 / SPORE_COUNT as f32;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SporeType {
  #[default]
  Pink,
}
