use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::LazyLock;

use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{TurnEnding, TurnStarting};
use frontbox_sound::SoundSystem;

use crate::hardware::city_map;
use crate::metrics;
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
  /// Music only plays during a turn. Modes changing outside of one (e.g. releasing exclusive as the turn ends)
  /// update state without starting music.
  turn_active: bool,
  /// Flashes the map insert of each active mode. Each mode has its own flash, so an exclusive mode and any number of
  /// non-exclusive modes can flash together.
  map_flashes: HashMap<ActiveMode, LedProgram1d>,
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
      turn_active: false,
      map_flashes: HashMap::new(),
    }
  }

  pub fn current_mode(&self) -> &Option<ExclusiveMode> {
    &self.exclusive_mode
  }

  /// The number of city modes completed this game, each lit on the city map
  pub fn completed_city_mode_count(&self) -> usize {
    self.exclusive_completions.len() + self.non_exclusive_completions.len()
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
      self.flash_map_insert(ActiveMode::Exclusive(mode));
      ctx.emit(ExclusiveModeStarted(mode.clone()));
      Ok(())
    }
  }

  pub fn complete_exclusive(&mut self, mode: ExclusiveMode, ctx: &ServiceContext) {
    log::info!("Modes: Exclusive completed {:?}", mode);
    self.release_exclusive(&mode, ctx);
    ctx.metrics.inc(&metrics::MODES_COMPLETED);
    ctx.metrics.inc(metrics::exclusive_mode_completed(&mode));
    self.exclusive_completions.insert(mode);
    let ctx = &ctx.for_system(self.handle);
    self.render_map(ctx);
    ctx.emit(CityModeCompleted {
      completed: self.completed_city_mode_count(),
    });
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
      self.stop_map_flash(&ActiveMode::Exclusive(*mode), ctx);
      ctx.emit(ExclusiveModeEnded);
    }
  }

  pub fn non_exclusive_active(&mut self, mode: NonExclusiveMode, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    if !self.music_priority.contains(&mode) {
      self.music_priority.push(mode.clone());
    }
    if self.is_default_music_playing() {
      log::info!(
        "ModeManager: non-exclusive active {:?} (playing music)",
        mode
      );
      self.crossfade_music(ctx);
    }
    self.flash_map_insert(ActiveMode::NonExclusive(mode));
  }

  pub fn non_exclusive_inactive(&mut self, mode: &NonExclusiveMode, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    self.music_priority.retain(|m| m != mode);
    log::info!("ModeManager: non-exclusive inactive (playing music)");
    self.crossfade_music(ctx);
    self.stop_map_flash(&ActiveMode::NonExclusive(mode.clone()), ctx);
  }

  pub fn complete_non_exclusive(&mut self, mode: NonExclusiveMode, ctx: &ServiceContext) {
    log::info!("Modes: Non-exclusive completed {:?}", mode);
    self.non_exclusive_inactive(&mode, ctx);
    ctx.metrics.inc(&metrics::MODES_COMPLETED);
    ctx
      .metrics
      .inc(metrics::non_exclusive_mode_completed(&mode));
    self.non_exclusive_completions.insert(mode);
    let ctx = &ctx.for_system(self.handle);
    self.render_map(ctx);
    ctx.emit(CityModeCompleted {
      completed: self.completed_city_mode_count(),
    });
  }

  fn stop_music(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<SoundSystem>()
      .stop_music(Duration::from_millis(500));
    self.current_music = None;
  }

  fn crossfade_music(&mut self, ctx: &SystemContext) {
    if !self.turn_active {
      return;
    }

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
        .systems
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

  fn render_non_exclusive_mode(&self, mode: &NonExclusiveMode, ctx: &SystemContext) {
    let color: ColorSequence = if self.is_non_exclusive_mode_complete(mode) {
      mode.color().into()
    } else {
      Rgba::default().into()
    };
    ctx.declare_leds(&mode.map_led(), color);
  }

  fn render_map(&self, ctx: &SystemContext) {
    self.render_non_exclusive_mode(&NonExclusiveMode::NimbusPromenade, ctx);
    self.render_non_exclusive_mode(&NonExclusiveMode::ApexTerraces, ctx);

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

  /// Flashes a mode's map insert over its completion color, leaving any other mode's flash running
  fn flash_map_insert(&mut self, mode: ActiveMode) {
    if let Some(led) = mode.map_led() {
      let color = mode.color();
      self.map_flashes.entry(mode).or_insert_with(|| {
        LedProgram1d::flash(led.at_z(1), ColorSequence::solid(color), Cycle::Forever)
      });
    }
  }

  fn stop_map_flash(&mut self, mode: &ActiveMode, ctx: &SystemContext) {
    if let Some(mut flash) = self.map_flashes.remove(mode) {
      flash.stop(ctx);
    }
  }
}

impl System for ModeManager {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<RoutineStarted<TurnStarting>>() {
      log::info!("ModeManager: turn starting (playing music)");
      self.turn_active = true;
      self.crossfade_music(ctx);
    } else if event.is::<RoutineStarted<TurnEnding>>() {
      self.turn_active = false;
      self.stop_music(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    for flash in self.map_flashes.values_mut() {
      flash.apply(delta, ctx);
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

impl NonExclusiveMode {
  fn color(&self) -> Rgba<u8> {
    match self {
      Self::NimbusPromenade => Rgba::magenta(),
      Self::ApexTerraces => Rgba::green(),
    }
  }

  fn map_led(&self) -> LedQ {
    match self {
      Self::NimbusPromenade => city_map::NIMBUS_PROMENADE.q(),
      Self::ApexTerraces => city_map::APEX_TERRACES.q(),
    }
  }
}

/// A running mode, exclusive or not
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ActiveMode {
  Exclusive(ExclusiveMode),
  NonExclusive(NonExclusiveMode),
}

impl ActiveMode {
  fn color(&self) -> Rgba<u8> {
    match self {
      Self::Exclusive(mode) => mode.color(),
      Self::NonExclusive(mode) => mode.color(),
    }
  }

  /// The mode's city map insert, if it has one
  fn map_led(&self) -> Option<LedQ> {
    match self {
      Self::Exclusive(ExclusiveMode::HydroCore) => Some(city_map::HYDRO_CORE.q()),
      Self::Exclusive(ExclusiveMode::SkyrailStation) => Some(city_map::SKYRAIL_STATION.q()),
      Self::Exclusive(ExclusiveMode::MeridianBasins) => Some(city_map::MERIDIAN_BASINS.q()),
      Self::Exclusive(ExclusiveMode::SolariumAtrium) => Some(city_map::SOLARIUM_ATRIUMS.q()),
      Self::Exclusive(ExclusiveMode::SporeMultiball | ExclusiveMode::Wizard) => None,
      Self::NonExclusive(mode) => Some(mode.map_led()),
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct ExclusiveModeStarted(pub ExclusiveMode);

#[derive(serde::Serialize, Event)]
pub struct ExclusiveModeEnded;

/// A city mode (district) was completed
#[derive(serde::Serialize, Event)]
pub struct CityModeCompleted {
  /// City modes completed this game, including this one
  pub completed: usize,
}
