use frontbox_core::prelude::Vec2;

use super::{RootRoute, Sprout};

/// The outlines of the leaves on the root along `route` when it has grown `growth` of the way: up to `count`, each
/// opening from nothing once the tip has passed it. Ones the tip hasn't reached are left out.
///
/// A whole one is about `size` long, pointed at its end, and starts just inside the root's edge so that the root
/// drawn over it hides where they join.
pub fn leaves(route: &RootRoute, growth: f32, count: usize, size: f32) -> Vec<Vec<Vec2>> {
  Sprout::along(route, count, SALT)
    .into_iter()
    .filter_map(|sprout| {
      let own = sprout.growth(growth, OPEN_OVER);
      if own <= 0.0 {
        return None;
      }
      let long = sprout.heading(LEAN.0, LEAN.1) * (size * own * (0.7 + 0.3 * sprout.pick(0)));
      let wide = long.perp();
      let stem = sprout.at + sprout.across * (0.5 * route.radius(sprout.part, growth));
      Some(
        SHAPE
          .iter()
          .map(|&(along, out)| stem + long * along + wide * out)
          .collect(),
      )
    })
    .collect()
}

/// Sets where leaves are apart from where anything else on the route is
const SALT: u32 = 0x1b87_3593;

/// How much further of the route the tip has gone when a leaf is fully open
const OPEN_OVER: f32 = 0.12;

/// The least and most a leaf turns away from the way the root grows, in degrees
const LEAN: (f32, f32) = (40.0, 75.0);

/// A leaf 1 long from its stem at (0, 0): how far along it, and how far out to the side
const SHAPE: [(f32, f32); 6] = [
  (0.0, 0.0),
  (0.3, 0.26),
  (0.65, 0.2),
  (1.0, 0.0),
  (0.65, -0.2),
  (0.3, -0.26),
];

#[cfg(test)]
mod tests {
  use super::*;

  fn route() -> RootRoute {
    RootRoute {
      target: Vec2::new(100.0, 0.0),
      through: Vec2::new(50.0, 0.0),
      meander: 0.1,
      thickness: 4.0,
      seed: 7,
    }
  }

  fn long(leaf: &[Vec2]) -> f32 {
    leaf[0].distance(leaf[3])
  }

  #[test]
  fn there_are_none_until_the_tip_reaches_one_and_then_more() {
    assert!(leaves(&route(), 0.05, 5, 4.0).is_empty());
    let half = leaves(&route(), 0.5, 5, 4.0).len();
    assert!(half > 0 && half < 5);
    assert_eq!(leaves(&route(), 1.0, 5, 4.0).len(), 5);
  }

  #[test]
  fn one_opens_as_the_tip_goes_on_to_no_more_than_its_size() {
    let first = |growth: f32| long(&leaves(&route(), growth, 5, 4.0)[0]);
    let part = Sprout::along(&route(), 5, SALT)[0].part;
    assert!(first(part + 0.03) < first(part + 0.06));
    let whole = first(1.0);
    assert!(whole > 2.7 && whole <= 4.001, "{whole}");
  }

  #[test]
  fn one_starts_inside_the_root() {
    let sprouts = Sprout::along(&route(), 5, SALT);
    for (leaf, sprout) in leaves(&route(), 1.0, 5, 4.0).iter().zip(&sprouts) {
      assert!(leaf[0].distance(sprout.at) < route().radius(sprout.part, 1.0));
    }
  }
}
