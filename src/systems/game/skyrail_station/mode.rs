use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::{GameManagementExt, PlayerTurnEnding};

use crate::hardware::drop_bank::{self, DropBankTargetHit};
use crate::hardware::lift_ramp::{LiftRampHit, LiftRampSystem};
use crate::hardware::{arc_ramp, lift_ramp};
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::skyrail_station::mode::State::*;
use crate::systems::game::{
  self, ExclusiveMode, ModeManager, SkyrailStationQualification, SkyrailStationStartable, points,
};

pub struct SkyrailStationMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  gi_effect: LedProgram1d,
  target_hits: u8,
  state: State,
  ramp_up: bool,
}

impl SkyrailStationMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect_ramp(),
      hit_effect: Self::hit_effect(),
      gi_effect: Self::gi_effect(),
      target_hits: 0,
      state: HitRamp,
      ramp_up: false,
    }
  }

  fn attention_effect_ramp() -> LedProgram1d {
    LedProgram1d::pulse(
      &*lift_ramp::HEX_CENTER_LED,
      *MODE_COLOR,
      Duration::bpm(128),
      Cycle::Forever,
    )
  }

  fn attention_effect_target() -> LedProgram1d {
    LedProgram1d::pulse(
      LedQ::any(vec![
        &drop_bank::TARGET1_LEDS.q(),
        &drop_bank::TARGET2_LEDS.q(),
        &drop_bank::TARGET3_LEDS.q(),
      ]),
      *MODE_COLOR,
      Duration::bpm(128),
      Cycle::Forever,
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::tween(
      LedQ::tag::<Playfield>().at_z(-1),
      Duration::from_millis(600),
      Curve::ExponentialOut,
      Cycle::Once,
      vec![
        ColorSequence::fade(*MODE_COLOR, Rgba::default()).shuffle(rand::random()),
        ColorSequence::solid(Rgba::default()),
      ],
    )
    .stopped()
  }

  fn gi_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      LedQ::any(vec![
        &LedQ::tag::<tags::GeneralIllumination>(),
        &arc_ramp::ARC_LEDS.q(),
        &arc_ramp::SUBWAY_LEDS.q(),
      ])
      .at_z(1),
      ColorSequence::solid(MODE_COLOR.lighten(0.3)),
    )
  }

  fn advance(&mut self, ctx: &SystemContext) {
    ctx.add_points(points::EXL_MODE_HIT);
    self.hit_effect.play();

    match self.state {
      HitTarget => {
        log::info!("Skyrail: HitTarget");
        self.target_hits += 1;
        self.attention_effect.stop(ctx);

        // check for completion
        if self.target_hits == 3 {
          log::info!("Skyrail: Final");
          self.state = Final;
          return;
        } else {
          self.state = HitRamp;
          self.attention_effect = Self::attention_effect_ramp();
          self.ramp_down(ctx);
        }
      }
      HitRamp => {
        log::info!("Skyrail: HitRamp");
        self.state = HitTarget;
        self.ramp_up(ctx);
        self.attention_effect.stop(ctx);
        self.attention_effect = Self::attention_effect_target();
      }
      _ => {}
    }
  }

  fn ramp_up(&mut self, ctx: &SystemContext) {
    if !self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
      self.ramp_up = true;
    }
  }

  fn ramp_down(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_down(ctx.into());
      self.ramp_up = false;
    }
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SkyrailStation, ctx);
    self.ramp_down(ctx);
    ctx.replace_self(SkyrailStationStartable::default());
  }

  fn complete(&mut self, ctx: &SystemContext) {
    self.ramp_down(ctx);
    ctx.add_points(game::points::EXL_COMPLETION);

    // TODO: epic reaction effect
    ctx
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::SkyrailStation, ctx);
    ctx.replace_self(SkyrailStationQualification::new());
  }
}

impl System for SkyrailStationMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SkyrailStation)
  }

  fn on_spawn(&mut self, _ctx: &SystemContext) {
    log::info!("SkyrailStation mode started");
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LiftRampHit>() && self.state == HitRamp {
      self.advance(ctx);
    } else if event.is::<DropBankTargetHit>() && self.state == HitTarget {
      self.advance(ctx);
    } else if event.is::<PlayerTurnEnding>() {
      self.revert_to_startable(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_down(ctx.into());
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    // self.hit_effect.apply(delta, ctx);
    self.gi_effect.apply(delta, ctx);

    if self.state == Shutdown && self.hit_effect.is_complete() {
      self.complete(ctx);
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
  HitRamp,
  HitTarget,
  Final,
  Shutdown,
}
