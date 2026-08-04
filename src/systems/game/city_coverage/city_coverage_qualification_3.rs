use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::{ScoopBallEntered, lift_ramp, lower_scoop};

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
      )),
      lower_scoop_effect: LedEffect::flash(
        lower_scoop::bolts_q(),
        Rgba::white(),
        Rgba::cyan(),
        Duration::from_millis(83 * 3),
      ),
    }
  }

  pub fn complete(&mut self, ctx: &Context) {
    ctx.add_points(50000);
    // TODO: launch menu

    // TEMPORARY: move this to menu system:
    ctx
      .systems
      .expect::<lower_scoop::LowerScoopSystem>()
      .set_mode(lower_scoop::LowerScoopMode::AutoEject, ctx);
  }
}

impl System for CityCoverageQualification3 {
  fn on_spawn(&mut self, ctx: &Context) {
    ctx
      .systems
      .expect::<lower_scoop::LowerScoopSystem>()
      .set_mode(lower_scoop::LowerScoopMode::ModeStart, ctx);
    ctx
      .systems
      .expect::<lift_ramp::LiftRampSystem>()
      .lift_up(ctx, Duration::from_millis(20));
  }

  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    if let Some(effect) = self.lift_ramp_effect.as_mut() {
      effect.apply(delta, ctx);
    }
    self.lower_scoop_effect.apply(delta, ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if event.is::<lift_ramp::LiftRampDown>() {
      if let Some(effect) = &mut self.lift_ramp_effect {
        effect.stop_and_clear(ctx);
      }
      self.lift_ramp_effect = None;
    } else if let Some(ScoopBallEntered(name)) = event.downcast_ref::<ScoopBallEntered>() {
      if (*name).eq(lower_scoop::SCOOP_NAME) {
        self.complete(ctx);
      } else if (*name).eq(lift_ramp::SCOOP_NAME) {
        ctx
          .systems
          .expect::<lift_ramp::LiftRampSystem>()
          .lift_down(ctx);
        self.complete(ctx);
      }
    }
  }
}
