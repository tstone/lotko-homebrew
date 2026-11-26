use frontbox_gfx::scene::{
  Generated, GeneratorFrame, GeneratorHandle, GeneratorSetting, ShapeGenerator,
};

use super::{RootRoute, RootSkeleton, rootlets, skin};

/// A root growing from (0, 0) to a target, by way of a point it passes through halfway, as the scene shape generator
/// `root`
///
/// A scene moves it along with `growth`, from 0 (nothing) to 1 (at the target), as keyframes or a variable. Nothing
/// about it changes with time otherwise, and the same `seed` is the same root every time it plays.
///
/// It's one area, wide at the base and tapering to the tip, with `gnarl` for how knotty its edges are. Up to
/// `rootlets` small roots grow out of its sides once the tip has passed them, each about `rootlet_length` long as a
/// part of the distance to the target. A sweep can follow its centerline to the tip.
///
/// [`RootLeavesGenerator`](super::RootLeavesGenerator) makes its leaves.
#[derive(Debug, Default)]
pub struct RootGenerator;

impl RootGenerator {
  /// The name scenes use it by
  pub const NAME: &'static str = "root";

  const SETTINGS: &'static [GeneratorSetting] = &[
    GeneratorSetting::new("target_x", 10.0),
    GeneratorSetting::new("target_y", 0.0),
    GeneratorSetting::new("through_x", 5.0),
    GeneratorSetting::new("through_y", 0.0),
    GeneratorSetting::new("growth", 1.0).range(0.0, 1.0),
    GeneratorSetting::new("meander", 0.15).range(0.0, 1.0),
    GeneratorSetting::new("thickness", 1.0).range(0.0, 16.0),
    GeneratorSetting::new("gnarl", 0.35).range(0.0, 1.0),
    GeneratorSetting::new("seed", 0.0).range(0.0, 999.0),
    GeneratorSetting::new("rootlets", 3.0).range(0.0, 12.0),
    GeneratorSetting::new("rootlet_length", 0.2).range(0.0, 1.0),
  ];

  /// Where it grows to, and the point it passes through halfway there, to drag in an editor
  const HANDLES: &'static [GeneratorHandle] = &[
    GeneratorHandle::new("target", "target_x", "target_y"),
    GeneratorHandle::new("through", "through_x", "through_y"),
  ];
}

impl ShapeGenerator for RootGenerator {
  fn settings(&self) -> &[GeneratorSetting] {
    Self::SETTINGS
  }

  fn has_path(&self) -> bool {
    true
  }

  fn handles(&self) -> &[GeneratorHandle] {
    Self::HANDLES
  }

  fn generate(&self, frame: &GeneratorFrame) -> Generated {
    let route = RootRoute::set_by(frame);
    let growth = frame.setting("growth");
    let gnarl = frame.setting("gnarl");
    let skeleton = RootSkeleton::grown(&route, growth);
    let small = rootlets(
      &route,
      growth,
      frame.setting("rootlets").round() as usize,
      frame.setting("rootlet_length"),
    );
    let mut outlines = vec![skin(&skeleton, gnarl, route.seed)];
    for (index, rootlet) in small.iter().enumerate() {
      outlines.push(skin(
        rootlet,
        gnarl,
        route.seed.wrapping_add(index as u32 + 1),
      ));
    }
    Generated::areas(outlines).path(skeleton.centerline(), false)
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use frontbox_core::prelude::Resources;
  use frontbox_scene::{Scene, ShapeGenerators};

  use super::super::RootLeavesGenerator;
  use super::*;

  #[test]
  fn the_test_scene_opens_with_it() {
    let mut resources = Resources::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets"));
    resources.add::<ShapeGenerators>(RootGenerator::NAME, Arc::new(RootGenerator));
    resources.add::<ShapeGenerators>(RootLeavesGenerator::NAME, Arc::new(RootLeavesGenerator));
    Scene::open(&resources, "dmd/root_test.scene").unwrap();
  }
}
