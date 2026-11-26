use frontbox_gfx::scene::{
  Generated, GeneratorFrame, GeneratorHandle, GeneratorSetting, ShapeGenerator,
};

use super::{RootRoute, leaves};

/// The leaves of a [`RootGenerator`](super::RootGenerator)'s root, as the scene shape generator `root_leaves`, for a
/// layer of their own in another colour
///
/// They're the leaves of the root with the same `target_x`, `target_y`, `through_x`, `through_y`, `growth`, `meander`, `thickness`, and `seed`,
/// so a scene gives both layers the same of those, and puts this one under the root's. `leaves` is how many there are
/// on a whole root, and `leaf_size` how long each is.
#[derive(Debug, Default)]
pub struct RootLeavesGenerator;

impl RootLeavesGenerator {
  /// The name scenes use it by
  pub const NAME: &'static str = "root_leaves";

  const SETTINGS: &'static [GeneratorSetting] = &[
    GeneratorSetting::new("target_x", 10.0),
    GeneratorSetting::new("target_y", 0.0),
    GeneratorSetting::new("through_x", 5.0),
    GeneratorSetting::new("through_y", 0.0),
    GeneratorSetting::new("growth", 1.0).range(0.0, 1.0),
    GeneratorSetting::new("meander", 0.15).range(0.0, 1.0),
    GeneratorSetting::new("thickness", 1.0).range(0.0, 16.0),
    GeneratorSetting::new("seed", 0.0).range(0.0, 999.0),
    GeneratorSetting::new("leaves", 4.0).range(0.0, 24.0),
    GeneratorSetting::new("leaf_size", 3.0).range(0.0, 16.0),
  ];

  /// Where it grows to, and the point it passes through halfway there, to drag in an editor
  const HANDLES: &'static [GeneratorHandle] = &[
    GeneratorHandle::new("target", "target_x", "target_y"),
    GeneratorHandle::new("through", "through_x", "through_y"),
  ];
}

impl ShapeGenerator for RootLeavesGenerator {
  fn settings(&self) -> &[GeneratorSetting] {
    Self::SETTINGS
  }

  fn handles(&self) -> &[GeneratorHandle] {
    Self::HANDLES
  }

  fn generate(&self, frame: &GeneratorFrame) -> Generated {
    let route = RootRoute::set_by(frame);
    Generated::areas(leaves(
      &route,
      frame.setting("growth"),
      frame.setting("leaves").round() as usize,
      frame.setting("leaf_size"),
    ))
  }
}
