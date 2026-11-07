use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::city_map;
use crate::hardware::lower_scoop::LowerScoopEjected;
use crate::hardware::main_shot::MainShotHit;
use crate::systems::game::{MULTIBALL_BALL_SAVE, Mode, ModeManager, ModeRun, points};
use crate::systems::sounds;

const REQUIRED_HITS: u8 = 10;
const HIT_POINTS: u32 = 1_000_000;

/// Stands in for all four mini-wizards until they're built (PLAN.md, Part 3), so that the route can be played to its
/// end. It has their shape: a multiball started from the menu, 2 balls in tier 1 and 3 in tier 2, that ends when one
/// ball or none is left (decision 5). Any main shot is a hit.
#[derive(Clone)]
pub struct MiniWizardMode {
  mode: Mode,
  run: ModeRun,
  progress_effect: LedPlayback,
  hits: u8,
  eject_id: Option<u64>,
}

impl MiniWizardMode {
  pub fn new(mode: Mode) -> Self {
    Self {
      mode,
      run: ModeRun::new(mode, REQUIRED_HITS),
      progress_effect: Self::progress_effect(mode, 0),
      hits: 0,
      eject_id: None,
    }
  }

  fn progress_effect(mode: Mode, hits: u8) -> LedPlayback {
    city_map::spore_count_effect(mode.color(), hits as f32 / REQUIRED_HITS as f32)
  }

  fn add_balls(&mut self, ctx: &SystemContext) {
    // on top of the one leaving the scoop
    self.eject_id = ctx.eject_balls(self.mode.tier(), Launch::Auto);
    ctx.request_ball_save_until_released();
  }

  fn on_hit(&mut self, ctx: &SystemContext) {
    self.hits += 1;
    ctx.add_points(HIT_POINTS * self.mode.tier() as u32);
    ctx.play_sfx(sounds::rising_organic_hit(self.hits.div_ceil(3)));
    self.progress_effect.stop(ctx);
    self.progress_effect = Self::progress_effect(self.mode, self.hits);

    if self.hits == REQUIRED_HITS {
      self.run.complete(
        points::EXL_COMPLETION,
        led_playback!("startable/hit.scene").param("color", self.mode.color()),
        ctx,
      );
    }
  }
}

impl System for MiniWizardMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(self.mode)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.eject_id = None;
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<LowerScoopEjected>() && self.eject_id.is_none() {
      self.add_balls(ctx);
    } else if event.is::<MainShotHit>() {
      self.on_hit(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the balls are in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
    } else if let Some(done) = event.downcast_ref::<RoutineCompleted<BallDraining>>()
      && done.completion == RoutineCompletion::AllSteps
      && done.routine.in_play <= 1
    {
      // played but not completed, so it stays in the menu to resume
      ctx
        .systems
        .expect::<ModeManager>()
        .end_mode(self.mode, ctx.into());
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if !self.run.apply(delta, ctx) {
      self.progress_effect.apply(delta, ctx);
    }
  }
}
