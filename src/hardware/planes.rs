use std::sync::LazyLock;

use frontbox::prelude::*;

pub static PLAYFIELD: LazyLock<ReferencePlane> = LazyLock::new(|| ReferencePlane {
  origin: Vec3::new(1.0, 3.25, 12.0),
  extent: Vec2::new(20.25, 45.0),
  rotation: Quat::from_axis_angle(Vec3::X, 6.5f32.to_radians()),
  parent: None,
});

pub static BACKBOARD: LazyLock<ReferencePlane> = LazyLock::new(|| ReferencePlane {
  origin: Vec3::new(0.875, 0.0, 0.0),
  extent: Vec2::new(18.5, 7.0),
  rotation: Quat::from_axis_angle(Vec3::X, 90f32.to_radians()),
  parent: Some(&*PLAYFIELD),
});

pub static BACKBOX: LazyLock<ReferencePlane> = LazyLock::new(|| ReferencePlane {
  origin: Vec3::new(-3.5, 4.0, 9.0),
  extent: Vec2::new(27.25, 7.0),
  rotation: Quat::from_axis_angle(Vec3::X, 90f32.to_radians()),
  parent: Some(&*PLAYFIELD),
});
