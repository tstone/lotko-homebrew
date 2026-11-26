use std::f32::consts::TAU;

use frontbox_core::prelude::{Vec2, Vec3};
use frontbox_gfx::prelude::{Rgba, RgbaImage};
use frontbox_gfx::scene::{GeneratorFrame, GeneratorSetting, ImageGenerator};

/// One of the Helix Vaults' vats, as the scene image generator `vat`
///
/// It's a slim glass tube standing between two metal caps, holding a glowing liquid that bubbles. The image is `width`
/// by `height` pixels, and clear around the vat. The glass above the liquid is see-through, so whatever a scene puts
/// behind the vat shows through it, darkened.
///
/// `level` is how full it is, from 0 (empty) to 1 (to the top), and `hue` is the liquid's color in degrees around the
/// color wheel (0 red, 80 the Helix Vaults' yellow-green, 240 blue). The glass and the caps stay as they are.
///
/// Bubbles rise from the bottom of the tube to the surface, where they're gone. `bubbles` is how many there are, from
/// 0 (still) to 1, and `variant` picks which ones: vats side by side each want their own, or they bubble in step.
/// Everything that moves repeats every `period` seconds, so it loops without a jump in a scene whose duration is a
/// multiple of that.
///
/// A row of them with depth is a scene's to make, a layer for each vat: one set back is drawn smaller, with a lower
/// `width` and `height`, behind the others and dimmed with a `Brightness` adjust. Setting its size here rather than
/// scaling its layer keeps its edges as sharp as a nearer one's.
///
/// It's shaded in a few flat tones with hard edges between them, as a DMD shows those better than a gradient.
#[derive(Debug, Default)]
pub struct VatGenerator;

impl VatGenerator {
  /// The name scenes use it by
  pub const NAME: &'static str = "vat";

  const SETTINGS: &'static [GeneratorSetting] = &[
    GeneratorSetting::new("level", 1.0).range(0.0, 1.0),
    GeneratorSetting::new("hue", 80.0).range(0.0, 360.0),
    GeneratorSetting::new("width", 12.0).range(6.0, 48.0),
    GeneratorSetting::new("height", 30.0).range(12.0, 64.0),
    GeneratorSetting::new("bubbles", 0.5).range(0.0, 1.0),
    GeneratorSetting::new("variant", 0.0).range(0.0, 999.0),
    GeneratorSetting::new("period", 4.0).range(0.5, 60.0),
  ];

  /// Samples along each side of a pixel, which its color is the average of, for smooth edges
  const SAMPLES: u32 = 4;
}

impl ImageGenerator for VatGenerator {
  fn settings(&self) -> &[GeneratorSetting] {
    Self::SETTINGS
  }

  fn is_animated(&self) -> bool {
    true
  }

  fn generate(&self, frame: &GeneratorFrame) -> RgbaImage {
    let vat = Vat::set_by(frame);
    RgbaImage::from_fn(vat.size.x as u32, vat.size.y as u32, |x, y| {
      let samples = Self::SAMPLES;
      // color is summed weighted by its alpha, so that a clear sample doesn't darken its pixel
      let (mut sum, mut alpha) = (Vec3::ZERO, 0.0);
      for i in 0..samples * samples {
        let within = Vec2::new((i % samples) as f32, (i / samples) as f32) + 0.5;
        let at = Vec2::new(x as f32, y as f32) + within / samples as f32;
        if let Some((color, opacity)) = vat.color(at) {
          sum += color * opacity;
          alpha += opacity;
        }
      }
      if alpha == 0.0 {
        return Rgba([0, 0, 0, 0]);
      }
      let color = (sum / alpha).clamp(Vec3::ZERO, Vec3::ONE) * 255.0;
      let alpha = alpha / (samples * samples) as f32 * 255.0;
      Rgba([
        color.x.round() as u8,
        color.y.round() as u8,
        color.z.round() as u8,
        alpha.round() as u8,
      ])
    })
  }
}

