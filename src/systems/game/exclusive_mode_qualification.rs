use std::marker::PhantomData;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::GameManagementExt;

use crate::hardware::{arc_ramp, center_orbit, dome_ramp, left_orbit, lift_ramp, right_orbit};
use crate::systems::game;
use crate::systems::game::ModeManager;

#[derive(Clone)]
pub struct ExclusiveModeQualification<T: ExclusiveModeQualifier + Clone> {
  hits: u8,
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  _t: PhantomData<T>,
}

impl<T> ExclusiveModeQualification<T>
where
  T: ExclusiveModeQualifier + Clone,
{
  pub fn new() -> Self {
    Self {
      hits: 0,
      attention_effect: T::attention_effect(),
      hit_effect: Self::hit_effect(),
      _t: PhantomData,
    }
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      LedQ::any(vec![
        &left_orbit::HEX_LEDS.q(),
        &dome_ramp::HEX_LEDS.q(),
        &arc_ramp::HEX_LEDS.q(),
        &center_orbit::HEX_LEDS.q(),
        &lift_ramp::HEX_LEDS.q(),
        &right_orbit::HEX_LEDS.q(),
      ])
      .at_z(1),
      ColorSequence::fade(Rgba::white(), Rgba::default()),
      Duration::from_millis(500),
      Curve::Linear,
      Cycle::Once,
    )
    .stopped()
  }

  fn on_qualifying_hit(&mut self, ctx: &SystemContext) {
    self.hits += 1;
    ctx.add_points(game::points::EXCL_QUAL_HIT);
    self.hit_effect.reset();
    self.hit_effect.play();
    if self.hits == T::REQUIRED_HITS {
      ctx.play_sfx(T::HIT_SND_KEY); // or make this part of the spec too
    }
  }

  fn shutdown(&mut self, ctx: &SystemContext) {
    self.attention_effect.stop(ctx);
    T::on_qualified(ctx);
  }
}

impl<T> System for ExclusiveModeQualification<T>
where
  T: ExclusiveModeQualifier + Clone,
{
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode().is_none()
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    if self.hits == T::REQUIRED_HITS && self.hit_effect.is_complete() {
      self.shutdown(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if T::is_qualifying_shot(event) {
      self.on_qualifying_hit(ctx);
    }
  }
}

pub trait ExclusiveModeQualifier: Send + Sync + 'static {
  const REQUIRED_HITS: u8;
  const HIT_SND_KEY: &'static str;

  fn is_qualifying_shot(event: &dyn Event) -> bool;
  fn hit_effect() -> LedProgram1d;
  fn attention_effect() -> LedProgram1d;
  fn on_qualified(ctx: &SystemContext);
}
