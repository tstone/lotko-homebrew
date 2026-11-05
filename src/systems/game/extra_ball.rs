use frontbox_core::prelude::*;
use frontbox_dmd::*;
use frontbox_gfx::canvas::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{
  ExtraBallGranted, ExtraBallUsed, GameManagementExt, TurnPhase, TurnStartReason, TurnStarting,
};

use crate::hardware::lower_scoop::{self, LowerScoopBallEnter};
use crate::hardware::trough::DRAIN_LED;
use crate::systems::dmd::ANNOUNCEMENT_DMD_LAYER;
use crate::systems::game::{CityModeCompleted, points};

/// How long the extra ball and shoot again screens are shown
const SCREEN_DURATION: Duration = Duration::from_secs(2);

/// What lit an extra ball. Each lights at most one per game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraBallSource {
  /// Completing the first district
  Progression,
  /// 10 drones down
  Sentinels,
  /// The super jackpot in the first Groundswell
  Groundswell,
}

/// Lights and collects extra balls (RULES.md, Extra balls). A source lights one (`light`), and a shot to the front of
/// the left scoop collects it; a ball coming in from the arc ramp's subway doesn't. Collecting grants the extra ball,
/// or awards points once the player is at the `MAX_EXTRA_BALLS` limit.
///
/// While one is lit, the scoop bolts flash white and the drain LED pulses. While the player has one to use, the drain
/// LED (shoot again) stays lit.
///
/// Belongs in the player template: what's lit, and which sources have been used, are the player's.
#[derive(Clone)]
pub struct ExtraBallSystem {
  /// Extra balls lit and not yet collected
  lit: u8,
  /// Sources that have lit one this game
  sources_used: Vec<ExtraBallSource>,
  lit_effect: LedProgram1d,
  lit_effect_playing: bool,
  screen_cue: Option<u64>,
  handle: SystemHandle,
}

impl ExtraBallSystem {
  pub fn new() -> Self {
    let lit_effect = LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::flash(
          (&*lower_scoop::BOLTS_Q).at_z(3),
          ColorSequence::solid(Rgba::white()),
          Cycle::Forever,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::pulse(
          DRAIN_LED.q().at_z(1),
          Rgba::white(),
          Duration::from_millis(1000),
          Cycle::Forever,
        ),
      )
      .stopped();

    Self {
      lit: 0,
      sources_used: Vec::new(),
      lit_effect,
      lit_effect_playing: false,
      screen_cue: None,
      handle: SystemHandle::default(),
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
    self.update_lit_effect(ctx);
    true
  }

  fn collect(&mut self, ctx: &SystemContext) {
    self.lit -= 1;
    if ctx.grant_extra_ball() {
      log::info!("Extra ball collected");
      self.show("EXTRA BALL", ctx);
    } else {
      log::info!("Extra ball collected past the limit, awarding points");
      ctx.add_points(points::EXTRA_BALL_AS_POINTS);
      self.show(&TextFormatting::number(points::EXTRA_BALL_AS_POINTS), ctx);
    }
    self.update_lit_effect(ctx);
  }

  fn update_lit_effect(&mut self, ctx: &SystemContext) {
    let lit = self.lit > 0;
    if lit == self.lit_effect_playing {
      return;
    }
    self.lit_effect_playing = lit;
    if lit {
      self.lit_effect.reset();
      self.lit_effect.play();
    } else {
      self.lit_effect.stop(ctx);
    }
  }

  /// The drain LED stays lit while the player has an extra ball to use
  fn update_shoot_again(&self, ctx: &SystemContext) {
    if ctx.extra_balls() > 0 {
      ctx.declare_leds(&DRAIN_LED.q(), ColorSequence::solid(Rgba::white()));
    } else {
      ctx.undeclare_leds(&DRAIN_LED.q());
    }
  }

  fn show(&mut self, label: &str, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      let mut frame = Container::new(Fill2d::Solid(Rgba::black()));
      frame.add(
        BOLD_PIXELS_10PX_REGULAR_FONT
          .center_aligned(label.to_string(), Rgba::white())
          .top_offset(11),
      );
      dmd.insert_layer(ANNOUNCEMENT_DMD_LAYER, frame.default_position());
    }
    if let Some(cue) = self.screen_cue.take() {
      ctx.cues.cancel(cue);
    }
    self.screen_cue = Some(ctx.cues.start(ScreenShown, SCREEN_DURATION.once()));
  }

  fn remove_screen(&self, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      dmd.remove_layer(ANNOUNCEMENT_DMD_LAYER);
    }
  }
}

impl System for ExtraBallSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(LowerScoopBallEnter {
      subway_entry: false,
    }) = event.downcast_ref::<LowerScoopBallEnter>()
      && self.lit > 0
      && ctx.states.is(TurnPhase::Active)
    {
      self.collect(ctx);
    } else if let Some(CityModeCompleted { completed: 1 }) =
      event.downcast_ref::<CityModeCompleted>()
    {
      self.light(ExtraBallSource::Progression, ctx.into());
    } else if event.is::<ExtraBallGranted>() || event.is::<ExtraBallUsed>() {
      self.update_shoot_again(ctx);
    } else if let Some(RoutineStarted { routine }) =
      event.downcast_ref::<RoutineStarted<TurnStarting>>()
    {
      self.update_shoot_again(ctx);
      self.update_lit_effect(ctx);
      if routine.reason == TurnStartReason::ExtraBall {
        self.show("SHOOT AGAIN", ctx);
      }
    } else if event.is::<ScreenShown>() {
      self.screen_cue = None;
      self.remove_screen(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.lit_effect.apply(delta, ctx);
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.remove_screen(ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct ScreenShown;