/// A vat as it's drawn for one image
struct Vat {
  /// How big the image is, in whole pixels
  size: Vec2,
  /// How tall each cap is
  cap: f32,
  /// How thick the glass is, which is also how far the tube is in from the sides of the caps
  wall: f32,
  /// The liquid's color at its brightest
  color: Vec3,
  /// The y of the liquid's surface when it's flat. At the bottom of the tube, there's no liquid.
  surface: f32,
  /// How far through the period it is, 0 to 1
  phase: f32,
  /// What sets its ripples apart from another vat's
  variant: f32,
  bubbles: Vec<Bubble>,
}

impl Vat {
  /// How many pixels of liquid there are to a bubble, with `bubbles` at 1
  const AREA_PER_BUBBLE: f32 = 10.0;

  /// The most bubbles there can be, however big the vat
  const MOST_BUBBLES: usize = 48;

  fn set_by(frame: &GeneratorFrame) -> Self {
    let size = Vec2::new(
      frame.setting("width").round(),
      frame.setting("height").round(),
    );
    let variant = frame.setting("variant");
    let phase = (frame.time / frame.setting("period")).fract();
    let mut vat = Self {
      size,
      cap: (size.y * 0.1).round().clamp(2.0, 5.0),
      wall: (size.x / 12.0).round().max(1.0),
      color: hue_color(frame.setting("hue")),
      surface: 0.0,
      phase,
      variant,
      bubbles: Vec::new(),
    };
    let (top, bottom) = (vat.cap, size.y - vat.cap);
    vat.surface = bottom - frame.setting("level") * (bottom - top);
    let inside = vat.inside();
    let count = frame.setting("bubbles") * inside * (bottom - top) * 2.0 / Self::AREA_PER_BUBBLE;
    vat.bubbles = (0..(count.round() as usize).min(Self::MOST_BUBBLES))
      .filter_map(|index| {
        Bubble::at(
          index as u32,
          variant as u32,
          phase,
          size.x / 2.0,
          inside,
          top..bottom,
        )
      })
      // one that's reached the surface is gone
      .filter(|bubble| bubble.center.y - bubble.radius > vat.surface + 1.0)
      .collect();
    vat
  }

  /// Half of how wide the inside of the tube is
  fn inside(&self) -> f32 {
    self.size.x / 2.0 - 2.0 * self.wall
  }

  /// Its color and how opaque it is at `at`, in pixels from the image's top left. `None` outside it.
  fn color(&self, at: Vec2) -> Option<(Vec3, f32)> {
    if !(0.0..self.size.x).contains(&at.x) || !(0.0..self.size.y).contains(&at.y) {
      return None;
    }
    // -1 at the left of the image, 1 at the right
    let across = at.x / self.size.x * 2.0 - 1.0;
    let (top, bottom) = (self.cap, self.size.y - self.cap);
    if at.y < top || at.y >= bottom {
      return self.cap_color(at, across);
    }
    let from_center = (at.x - self.size.x / 2.0).abs();
    let inside = self.inside();
    if from_center > inside + self.wall {
      return None;
    }
    if from_center > inside {
      // the glass, with the light on its left
      let glass = if across < 0.0 {
        GLASS_LIT
      } else {
        GLASS_SHADED
      };
      return Some((glass, 0.9));
    }
    // -1 at the left of the liquid, 1 at the right
    let across = (at.x - self.size.x / 2.0) / inside;
    let (color, opacity) = self.contents(at, across);
    // a streak of light runs down the glass, left of the middle, and stops short of the bottom
    let streak = (-0.62..-0.3).contains(&across) && at.y < top + (bottom - top) * 0.72;
    Some(if streak {
      (color.lerp(Vec3::ONE, 0.35), opacity.max(0.6))
    } else {
      (color, opacity)
    })
  }

