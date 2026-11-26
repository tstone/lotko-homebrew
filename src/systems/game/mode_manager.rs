use std::collections::HashMap;
use std::path::PathBuf;

use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_pinball::{TurnEnding, TurnStarting};
use frontbox_sound::{SoundSystem, SoundSystemExt};

use crate::systems::game::{Mode, ModeState, Multiball, Perk, Progression, music_file};
use crate::systems::{Callout, SayExt, sounds};
use crate::{configs, metrics};

/// How long an insert takes to pulse once for a mode that can be chosen, which is calmer than a running mode's flash
const OFFERED_PULSE: Duration = Duration::from_millis(900);

/// What Hydro's perk adds to each mode timer
const LONGER_TIMERS: Duration = Duration::from_secs(5);

/// The Sentinels' track, over everything while they're out
const SENTINELS_MUSIC: &str = "65daysofstatic-blackspots.mp3";

/// What's running for the current player: a mode and a multiball, in a slot each, and how far they've got up the
/// route. Owns the music, the city map and the rules for what can run together.
///
/// A mode that loses its slot isn't over. Its system goes inactive and keeps its progress, and resuming it makes it
/// active again. See [`ModeRun`](crate::systems::game::ModeRun).
///
/// The Sentinels take over above everything (DESIGN.md, Sentinels): a district running when they're dispatched gives
/// up its slot until they're defeated, then picks up again in play.
#[derive(Clone)]
pub struct ModeManager {
  handle: SystemHandle,
  mode: Option<Mode>,
  multiball: Option<Multiball>,
  /// The Sentinels have taken over
  sentinels: bool,
  /// The district the Sentinels took the slot from, to give back when they're defeated
  suspended: Option<Mode>,
  /// A district given back by the Sentinels that hasn't resumed yet, so it resumes in play rather than in the scoop
  returning: Option<Mode>,
  progression: Progression,
  current_music: Option<PathBuf>,
  /// Music only plays during a turn. Modes changing outside of one (e.g. a slot clearing as the turn ends) update
  /// state without starting music.
  turn_active: bool,
  /// The rows of the mode select menu while it's open
  menu_rows: Vec<Mode>,
  highlighted: Option<Mode>,
  /// The scoop is lit, so the districts it can start pulse
  scoop_lit: bool,
  /// Pulses the map insert of each mode that wants it, with how it's pulsing
  map_pulses: HashMap<Mode, (MapPulse, ScenePlayer)>,
}

impl ModeManager {
  pub fn new() -> Self {
    Self {
      handle: SystemHandle::default(),
      mode: None,
      multiball: None,
      sentinels: false,
      suspended: None,
      returning: None,
      progression: Progression::new(),
      current_music: None,
      turn_active: false,
      menu_rows: Vec::new(),
      highlighted: None,
      scoop_lit: false,
      map_pulses: HashMap::new(),
    }
  }

  pub fn current_mode(&self) -> Option<Mode> {
    self.mode
  }

  pub fn current_multiball(&self) -> Option<Multiball> {
    self.multiball
  }

  /// No mode is running and the Sentinels haven't taken over, so the scoop can qualify and open the menu
  pub fn is_between_modes(&self) -> bool {
    self.mode.is_none() && !self.sentinels
  }

  pub fn sentinels_running(&self) -> bool {
    self.sentinels
  }

  /// Whether the Sentinels could take over now. They wait out a multiball, and a mode that uses the drops or adds balls.
  pub fn sentinels_can_take_over(&self) -> bool {
    !self.sentinels
      && self.multiball.is_none()
      && self.mode.is_none_or(|mode| !mode.uses_drops_or_balls())
  }

  /// The Sentinels take over, suspending the district that's running
  pub fn start_sentinels(&mut self, ctx: &ServiceContext) -> Result<(), String> {
    if !self.sentinels_can_take_over() {
      return self.refuse(format!(
        "the Sentinels can't take over from {:?} and {:?}",
        self.mode, self.multiball
      ));
    }
    log::info!("ModeManager: the Sentinels take over from {:?}", self.mode);
    let ctx = &ctx.for_system(self.handle);
    self.sentinels = true;
    self.suspended = self.mode.take();
    self.slots_changed(ctx);
    self.render_map(ctx);
    Ok(())
  }

