use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::*;

use crate::hardware::{ScoopBallEntered, lift_ramp, lower_scoop};
use crate::systems::sounds;

#[derive(Clone)]
pub struct CityCoverageQualification3 {
  lift_ramp_effect: Option<LedEffect>,
  lower_scoop_effect: LedEffect,
}

impl CityCoverageQualification3 {
  pub fn new() -> Self {
    Self {
      lift_ramp_effect: Some(LedEffect::flash(
        lift_ramp::BOLT_LED.q(),
        Rgba::white(),
        Rgba::cyan(),
        Duration::from_millis(83 * 3),
        Cycle::Forever,
      )),
      lower_scoop_effect: LedEffect::flash(
        lower_scoop::bolts_q(),
        Rgba::white(),
        Rgba::cyan(),
        Duration::from_millis(83 * 3),
        Cycle::Forever,
      ),
    }
  }

  pub fn complete(&mut self, ctx: &SystemContext) {
    ctx.play_sfx(sounds::LANE_HIT_COMPLETE);
    ctx.add_points(50000);

    // clear effects
    if let Some(effect) = &mut self.lift_ramp_effect {
      effect.stop(ctx);
    }
    self.lift_ramp_effect = None;
    self.lower_scoop_effect.stop(ctx);

    // TODO: launch menu

    // TEMPORARY: move this to menu system:
    ctx
      .expect::<lower_scoop::LowerScoopSystem>()
      .set_mode(lower_scoop::LowerScoopMode::AutoEject, ctx);
  }
}

impl System for CityCoverageQualification3 {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<lower_scoop::LowerScoopSystem>()
      .set_mode(lower_scoop::LowerScoopMode::ModeStart, ctx);
    ctx
      .expect::<lift_ramp::LiftRampSystem>()
      .lift_up(ctx.into());

    // TODO: should this hurry up time be flexed up or down depending on other achievements? (yes)
    ctx.cue(LiftRampHurryUpDone, Cue::Once(Duration::from_secs(25)));
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(effect) = self.lift_ramp_effect.as_mut() {
      effect.apply(delta, ctx);
    }
    self.lower_scoop_effect.apply(delta, ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LiftRampHurryUpDone>() {
      ctx
        .expect::<lift_ramp::LiftRampSystem>()
        .lift_down(ctx.into());
      if let Some(effect) = &mut self.lift_ramp_effect {
        effect.stop(ctx);
      }
      self.lift_ramp_effect = None;
    } else if let Some(ScoopBallEntered(name)) = event.downcast_ref::<ScoopBallEntered>() {
      if (*name).eq(lower_scoop::SCOOP_NAME) {
        self.complete(ctx);
      } else if (*name).eq(lift_ramp::SCOOP_NAME) {
        ctx
          .expect::<lift_ramp::LiftRampSystem>()
          .lift_down(ctx.into());
        self.complete(ctx);
      }
    }
  }
}

#[derive(serde::Serialize, Event)]
struct LiftRampHurryUpDone;
