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
    ExclusiveMode::SkyrailStation,
    PathBuf::from("/userdata/home/armsom/music/deadmau5-arguru2k19.mp3"),
  );
  map.insert(
    ExclusiveMode::SolariumAtrium,
    PathBuf::from("/userdata/home/armsom/music/fehrplay-arcadia.mp3"),
  );
  map.insert(
    ExclusiveMode::MeridianBasins,
    PathBuf::from("/userdata/home/armsom/music/glitch-mob-cant-kill-us.mp3"),
  );
  map.insert(
    ExclusiveMode::SporeMultiball,
    PathBuf::from("/userdata/home/armsom/music/rezmau5-infraliminal.mp3"),
  );
  map
});
static NON_EXCL_MODE_MUSIC: LazyLock<HashMap<NonExclusiveMode, PathBuf>> = LazyLock::new(|| {
  let mut map = HashMap::new();
  map.insert(
    NonExclusiveMode::NimbusPromenade,
    PathBuf::from("/userdata/home/armsom/music/wice-5omething.mp3"),
  );
  map
});

#[derive(Clone)]
pub struct ModeManager {
  handle: SystemHandle,
  exclusive_mode: Option<ExclusiveMode>,
  music_priority: Vec<NonExclusiveMode>,
  exclusive_completions: HashSet<ExclusiveMode>,
  non_exclusive_completions: HashSet<NonExclusiveMode>,
  current_music: Option<PathBuf>,
}

impl ModeManager {
  pub fn new() -> Self {
    Self {
      handle: SystemHandle::default(),
      exclusive_mode: None,
      music_priority: Vec::new(),
      exclusive_completions: HashSet::new(),
      non_exclusive_completions: HashSet::new(),
      current_music: None,
    }
  }

  pub fn current_mode(&self) -> &Option<ExclusiveMode> {
    &self.exclusive_mode
  }

  pub fn take_exclusive(
    &mut self,
    mode: ExclusiveMode,
    ctx: &ServiceContext,
  ) -> Result<(), String> {
    if let Some(existing) = &self.exclusive_mode {
      let msg = format!(
        "Cannot start {:?} because {:?} already has exclusive.",
        mode, existing
      );
      log::warn!("{}", msg);
      Err(msg)
    } else {
      let ctx = &ctx.for_system(self.handle);
      self.exclusive_mode = Some(mode);
      log::info!("ModeManager: taking exclusive (playing music)");
      self.crossfade_music(ctx);
      ctx.emit(ExclusiveModeStarted(mode.clone()));
      Ok(())
    }
  }

  pub fn complete_exclusive(&mut self, mode: ExclusiveMode, ctx: &ServiceContext) {
    log::info!("Modes: Exclusive completed {:?}", mode);
    self.release_exclusive(&mode, ctx);
    self.exclusive_completions.insert(mode);
    let ctx = &ctx.for_system(self.handle);
    self.render_map(ctx);
  }

  pub fn release_exclusive(&mut self, mode: &ExclusiveMode, ctx: &ServiceContext) {
    if self
      .exclusive_mode
      .as_ref()
      .map(|m| m == mode)
      .unwrap_or(false)
    {
      let ctx = &ctx.for_system(self.handle);
      self.exclusive_mode = None;
      log::info!("ModeManager: releasing_exclusive (playing music)");
      self.crossfade_music(ctx);
      ctx.emit(ExclusiveModeEnded);
    }
  }

  pub fn non_exclusive_active(&mut self, mode: NonExclusiveMode, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    self.music_priority.push(mode.clone());
    if self.is_default_music_playing() {
      log::info!(
        "ModeManager: non-exclusive active {:?} (playing music)",
        mode
      );
      self.crossfade_music(ctx);
    }
  }

  pub fn non_exclusive_inactive(&mut self, mode: &NonExclusiveMode, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    self.music_priority.retain(|m| m != mode);
    log::info!("ModeManager: non-exclusive inactive (playing music)");
    self.crossfade_music(ctx);
  }

