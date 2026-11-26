use frontbox_core::prelude::{Vec2, Vec3};
use frontbox_gfx::prelude::{Rgba, RgbaImage};
use frontbox_gfx::scene::{GeneratorFrame, GeneratorSetting, ImageGenerator};

/// The Steward's head and shoulders, as the scene image generator `steward`
///
/// It's a figure in the form of a person, made of white plastic shells over silver metal, with a visor of red light
/// where its eyes would be. The image is [`Self::WIDTH`] by [`Self::HEIGHT`] pixels, sized for a DMD 32 pixels tall,
/// and clear around the figure, so a scene puts whatever it likes behind it.
///
/// The red is what a scene moves. `speech` is how hard it's speaking, from 0 (silent) to 1: a scene has it follow the
/// loudness of the Steward's line with an external envelope, and the visor brightens and spills red over its face with
/// every syllable. `idle` is how bright the visor is when it's silent, and `look` moves the brightest part of the
/// visor from its right (-1) to its left (1), as a glance does.
///
/// `menace` is how much of a foe it looks, from 0 (the servant it presents itself as) to 1: its cranium comes down
/// over its visor as a brow, its head sinks between shoulders that hunch up and darken to gunmetal, its ears sweep up
/// into fins, and the light on it moves to below.
///
/// `angular` is the shape of its head, from an egg (0) to a rectangle with rounded corners (1).
///
/// It's shaded in a few flat tones with hard edges between them, as a DMD shows those better than a gradient.
///
/// Nothing about it changes with time: the same settings are the same image.
#[derive(Debug, Default)]
pub struct StewardGenerator;

impl StewardGenerator {
  /// The name scenes use it by
  pub const NAME: &'static str = "steward";

  /// How many pixels wide its image is
  pub const WIDTH: u32 = 36;

  /// How many pixels tall its image is
  pub const HEIGHT: u32 = 32;

  const SETTINGS: &'static [GeneratorSetting] = &[
    GeneratorSetting::new("speech", 0.0).range(0.0, 1.0),
    GeneratorSetting::new("idle", 0.3).range(0.0, 1.0),
    GeneratorSetting::new("look", 0.0).range(-1.0, 1.0),
    GeneratorSetting::new("menace", 0.0).range(0.0, 1.0),
    GeneratorSetting::new("angular", 0.0).range(0.0, 1.0),
  ];

  /// Samples along each side of a pixel, which its color is the average of, for smooth edges
  const SAMPLES: u32 = 4;
}

impl ImageGenerator for StewardGenerator {
  fn settings(&self) -> &[GeneratorSetting] {
    Self::SETTINGS
  }

  fn generate(&self, frame: &GeneratorFrame) -> RgbaImage {
    let idle = frame.setting("idle");
    let menace = frame.setting("menace");
    let visor = Visor {
      level: idle + (1.0 - idle) * frame.setting("speech"),
      look: frame.setting("look"),
    };
    let figure = Figure::new(visor, menace, frame.setting("angular"));
    RgbaImage::from_fn(Self::WIDTH, Self::HEIGHT, |x, y| {
      let samples = Self::SAMPLES;
      let (mut sum, mut covered) = (Vec3::ZERO, 0.0);
      for i in 0..samples * samples {
        let within = Vec2::new((i % samples) as f32, (i / samples) as f32) + 0.5;
        let at = Vec2::new(x as f32, y as f32) + within / samples as f32;
        if let Some(color) = figure.color(at) {
          sum += color;
          covered += 1.0;
        }
      }
      if covered == 0.0 {
        return Rgba([0, 0, 0, 0]);
      }
      let color = (sum / covered).clamp(Vec3::ZERO, Vec3::ONE) * 255.0;
      let alpha = covered / (samples * samples) as f32 * 255.0;
      Rgba([
        color.x.round() as u8,
        color.y.round() as u8,
        color.z.round() as u8,
        alpha.round() as u8,
      ])
    })
  }
}

/// The Steward as it's drawn for one image
struct Figure {
  visor: Visor,
  /// How much of a foe it looks, 0 to 1
  menace: f32,
  /// How far its head is from an egg (0) to a rectangle with rounded corners (1)
  angular: f32,
  /// Where the light falls from, toward the viewer
  light: Vec3,
}

impl Figure {
  /// The x the figure is mirrored around
  const CENTER_X: f32 = 18.0;

