use frontbox_core::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp::ArcRampHit;
use crate::hardware::lower_scoop::{LowerScoopBallEnter, LowerScoopEjected};
use crate::systems::game::points;
use crate::systems::{Callout, SayExt, sounds};

/// How long after the scoop ejects that an arc ramp shot is a reflex shot
const REFLEX_WINDOW: Duration = Duration::from_secs(2);

/// Reflex shots (DESIGN.md, ArcStreak): hitting the ball straight off the lower scoop's eject up the arc ramp. The ramp
/// feeds the subway back to the scoop, so reflex shots can be looped. The first pays 2M, the second 3M, and each after
/// 5M.
///
/// The streak breaks when an eject goes by without a reflex shot, or the ball comes into the scoop some way other than
/// the subway.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct ArcStreakSystem {
  /// Reflex shots in the current streak
  streak: usize,
  window_cue: Option<u64>,
}

impl ArcStreakSystem {
  pub fn new() -> Self {
    Self {
      streak: 0,
      window_cue: None,
    }
  }

  fn on_ramp_hit(&mut self, ctx: &SystemContext) {
    let Some(cue) = self.window_cue.take() else {
      return;
    };
    ctx.cues.cancel(cue);

    let award = points::ARC_STREAK[self.streak.min(points::ARC_STREAK.len() - 1)];
    self.streak += 1;
    log::info!("ArcStreak: reflex shot {} for {award}", self.streak);
    ctx.add_points(award);
    ctx.play_sfx(sounds::ARP_HIT1);
    ctx.say(match self.streak {
      1 => Callout::TwoMillion,
      2 => Callout::ThreeMillion,
      _ => Callout::FiveMillion,
    });
  }

  fn break_streak(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.window_cue.take() {
      ctx.cues.cancel(cue);
    }
    if self.streak > 0 {
      log::info!("ArcStreak: broken after {}", self.streak);
    }
    self.streak = 0;
  }
}

impl System for ArcStreakSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.break_streak(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LowerScoopEjected>() {
      self.window_cue = Some(ctx.cues.start(ReflexWindowClosed, REFLEX_WINDOW.once()));
    } else if event.is::<ArcRampHit>() {
      self.on_ramp_hit(ctx);
    } else if event.is::<ReflexWindowClosed>() {
      self.window_cue = None;
      self.break_streak(ctx);
    } else if let Some(enter) = event.downcast_ref::<LowerScoopBallEnter>()
      && !enter.subway_entry
    {
      self.break_streak(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ReflexWindowClosed;
