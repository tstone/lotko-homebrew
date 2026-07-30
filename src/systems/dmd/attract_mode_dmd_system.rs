use frontbox_canvas::{Container, Gif, Horizontal, Layer, StaticImage, Vertical};
use frontbox_pin2dmd::menu::DmdMenuSystem;
use frontbox_pin2dmd::{DmdSystem, SIGI_BOLD_7PX_FONT};
use image::RgbaImage;
use std::path::PathBuf;

use frontbox::animation::{Accumulator, Animation, Curve, Sequence, Tween};
use frontbox::prelude::*;
use frontbox_canvas::animation::Frame;
use frontbox_turn_based::*;

pub struct AttractModeDmdSystem {
  bio_spore: Vec<Frame>,
  animation: Sequence<Duration, RgbaImage>,
}

impl AttractModeDmdSystem {
  pub fn new() -> Self {
    let frames = Gif::decode_from_path(local_asset("gif/bio-spore-pink.gif"));
    Self {
      animation: Self::rnd_animation(&frames),
      bio_spore: frames,
    }
  }

  pub fn rnd_animation(frames: &Vec<Frame>) -> Sequence<Duration, RgbaImage> {
    let end_frame = rand::random_range(1..frames.len());

    Tween::ping_pong(
      Duration::from_millis(rand::random_range(2000..=3000)),
      Curve::EaseInOut,
      vec![
        frames[0].buffer().clone(),
        frames[end_frame].buffer().clone(),
      ],
      Cycle::Times(rand::random_range(1..=2)),
    )
  }
}

impl System for AttractModeDmdSystem {
  fn is_active(&self, ctx: &Context) -> bool {
    // Either the game isn't running or the menu isn't active
    !ctx.is_game_started()
      && !ctx
        .systems
        .get::<DmdMenuSystem>()
        .map(|menu| menu.is_active(ctx))
        .unwrap_or(false)
  }

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

      let mut window = Container::transparent();

      window.add(
        StaticImage::from_image(img)
          .width(w)
          .height(h)
          .horizontal(Horizontal::Centered)
          .vertical(Vertical::Centered),
      );

      let mut text_row = Container::transparent().with_padding_all(5);

      text_row.add(
        SIGI_BOLD_7PX_FONT
          .left_aligned("press", Rgba::cyan().lighten(0.35))
          .default_position(),
      );

      text_row.add(
        SIGI_BOLD_7PX_FONT
          .left_aligned("start", Rgba::cyan().lighten(0.35))
          .left_offset(81),
      );

      window.add(text_row.top_offset(7));
      dmd.insert_layer(0, window.default_position());
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
