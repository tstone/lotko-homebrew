use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;

use crate::hardware::lower_scoop::LowerScoopEjected;
use crate::systems::game::{MULTIBALL_BALL_SAVE, Mode, ModeManager, ModeRun, points};

/// What every mini-wizard shares on top of its [`ModeRun`]: the balls, the ball save, the end and the total.
///
/// - Tier 1 is 2-ball and tier 2 is 3-ball. The extra balls launch once the ball leaves the scoop, and the ball save is
///   timed from when they're in play.
/// - It ends when a drain that stands leaves one ball or none (decision 5). It counts as played, and if it wasn't
///   completed it stays in the menu to resume, keeping its progress.
/// - What it scores is its total, for its high score.
///
/// The mini-wizard's system calls it from the same places as a [`ModeRun`], and scores through [`Self::score`].
#[derive(Clone)]
pub struct MiniWizardRun {
  mode: Mode,
  run: ModeRun,
  /// The extra balls launched for this run of it
  balls_added: bool,
  eject_id: Option<u64>,
  total: u64,
}

impl MiniWizardRun {
  pub fn new(mode: Mode, required: u8) -> Self {
    Self {
      mode,
      run: ModeRun::new(mode, required),
      balls_added: false,
      eject_id: None,
      total: 0,
    }
  }

  /// What it has scored, over every run of it this game
  // For the mini-wizard high scores (PLAN.md, Part 5)
  #[allow(dead_code)]
  pub fn total(&self) -> u64 {
    self.total
  }

  pub fn begin(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
  }

  /// See [`ModeRun::apply`]
  pub fn apply(&mut self, delta: Duration, ctx: &SystemContext) -> bool {
    self.run.apply(delta, ctx)
  }

  pub fn interrupt(&mut self, ctx: &SystemContext) {
    self.balls_added = false;
    self.eject_id = None;
    self.run.interrupt(ctx);
  }

  pub fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<LowerScoopEjected>() && !self.balls_added {
      self.add_balls(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the balls are in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
    } else if let Some(done) = event.downcast_ref::<RoutineCompleted<BallDraining>>()
      && done.completion == RoutineCompletion::AllSteps
      && done.routine.in_play <= 1
    {
      log::info!("{:?}: ended, {} scored so far", self.mode, self.total);
      ctx
        .systems
        .expect::<ModeManager>()
        .end_mode(self.mode, ctx.into());
    }
  }

  pub fn score(&mut self, points: u64, ctx: &SystemContext) {
    ctx.add_points(points);
    self.total += points as u64;
  }

  /// See [`ModeRun::progress`]
  pub fn progress(&mut self, done: u8) -> Option<u8> {
    self.run.progress(done)
  }

  pub fn complete(&mut self, effect: LedPlayback, ctx: &SystemContext) {
    self.total += points::mode_completion(self.mode) as u64;
    log::info!("{:?}: completed, {} scored", self.mode, self.total);
    self.run.complete(effect, ctx);
  }

  fn add_balls(&mut self, ctx: &SystemContext) {
    self.balls_added = true;
    // on top of the one leaving the scoop
    self.eject_id = ctx.eject_balls(self.mode.tier(), Launch::Auto);
    ctx.request_ball_save_until_released();
  }
}