  fn new(visor: Visor, menace: f32, angular: f32) -> Self {
    // from up and to the left, moving to below as it turns
    let light = Vec3::new(-0.45, -0.62, 0.64).lerp(Vec3::new(-0.15, 0.62, 0.64), menace);
    Self {
      visor,
      menace,
      angular,
      light: light.normalize(),
    }
  }

  /// Its color at `at`, in pixels from the image's top left. `None` outside it.
  fn color(&self, at: Vec2) -> Option<Vec3> {
    self
      .head(at)
      .or_else(|| self.ear(at))
      .or_else(|| self.fin(at))
      .or_else(|| self.collar(at))
      .or_else(|| self.neck(at))
      .or_else(|| self.shoulders(at))
  }

  /// The head: a white faceplate under a metal cranium, with the visor across it. With menace the cranium comes
  /// down over the visor as a brow.
  fn head(&self, at: Vec2) -> Option<Vec3> {
    const CENTER: Vec2 = Vec2::new(Figure::CENTER_X, 10.6);
    const RADII: Vec2 = Vec2::new(7.7, 10.0);
    let from = at - CENTER;
    // narrower toward the chin, and less so the more angular it is
    let taper = 0.27 * (1.0 - 0.6 * self.angular);
    let down = (from.y / RADII.y).max(0.0);
    let normal = Vec2::new(
      from.x / (RADII.x * (1.0 - taper * down * down)),
      from.y / RADII.y,
    );
    let corners = 2.0 + 4.0 * self.angular;
    if normal.x.abs().powf(corners) + normal.y.abs().powf(corners) > 1.0 {
      return None;
    }
    let visor = &self.visor;
    let distance = Visor::distance(at);
    if distance < 0.0 {
      return Some(visor.glass(at, -distance / Visor::RADIUS));
    }
    if distance < Visor::BEZEL {
      return Some(visor.bezel());
    }
    let lit = self.lit(normal);
    let across = from.x.abs();
    // the cranium meets the faceplate along a line that dips at the temples, or just over the visor as a brow
    let open = 4.4 + 0.036 * from.x * from.x;
    let brow = open + (Visor::TOP - Visor::BEZEL - 0.3 - open) * self.menace;
    let surface = if at.y < brow - 0.35 {
      // a raised plate runs back over the top of the cranium, and a brow shades what's under it
      if distance < Visor::BEZEL + 1.1 * self.menace {
        METAL[0]
      } else if (across - 1.7).abs() < 0.3 {
        SEAM
      } else if across < 1.7 {
        metal(lit + 0.25)
      } else {
        metal(lit)
      }
    } else if at.y < brow + 0.35 {
      SEAM
    } else {
      // a cheek plate's edge runs down and in from each end of the visor
      let cheek = 5.6 - 0.42 * (at.y - 11.5);
      let on_cheek = (12.0..15.5).contains(&at.y) && (across - cheek).abs() < 0.3;
      if on_cheek { SEAM } else { plastic(lit, 0.0) }
    };
    Some(visor.cast_on(surface, distance))
  }

  /// A metal disc on either side of the head, where an ear would be, with a hub in the middle of it. It shrinks away
  /// with menace, as a [fin](Self::fin) takes its place.
  fn ear(&self, at: Vec2) -> Option<Vec3> {
    let radius = 2.0 * (1.0 - self.menace);
    if radius < 0.5 {
      return None;
    }
    let from = Vec2::new((at.x - Self::CENTER_X).abs() - 7.7, at.y - 10.8);
    let normal = from / radius;
    let out = normal.length_squared();
    if out > 1.0 {
      return None;
    }
    Some(if out < 0.1 {
      METAL[3]
    } else if out < 0.4 {
      SEAM
    } else {
      metal(self.lit(normal))
    })
  }

  /// A metal blade that sweeps up and out from either side of the head, as long as there's menace
  fn fin(&self, at: Vec2) -> Option<Vec3> {
    const BASE: f32 = 12.5;
    let length = 8.0 * self.menace;
    if length < 0.5 || !(BASE - length..BASE).contains(&at.y) {
      return None;
    }
    // 0 where it leaves the head, 1 at its tip
    let along = (BASE - at.y) / length;
    let across = (at.x - Self::CENTER_X).abs();
    let inner = 6.6 + 2.8 * along * self.menace;
    let outer = inner + (0.7 + 2.3 * (1.0 - along)) * self.menace;
    if !(inner..outer).contains(&across) {
      return None;
    }
    Some(if across < inner + 0.5 {
      METAL[0]
    } else if across > outer - 0.8 {
      METAL[3]
    } else {
      METAL[2]
    })
  }

