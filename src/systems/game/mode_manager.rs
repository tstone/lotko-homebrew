use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox_sound::SoundSystem;
use frontbox_turn_based::{PlayerTurnBeginning, PlayerTurnEnding};

use crate::hardware::city_map;
use crate::systems::game::ExclusiveMode;

static DEFAULT_MUSIC: LazyLock<PathBuf> =
  LazyLock::new(|| PathBuf::from("/userdata/home/armsom/music/colyn-rushing.mp3"));
static EXCL_MODE_MUSIC: LazyLock<HashMap<ExclusiveMode, PathBuf>> = LazyLock::new(|| {
  let mut map = HashMap::new();
  map.insert(
    ExclusiveMode::HydroCore,
    PathBuf::from("/userdata/home/armsom/music/inzo-wonder.mp3"),
  );
  map.insert(
    ExclusiveMode::MeridianBasins,
    PathBuf::from("/userdata/home/armsom/music/wice-5omething.mp3"),
  );
  map.insert(
    ExclusiveMode::SkyrailStation,
    PathBuf::from("/userdata/home/armsom/music/deadmau5-arguru2k19.mp3"),
  );
  map
});
static NON_EXCL_MODE_MUSIC: LazyLock<HashMap<NonExclusiveMode, PathBuf>> = LazyLock::new(|| {
  let mut map = HashMap::new();
  map.insert(
    NonExclusiveMode::NimbusPromenade,
    PathBuf::from("/userdata/home/armsom/music/hypixi-engage.mp3"),
  );
  map
});

#[derive(Clone)]
pub struct ModeManager {
  exclusive_mode: Option<ExclusiveMode>,
  music_priority: Vec<NonExclusiveMode>,
  exclusive_completions: HashSet<ExclusiveMode>,
  non_exclusive_completions: HashSet<NonExclusiveMode>,
  render: bool,
  default_music_playing: bool,
}

impl ModeManager {
  pub fn new() -> Self {
    Self {
      exclusive_mode: None,
      music_priority: Vec::new(),
      exclusive_completions: HashSet::new(),
      non_exclusive_completions: HashSet::new(),
      render: false,
      default_music_playing: true,
    }
  }

  pub fn current_mode(&self) -> &Option<ExclusiveMode> {
    &self.exclusive_mode
  }

  pub fn take_exclusive(&mut self, mode: ExclusiveMode, ctx: &SystemContext) -> Result<(), String> {
    if let Some(existing) = &self.exclusive_mode {
      let msg = format!(
        "Cannot start {:?} because {:?} already has exclusive.",
        mode, existing
      );
      log::warn!("{}", msg);
      Err(msg)
    } else {
      self.exclusive_mode = Some(mode);
      self.crossfade_music(ctx);
      ctx.emit(ExclusiveModeStarted(mode.clone()));
      Ok(())
    }
  }

  pub fn complete_exclusive(&mut self, mode: ExclusiveMode, ctx: &SystemContext) {
    self.release_exclusive(&mode, ctx);
    self.exclusive_completions.insert(mode);
    self.render = true;
  }

  pub fn release_exclusive(&mut self, mode: &ExclusiveMode, ctx: &SystemContext) {
    if self
      .exclusive_mode
      .as_ref()
      .map(|m| m == mode)
      .unwrap_or(false)
    {
      self.exclusive_mode = None;
      self.crossfade_music(ctx);
      ctx.emit(ExclusiveModeEnded);
    }
  }

  pub fn non_exclusive_active(&mut self, mode: NonExclusiveMode, ctx: &SystemContext) {
    self.music_priority.push(mode);
    if self.default_music_playing {
      self.crossfade_music(ctx);
    }
  }

  pub fn non_exclusive_inactive(&mut self, mode: &NonExclusiveMode, ctx: &SystemContext) {
    self.music_priority.retain(|m| m != mode);
    self.crossfade_music(ctx);
  }

  pub fn complete_non_exclusive(&mut self, mode: NonExclusiveMode, ctx: &SystemContext) {
    self.non_exclusive_inactive(&mode, ctx);
    self.non_exclusive_completions.insert(mode);
    self.render = true;
  }

  fn stop_music(&self, ctx: &SystemContext) {
    ctx
      .expect::<SoundSystem>()
      .stop_music(Duration::from_millis(500));
  }

  fn on_turn_starting(&mut self, ctx: &SystemContext) {
    self.crossfade_music(ctx);
  }

  fn crossfade_music(&mut self, ctx: &SystemContext) {
    let path = match (&self.exclusive_mode, self.music_priority.get(0)) {
      (Some(mode), _) => EXCL_MODE_MUSIC.get(&mode),
      (_, Some(mode)) => NON_EXCL_MODE_MUSIC.get(&mode),
      (None, None) => {
        self.default_music_playing = true;
        Some(&*DEFAULT_MUSIC)
      }
    };
    let path = if let Some(path) = path {
      path
    } else {
      self.default_music_playing = true;
      &*DEFAULT_MUSIC
    };
    ctx
      .expect::<SoundSystem>()
      .play_music(path, Duration::from_millis(1000));
  }
}

impl System for ModeManager {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<PlayerTurnBeginning>() {
      self.on_turn_starting(ctx);
    } else if event.is::<PlayerTurnEnding>() {
      self.stop_music(ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.render {
      if self
        .non_exclusive_completions
        .contains(&NonExclusiveMode::NimbusPromenade)
      {
        ctx.declare_leds(&city_map::NIMBUS_PROMENADE.q(), Rgba::white().into());
      }

      if self
        .non_exclusive_completions
        .contains(&NonExclusiveMode::ApexTerraces)
      {
        ctx.declare_leds(&city_map::APEX_TERRACES.q(), Rgba::white().into());
      }

      if self
        .exclusive_completions
        .contains(&ExclusiveMode::HydroCore)
      {
        ctx.declare_leds(&city_map::HYDRO_CORE.q(), Rgba::white().into());
      }

      if self
        .exclusive_completions
        .contains(&ExclusiveMode::SkyrailStation)
      {
        ctx.declare_leds(&city_map::SKYRAIL_STATION.q(), Rgba::white().into());
      }

      if self
        .exclusive_completions
        .contains(&ExclusiveMode::MeridianBasins)
      {
        ctx.declare_leds(&city_map::MERIDIAN_BASINS.q(), Rgba::white().into());
      }

      if self
        .exclusive_completions
        .contains(&ExclusiveMode::SolariumAtrium)
      {
        ctx.declare_leds(&city_map::SOLARIUM_ATRIUMS.q(), Rgba::white().into());
      }
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.stop_music(ctx);
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonExclusiveMode {
  NimbusPromenade,
  ApexTerraces,
}

#[derive(serde::Serialize, Event)]
pub struct ExclusiveModeStarted(pub ExclusiveMode);

#[derive(serde::Serialize, Event)]
pub struct ExclusiveModeEnded;
