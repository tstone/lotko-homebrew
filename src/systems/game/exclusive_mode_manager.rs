use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox_sound::SoundSystem;

static MODE_MUSIC: LazyLock<HashMap<ExclusiveMode, PathBuf>> = LazyLock::new(|| {
  let mut map = HashMap::new();
  map.insert(
    ExclusiveMode::HydroCore,
    PathBuf::from("/userdata/home/armsom/music/inzo-wonder.mp3"),
  );
  map
});

#[derive(Clone)]
pub struct ExclusiveModeManager {
  exclusive_mode: Option<ExclusiveMode>,
}

impl ExclusiveModeManager {
  pub fn new() -> Self {
    Self {
      exclusive_mode: None,
    }
  }

  pub fn current_mode(&self) -> &Option<ExclusiveMode> {
    &self.exclusive_mode
  }

  pub fn take_exclusive(&mut self, mode: ExclusiveMode, ctx: &SystemContext) -> Result<(), String> {
    if self.exclusive_mode.is_some() {
      let msg = format!(
        "Cannot start {:?} because {:?} is already exclusive.",
        mode,
        self.exclusive_mode.as_ref().unwrap()
      );
      log::warn!("{}", msg);
      Err(msg)
    } else {
      if let Some(path) = MODE_MUSIC.get(&mode) {
        ctx
          .expect::<SoundSystem>()
          .crossfade_music(path, Duration::from_millis(1000));
      }

      self.exclusive_mode = Some(mode);
      Ok(())
    }
  }
}

impl System for ExclusiveModeManager {
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    let path = self
      .exclusive_mode
      .as_ref()
      .and_then(|mode| MODE_MUSIC.get(&mode))
      .cloned()
      .unwrap_or(PathBuf::from(
        "/userdata/home/armsom/music/colyn-rushing.mp3",
      ));

    ctx.expect::<SoundSystem>().play_music(path);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExclusiveMode {
  SolariumAtrium,
  HydroCore,
  SkyrailStation,
  MeridianBasins,
  Wizard,
}