  pub fn complete_non_exclusive(&mut self, mode: NonExclusiveMode, ctx: &ServiceContext) {
    log::info!("Modes: Non-exclusive completed {:?}", mode);
    self.non_exclusive_inactive(&mode, ctx);
    self.non_exclusive_completions.insert(mode);
    let ctx = &ctx.for_system(self.handle);
    self.render_map(ctx);
  }

  fn stop_music(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<SoundSystem>()
      .stop_music(Duration::from_millis(500));
    self.current_music = None;
  }

  fn crossfade_music(&mut self, ctx: &SystemContext) {
    let path = match (&self.exclusive_mode, self.music_priority.get(0)) {
      (Some(mode), _) => EXCL_MODE_MUSIC.get(&mode),
      (_, Some(mode)) => NON_EXCL_MODE_MUSIC.get(&mode),
      (None, None) => Some(&*DEFAULT_MUSIC),
    };
    let path = if let Some(path) = path {
      path
    } else {
      &*DEFAULT_MUSIC
    };

    let already_playing_music = self
      .current_music
      .as_ref()
      .map(|p| p == path)
      .unwrap_or(false);

    if !already_playing_music {
      self.current_music = Some(path.clone());
      ctx
        .expect::<SoundSystem>()
        .play_music(path, Duration::from_millis(750));
    }
  }

  fn is_default_music_playing(&self) -> bool {
    self
      .current_music
      .as_ref()
      .map(|p| *p == *DEFAULT_MUSIC)
      .unwrap_or(false)
  }

  fn is_exclusive_mode_complete(&self, mode: &ExclusiveMode) -> bool {
    self.exclusive_completions.contains(mode)
  }

  fn is_non_exclusive_mode_complete(&self, mode: &NonExclusiveMode) -> bool {
    self.non_exclusive_completions.contains(mode)
  }

  fn render_exclusive_mode<T: Contextual<LedIdentifications>>(
    &self,
    targets: &T,
    mode: &ExclusiveMode,
    ctx: &SystemContext,
  ) {
    let color: ColorSequence = if self.is_exclusive_mode_complete(mode) {
      mode.color().into()
    } else {
      Rgba::default().into()
    };
    ctx.declare_leds(targets, color);
  }

  fn render_non_exclusive_mode<T: Contextual<LedIdentifications>>(
    &self,
    targets: &T,
    mode: &NonExclusiveMode,
    color: Rgba<u8>,
    ctx: &SystemContext,
  ) {
    let color: ColorSequence = if self.is_non_exclusive_mode_complete(mode) {
      color.into()
    } else {
      Rgba::default().into()
    };
    ctx.declare_leds(targets, color);
  }

  fn render_map(&self, ctx: &SystemContext) {
    self.render_non_exclusive_mode(
      &city_map::NIMBUS_PROMENADE.q(),
      &NonExclusiveMode::NimbusPromenade,
      Rgba::magenta(),
      ctx,
    );
    self.render_non_exclusive_mode(
      &city_map::APEX_TERRACES.q(),
      &NonExclusiveMode::ApexTerraces,
      Rgba::green(),
      ctx,
    );

    self.render_exclusive_mode(&city_map::HYDRO_CORE.q(), &ExclusiveMode::HydroCore, ctx);
    self.render_exclusive_mode(
      &city_map::SKYRAIL_STATION.q(),
      &ExclusiveMode::SkyrailStation,
      ctx,
    );
    self.render_exclusive_mode(
      &city_map::MERIDIAN_BASINS.q(),
      &ExclusiveMode::MeridianBasins,
      ctx,
    );
    self.render_exclusive_mode(
      &city_map::SOLARIUM_ATRIUMS.q(),
      &ExclusiveMode::SolariumAtrium,
      ctx,
    );
  }
}

impl System for ModeManager {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = *ctx.current_handle();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<PlayerTurnBeginning>() {
      log::info!("ModeManager: turn starting (playing music)");
      self.crossfade_music(ctx);
    } else if event.is::<PlayerTurnEnding>() {
      self.stop_music(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    self.render_map(ctx);
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
