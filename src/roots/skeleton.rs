use frontbox_core::prelude::Vec2;
use frontbox_gfx::scene::GeneratorFrame;

/// A root's centerline at one stage of its growth: joints from its base to its tip, each with how thick the root is
/// there. Everything drawn of a root (its outline, its leaves) is made from this.
///
/// A root grows along a route that's fixed by what it's made from, so the part that's grown never moves as the tip
/// goes on.
#[derive(Debug, Clone, PartialEq)]
pub struct RootSkeleton {
  joints: Vec<Joint>,
}

impl RootSkeleton {
  /// Segments in a whole route, however long it is
  const SEGMENTS: usize = 24;

  /// The root along `route`, grown `growth` of the way, from 0 to 1. At 0 it has no joints.
  pub fn grown(route: &RootRoute, growth: f32) -> Self {
    let growth = growth.clamp(0.0, 1.0);
    if growth <= 0.0 {
      return Self { joints: Vec::new() };
    }
    let points: Vec<Vec2> = (0..=Self::SEGMENTS)
      .map(|i| route.at(i as f32 / Self::SEGMENTS as f32))
      .collect();
    let whole: f32 = points
      .windows(2)
      .map(|pair| pair[0].distance(pair[1]))
      .sum();
    let length = whole * growth;
    let radius = |along: f32| {
      if whole > 0.0 {
        route.radius(along / whole, growth)
      } else {
        0.0
      }
    };

    let mut joints = vec![Joint {
      at: points[0],
      radius: radius(0.0),
    }];
    let mut along = 0.0;
    for pair in points.windows(2) {
      let step = pair[0].distance(pair[1]);
      if along + step >= length {
        // the tip, part of the way along this segment
        let part = if step > 0.0 {
          (length - along) / step
        } else {
          0.0
        };
        if part > 0.0 {
          joints.push(Joint {
            at: pair[0].lerp(pair[1], part),
            radius: 0.0,
          });
        }
        break;
      }
      along += step;
      joints.push(Joint {
        at: pair[1],
        radius: radius(along),
      });
    }
    Self { joints }
  }

  pub fn joints(&self) -> &[Joint] {
    &self.joints
  }

  /// The same root with its base at `base` rather than (0, 0)
  pub fn moved(mut self, base: Vec2) -> Self {
    for joint in &mut self.joints {
      joint.at += base;
    }
    self
  }

  /// The points of its centerline, base to tip
  pub fn centerline(&self) -> Vec<Vec2> {
    self.joints.iter().map(|joint| joint.at).collect()
  }

  /// The unit direction across the root at joint `index`, to the left of the way it grows. Zero for a root of fewer
  /// than two joints.
  pub fn across(&self, index: usize) -> Vec2 {
    let last = self.joints.len().saturating_sub(1);
    let before = self.joints[index.saturating_sub(1)].at;
    let after = self.joints[(index + 1).min(last)].at;
    (after - before).perp().normalize_or_zero()
  }
}

/// One point of a [`RootSkeleton`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Joint {
  pub at: Vec2,
  /// Half the root's width here. 0 at the tip.
  pub radius: f32,
}

/// Where a root goes when it's whole: from (0, 0) in a curve through `through` to `target`, wandering to each side on
/// the way
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RootRoute {
  pub target: Vec2,
  /// A point it passes through halfway. Halfway to `target` is the straight way there.
  pub through: Vec2,
  /// About how far it wanders to each side of its curve, as a part of the distance to `target`. 0 doesn't wander.
  pub meander: f32,
  /// Half the width of a whole root at its base
  pub thickness: f32,
  /// Which way it wanders. The same seed is the same route.
  pub seed: u32,
}

impl RootRoute {
  /// The route a generator's settings give: `target_x`, `target_y`, `through_x`, `through_y`, `meander`, `thickness`,
  /// and `seed`
  pub fn set_by(frame: &GeneratorFrame) -> Self {
    Self {
      target: Vec2::new(frame.setting("target_x"), frame.setting("target_y")),
      through: Vec2::new(frame.setting("through_x"), frame.setting("through_y")),
      meander: frame.setting("meander"),
      thickness: frame.setting("thickness"),
      seed: frame.setting("seed") as u32,
    }
  }

  /// Half the root's width `part` of the way along the whole route, from 0 to 1, when it has grown `growth` of the
  /// way. A root that's just started is thinner at its base than one that's grown, and it's 0 from its tip on.
  pub fn radius(&self, part: f32, growth: f32) -> f32 {
    if growth <= 0.0 {
      return 0.0;
    }
    self.thickness * growth.sqrt() * (1.0 - part / growth).max(0.0)
  }

  /// The point `t` of the way along it, from 0 at the base to 1 at the target
  fn at(&self, t: f32) -> Vec2 {
    // a curve that's at `through` halfway
    let pull = self.through * 2.0 - self.target * 0.5;
    let curve = pull * (2.0 * t * (1.0 - t)) + self.target * (t * t);
    // nothing at either end or halfway, so it starts at the base, passes through `through`, and arrives at the target
    let reach = (t * std::f32::consts::PI).sin();
    let wander = self.wander(t) - self.wander(0.5);
    curve + self.target.perp() * (self.meander * reach * wander)
  }

  /// Which side of its curve it's off to `t` of the way along, and how far, from -1 to 1
  fn wander(&self, t: f32) -> f32 {
    0.65 * noise(t * 3.0, self.seed) + 0.35 * noise(t * 7.0, self.seed ^ 0x9e37_79b9)
  }
}

