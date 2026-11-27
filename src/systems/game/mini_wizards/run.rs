use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_pinball::*;

use crate::hardware::lower_scoop::Lower;
use crate::leaderboards;
use crate::systems::game::{MULTIBALL_BALL_SAVE, Mode, ModeManager, ModeRun, points};

/// What every mini-wizard shares on top of its [`ModeRun`]: the balls, the ball save, the end and the total.
///
/// - Every one is 2-ball, so with Heartwood stacked on top play stays at 4 balls or fewer. The second ball launches
///   once the first leaves the scoop, and the ball save is timed from when it's in play.
/// - It ends when a drain that stands leaves one ball or none (decision 5), unless it keeps going on one ball
///   ([`Mode::ends_with_multiball`]). It counts as played, and if it wasn't completed it stays in the menu to resume,
///   keeping its progress. An earned one ([`Mode::is_earned`]) doesn't: it's over, and has to be earned again.
/// - What it scores is its total, which goes to its leaderboard if it has one
///   ([`leaderboards::mini_wizard_total`]). The total runs over the whole game for a place, as it's resumed, and over
///   one run for an earned mode.
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

  pub fn begin(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
  }

  /// See [`ModeRun::apply`]
  pub fn apply(&mut self, ctx: &SystemContext) -> bool {
    self.run.apply(ctx)
  }

  pub fn interrupt(&mut self, ctx: &SystemContext) {
    self.balls_added = false;
    self.eject_id = None;
    self.run.interrupt(ctx);
    // an earned mode isn't resumed, so the next time it's lit starts fresh
    if self.mode.is_earned() {
      ctx.systems.despawn_self();
    }
  }

  pub fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<ScoopEjected<Lower>>() && !self.balls_added {
      self.add_balls(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the balls are in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
    } else if let Some(done) = event.downcast_ref::<RoutineCompleted<BallDraining>>()
      && done.completion == RoutineCompletion::AllSteps
      && done.routine.in_play <= 1
      && self.mode.ends_with_multiball()
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
    self.submit_total(ctx);
  }

  /// See [`ModeRun::progress`]
  pub fn progress(&mut self, done: u8) -> Option<u8> {
    self.run.progress(done)
  }

  pub fn complete(&mut self, scene: PlayableScene<Completes>, ctx: &SystemContext) {
    self.total += points::mode_completion(self.mode) as u64;
    self.submit_total(ctx);
    log::info!("{:?}: completed, {} scored", self.mode, self.total);
    self.run.complete(scene, ctx);
  }

  /// Offers the total so far for the mode's leaderboard, if it has one
  fn submit_total(&self, ctx: &SystemContext) {
    if let Some(board) = leaderboards::mini_wizard_total(self.mode) {
      leaderboards::submit(board, self.total, ctx);
    }
  }

  fn add_balls(&mut self, ctx: &SystemContext) {
    self.balls_added = true;
    // on top of the one leaving the scoop
    self.eject_id = ctx.eject_balls(1, Launch::Auto);
    ctx.request_ball_save_until_released();
  }
}