  /// How far up the shoulders and collar are hunched, around a head that sinks into them
  fn hunch(&self) -> f32 {
    2.0 * self.menace
  }

  /// The metal ring the neck goes into, in segments
  fn collar(&self, at: Vec2) -> Option<Vec3> {
    let normal = Vec2::new(
      (at.x - Self::CENTER_X) / 7.0,
      (at.y + self.hunch() - 24.6) / 1.7,
    );
    if normal.length_squared() > 1.0 {
      return None;
    }
    let across = (at.x - Self::CENTER_X).abs();
    Some(
      if (across - 2.5).abs() < 0.3 || (across - 5.0).abs() < 0.3 {
        SEAM
      } else {
        metal(self.lit(Vec2::new(normal.x, normal.y * 0.5 - 0.4)))
      },
    )
  }

  /// The neck: a ringed metal core, with a piston either side of it that runs from under the jaw out to the collar
  fn neck(&self, at: Vec2) -> Option<Vec3> {
    /// Half of how wide the core is
    const CORE: f32 = 2.5;
    /// Half of how wide a piston is
    const PISTON: f32 = 1.0;
    if !(17.5..25.0).contains(&at.y) {
      return None;
    }
    let across = (at.x - Self::CENTER_X).abs();
    if across <= CORE {
      // every other row is the gap between two rings
      let ring = if at.y.rem_euclid(2.0) < 1.0 {
        1.0
      } else {
        0.45
      };
      // the chin's shadow falls on the top of it
      let shadow = if at.y < 21.0 { 0.6 } else { 1.0 };
      let across = (at.x - Self::CENTER_X) / CORE;
      return Some(metal(self.lit(Vec2::new(across * 0.8, -0.2))) * ring * shadow);
    }
    // a piston leans outward on its way down, with a sleeve around the lower part of its rod
    let center = 3.7 + 0.28 * (at.y - 19.0);
    let sleeve = at.y >= 21.6 - self.hunch();
    let half = if sleeve { PISTON } else { PISTON * 0.7 };
    let from = (across - center) / half;
    if from.abs() > 1.0 {
      // what's behind the core and the pistons, between them
      return (across < center).then_some(SEAM);
    }
    let from = from * (at.x - Self::CENTER_X).signum();
    Some(if sleeve {
      metal(self.lit(Vec2::new(from * 0.8, -0.2)))
    } else {
      plastic(self.lit(Vec2::new(from * 0.8, -0.1)), 0.0)
    })
  }

  /// The shoulders: a shell either side of a metal chest, squared off at the top. The shells are white, and with
  /// menace they darken to gunmetal, square off further, and hunch up.
  fn shoulders(&self, at: Vec2) -> Option<Vec3> {
    const CENTER: Vec2 = Vec2::new(Figure::CENTER_X, 33.6);
    const RADII: Vec2 = Vec2::new(17.8, 10.2);
    let at = Vec2::new(at.x, at.y + self.hunch());
    let normal = (at - CENTER) / RADII;
    let corners = 2.8 + 2.2 * self.menace;
    if normal.x.abs().powf(corners) + normal.y.abs().powf(corners) > 1.0 {
      return None;
    }
    let across = (at.x - Self::CENTER_X).abs();
    // the chest widens downward, and each shoulder's shell is a plate of its own
    let chest = 3.8 + 0.4 * (at.y - 25.0);
    let lit = self.lit(Vec2::new(normal.x * 0.9, normal.y.min(0.0) * 0.95));
    if across < chest - 0.4 {
      // vents are cut across the chest
      let on_vent =
        across < chest - 1.6 && (at.y.rem_euclid(2.0) - 1.5).abs() < 0.35 && at.y > 26.5;
      return Some(if on_vent { SEAM } else { metal(lit) });
    }
    // the shell over the upper arm is jointed to the one over the shoulder, which a bolt holds down
    let on_seam = across < chest + 0.4 || (across - 11.6).abs() < 0.35;
    let on_joint = across > 11.6 && (at.y - 29.3).abs() < 0.3;
    let on_bolt = Vec2::new(across - 9.2, at.y - 27.3).length() < 0.75;
    Some(if on_seam || on_joint || on_bolt {
      SEAM
    } else if across < chest + 1.5 {
      // a metal trim runs along the edge of the chest
      metal(lit + 0.2)
    } else {
      plastic(lit, self.menace)
    })
  }

