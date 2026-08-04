use frontbox::prelude::Rgba;

pub trait Spore {
  fn color() -> Rgba<u8>;
}
