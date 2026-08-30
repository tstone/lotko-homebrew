use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::{GameManagementExt, PlayerTurnEnding};

use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::{lift_ramp, pop_cluster};
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::skyrail_station::mode::State::*;
use crate::systems::game::{
  self, ExclusiveMode, ModeManager, SkyrailStationQualification, SkyrailStationStartable, points,
};

pub struct SkyrailStationMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  remaining_targets: Vec<&'static str>,
  current_target: Option<&'static str>,
  state: State,
}

impl SkyrailStationMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect_ramp(),
      hit_effect: Self::hit_effect(),
      remaining_targets: vec![
        pop_cluster::left::TARGET_SWITCH.name,
        pop_cluster::upper_right::TARGET_SWITCH.name,
        pop_cluster::lower_right::TARGET_SWITCH.name,
      ],
      current_target: None,
      state: HitRamp,
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

  fn attention_effect_target(target: &'static str) -> LedProgram1d {
    LedProgram1d::pulse(
      LedQ::name(target),
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

  fn advance(&mut self, ctx: &SystemContext) {
    ctx.add_points(points::EXL_MODE_HIT);
    self.hit_effect.play();

    // check for completion
    if self.remaining_targets.len() == 0 {
      self.state = Shutdown;
      return;
    }

    match self.state {
      HitRamp => {
        self.state = HitTarget;
        let idx = rand::random_range(0..self.remaining_targets.len());
        let target = self.remaining_targets.swap_remove(idx);
        self.attention_effect.stop(ctx);
        self.attention_effect = Self::attention_effect_target(target);
      }
      HitTarget => {
        self.state = HitRamp;
        self.current_target = None;
        self.attention_effect.stop(ctx);
        self.attention_effect = Self::attention_effect_ramp();
      }
      _ => {}
    }
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SkyrailStation, ctx);
    ctx.replace_self(SkyrailStationStartable::new());
  }

  fn complete(&mut self, ctx: &SystemContext) {
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

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LiftRampHit>() && self.state == HitRamp {
      self.advance(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && let Some(target_name) = self.current_target
      && event.switch.name == target_name
    {
      self.advance(ctx);
    } else if event.is::<PlayerTurnEnding>() {
      self.revert_to_startable(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);

    if self.state == Shutdown && self.hit_effect.is_complete() {
      self.complete(ctx);
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
  HitRamp,
  HitTarget,
  Shutdown,
}
