use frontbox_core::prelude::*;
use frontbox_gfx::scene::NumberFormat;
use frontbox_lighting::prelude::*;
use frontbox_pinball::{
  ExtraBallGranted, ExtraBallUsed, GameManagerExt, TurnStartReason, TurnStarting,
};

use crate::hardware::trough::DRAIN_LED;
use crate::systems::dmd::{ANNOUNCEMENT_DMD_LAYER, DmdScreen};
use crate::systems::game::{MiniWizardCompleted, points};
use crate::systems::{Callout, SayExt};

/// How long the extra ball and shoot again screens are shown
const SCREEN_DURATION: Duration = Duration::from_secs(2);

/// What lit an extra ball. Each lights at most one per game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraBallSource {
  /// Completing the first mini-wizard
  MiniWizard,
  /// 10 drones down
  Sentinels,
  /// Spreading the roots on all three orbits in one ball
  RootNetwork,
}

/// Lights and collects extra balls (DESIGN.md, Extra balls). A source lights one (`light`), and a shot to the front of
/// the left scoop collects it (`collect_at_scoop`, from `ModeSelectSystem`, which goes first with anything lit); a ball
/// coming in from the arc ramp's subway doesn't. Collecting grants the extra ball, or awards points once the player is
/// at the `MAX_EXTRA_BALLS` limit.
///
/// While one is lit, the scoop bolts flash white. While the player has one to use, the drain LED (shoot again) is amber.
///
/// Belongs in the player template: what's lit, and which sources have been used, are the player's.
#[derive(Clone)]
pub struct ExtraBallSystem {
  /// Extra balls lit and not yet collected
  lit: u8,
  /// Sources that have lit one this game
  sources_used: Vec<ExtraBallSource>,
  lit_scene: ScenePlayer,
  screen_timer: Option<u64>,
  /// The screen showing is a collect, which the scoop is waiting on
  collecting: bool,
  handle: SystemHandle,
  screen: DmdScreen,
}

impl ExtraBallSystem {
  pub fn new() -> Self {
    let lit_scene = ScenePlayer::new(scene!("extra_ball_lit.scene"));

    Self {
      lit: 0,
      sources_used: Vec::new(),
      lit_scene,
      screen_timer: None,
      collecting: false,
      handle: SystemHandle::default(),
      screen: DmdScreen::new("dmd/announcement.scene", ANNOUNCEMENT_DMD_LAYER),
    }
  }

  /// Light an extra ball, unless `source` already lit one this game. Returns whether it was lit.
  pub fn light(&mut self, source: ExtraBallSource, ctx: &ServiceContext) -> bool {
    if self.sources_used.contains(&source) {
      return false;
    }
    let ctx = &ctx.for_system(self.handle);
    self.sources_used.push(source);
    self.lit += 1;
    log::info!("Extra ball lit by {:?}, {} lit", source, self.lit);
    self.show("EXTRA BALL LIT", ctx);
    ctx.say(Callout::UnregisteredResident);
    self.update_lit_scene(ctx);
    true
  }

  /// Collect a lit extra ball, if there is one. Returns whether there was, in which case [`ExtraBallCollected`] follows
  /// when its animation has played.
  pub fn collect_at_scoop(&mut self, ctx: &ServiceContext) -> bool {
    if self.lit == 0 {
      return false;
    }
    self.collect(&ctx.for_system(self.handle));
    self.collecting = true;
    true
  }

  fn collect(&mut self, ctx: &SystemContext) {
    self.lit -= 1;
    ctx.say(Callout::ErrorInvalidResident);
    if ctx.grant_extra_ball() {
      log::info!("Extra ball collected");
      self.show("EXTRA BALL", ctx);
      ctx.say(Callout::OakExtraBall);
      self.update_shoot_again(ctx);
    } else {
      log::info!("Extra ball collected past the limit, awarding points");
      ctx.add_points(points::EXTRA_BALL_AS_POINTS);
      self.show(&NumberFormat::Grouped.write(points::EXTRA_BALL_AS_POINTS as i64), ctx);
    }
    self.update_lit_scene(ctx);
  }

  fn update_lit_scene(&mut self, ctx: &SystemContext) {
    if self.lit > 0 {
      self.lit_scene.start(ctx);
    } else {
      self.lit_scene.stop(ctx);
    }
  }

  /// The drain LED stays lit while the player has an extra ball to use
  fn update_shoot_again(&self, ctx: &SystemContext) {
    if ctx.extra_balls() > 0 {
      ctx.declare_lights(&DRAIN_LED.q(), 0, Rgba::amber());
    } else {
      ctx.undeclare_lights(&DRAIN_LED.q(), 0);
    }
  }

  fn show(&mut self, label: &str, ctx: &SystemContext) {
    self.screen.show([("text", label.into()), ("color", Rgba::white().into())], ctx);
    if let Some(timer) = self.screen_timer.take() {
      ctx.timers.cancel(timer);
    }
    self.screen_timer = Some(ctx.timers.start(ScreenShown, SCREEN_DURATION.once()));
  }

  fn remove_screen(&mut self, ctx: &SystemContext) {
    self.screen.clear(ctx);
  }
}

impl System for ExtraBallSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<MiniWizardCompleted>() {
      self.light(ExtraBallSource::MiniWizard, ctx.into());
    } else if event.is::<ExtraBallGranted>() || event.is::<ExtraBallUsed>() {
      self.update_shoot_again(ctx);
    } else if let Some(RoutineStarted { routine }) =
      event.downcast_ref::<RoutineStarted<TurnStarting>>()
    {
      self.update_shoot_again(ctx);
      self.update_lit_scene(ctx);
      if routine.reason == TurnStartReason::ExtraBall {
        self.show("SHOOT AGAIN", ctx);
        ctx.say(Callout::OakShootAgain);
      }
    } else if event.is::<ScreenShown>() {
      self.screen_timer = None;
      self.remove_screen(ctx);
      if self.collecting {
        self.collecting = false;
        ctx.emit(ExtraBallCollected);
      }
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.remove_screen(ctx);
  }
}

/// An extra ball was collected and its animation has played, so the scoop can move on
#[derive(serde::Serialize, Event)]
pub struct ExtraBallCollected;

#[derive(serde::Serialize, Event)]
struct ScreenShown;