/// Smooth noise from -1 to 1, the same for the same `x` and `seed`
fn noise(x: f32, seed: u32) -> f32 {
  let cell = x.floor();
  let part = x - cell;
  let ease = part * part * (3.0 - 2.0 * part);
  let from = hash(cell as i32, seed);
  let to = hash(cell as i32 + 1, seed);
  from + (to - from) * ease
}

/// A number from -1 to 1 for a whole `cell` and `seed`
pub(super) fn hash(cell: i32, seed: u32) -> f32 {
  let mut bits = (cell as u32).wrapping_mul(0x85eb_ca6b) ^ seed.wrapping_mul(0xc2b2_ae35);
  bits ^= bits >> 16;
  bits = bits.wrapping_mul(0x7feb_352d);
  bits ^= bits >> 15;
  bits = bits.wrapping_mul(0x846c_a68b);
  bits ^= bits >> 16;
  bits as f32 / u32::MAX as f32 * 2.0 - 1.0
}

#[cfg(test)]
mod tests {
  use super::*;

  fn route() -> RootRoute {
    RootRoute {
      target: Vec2::new(100.0, 0.0),
      through: Vec2::new(50.0, 0.0),
      meander: 0.2,
      thickness: 4.0,
      seed: 7,
    }
  }

  fn length(skeleton: &RootSkeleton) -> f32 {
    skeleton
      .centerline()
      .windows(2)
      .map(|pair| pair[0].distance(pair[1]))
      .sum()
  }

  #[test]
  fn a_whole_root_goes_from_the_base_to_the_target() {
    let whole = RootSkeleton::grown(&route(), 1.0);
    let joints = whole.joints();
    assert_eq!(joints.len(), RootSkeleton::SEGMENTS + 1);
    assert_eq!(joints[0].at, Vec2::ZERO);
    assert!(joints.last().unwrap().at.distance(route().target) < 0.001);
  }

  #[test]
  fn a_root_that_hasnt_grown_has_no_joints() {
    assert!(RootSkeleton::grown(&route(), 0.0).joints().is_empty());
    assert!(RootSkeleton::grown(&route(), -1.0).joints().is_empty());
  }

  #[test]
  fn its_length_is_its_growth_of_the_whole() {
    let whole = length(&RootSkeleton::grown(&route(), 1.0));
    for growth in [0.01, 0.25, 0.5, 0.77] {
      let grown = length(&RootSkeleton::grown(&route(), growth));
      assert!((grown - whole * growth).abs() < 0.01, "{growth}: {grown}");
    }
  }

  #[test]
  fn what_has_grown_stays_where_it_is_as_the_tip_goes_on() {
    let half = RootSkeleton::grown(&route(), 0.5);
    let whole = RootSkeleton::grown(&route(), 1.0);
    // all but the tip, which is part of the way along a segment
    let settled = half.joints().len() - 1;
    assert!(settled > 1);
    for index in 0..settled {
      assert_eq!(half.joints()[index].at, whole.joints()[index].at);
    }
  }

  #[test]
  fn it_tapers_from_its_base_to_nothing_at_its_tip() {
    for growth in [0.3, 1.0] {
      let skeleton = RootSkeleton::grown(&route(), growth);
      let radii: Vec<f32> = skeleton.joints().iter().map(|joint| joint.radius).collect();
      assert!(radii.windows(2).all(|pair| pair[0] > pair[1]), "{radii:?}");
      assert!(*radii.last().unwrap() < 0.001);
    }
  }

  #[test]
  fn a_young_root_is_thinner_at_its_base() {
    let young = RootSkeleton::grown(&route(), 0.25).joints()[0].radius;
    let whole = RootSkeleton::grown(&route(), 1.0).joints()[0].radius;
    assert_eq!(whole, 4.0);
    assert_eq!(young, 2.0);
  }

  #[test]
  fn it_wanders_no_further_than_its_meander_and_a_seed_picks_the_way() {
    let straight = RootRoute {
      meander: 0.0,
      ..route()
    };
    assert!(
      RootSkeleton::grown(&straight, 1.0)
        .joints()
        .iter()
        .all(|joint| joint.at.y == 0.0)
    );
    let furthest = |route: &RootRoute| {
      RootSkeleton::grown(route, 1.0)
        .joints()
        .iter()
        .map(|joint| joint.at.y.abs())
        .fold(0.0, f32::max)
    };
    let wandered = furthest(&route());
    assert!(wandered > 1.0 && wandered <= 40.0, "{wandered}");
    let other = RootRoute { seed: 8, ..route() };
    assert_eq!(
      RootSkeleton::grown(&route(), 1.0),
      RootSkeleton::grown(&route(), 1.0)
    );
    assert_ne!(
      RootSkeleton::grown(&route(), 1.0),
      RootSkeleton::grown(&other, 1.0)
    );
  }

  #[test]
  fn it_passes_through_its_through_point_however_it_wanders() {
    let bent = RootRoute {
      through: Vec2::new(40.0, 30.0),
      ..route()
    };
    let whole = RootSkeleton::grown(&bent, 1.0);
    let middle = whole.joints()[RootSkeleton::SEGMENTS / 2].at;
    assert!(middle.distance(bent.through) < 0.001, "{middle}");
    assert_eq!(whole.joints()[0].at, Vec2::ZERO);
    assert!(whole.joints().last().unwrap().at.distance(bent.target) < 0.001);
  }

  #[test]
  fn across_is_to_the_left_of_the_way_it_grows() {
    let straight = RootRoute {
      meander: 0.0,
      ..route()
    };
    let skeleton = RootSkeleton::grown(&straight, 1.0);
    for index in [0, 5, RootSkeleton::SEGMENTS] {
      assert_eq!(skeleton.across(index), Vec2::new(0.0, 1.0));
    }
  }
}
