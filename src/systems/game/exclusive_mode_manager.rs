use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox_sound::{SoundSystem, SoundSystemExt};
use frontbox_turn_based::{GameManager, PlayerTurnBeginning};

use crate::systems::sounds;

static MODE_MUSIC: LazyLock<HashMap<ExclusiveMode, PathBuf>> = LazyLock::new(|| {
  let mut map = HashMap::new();
  map.insert(
    ExclusiveMode::None,
    PathBuf::from("/userdata/home/armsom/music/colyn-rushing.mp3"),
  );
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
      Self::crossfade_music(&mode, ctx);
      self.exclusive_mode = Some(mode);
      Ok(())
    }
  }

  pub fn release_exclusive(&mut self, mode: ExclusiveMode, ctx: &SystemContext) {
    if self.exclusive_mode == Some(mode) {
      self.exclusive_mode = None;
      Self::crossfade_music(&ExclusiveMode::None, ctx);
    }
  }

  fn crossfade_music(mode: &ExclusiveMode, ctx: &SystemContext) {
    if let Some(path) = MODE_MUSIC.get(mode) {
      ctx
        .expect::<SoundSystem>()
        .play_music(path, Duration::from_millis(1000));
    }
  }
}

impl System for ExclusiveModeManager {
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    let path = MODE_MUSIC
      .get(self.exclusive_mode.as_ref().unwrap_or(&ExclusiveMode::None))
      .unwrap();

    ctx
      .expect::<SoundSystem>()
      .play_music(path, Duration::from_millis(500));
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<SoundSystem>()
      .stop_music(Duration::from_millis(500));
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExclusiveMode {
  SolariumAtrium,
  HydroCore,
  SkyrailStation,
  MeridianBasins,
  SporeCountMultiball,
  Wizard,
  None,
}