  /// What's in the tube at `at`, `across` it from -1 to 1: the liquid with its bubbles, or the empty glass over it
  fn contents(&self, at: Vec2, across: f32) -> (Vec3, f32) {
    let bottom = self.size.y - self.cap;
    // the surface rocks a little, and not at all once there's nothing left
    let ripple = 0.45 * (TAU * (across * 0.6 + self.phase * 2.0 + self.variant * 0.37)).sin();
    let under = at.y - (self.surface + ripple);
    if self.surface >= bottom || under < 0.0 {
      // what's left on the glass tints it
      return (EMPTY + self.color * 0.1, 0.55);
    }
    let pale = self.color.lerp(Vec3::ONE, 0.6);
    if under < 1.0 {
      return (pale, 1.0);
    }
    if self.bubbles.iter().any(|bubble| bubble.covers(at)) {
      return (pale, 1.0);
    }
    // the tube is round, so the liquid is dark at its sides and brightest where the light comes through
    let tone = if across.abs() > 0.8 {
      0.42
    } else if (-0.3..0.35).contains(&across) {
      1.0
    } else {
      0.74
    };
    (self.color * tone, 1.0)
  }

  /// A cap at `at`, `across` it from -1 to 1: metal that's round like the tube, with its outer corners cut off and a
  /// dark line where it meets the glass
  fn cap_color(&self, at: Vec2, across: f32) -> Option<(Vec3, f32)> {
    // how far it is from the end of the vat, and from the glass
    let from_end = at.y.min(self.size.y - at.y);
    let from_glass = self.cap - from_end;
    if from_end < 1.0 && (at.x < 1.0 || at.x > self.size.x - 1.0) {
      return None;
    }
    let metal = if from_glass < 1.0 {
      SEAM
    } else if across > 0.8 {
      METAL[0]
    } else if (-0.7..-0.25).contains(&across) {
      METAL[3]
    } else if (-0.25..0.4).contains(&across) {
      METAL[2]
    } else {
      METAL[1]
    };
    Some((metal, 1.0))
  }
}

/// A bubble in a vat's liquid, where it is at one moment
struct Bubble {
  center: Vec2,
  radius: f32,
}

impl Bubble {
  /// The bubble numbered `index` of the vat `variant`, `phase` through the period (0 to 1), in a tube whose inside is
  /// `inside` to either side of `center_x` and `rows` from top to bottom. `None` for a tube too narrow to hold it.
  ///
  /// It rises the height of the tube a whole number of times a period, so that it's where it started when the period
  /// comes around, and each time up a different part of the tube.
  fn at(
    index: u32,
    variant: u32,
    phase: f32,
    center_x: f32,
    inside: f32,
    rows: std::ops::Range<f32>,
  ) -> Option<Self> {
    let seed = variant.wrapping_mul(7919).wrapping_add(index);
    // a small one is a single pixel, and a big one a few across
    let radius = (0.5 + 0.8 * unit(seed, 0) * unit(seed, 1)) * (inside / 4.0).clamp(0.75, 3.0);
    let reach = inside - radius - 0.25;
    if reach < 0.0 {
      return None;
    }
    // bigger bubbles rise faster
    let climbs = 1 + (radius * 1.5 + unit(seed, 2)) as u32;
    let climbed = phase * climbs as f32 + unit(seed, 3);
    let climb = climbed as u32 % climbs;
    let up = climbed.fract();
    let sway = reach.min(0.7) * (TAU * (up * 2.5 + unit(seed, 4))).sin();
    let x = center_x + (reach - sway.abs()) * (unit(seed, 5 + climb) * 2.0 - 1.0) + sway;
    Some(Self {
      center: Vec2::new(x, rows.end - up * (rows.end - rows.start)),
      radius,
    })
  }

  fn covers(&self, at: Vec2) -> bool {
    self.center.distance_squared(at) < self.radius * self.radius
  }
}

/// The dark line where two parts meet
const SEAM: Vec3 = Vec3::new(0.13, 0.14, 0.17);

/// The tones of silver metal, from in shadow to its highlight
const METAL: [Vec3; 4] = [
  Vec3::new(0.2, 0.22, 0.28),
  Vec3::new(0.38, 0.42, 0.5),
  Vec3::new(0.6, 0.64, 0.71),
  Vec3::new(0.93, 0.95, 1.0),
];

/// The edge of the glass with the light on it
const GLASS_LIT: Vec3 = Vec3::new(0.62, 0.7, 0.8);