  /// The Sentinels were defeated, and the district they suspended picks up again in play
  pub fn end_sentinels(&mut self, ctx: &ServiceContext) {
    if !self.sentinels {
      return;
    }
    log::info!(
      "ModeManager: the Sentinels are defeated, giving back {:?}",
      self.suspended
    );
    let ctx = &ctx.for_system(self.handle);
    self.sentinels = false;
    self.mode = self.suspended.take();
    self.returning = self.mode;
    self.slots_changed(ctx);
    self.render_map(ctx);
  }

  /// Whether `mode` is resuming in play, given back by the Sentinels, rather than from the menu. Answers once.
  pub fn take_returning(&mut self, mode: Mode) -> bool {
    let returning = self.returning == Some(mode);
    if returning {
      self.returning = None;
    }
    returning
  }

  /// What the mode select menu offers
  pub fn available_modes(&self) -> Vec<Mode> {
    self.progression.available_modes(self.mode)
  }

  /// Light an earned mode ([`Mode::is_earned`]) at the scoop. It's offered there without qualifying. Returns whether it
  /// lit, which it doesn't while it's already lit or waiting to be resumed.
  pub fn light_earned(&mut self, mode: Mode) -> bool {
    let lit = self.progression.light(mode);
    if lit {
      log::info!("ModeManager: {mode:?} lit");
    }
    lit
  }

  /// Earned modes that are lit and not started yet
  pub fn lit_earned_modes(&self) -> Vec<Mode> {
    self.progression.lit_earned_modes()
  }

  /// For testing, every mode is open this game, and the scoop needs no qualifying
  pub fn all_modes_open(&self) -> bool {
    self.progression.is_all_open()
  }

  /// The place played in tier 1 and Nimbus Promenade, gate 1, are both completed, which lights an extra ball
  pub fn tier_one_and_gate_one_completed(&self) -> bool {
    self.progression.steps_completed(2)
  }

  // For Steward integrity (TODO.md, Sound)
  #[allow(dead_code)]
  pub fn districts_played(&self) -> usize {
    self.progression.districts_played()
  }

  pub fn districts_completed(&self) -> usize {
    self.progression.districts_completed()
  }

  pub fn has_perk(&self, perk: Perk) -> bool {
    self.progression.has_perk(perk)
  }

  /// How long a mode timer or hurry-up runs, given its `base`. Hydro's perk adds 5s to every one.
  pub fn mode_timer(&self, base: Duration) -> Duration {
    if self.has_perk(Perk::LongerTimers) {
      base + LONGER_TIMERS
    } else {
      base
    }
  }

  /// The color the playfield is lit in: a mini-wizard's, the multiball's, then the district's. White when nothing is
  /// running.
  pub fn illumination(&self) -> Rgba<u8> {
    match (self.mode, self.multiball) {
      (Some(mode), _) if mode.is_mini_wizard() => mode.color(),
      (_, Some(multiball)) => multiball.color(),
      (Some(mode), None) => mode.color(),
      (None, None) => Rgba::white(),
    }
  }

