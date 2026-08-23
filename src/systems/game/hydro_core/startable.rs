use frontbox::animation::*;
use frontbox::prelude::*;
use frontbox_turn_based::GameManagementExt;

use crate::hardware::arc_ramp;
use crate::hardware::lower_scoop;
use crate::hardware::lower_scoop::LowerScoopBallEnter;
use crate::systems::game::ExclusiveMode;
use crate::systems::game::ExclusiveModeManager;
use crate::systems::game::HydroCoreMode;
use crate::systems::game::hydro_core;
use crate::systems::game::hydro_core::MODE_COLOR;
use crate::systems::game::hydro_core::startable::State::*;

#[derive(Clone)]
pub struct HydroCoreStartable {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  state: State,
}

impl HydroCoreStartable {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(),
      state: Cooldown,
    }
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::flash(
      Q::any(vec![&*lower_scoop::BOLTS_Q, &*arc_ramp::HEX_CENTER_LED]),
      ColorSequence::solid(*MODE_COLOR),
      Cycle::Forever,
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::tween(
      Q::tag::<tags::Playfield>().at_z(-1),
      Duration::from_millis(500),
      Curve::ExponentialOut,
      Cycle::Once,
      vec![
        ColorSequence::solid(*MODE_COLOR),
        ColorSequence::solid(Rgba::default()),
      ],
    )
  }

  fn attempt_start(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Ok(..) = ctx
      .expect::<ExclusiveModeManager>()
      .take_exclusive(ExclusiveMode::HydroCore, ctx)
    {
      self.start(ctx);
    }
  }

  fn start(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: started.");
    self.hit_effect.play();
    self.state = Shutdown;

    ctx.add_points(hydro_core::points::START);
  }
}

impl System for HydroCoreStartable {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .expect::<ExclusiveModeManager>()
      .current_mode()
      .is_none()
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.cue(Resume, Cue::Once(Duration::from_millis(1250)));
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);

    if self.state == Shutdown && self.hit_effect.is_complete() {
      self.attention_effect.stop(ctx);
      ctx.replace_self(HydroCoreMode::new());
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Resume>() {
      self.state = Startable;
    } else if event.is::<LowerScoopBallEnter>() && self.state == Startable {
      self.attempt_start(ctx);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Cooldown,
  Startable,
  Shutdown,
}

#[derive(serde::Serialize, Event)]
struct Resume;
