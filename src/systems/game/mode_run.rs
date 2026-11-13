use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{BallSaveExt, GameManagementExt};
use frontbox_sound::{CalloutCompleted, SoundSystemExt};

use crate::hardware::lower_scoop::{LowerScoopEjected, LowerScoopSystem};
use crate::systems::game::{EffectsSystem, Mode, ModeManager, points};
use crate::systems::{SayExt, sounds};

/// The ball save a mode starts with, as the scoop eject is a common drain
const START_BALL_SAVE: Duration = Duration::from_secs(8);

/// The parts every mode shares, from the phases in DESIGN.md: the start, the intensity stages and the completion. Each
/// mode's system owns one and passes it what it needs to.
///
/// A mode is started by [`ModeManager::start_mode`], which spawns its system. When the turn ends the slot empties and
/// the system goes inactive, keeping its progress, and with it its cues and effects, which don't tick while it's
/// inactive. Resuming makes it active again. So a mode's lifecycle comes down to:
///
/// - `on_spawn` and `on_reactivate`: [`ModeRun::begin`]
/// - `on_deactivate`: [`ModeRun::interrupt`]
/// - `on_tick`: [`ModeRun::apply`], and nothing else while it says the mode is starting
/// - `on_event`: [`ModeRun::on_event`]
#[derive(Clone)]
pub struct ModeRun {
  mode: Mode,
  /// Hits to the end of the mode
  required: u8,
  /// The highest intensity stage reached
  stage: u8,
  /// The intro, while it's playing with the ball held in the scoop
  intro: Option<LedPlayback>,
  /// The start line the intro is still waiting on, by key
  intro_line: Option<&'static str>,
  /// The ball was let go and hasn't left the scoop yet
  awaiting_eject: bool,
}

impl ModeRun {
  pub fn new(mode: Mode, required: u8) -> Self {
    Self {
      mode,
      required,
      stage: 0,
      intro: None,
      intro_line: None,
      awaiting_eject: false,
    }
  }

  /// The start phase: the ball waits in the scoop through the intro and the Steward's announcement
  pub fn begin(&mut self, ctx: &SystemContext) {
    log::info!("ModeRun: {:?} begins", self.mode);
    ctx
      .systems
      .expect::<LowerScoopSystem>()
      .hold(ctx.id(), ctx.into());
    for line in self.mode.start_lines() {
      ctx.say(*line);
    }
    self.intro_line = self.mode.start_lines().last().map(|line| line.key());
    self.intro = Some(led_playback!("startable/hit.scene").param("color", self.mode.color()));
  }

  /// Plays the intro until both it and the start lines are done, then lets the ball go. Returns whether the mode is
  /// still starting.
  pub fn apply(&mut self, delta: Duration, ctx: &SystemContext) -> bool {
    let Some(intro) = self.intro.as_mut() else {
      return false;
    };

    intro.apply(delta, ctx);
    if intro.is_complete() && self.intro_line.is_none() {
      intro.stop(ctx);
      self.intro = None;
      self.awaiting_eject = true;
      self.release_ball(ctx);
      return false;
    }
    true
  }

  pub fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if self.awaiting_eject && event.is::<LowerScoopEjected>() {
      self.awaiting_eject = false;
      ctx.request_ball_save(START_BALL_SAVE);
    } else if let Some(line) = event.downcast_ref::<CalloutCompleted>() {
      self.line_completed(line.key);
    }
  }

  fn line_completed(&mut self, key: &'static str) {
    if self.intro_line == Some(key) {
      self.intro_line = None;
    }
  }

  /// The mode has stopped running part way through, e.g. the turn ended during the intro
  pub fn interrupt(&mut self, ctx: &SystemContext) {
    if let Some(mut intro) = self.intro.take() {
      intro.stop(ctx);
      self.release_ball(ctx);
    }
    self.intro_line = None;
    self.awaiting_eject = false;
  }

  /// Report how many of the mode's hits are done. Returns the intensity stage, of 25, 50 or 75 percent, if this
  /// reached a new one.
  pub fn progress(&mut self, done: u8) -> Option<u8> {
    let percent = done as u32 * 100 / self.required.max(1) as u32;
    let stage = [75, 50, 25]
      .into_iter()
      .find(|stage| percent >= *stage as u32)?;
    (stage > self.stage).then(|| {
      self.stage = stage;
      stage
    })
  }

  /// Start the intensity stages over, for a mode whose progress can go back
  pub fn reset_progress(&mut self) {
    self.stage = 0;
  }

  /// Score a shot in the mode at its step's value, `times` over for a burst or a Root Lash
  pub fn score_hit(&self, times: u8, ctx: &SystemContext) {
    ctx.add_points(points::mode_hit(self.mode) * times as u64);
  }

  /// The completion phase: the mode ends here, and the takeover plays on without it, so a drain during `effect` can't
  /// take the completion back. It's worth its step's completion value.
  pub fn complete(&self, effect: LedPlayback, ctx: &SystemContext) {
    ctx.add_points(points::mode_completion(self.mode));
    ctx.play_sfx(sounds::city_hit(1.0));
    for line in self.mode.completion_lines() {
      ctx.say(*line);
    }
    ctx.systems.expect::<EffectsSystem>().play(effect);
    ctx
      .systems
      .expect::<ModeManager>()
      .complete(self.mode, ctx.into());
    // a city mode can only be completed once per game
    ctx.systems.despawn_self();
  }

  fn release_ball(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LowerScoopSystem>()
      .release(ctx.id(), ctx.into());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn only_the_last_start_line_ends_the_wait() {
    let mut run = ModeRun::new(Mode::NimbusPromenade, 8);
    let lines = Mode::NimbusPromenade.start_lines();
    run.intro_line = lines.last().map(|line| line.key());

    run.line_completed(lines[0].key());
    assert!(run.intro_line.is_some());

    run.line_completed(lines[1].key());
    assert!(run.intro_line.is_none());
  }

  #[test]
  fn each_stage_is_reported_once_as_it_is_reached() {
    let mut run = ModeRun::new(Mode::SolariumAtrium, 8);
    let stages: Vec<_> = (1..=8).map(|done| run.progress(done)).collect();
    assert_eq!(
      stages,
      vec![None, Some(25), None, Some(50), None, Some(75), None, None]
    );
  }

  #[test]
  fn a_jump_reports_the_stage_it_lands_on() {
    let mut run = ModeRun::new(Mode::MeridianBasins, 6);
    assert_eq!(run.progress(5), Some(75));
    assert_eq!(run.progress(6), None);
  }

  #[test]
  fn progress_can_go_back_and_be_reached_again_after_a_reset() {
    let mut run = ModeRun::new(Mode::HydroCore, 4);
    assert_eq!(run.progress(2), Some(50));
    assert_eq!(run.progress(1), None);
    assert_eq!(run.progress(2), None);
    run.reset_progress();
    assert_eq!(run.progress(1), Some(25));
    assert_eq!(run.progress(2), Some(50));
  }
}