  /// Start a mode or resume one, taking the mode slot. Its system takes it from there.
  pub fn start_mode(&mut self, mode: Mode, ctx: &ServiceContext) -> Result<(), String> {
    if self.sentinels {
      return self.refuse(format!(
        "{mode:?} can't start while the Sentinels are running"
      ));
    }
    if let Some(existing) = self.mode {
      return self.refuse(format!(
        "{mode:?} can't start while {existing:?} is running"
      ));
    }
    if let Some(multiball) = self.multiball.filter(|m| !m.stacks_with(mode)) {
      return self.refuse(format!("{mode:?} can't run with {multiball:?}"));
    }

    let ctx = &ctx.for_system(self.handle);
    let resuming = self.progression.state(mode) == ModeState::Played;
    log::info!(
      "ModeManager: {} {:?}",
      if resuming { "resuming" } else { "starting" },
      mode
    );
    self.progression.start(mode);
    self.mode = Some(mode);
    if !resuming {
      // the braam is kept for the game's biggest moments, the first of which is reaching the top of the city: the finale
      if mode == Mode::SolariumAtrium {
        ctx.play_sfx(sounds::BRAAM_BIO1);
      }
      mode.spawn(ctx);
    }
    self.slots_changed(ctx);
    self.render_map(ctx);
    ctx.emit(ModeStarted(mode));
    Ok(())
  }

  /// Give up the mode slot, leaving the mode to be resumed, unless it's earned ([`Mode::is_earned`])
  pub fn end_mode(&mut self, mode: Mode, ctx: &ServiceContext) {
    if self.mode == Some(mode) {
      let ctx = &ctx.for_system(self.handle);
      self.clear_mode(ctx);
    }
  }

  /// A mode is done for the game. This doesn't need it to be running, as one can finish while a ball is ending.
  pub fn complete(&mut self, mode: Mode, ctx: &ServiceContext) {
    log::info!("ModeManager: completed {:?}", mode);
    let ctx = &ctx.for_system(self.handle);
    self.progression.complete(mode);
    ctx.metrics.inc(&metrics::MODES_COMPLETED);
    ctx.metrics.inc(metrics::mode_completed(&mode));
    if self.mode == Some(mode) {
      self.clear_mode(ctx);
    }
    self.render_map(ctx);
    if mode.is_mini_wizard() {
      ctx.emit(MiniWizardCompleted { mode });
    } else {
      ctx.emit(DistrictCompleted {
        completed: self.progression.districts_completed(),
      });
    }
  }

  /// Take the multiball slot, and start the multiball's system. It's a child of this system, which is always active,
  /// rather than of the one that started it: that one may stop being active once a multiball runs, and its children
  /// with it.
  pub fn take_multiball(
    &mut self,
    multiball: Multiball,
    ctx: &ServiceContext,
  ) -> Result<(), String> {
    if self.sentinels {
      return self.refuse(format!(
        "{multiball:?} can't start while the Sentinels are running"
      ));
    }
    if let Some(existing) = self.multiball {
      return self.refuse(format!("{multiball:?} can't start during {existing:?}"));
    }
    if let Some(mode) = self.mode.filter(|mode| !multiball.stacks_with(*mode)) {
      return self.refuse(format!("{multiball:?} can't run with {mode:?}"));
    }

    let ctx = &ctx.for_system(self.handle);
    self.multiball = Some(multiball);
    multiball.spawn(ctx);
    self.slots_changed(ctx);
    Ok(())
  }

  pub fn release_multiball(&mut self, multiball: Multiball, ctx: &ServiceContext) {
    if self.multiball == Some(multiball) {
      let ctx = &ctx.for_system(self.handle);
      self.multiball = None;
      self.slots_changed(ctx);
    }
  }

  /// Show the mode select menu on the map: its rows pulse and the highlighted one is solid
  pub fn show_menu(&mut self, rows: Vec<Mode>, highlighted: Mode, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    self.menu_rows = rows;
    self.highlighted = Some(highlighted);
    self.render_map(ctx);
  }

  /// The scoop lit or went out. The available districts only pulse while it's lit.
  pub fn set_scoop_lit(&mut self, lit: bool, ctx: &ServiceContext) {
    if self.scoop_lit != lit {
      self.scoop_lit = lit;
      self.render_map(&ctx.for_system(self.handle));
    }
  }

  pub fn hide_menu(&mut self, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    self.menu_rows.clear();
    self.highlighted = None;
    self.render_map(ctx);
  }

  fn refuse(&self, reason: String) -> Result<(), String> {
    log::warn!("ModeManager: {reason}");
    Err(reason)
  }

