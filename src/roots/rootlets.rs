use super::{RootRoute, RootSkeleton, Sprout};

/// The rootlets of the root along `route` when it has grown `growth` of the way: up to `count` small roots out of
/// its sides, each leaning along it at an angle of its own, started when the tip passed it and growing as the tip
/// goes on. Ones the tip hasn't reached are
/// left out.
///
/// The longest a whole one gets is `length`, as a part of the distance to the route's target, and each has a length
/// of its own, down to a quarter of that. Each is thinner than the root where it leaves it.
pub fn rootlets(route: &RootRoute, growth: f32, count: usize, length: f32) -> Vec<RootSkeleton> {
  Sprout::along(route, count, SALT)
    .into_iter()
    .filter_map(|sprout| {
      let own = sprout.growth(growth, GROWN_OVER);
      if own <= 0.0 {
        return None;
      }
      let reach = route.target.length() * length * (SHORTEST + (1.0 - SHORTEST) * sprout.pick(0));
      let target = sprout.heading(LEAN.0, LEAN.1) * reach;
      let rootlet = RootRoute {
        target,
        through: target * 0.5,
        meander: route.meander,
        thickness: THICKNESS * route.radius(sprout.part, growth),
        seed: sprout.seed,
      };
      Some(RootSkeleton::grown(&rootlet, own).moved(sprout.at))
    })
    .collect()
}

/// Sets where rootlets are apart from where anything else on the route is
const SALT: u32 = 0x7f4a_7c15;

/// How much further of the route the tip has gone when a rootlet is whole
const GROWN_OVER: f32 = 0.3;

/// The shortest a whole rootlet is, as a part of the longest
const SHORTEST: f32 = 0.25;

/// The least and most a rootlet turns away from the way the root grows, in degrees
const LEAN: (f32, f32) = (12.0, 55.0);

/// A rootlet's width at its base, as a part of the root's there
const THICKNESS: f32 = 0.7;

#[cfg(test)]
mod tests {
  use frontbox_core::prelude::Vec2;

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

  #[test]
  fn there_are_none_until_the_tip_reaches_one_and_then_more() {
    assert!(rootlets(&route(), 0.05, 5, 0.2).is_empty());
    let half = rootlets(&route(), 0.5, 5, 0.2).len();
    let whole = rootlets(&route(), 1.0, 5, 0.2).len();
    assert!(half > 0 && half < whole);
    assert_eq!(whole, 5);
  }

  #[test]
  fn one_starts_on_the_root_and_is_thinner_than_it_there() {
    let sprouts = Sprout::along(&route(), 5, SALT);
    for (rootlet, sprout) in rootlets(&route(), 1.0, 5, 0.2).iter().zip(&sprouts) {
      let base = rootlet.joints()[0];
      assert_eq!(base.at, sprout.at);
      assert!(base.radius < route().radius(sprout.part, 1.0));
    }
  }

  #[test]
  fn they_are_of_different_lengths() {
    let longs: Vec<f32> = rootlets(&route(), 1.0, 12, 0.2)
      .iter()
      // the ones the tip passed long enough ago to be whole
      .take(8)
      .map(|rootlet| {
        rootlet.joints()[0]
          .at
          .distance(rootlet.joints().last().unwrap().at)
      })
      .collect();
    let longest = longs.iter().copied().fold(0.0, f32::max);
    let shortest = longs.iter().copied().fold(f32::MAX, f32::min);
    assert!(longest > shortest * 1.8, "{longs:?}");
  }

  #[test]
  fn one_keeps_growing_as_the_tip_goes_on_and_is_no_longer_than_its_length() {
    let long = |growth: f32| {
      let first = &rootlets(&route(), growth, 5, 0.2)[0];
      let joints = first.joints();
      joints[0].at.distance(joints.last().unwrap().at)
    };
    assert!(long(0.2) < long(0.3));
    assert!(long(1.0) <= 20.0);
    assert!(long(1.0) >= 4.99);
  }
}
