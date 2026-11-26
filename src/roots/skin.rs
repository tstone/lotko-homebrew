use frontbox_core::prelude::Vec2;

use super::RootSkeleton;
use super::skeleton::hash;

/// The outline of a root around `skeleton`: up its left side from the base to the tip, and back down its right. Empty
/// for a root with no length.
///
/// Its edges run straight from joint to joint, and `gnarl` makes it wider or narrower at each one, on each side apart,
/// by up to that part of its width there: 0 is a smooth taper, as a tentacle has, and more is knottier. A joint keeps
/// its knots as the root grows past it, and the same `seed` gives the same ones.
pub fn skin(skeleton: &RootSkeleton, gnarl: f32, seed: u32) -> Vec<Vec2> {
  let joints = skeleton.joints();
  if joints.len() < 2 {
    return Vec::new();
  }
  let tip = joints.len() - 1;
  let side = |index: usize, side: Side| {
    let joint = joints[index];
    let knot = 1.0 + gnarl * hash(index as i32, seed ^ side.salt());
    joint.at + skeleton.across(index) * (joint.radius * knot * side.sign())
  };
  let left = (0..tip).map(|index| side(index, Side::Left));
  let right = (0..tip).rev().map(|index| side(index, Side::Right));
  left
    .chain(std::iter::once(joints[tip].at))
    .chain(right)
    .collect()
}

/// A side of a root, looking the way it grows
#[derive(Debug, Clone, Copy)]
enum Side {
  Left,
  Right,
}

impl Side {
  /// Which way it is along [`RootSkeleton::across`]
  fn sign(self) -> f32 {
    match self {
      Self::Left => 1.0,
      Self::Right => -1.0,
    }
  }

  /// What sets its knots apart from the other side's
  fn salt(self) -> u32 {
    match self {
      Self::Left => 0x51ed_270b,
      Self::Right => 0xa3c5_9ac3,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::RootRoute;
  use super::*;

  fn skeleton(growth: f32) -> RootSkeleton {
    let route = RootRoute {
      target: Vec2::new(100.0, 0.0),
      through: Vec2::new(50.0, 0.0),
      meander: 0.0,
      thickness: 4.0,
      seed: 7,
    };
    RootSkeleton::grown(&route, growth)
  }

  #[test]
  fn it_goes_up_one_side_to_the_tip_and_back_down_the_other() {
    let skeleton = skeleton(1.0);
    let joints = skeleton.joints().len();
    let outline = skin(&skeleton, 0.0, 0);
    assert_eq!(outline.len(), joints * 2 - 1);
    assert_eq!(outline[0], Vec2::new(0.0, 4.0));
    assert_eq!(outline[joints - 1], Vec2::new(100.0, 0.0));
    assert_eq!(*outline.last().unwrap(), Vec2::new(0.0, -4.0));
  }

  #[test]
  fn without_gnarl_each_side_is_the_joints_radius_away() {
    let skeleton = skeleton(0.6);
    let outline = skin(&skeleton, 0.0, 0);
    for (index, joint) in skeleton.joints().iter().enumerate() {
      let left = outline[index];
      let right = outline[outline.len() - 1 - index];
      assert!((left.distance(joint.at) - joint.radius).abs() < 0.001);
      assert!((right.distance(joint.at) - joint.radius).abs() < 0.001);
    }
  }

  #[test]
  fn gnarl_widens_and_narrows_each_side_by_no_more_than_its_part() {
    let skeleton = skeleton(1.0);
    let outline = skin(&skeleton, 0.5, 3);
    let mut differed = false;
    for (index, joint) in skeleton.joints().iter().enumerate().take(20) {
      let left = outline[index].distance(joint.at) / joint.radius;
      let right = outline[outline.len() - 1 - index].distance(joint.at) / joint.radius;
      assert!((0.499..=1.501).contains(&left), "{left}");
      assert!((0.499..=1.501).contains(&right), "{right}");
      differed |= (left - right).abs() > 0.05;
    }
    assert!(differed);
    assert_eq!(skin(&skeleton, 0.5, 3), outline);
    assert_ne!(skin(&skeleton, 0.5, 4), outline);
  }

  #[test]
  fn a_joint_keeps_its_knots_as_the_root_grows_past_it() {
    let knot = |growth: f32| {
      let skeleton = skeleton(growth);
      let joint = skeleton.joints()[2];
      skin(&skeleton, 0.5, 3)[2].distance(joint.at) / joint.radius
    };
    assert!((knot(0.5) - knot(1.0)).abs() < 0.001);
  }

  #[test]
  fn a_root_with_no_length_has_no_outline() {
    assert!(skin(&skeleton(0.0), 0.5, 3).is_empty());
  }
}