  /// How much light a surface gets, 0 to 1, where `normal` is how far across and down it faces: (0, 0) faces the
  /// viewer, and a length of 1 faces sideways
  fn lit(&self, normal: Vec2) -> f32 {
    let out = (1.0 - normal.length_squared()).max(0.0).sqrt();
    normal.extend(out).dot(self.light).max(0.0)
  }
}

/// The light in the Steward's visor
struct Visor {
  /// How bright it is, 0 to 1
  level: f32,
  /// Where its brightest part is across it, -1 to 1
  look: f32,
}

impl Visor {
  /// The middle of the slit
  const CENTER: Vec2 = Vec2::new(Figure::CENTER_X, 9.3);

  /// How far the slit's ends are from its middle, before they round off
  const HALF_LENGTH: f32 = 4.4;

  /// Half of how tall the slit is
  const RADIUS: f32 = 1.65;

  /// The y of the top of the slit
  const TOP: f32 = Self::CENTER.y - Self::RADIUS;

  /// How wide the dark frame around the slit is
  const BEZEL: f32 = 0.5;

  /// The red it casts on what's around it
  const CAST: Vec3 = Vec3::new(1.0, 0.1, 0.06);

  /// How far `at` is outside the slit, below 0 inside it
  fn distance(at: Vec2) -> f32 {
    let from = at - Self::CENTER;
    let along = from.x.clamp(-Self::HALF_LENGTH, Self::HALF_LENGTH);
    (from - Vec2::new(along, 0.0)).length() - Self::RADIUS
  }

  /// The glass and the light behind it at `at`, `depth` into the slit from its edge (0) to its middle (1): a dim edge,
  /// a lit body, and a hot spot where it's looking
  fn glass(&self, at: Vec2, depth: f32) -> Vec3 {
    let spot = (at.x - (Self::CENTER.x + self.look * 3.4)).abs();
    let bright = self.level
      * if spot < 1.6 {
        1.0
      } else if depth < 0.3 {
        0.5
      } else {
        0.72
      };
    Vec3::new(
      0.14 + 0.86 * bright,
      0.02 + 0.5 * bright.powi(3),
      0.02 + 0.3 * bright.powi(3),
    )
  }

  /// The frame around the slit, which glows a little with the light in it
  fn bezel(&self) -> Vec3 {
    SEAM.lerp(Vec3::new(0.6, 0.05, 0.04), 0.5 * self.level)
  }

  /// `color` with the red the visor casts on what's around it, `distance` outside the slit. The red is two bands
  /// around the visor, which reach farther and deepen the brighter it is.
  fn cast_on(&self, color: Vec3, distance: f32) -> Vec3 {
    let reach = 0.5 + 2.3 * self.level * self.level;
    let from = distance - Self::BEZEL;
    let cast = if from < reach * 0.45 {
      0.75
    } else if from < reach {
      0.35
    } else {
      0.0
    };
    color.lerp(Self::CAST, cast * self.level)
  }
}

/// The dark line where two parts meet
const SEAM: Vec3 = Vec3::new(0.13, 0.14, 0.17);

/// The tones of white plastic, from in shadow to fully lit
const PLASTIC: [Vec3; 3] = [
  Vec3::new(0.42, 0.47, 0.56),
  Vec3::new(0.74, 0.78, 0.85),
  Vec3::new(0.98, 0.99, 1.0),
];

/// The tones of gunmetal plastic, from in shadow to fully lit
const GUNMETAL: [Vec3; 3] = [
  Vec3::new(0.1, 0.11, 0.15),
  Vec3::new(0.21, 0.23, 0.29),
  Vec3::new(0.36, 0.39, 0.47),
];

/// The tones of silver metal, from in shadow to its highlight
const METAL: [Vec3; 4] = [
  Vec3::new(0.2, 0.22, 0.28),
  Vec3::new(0.38, 0.42, 0.5),
  Vec3::new(0.6, 0.64, 0.71),
  Vec3::new(0.93, 0.95, 1.0),
];

