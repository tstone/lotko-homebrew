use std::collections::BTreeMap;
use std::sync::Mutex;

use frontbox_core::prelude::*;
use frontbox_gfx::scene::Value;
use frontbox_scene::PlaybackId;
use frontbox_scene::prelude::*;

/// The DMD, by the name it's added to the `DisplaySystem` under
pub const DMD: &str = "Dmd";
pub const DMD_WIDTH: u32 = 128;
pub const DMD_HEIGHT: u32 = 32;

/// The screen shown on each z, by any system. A screen takes the place of the one on its z, so one system can leave a
/// screen up for another to replace.
static SHOWN: Mutex<BTreeMap<i8, PlaybackId>> = Mutex::new(BTreeMap::new());

/// A scene from the app's resources, shown on the DMD with each of its display layers at `z`. It's owned by the system
/// that shows it, so it's hidden while that system is inactive and goes when it despawns.
#[derive(Clone)]
pub struct DmdScreen {
  name: &'static str,
  z: i8,
  /// Opened the first time it's shown, from the resources of the system showing it
  scene: Option<Scene>,
  /// What this last showed, which may since have been replaced
  shown: Option<PlaybackId>,
}

impl DmdScreen {
  pub fn new(name: &'static str, z: i8) -> Self {
    Self {
      name,
      z,
      scene: None,
      shown: None,
    }
  }

  /// Shows the scene with `variables`, in place of what's on its z
  pub fn show<'a>(
    &mut self,
    variables: impl IntoIterator<Item = (&'a str, Value)>,
    ctx: &SystemContext,
  ) {
    let Some(scene) = self.opened(ctx) else {
      return;
    };
    let scene = variables
      .into_iter()
      .fold(PlayableScene::new(scene), |scene, (name, value)| {
        scene.variable(name, value)
      });
    self.shown = ctx.systems.with::<SceneSystem, _>(|scenes| {
      let id = scenes.start(ctx.id(), &scene);
      // removed in the same tick the new one starts, so nothing shows through between them
      if let Some(replaced) = SHOWN.lock().unwrap().insert(self.z, id) {
        scenes.remove(replaced);
      }
      id
    });
  }

  /// Takes off what this showed, unless another screen has already replaced it
  pub fn clear(&mut self, ctx: &SystemContext) {
    let Some(id) = self.shown.take() else {
      return;
    };
    let mut shown = SHOWN.lock().unwrap();
    if shown.get(&self.z) == Some(&id) {
      shown.remove(&self.z);
      ctx
        .systems
        .with::<SceneSystem, _>(|scenes| scenes.remove(id));
    }
  }

  fn opened(&mut self, ctx: &SystemContext) -> Option<Scene> {
    if self.scene.is_none() {
      match Scene::open(&ctx.resources, self.name) {
        Ok(mut scene) => {
          let layers = scene
            .layers
            .iter_mut()
            .filter_map(SceneLayer::as_display_mut);
          layers.for_each(|layer| layer.z = self.z);
          self.scene = Some(scene);
        }
        Err(error) => log::error!("Couldn't open {}: {error}", self.name),
      }
    }
    self.scene.clone()
  }
}
