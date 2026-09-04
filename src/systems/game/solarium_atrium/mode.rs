use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::{GameManagementExt, PlayerTurnEnding};

use crate::game::solarium_atrium::MODE_COLOR;
use crate::hardware::arc_ramp::ArcRampHit;
use crate::hardware::dome_ramp::DomeRampHit;
use crate::hardware::{arc_ramp, lift_ramp};
use crate::systems::game::{
  self, ExclusiveMode, LeftScoopStartable, ModeManager, SolariumAtriumQualification,
};

pub struct SolariumAtriumMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  gi_effect: LedProgram1d,
  ramp_hits: u8,
  multiplier: f32,
}

impl SolariumAtriumMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(),
      gi_effect: Self::gi_effect(),
      ramp_hits: 0,
      multiplier: 1.0,
    }
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::pulse(
      &*lift_ramp::HEX_CENTER_LED,
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

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SolariumAtrium, ctx);
    ctx.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::SolariumAtrium,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn add_multiple(&mut self) {
    self.multiplier = (self.multiplier + 0.5).min(3.0);
  }

  fn ramp_hit(&mut self, ctx: &SystemContext) {
    self.ramp_hits += 1;
    self.hit_effect.play();
    ctx.add_points((game::points::EXL_MODE_HIT as f32 * self.multiplier) as u32);
    self.multiplier = 1.0;
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::EXL_COMPLETION);

    // TODO: epic reaction effect
    ctx
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::SolariumAtrium, ctx);
    ctx.replace_self(SolariumAtriumQualification::new());
  }

  fn is_complete(&self) -> bool {
    self.ramp_hits == 3
  }
}

impl System for SolariumAtriumMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SolariumAtrium)
  }

  fn on_spawn(&mut self, _ctx: &SystemContext) {
    log::info!("Solarium Atrium mode started");
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<PlayerTurnEnding>() {
      self.revert_to_startable(ctx);
    } else if event.is::<ArcRampHit>() {
      self.add_multiple();
    } else if event.is::<DomeRampHit>() {
      self.ramp_hit(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.gi_effect.apply(delta, ctx);

    if self.hit_effect.is_complete() && self.is_complete() {
      self.complete(ctx);
    }
  }
}
