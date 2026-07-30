use frontbox::prelude::*;
use frontbox_sound::*;

use crate::hardware::lower_scoop::LOWER_SCOOP_EJECT_SND;

pub mod sounds {
  pub static LOWER_SCOOP_EJECT: &[u8] = include_bytes!("../assets/sounds/lower-scoop-exit.mp3");
}

impl System for SoundLoaderSystem {
  fn on_spawn(&mut self, ctx: &Context) {
    if let Some(mut snd) = ctx.systems.get::<SoundSystem>() {
      snd.preload_embedded(LOWER_SCOOP_EJECT_SND, sounds::LOWER_SCOOP_EJECT);
    }

    // We're done. Sounds will remain preloaded.
    ctx.despawn_self();
  }
}

pub struct SoundLoaderSystem;

impl SoundLoaderSystem {
  pub fn new() -> Self {
    Self
  }
}