/// Plastic in flat tones: bright wherever much light reaches, and darker in shadow. It's white, and gunmetal as
/// `dark` goes to 1.
fn plastic(lit: f32, dark: f32) -> Vec3 {
  let tone = if lit > 0.26 {
    2
  } else if lit > 0.1 {
    1
  } else {
    0
  };
  PLASTIC[tone].lerp(GUNMETAL[tone], dark)
}

/// Silver metal, in flat tones: darker than plastic, with a hard highlight
fn metal(lit: f32) -> Vec3 {
  METAL[if lit > 0.86 {
    3
  } else if lit > 0.55 {
    2
  } else if lit > 0.25 {
    1
  } else {
    0
  }]
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use frontbox_core::prelude::Resources;
  use frontbox_scene::{ImageGenerators, Scene};

  use super::*;

  /// The image with its visor idling at 0.3, and no menace
  fn drawn(speech: f32, look: f32) -> RgbaImage {
    drawn_with_menace(speech, look, 0.0)
  }

  /// The image with its visor idling at 0.3
  fn drawn_with_menace(speech: f32, look: f32, menace: f32) -> RgbaImage {
    drawn_with(&[speech, 0.3, look, menace, 0.0])
  }

  /// The image with a value for each of its settings, in order
  fn drawn_with(values: &[f32]) -> RgbaImage {
    let values = values.to_vec();
    StewardGenerator.generate(&GeneratorFrame::new(
      0.0,
      0,
      0,
      StewardGenerator::SETTINGS,
      &values,
    ))
  }

  #[test]
  fn its_visor_reddens_as_it_speaks_and_casts_red_on_its_face() {
    let (silent, speaking) = (drawn(0.0, 0.0), drawn(1.0, 0.0));
    // in the middle of the visor, and on the faceplate just under it
    for (x, y) in [(18, 9), (18, 11)] {
      let [red, green, ..] = speaking.get_pixel(x, y).0;
      assert!(red > silent.get_pixel(x, y)[0], "at ({x}, {y})");
      assert!(red > green, "at ({x}, {y})");
    }
    // and no farther than its face
    assert_eq!(silent.get_pixel(18, 30), speaking.get_pixel(18, 30));
  }

  #[test]
  fn a_look_moves_the_brightest_part_of_its_visor() {
    let (right, left) = (drawn(1.0, -1.0), drawn(1.0, 1.0));
    assert!(right.get_pixel(15, 9)[1] > left.get_pixel(15, 9)[1]);
    assert!(left.get_pixel(20, 9)[1] > right.get_pixel(20, 9)[1]);
  }

  #[test]
  fn its_clear_around_the_figure() {
    let image = drawn(1.0, 0.0);
    assert_eq!(
      (image.width(), image.height()),
      (StewardGenerator::WIDTH, StewardGenerator::HEIGHT)
    );
    for (x, y) in [(0, 0), (35, 0), (2, 16), (33, 16)] {
      assert_eq!(image.get_pixel(x, y)[3], 0, "at ({x}, {y})");
    }
    assert_eq!(image.get_pixel(18, 14)[3], 255);
  }

  #[test]
  fn menace_brings_its_brow_down_over_the_same_visor() {
    let (servant, foe) = (drawn(0.0, 0.0), drawn_with_menace(0.0, 0.0, 1.0));
    // just over the visor, where the white of its forehead is
    assert!(servant.get_pixel(18, 6)[1] > foe.get_pixel(18, 6)[1] + 40);
    assert_eq!(servant.get_pixel(18, 9), foe.get_pixel(18, 9));
    for (x, y) in [(0, 0), (35, 0), (2, 16), (33, 16)] {
      assert_eq!(foe.get_pixel(x, y)[3], 0, "at ({x}, {y})");
    }
  }

  #[test]
  fn an_angular_head_fills_the_corners_an_egg_leaves_clear() {
    let (egg, angular) = (drawn(0.0, 0.0), drawn_with(&[0.0, 0.3, 0.0, 0.0, 1.0]));
    assert_eq!(egg.get_pixel(12, 2)[3], 0);
    assert_eq!(angular.get_pixel(12, 2)[3], 255);
    assert_eq!(angular.get_pixel(23, 2)[3], 255);
  }

  #[test]
  fn the_test_scene_opens_with_it() {
    let mut resources = Resources::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets"));
    resources.add::<ImageGenerators>(StewardGenerator::NAME, Arc::new(StewardGenerator));
    Scene::open(&resources, "dmd/steward_test.scene").unwrap();
  }
}
