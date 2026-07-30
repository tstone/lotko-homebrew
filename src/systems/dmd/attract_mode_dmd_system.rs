use frontbox_canvas::{Gif, Horizontal, Layer, StaticImage, Vertical};
use frontbox_pin2dmd::DmdSystem;
use image::RgbaImage;
use std::path::PathBuf;

use frontbox::animation::{Accumulator, Animation, Curve, Sequence, Tween};
use frontbox::prelude::*;
use frontbox_canvas::animation::Frame;

pub struct AttractModeDmdSystem {
  bio_spore: Vec<Frame>,
  animation: Sequence<Duration, RgbaImage>,
}

impl AttractModeDmdSystem {
  pub fn new() -> Self {
    let frames = Gif::decode_from_path(local_asset("../../assets/gif/bio-spore-pink"));
    Self {
      animation: Self::rnd_animation(&frames),
      bio_spore: frames,
    }
  }

  pub fn rnd_animation(frames: &Vec<Frame>) -> Sequence<Duration, RgbaImage> {
    let repeat = rand::random_range(1..=4);
    let end_frame = rand::random_range(1..frames.len());

    Tween::ping_pong(
      Duration::from_millis(1250),
      Curve::EaseInOut,
      vec![
        frames[0].buffer().clone(),
        frames[end_frame].buffer().clone(),
      ],
      Cycle::Times(repeat),
    )
  }
}

impl System for AttractModeDmdSystem {
  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    if self.animation.is_complete() {
      self.animation = Self::rnd_animation(&self.bio_spore);
    } else {
      self.animation.accumulate(delta);
    }

    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      let img = self.animation.sample();
      let w = img.width();
      let h = img.height();

      dmd.insert_layer(
        0,
        StaticImage::from_image(img)
          .width(w)
          .height(h)
          .horizontal(Horizontal::Centered)
          .vertical(Vertical::Centered),
      );
    }
  }
}

fn local_asset(path: &str) -> PathBuf {
  PathBuf::from(format!(
    "{}/src/assets/{}",
    env!("CARGO_MANIFEST_DIR"),
    path
  ))
}
