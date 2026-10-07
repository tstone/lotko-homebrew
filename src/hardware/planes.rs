use std::sync::LazyLock;

use frontbox::prelude::*;

pub static PLAYFIELD: LazyLock<ReferencePlane> = LazyLock::new(|| {
  ReferencePlane::new("Playfield")
    .origin(Vec3::new(0.875, 3.25, 13.5))
    .extent(Vec2::new(20.25, 45.0))
    .rotation(Quat::from_rotation_x(-3.5f32.to_radians()))
    .image("src/assets/playfield.png")
    .build()
});

pub static BACKBOARD: LazyLock<ReferencePlane> = LazyLock::new(|| {
  ReferencePlane::new("Backboard")
    .parent(&PLAYFIELD)
    .origin(Vec3::new(0.875, 0.0, 0.0))
    .extent(Vec2::new(18.5, 7.0))
    .rotation(Quat::from_axis_angle(Vec3::X, 90f32.to_radians()))
    .build()
});

pub static BACKBOX_PANEL: LazyLock<ReferencePlane> = LazyLock::new(|| {
  ReferencePlane::new("Backbox")
    .origin(Vec3::new(-3.0, 6.0, 23.0))
    .extent(Vec2::new(27.25, 27.0))
    .rotation(Quat::from_axis_angle(Vec3::X, 90f32.to_radians()))
    .build()
});

pub static CABINET_FRONT: LazyLock<ReferencePlane> = LazyLock::new(|| {
  ReferencePlane::new("Cabinet Front")
    .origin(Vec3::new(0.0, 50.0, 13.0))
    .extent(Vec2::new(22.0, 16.25))
    .rotation(Quat::from_axis_angle(Vec3::X, -90f32.to_radians()))
    .build()
});
