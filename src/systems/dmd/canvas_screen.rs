use std::collections::BTreeMap;
use std::sync::Mutex;

use frontbox_core::prelude::*;
use frontbox_gfx::canvas::{CanvasView, Position, PositionedLayer, Size};
use frontbox_gfx::scene::{Fill, FillNode, Layer, Picture};
use frontbox_scene::PlaybackId;
use frontbox_scene::prelude::*;
use image::RgbaImage;

/// The DMD, by the name it's added to the `DisplaySystem` under
pub const DMD: &str = "Dmd";
pub const DMD_WIDTH: u32 = 128;
pub const DMD_HEIGHT: u32 = 32;

/// The screen shown on each z, by any system. A screen takes the place of the one on its z, as a layer of the old
/// `DmdSystem` did, so one system can leave a screen up for another to replace.
static SHOWN: Mutex<BTreeMap<i8, PlaybackId>> = Mutex::new(BTreeMap::new());

/// A screen drawn with `frontbox_gfx::canvas`, shown on the DMD as a scene until it can be drawn as one. Each draw is an
/// image in a `Display` layer at `z`, owned by the system that shows it, so it's hidden while that system is inactive and
/// goes when it despawns.
#[derive(Clone)]
pub struct CanvasScreen {
  z: i8,
  /// What this last showed, which may since have been replaced
  shown: Option<PlaybackId>,
}

impl CanvasScreen {
  pub fn new(z: i8) -> Self {
    Self { z, shown: None }
  }

  /// Shows `layer` in place of what's on its z. What it leaves transparent shows the screens below.
  pub fn show(&mut self, layer: impl PositionedLayer, ctx: &SystemContext) {
    let mut image = RgbaImage::new(DMD_WIDTH, DMD_HEIGHT);
    layer.render_relative(&mut CanvasView::new(Position::zero(), Size::new(DMD_WIDTH, DMD_HEIGHT), &mut image));

    let fill = Fill::new(FillNode::image("canvas"));
    let mut scene = Scene::new(vec![DisplayLayer::new(DMD, Layer::new(fill)).at_z(self.z).into()]);
    scene.images.insert("canvas", Picture::new(image));
    self.shown = ctx.systems.with::<SceneSystem, _>(|scenes| {
      let id = scenes.start(ctx.id(), &PlayableScene::new(scene));
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
      ctx.systems.with::<SceneSystem, _>(|scenes| scenes.remove(id));
    }
  }
}
