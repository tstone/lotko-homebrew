use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::{GameManagementExt, PlayerTurnActive};

use crate::game::solarium_atrium::MODE_COLOR;
use crate::hardware::arc_ramp::{self, ArcRampHit};
use crate::hardware::dome_ramp::{self, DomeRampHit};
use crate::hardware::flashers::{self, FlashersSystem};
use crate::hardware::lift_ramp::{self, LiftRampHit};
use crate::hardware::{backbox, city_map};
use crate::systems::game::{
  self, ExclusiveMode, LeftScoopStartable, ModeManager, SolariumAtriumQualification,
};
use crate::systems::sounds;

static REQUIRED_HITS: u8 = 6;

pub struct SolariumAtriumMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  intensity_effect: LedProgram1d,
  progress_effect: LedProgram1d,
  ramp_hits: u8,
  last_ramp: Option<Ramp>,
}

impl SolariumAtriumMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(2),
      intensity_effect: Self::intensity_effect(),
      progress_effect: Self::progress_effect(0),
      ramp_hits: 0,
      last_ramp: None,
    }
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::pulse(
      LedQ::any(vec![
        &*dome_ramp::HEX_CENTER_LED,
        &*arc_ramp::HEX_CENTER_LED,
        &*lift_ramp::HEX_CENTER_LED,
      ]),
      *MODE_COLOR,
      Duration::bpm(128),
      Cycle::Forever,
    )
  }

  fn hit_effect(flash_count: u32) -> LedProgram1d {
    LedProgram1d::flash(
      LedQ::tag::<Playfield>().at_z(-1),
      (*MODE_COLOR).into(),
      Cycle::Times(flash_count),
    )
    .stopped()
  }

  fn intensity_effect() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::rotating(
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
        ColorSequence::exact(vec![Rgba::white(), Rgba::white().lighten(0.4)]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        backbox::LEFT_SPEAKER_LEDS.q().reverse(),
        ColorSequence::exact(vec![*MODE_COLOR, *MODE_COLOR, *MODE_COLOR, *MODE_COLOR]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        backbox::LEFT_SPEAKER_LEDS.q().reverse(),
        ColorSequence::exact(vec![*MODE_COLOR, *MODE_COLOR, *MODE_COLOR, *MODE_COLOR]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
    ])
    .stopped()
  }

  fn progress_effect(hits: u8) -> LedProgram1d {
    LedProgram1d::fixed(
      city_map::SPORE_COUNT_BAR.q().reverse(),
      ColorSequence::solid(*MODE_COLOR)
        .padding_right(Extent::Relative(1.0 - (hits as f32 / REQUIRED_HITS as f32))),
    )
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SolariumAtrium, ctx.into());
    ctx.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::SolariumAtrium,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn ramp_hit(&mut self, ramp_hit: Ramp, ctx: &SystemContext) {
    self.ramp_hits += 1;
    self.hit_effect.reset();
    ctx
      .expect::<FlashersSystem>()
      .flash(3, ColorSequence::tile(vec![*MODE_COLOR, Rgba::default()]));

    let points = match (self.last_ramp.as_ref(), &ramp_hit) {
      (Some(Ramp::DomeRamp), Ramp::LiftRamp) => game::points::EXL_MODE_HIT as f32 * 2.0,
      (Some(Ramp::LiftRamp), Ramp::DomeRamp) => game::points::EXL_MODE_HIT as f32 * 2.0,
      _ => game::points::EXL_MODE_HIT as f32,
    };
    ctx.add_points(points as u32);

    if self.ramp_hits == (REQUIRED_HITS - 2) {
      self.intensity_effect.play();
    } else if self.ramp_hits == REQUIRED_HITS {
      ctx.play_sfx(sounds::ARP_HIT1);
      self.hit_effect = Self::hit_effect(4);
    }
    self.hit_effect.play();

    self.last_ramp = Some(ramp_hit);
    self.progress_effect = Self::progress_effect(self.ramp_hits);
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::EXL_COMPLETION);

    // TODO: epic reaction effect
    ctx
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::SolariumAtrium, ctx.into());
    ctx.replace_self(SolariumAtriumQualification::new());
  }

  fn is_complete(&self) -> bool {
    self.ramp_hits == REQUIRED_HITS
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
    if event.is::<PlayerTurnActive>() {
      self.revert_to_startable(ctx);
    } else if event.is::<DomeRampHit>() {
      self.ramp_hit(Ramp::DomeRamp, ctx);
    } else if event.is::<ArcRampHit>() {
      self.ramp_hit(Ramp::ArcRamp, ctx);
    } else if event.is::<LiftRampHit>() {
      self.ramp_hit(Ramp::LiftRamp, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.hit_effect.is_complete() && self.is_complete() {
      self.complete(ctx);
    }

    self.progress_effect.apply(delta, ctx);
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
  }
}

enum Ramp {
  DomeRamp,
  ArcRamp,
  LiftRamp,
}
