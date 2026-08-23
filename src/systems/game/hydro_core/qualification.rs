use frontbox::animation::*;
use frontbox::prelude::*;
use frontbox_turn_based::GameManagementExt;

use crate::hardware::arc_ramp;
use crate::hardware::arc_ramp::ArcRampHit;
use crate::systems::game::ExclusiveModeManager;
use crate::systems::game::HydroCoreStartable;
use crate::systems::game::hydro_core;
use crate::systems::game::hydro_core::qualification::State::*;

const REQUIRED_HITS: u8 = 2;

#[derive(Clone)]
pub struct HydroCoreQualification {
  state: State,
  hits: u8,
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
}

impl HydroCoreQualification {
  pub fn new() -> Self {
    Self {
      state: Qualifying,
      hits: 0,
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(),
    }
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      (&*arc_ramp::HEX_CENTER_LED).at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::rotating(
          (&*arc_ramp::HEX_CIRCLE_LEDS).at_z(1),
          ColorSequence::fade(Rgba::white(), Rgba::default()),
          Duration::from_millis(500),
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::rotating(
          arc_ramp::ARC_LEDS.q().at_z(1),
          ColorSequence::fade(Rgba::white(), Rgba::default()),
          Duration::from_millis(500),
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .stopped()
  }

  fn on_qualifying_hit(&mut self, ctx: &SystemContext) {
    self.hits += 1;
    log::info!("HydroCore: Qualifying hit ({})", self.hits);

    ctx.add_points(hydro_core::points::QUAL_HIT);
    self.hit_effect.reset();
    self.hit_effect.play();

    if self.hits == REQUIRED_HITS {
      self.state = Shutdown;
    } else {
      self.state = Cooldown;

      // because on the arc ramp the ball can roll up then down again and double trigger qualification
      // start a cooldown period to prevent this
      ctx.cue(Resume, Cue::Once(Duration::from_millis(2500)));
    }
  }
}

impl System for HydroCoreQualification {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .expect::<ExclusiveModeManager>()
      .current_mode()
      .is_none()
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);

    if self.state == Shutdown && self.hit_effect.is_complete() {
      self.attention_effect.stop(ctx);
      ctx.replace_self(HydroCoreStartable::new());
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<ArcRampHit>() && self.state == Qualifying {
      self.on_qualifying_hit(ctx);
    } else if event.is::<Resume>() {
      self.state = Qualifying;
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
  /// When qualifying shots are allowed
  Qualifying,
  /// When qualifying shots are not allowed (on cooldown)
  Cooldown,
  /// When qualifications have been met and the final animations need to play out
  Shutdown,
}

#[derive(serde::Serialize, Event)]
struct Resume;