  fn clear_mode(&mut self, ctx: &SystemContext) {
    if let Some(mode) = self.mode.take() {
      self.progression.end(mode);
    }
    self.slots_changed(ctx);
    self.render_map(ctx);
    ctx.emit(ModeEnded);
  }

  fn slots_changed(&mut self, ctx: &SystemContext) {
    self.crossfade_music(ctx);
    ctx.emit(ModeSlotsChanged);
  }

  /// The Steward's status line, which is less sure of itself once a district has fallen
  fn announce_ball(&self, ctx: &SystemContext) {
    ctx.play_sfx(sounds::SCAN_LIGHT);
    ctx.say(if self.districts_completed() == 0 {
      Callout::AllSystemsWithinOperationalTolerance
    } else {
      Callout::ContinuingCityMaintenance
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

    let path = if self.sentinels {
      music_file(SENTINELS_MUSIC)
    } else {
      music_for(self.mode, self.multiball)
    };
    if self.current_music.as_ref() != Some(&path) {
      log::info!("ModeManager: playing {}", path.display());
      self.current_music = Some(path.clone());
      ctx
        .systems
        .expect::<SoundSystem>()
        .play_music(&path, Duration::from_millis(750));
    }
  }

  /// Every district's insert: off, a pulse, solid in its color, or green once it's completed. An available district only
  /// pulses once the scoop is lit to start it.
  fn render_map(&mut self, ctx: &SystemContext) {
    for mode in Mode::DISTRICTS {
      let Some(led) = mode.map_led() else { continue };
      let state = self.progression.state(mode);
      let pulse = if self.highlighted == Some(mode) {
        None
      } else if self.mode == Some(mode) {
        Some(MapPulse::Running)
      } else if (self.scoop_lit && state == ModeState::Available) || self.menu_rows.contains(&mode)
      {
        Some(MapPulse::Offered)
      } else {
        None
      };
      self.set_map_pulse(mode, &led, pulse, ctx);

      // what's under a pulse, which needs nothing there while it's only offered
      let color = match state {
        ModeState::Completed => Rgba::green(),
        // lost for the game
        ModeState::Played if self.mode != Some(mode) && self.progression.is_out_of_tries(mode) => {
          Rgba::default()
        }
        ModeState::Played => mode.color(),
        _ if self.highlighted == Some(mode) => mode.color(),
        _ => Rgba::default(),
      };
      ctx.declare_lights(&led, 0, color);
    }
  }

  fn set_map_pulse(
    &mut self,
    mode: Mode,
    led: &LightQ,
    pulse: Option<MapPulse>,
    ctx: &SystemContext,
  ) {
    if self.map_pulses.get(&mode).map(|(current, _)| *current) == pulse {
      return;
    }
    if let Some((_, mut playing)) = self.map_pulses.remove(&mode) {
      playing.stop(ctx);
    }
    if let Some(pulse) = pulse {
      let scene = scene!("map_flash.scene", led.clone()).variable("color", mode.color());
      let scene = match pulse {
        MapPulse::Running => scene,
        MapPulse::Offered => scene.over(OFFERED_PULSE),
      };
      self
        .map_pulses
        .insert(mode, (pulse, ScenePlayer::started(scene, ctx)));
    }
  }
}

impl System for ModeManager {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
    if configs::all_modes_open(ctx) {
      log::info!("ModeManager: all modes are open for testing");
      self.progression.open_all();
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<RoutineStarted<TurnStarting>>() {
      log::info!("ModeManager: turn starting (playing music)");
      self.turn_active = true;
      self.crossfade_music(ctx);
      self.render_map(ctx);
      self.announce_ball(ctx);
    } else if event.is::<RoutineStarted<TurnEnding>>() {
      self.turn_active = false;
      self.stop_music(ctx);
      // the Sentinels end with the ball, and a district they suspended is left to resume from the menu
      self.sentinels = false;
      self.suspended = None;
      self.returning = None;
      // the mode stops here so that it doesn't run through the end of the ball, and it's there to resume
      if self.mode.is_some() {
        self.clear_mode(ctx);
      } else {
        self.slots_changed(ctx);
      }
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    self.render_map(ctx);
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.stop_music(ctx);
  }
}

/// Why a map insert is pulsing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapPulse {
  /// The mode can be chosen
  Offered,
  /// The mode is running
  Running,
}

/// The track to play: a mini-wizard's, the multiball's, the district's, and the default for anything without one.
/// A multiball whose track doesn't play over modes leaves the district's playing.
fn music_for(mode: Option<Mode>, multiball: Option<Multiball>) -> PathBuf {
  let mini_wizard = mode.filter(Mode::is_mini_wizard);
  let track = match (mini_wizard, multiball, mode) {
    (Some(mini_wizard), _, _) => mini_wizard.music(),
    (_, Some(multiball), None) => multiball.music(),
    (_, Some(multiball), Some(_)) if multiball.music_plays_over_modes() => multiball.music(),
    (_, _, Some(mode)) => mode.music(),
    _ => None,
  };
  track.unwrap_or_else(|| music_file("colyn-rushing.mp3"))
}

#[derive(serde::Serialize, Event)]
pub struct ModeStarted(pub Mode);

/// The mode slot emptied, by the mode completing or stopping
#[derive(serde::Serialize, Event)]
pub struct ModeEnded;

/// A mode or a multiball took or gave up its slot
#[derive(serde::Serialize, Event)]
pub struct ModeSlotsChanged;

/// A district was completed
#[derive(serde::Serialize, Event)]
pub struct DistrictCompleted {
  /// Districts completed this game, including this one
  pub completed: usize,
}

/// A mini-wizard was completed
#[derive(serde::Serialize, Event)]
pub struct MiniWizardCompleted {
  pub mode: Mode,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn heartwood_keeps_the_district_music_playing() {
    assert_eq!(
      music_for(Some(Mode::HydroCore), Some(Multiball::Heartwood)),
      Mode::HydroCore.music().unwrap()
    );
    assert_eq!(
      music_for(None, Some(Multiball::Heartwood)),
      Multiball::Heartwood.music().unwrap()
    );
  }

  #[test]
  fn a_mini_wizard_keeps_its_slot_in_the_music_over_a_multiball() {
    assert_eq!(
      music_for(Some(Mode::CalderaGenerator), Some(Multiball::Heartwood)),
      Mode::CalderaGenerator.music().unwrap()
    );
  }

  #[test]
  fn a_district_plays_its_own_track_and_nothing_plays_the_default() {
    assert_eq!(
      music_for(Some(Mode::HydroCore), None),
      Mode::HydroCore.music().unwrap()
    );
    assert_eq!(music_for(None, None), music_file("colyn-rushing.mp3"));
  }

  #[test]
  fn the_sentinels_wait_for_the_drops_and_extra_balls() {
    let mut manager = ModeManager::new();
    assert!(manager.sentinels_can_take_over());
    manager.mode = Some(Mode::HydroCore);
    assert!(manager.sentinels_can_take_over());
    manager.mode = Some(Mode::CalderaGenerator);
    assert!(!manager.sentinels_can_take_over());
    manager.mode = None;
    manager.multiball = Some(Multiball::Heartwood);
    assert!(!manager.sentinels_can_take_over());
  }

  #[test]
  fn nothing_starts_under_the_sentinels() {
    let mut manager = ModeManager::new();
    manager.sentinels = true;
    assert!(!manager.is_between_modes());
    assert!(!manager.sentinels_can_take_over());
  }

  #[test]
  fn a_district_given_back_resumes_in_play_once() {
    let mut manager = ModeManager::new();
    manager.returning = Some(Mode::NimbusPromenade);
    assert!(!manager.take_returning(Mode::HydroCore));
    assert!(manager.take_returning(Mode::NimbusPromenade));
    assert!(!manager.take_returning(Mode::NimbusPromenade));
  }
}
