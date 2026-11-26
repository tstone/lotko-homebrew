use frontbox_core::prelude::Vec2;

use super::skeleton::hash;
use super::{RootRoute, RootSkeleton};

/// A place on a root where something grows out of its side: a rootlet or a leaf
///
/// Where they are is fixed by the route, so one stays put as the root grows past it, and starts growing itself when
/// the root's tip gets there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprout {
  /// Where it is on the root's centerline
  pub at: Vec2,
  /// How far along the route that is, from 0 to 1
  pub part: f32,
  /// The unit direction straight out of the side of the root it's on
  pub across: Vec2,
  /// The unit direction the root grows in here
  pub along: Vec2,
  /// What sets it apart from the route's others
  pub seed: u32,
}

impl Sprout {
  /// `count` of them spread along `route`, on alternate sides. `salt` sets one kind's apart from another's on the
  /// same route.
  pub fn along(route: &RootRoute, count: usize, salt: u32) -> Vec<Self> {
    let seed = route.seed ^ salt;
    let first = if hash(-1, seed) < 0.0 { 1.0 } else { -1.0 };
    (0..count)
      .filter_map(|index| {
        let cell = index as i32;
        let slot = (index as f32 + 0.5 + 0.35 * hash(cell, seed)) / count as f32;
        let part = 0.08 + 0.8 * slot;
        // the root grown to here has its tip here
        let up_to = RootSkeleton::grown(route, part);
        let [.., before, tip] = up_to.joints() else {
          return None;
        };
        let along = (tip.at - before.at).normalize_or_zero();
        let side = if index % 2 == 0 { first } else { -first };
        Some(Self {
          at: tip.at,
          part,
          across: along.perp() * side,
          along,
          seed: seed.wrapping_add((index as u32 + 1).wrapping_mul(0x9e37_79b9)),
        })
      })
      .collect()
  }

  /// How much of it has grown, from 0 to 1, when the root has grown `growth` of the way: none until the root's tip
  /// reaches it, and all once the tip has gone on `over` of the route further
  pub fn growth(&self, growth: f32, over: f32) -> f32 {
    ((growth - self.part) / over).clamp(0.0, 1.0)
  }

  /// A unit direction for it to grow in: out of its side of the root, at an angle of its own from `least` to `most`
  /// degrees away from the way the root grows. 90 is straight out, and less leans along the root.
  pub fn heading(&self, least: f32, most: f32) -> Vec2 {
    let lean = (least + (most - least) * self.pick(-1)).to_radians();
    self.along * lean.cos() + self.across * lean.sin()
  }

  /// A number from 0 to 1 that's its own, and another for another `which`
  pub fn pick(&self, which: i32) -> f32 {
    (hash(which, self.seed) + 1.0) / 2.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn route() -> RootRoute {
    RootRoute {
      target: Vec2::new(100.0, 0.0),
      through: Vec2::new(50.0, 0.0),
      meander: 0.0,
      thickness: 4.0,
      seed: 7,
    }
  }

  #[test]
  fn they_are_spread_along_the_route_in_order_on_alternate_sides() {
    let sprouts = Sprout::along(&route(), 6, 1);
    assert_eq!(sprouts.len(), 6);
    assert!(sprouts.windows(2).all(|pair| pair[0].part < pair[1].part));
    assert!(
      sprouts
        .windows(2)
        .all(|pair| pair[0].across == -pair[1].across)
    );
    for sprout in &sprouts {
      assert!((0.08..=0.88).contains(&sprout.part));
      assert!((sprout.at - Vec2::new(100.0 * sprout.part, 0.0)).length() < 0.01);
      assert_eq!(sprout.across.x, 0.0);
      assert_eq!(sprout.along, Vec2::X);
    }
  }

  #[test]
  fn a_salt_or_a_seed_puts_them_elsewhere() {
    let sprouts = Sprout::along(&route(), 4, 1);
    assert_eq!(Sprout::along(&route(), 4, 1), sprouts);
    assert_ne!(Sprout::along(&route(), 4, 2), sprouts);
    assert_ne!(
      Sprout::along(&RootRoute { seed: 8, ..route() }, 4, 1),
      sprouts
    );
  }

  #[test]
  fn one_starts_growing_when_the_tip_reaches_it() {
    let sprout = Sprout::along(&route(), 1, 1)[0];
    assert_eq!(sprout.growth(sprout.part - 0.01, 0.2), 0.0);
    assert!((sprout.growth(sprout.part + 0.1, 0.2) - 0.5).abs() < 0.001);
    assert_eq!(sprout.growth(sprout.part + 0.3, 0.2), 1.0);
  }

  #[test]
  fn each_heads_out_of_its_side_at_an_angle_of_its_own() {
    let sprouts = Sprout::along(&route(), 8, 1);
    let mut angles = Vec::new();
    for sprout in &sprouts {
      let heading = sprout.heading(20.0, 60.0);
      assert!((heading.length() - 1.0).abs() < 0.001);
      assert!(heading.dot(sprout.across) > 0.0);
      let angle = heading.dot(sprout.along).acos().to_degrees();
      assert!((19.99..=60.01).contains(&angle), "{angle}");
      angles.push(angle);
    }
    let spread = angles.iter().fold(0.0, |most: f32, a| most.max(*a))
      - angles.iter().fold(90.0, |least: f32, a| least.min(*a));
    assert!(spread > 10.0, "{angles:?}");
  }

  #[test]
  fn a_route_that_goes_nowhere_has_none() {
    let nowhere = RootRoute {
      target: Vec2::ZERO,
      through: Vec2::ZERO,
      ..route()
    };
    assert!(Sprout::along(&nowhere, 4, 1).is_empty());
  }
}