/// The edge of the glass away from the light
const GLASS_SHADED: Vec3 = Vec3::new(0.3, 0.35, 0.45);

/// What shows through the glass where there's no liquid behind it, before the liquid's tint
const EMPTY: Vec3 = Vec3::new(0.03, 0.04, 0.06);

/// The brightest, purest color at `hue` degrees around the color wheel
fn hue_color(hue: f32) -> Vec3 {
  let channel = |offset: f32| {
    let at = (hue / 60.0 + offset).rem_euclid(6.0);
    (2.0 - (at - 3.0).abs()).clamp(0.0, 1.0)
  };
  Vec3::new(channel(3.0), channel(1.0), channel(5.0))
}

/// A number from 0 up to 1 that's always the same for a `seed` and `which` of its numbers, and unlike its others
fn unit(seed: u32, which: u32) -> f32 {
  let mut bits = seed
    .wrapping_mul(0x9E37_79B9)
    .wrapping_add(which.wrapping_mul(0x85EB_CA6B));
  bits ^= bits >> 16;
  bits = bits.wrapping_mul(0x7FEB_352D);
  bits ^= bits >> 15;
  bits = bits.wrapping_mul(0x846C_A68B);
  bits ^= bits >> 16;
  (bits >> 8) as f32 / (1 << 24) as f32
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use frontbox_core::prelude::Resources;
  use frontbox_scene::{ImageGenerators, Scene};

  use super::*;

  /// What a vat is set to, with every setting at its default
  #[derive(Clone, Copy)]
  struct Set {
    time: f32,
    level: f32,
    hue: f32,
    width: f32,
    height: f32,
    bubbles: f32,
    variant: f32,
    period: f32,
  }

  impl Default for Set {
    fn default() -> Self {
      Self {
        time: 0.0,
        level: 1.0,
        hue: 80.0,
        width: 12.0,
        height: 30.0,
        bubbles: 0.5,
        variant: 0.0,
        period: 4.0,
      }
    }
  }

  fn drawn(set: Set) -> RgbaImage {
    let values = [
      set.level,
      set.hue,
      set.width,
      set.height,
      set.bubbles,
      set.variant,
      set.period,
    ];
    VatGenerator.generate(&GeneratorFrame::new(
      set.time,
      0,
      0,
      VatGenerator::SETTINGS,
      &values,
    ))
  }

  /// A vat with no bubbles, so that a pixel of its liquid is the liquid
  fn still(set: Set) -> RgbaImage {
    drawn(Set {
      bubbles: 0.0,
      ..set
    })
  }

  #[test]
  fn its_as_big_as_its_set_to_be_and_clear_beside_the_tube() {
    let image = drawn(Set::default());
    assert_eq!((image.width(), image.height()), (12, 30));
    assert_eq!(image.get_pixel(0, 15)[3], 0);
    assert_eq!(image.get_pixel(11, 15)[3], 0);
    // the caps are wider than the tube
    assert_eq!(image.get_pixel(0, 1)[3], 255);
    let small = drawn(Set {
      width: 8.0,
      height: 20.0,
      ..Set::default()
    });
    assert_eq!((small.width(), small.height()), (8, 20));
  }

  #[test]
  fn the_liquid_stands_as_high_as_its_level() {
    let half = still(Set {
      level: 0.5,
      ..Set::default()
    });
    // the tube runs from row 3 to row 27, so half full its surface is at row 15
    assert_eq!(half.get_pixel(6, 22)[3], 255);
    assert!(half.get_pixel(6, 22)[1] > 200);
    // over it the glass is see-through, and dark
    assert!(half.get_pixel(6, 8)[3] < 200);
    assert!(half.get_pixel(6, 8)[1] < 60);
    let empty = still(Set {
      level: 0.0,
      ..Set::default()
    });
    assert!(empty.get_pixel(6, 26)[1] < 60);
    let full = still(Set::default());
    assert!(full.get_pixel(6, 6)[1] > 200);
  }

  #[test]
  fn its_hue_colors_the_liquid_and_nothing_else() {
    let (green, blue) = (
      still(Set::default()),
      still(Set {
        hue: 240.0,
        ..Set::default()
      }),
    );
    let [red, green_in, blue_in, _] = blue.get_pixel(6, 20).0;
    assert!(blue_in > 200 && red < 60 && green_in < 60);
    assert!(green.get_pixel(6, 20)[1] > 200);
    // a cap, and the glass
    for (x, y) in [(6, 0), (1, 15)] {
      assert_eq!(green.get_pixel(x, y), blue.get_pixel(x, y), "at ({x}, {y})");
    }
  }

  #[test]
  fn bubbles_move_and_come_back_around_each_period() {
    let at = |time: f32| {
      drawn(Set {
        time,
        ..Set::default()
      })
    };
    assert_ne!(at(0.0), still(Set::default()));
    assert_ne!(at(0.0), at(1.0));
    assert_eq!(at(1.0), at(5.0));
  }

  #[test]
  fn a_variant_has_bubbles_of_its_own() {
    let other = drawn(Set {
      variant: 1.0,
      ..Set::default()
    });
    assert_ne!(drawn(Set::default()), other);
  }

  #[test]
  fn nothing_bubbles_over_the_surface() {
    for step in 0..40 {
      let set = Set {
        time: step as f32 * 0.1,
        level: 0.3,
        bubbles: 1.0,
        ..Set::default()
      };
      // the surface is at row 19.8, and rocks less than a row
      for y in 4..18 {
        assert_eq!(
          drawn(set).get_pixel(6, y),
          still(set).get_pixel(6, y),
          "row {y} at {}",
          set.time
        );
      }
    }
  }

  #[test]
  fn the_test_scene_opens_with_it() {
    let mut resources = Resources::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets"));
    resources.add::<ImageGenerators>(VatGenerator::NAME, Arc::new(VatGenerator));
    Scene::open(&resources, "dmd/vat_test.scene").unwrap();
  }

  /// Writes the vat to PNGs to look at, enlarged, when `VAT_PREVIEW` names a directory to put them in
  #[test]
  fn preview() {
    let Ok(directory) = std::env::var("VAT_PREVIEW") else {
      return;
    };
    const SCALE: u32 = 8;
    // a row of vats as a scene would lay them out, at a few moments, one row of the sheet for each
    let vats = [
      (4, 5, 10.0, 24.0, 0.9, 80.0, 0.55),
      (30, 5, 10.0, 24.0, 0.35, 80.0, 0.55),
      (16, 1, 12.0, 30.0, 0.7, 80.0, 1.0),
      (44, 1, 12.0, 30.0, 0.0, 80.0, 1.0),
      (60, 1, 12.0, 30.0, 1.0, 190.0, 1.0),
      (76, 1, 12.0, 30.0, 0.5, 300.0, 1.0),
      (92, 0, 20.0, 32.0, 0.8, 80.0, 1.0),
    ];
    let times = [0.0, 0.25, 0.5, 2.0];
    let mut sheet = RgbaImage::from_pixel(128, 34 * times.len() as u32, Rgba([8, 10, 16, 255]));
    for (row, time) in times.into_iter().enumerate() {
      for (index, (x, y, width, height, level, hue, brightness)) in vats.into_iter().enumerate() {
        let vat = drawn(Set {
          time,
          level,
          hue,
          width,
          height,
          variant: index as f32,
          ..Set::default()
        });
        for (vat_x, vat_y, pixel) in vat.enumerate_pixels() {
          let under = sheet.get_pixel_mut(x + vat_x, y + vat_y + 34 * row as u32);
          let alpha = pixel[3] as f32 / 255.0;
          for channel in 0..3 {
            let over = pixel[channel] as f32 * brightness;
            under[channel] = (over * alpha + under[channel] as f32 * (1.0 - alpha)).round() as u8;
          }
        }
      }
    }
    let enlarged = image::imageops::resize(
      &sheet,
      sheet.width() * SCALE,
      sheet.height() * SCALE,
      image::imageops::FilterType::Nearest,
    );
    enlarged.save(format!("{directory}/vats.png")).unwrap();
  }
}
