use frontbox::animation::*;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::arc_ramp::{self, ArcRampHit};
use crate::hardware::backbox::{LEFT_SPEAKER_LEDS, RIGHT_SPEAKER_LEDS};
use crate::hardware::captive_ball;
use crate::hardware::center_orbit::{self, CenterOrbitHit};
use crate::hardware::dome_ramp::{self, DomeRampHit};
use crate::hardware::flashers::{self};
use crate::hardware::left_orbit::{self, LeftOrbitHit};
use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::hardware::{city_map, lift_ramp, right_orbit};
use crate::systems::game::ExclusiveMode;
use crate::systems::game::ModeManager;
use crate::systems::game::hydro_core::MODE_COLOR;

// TODO: super jackpot

#[derive(Clone)]
pub struct SporeMultiballMode {
  attention_effect: LedProgram1d,
  super_jackpot_attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  super_jackpot_hit_effect: LedProgram1d,
  progress_effect: LedProgram1d,
  hits: u8,
  super_jackpot_active: bool,
  cue_id: Option<u64>,
}

impl SporeMultiballMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(),
      super_jackpot_attention_effect: Self::super_jackpot_attention_effect(),
      super_jackpot_hit_effect: Self::super_jackpot_hit_effect(),
      progress_effect: Self::hit_effect(), // TODO: handle progress until super jackpot
      hits: 0,
      super_jackpot_active: false,
      cue_id: None,
    }
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::rotating(
        &*left_orbit::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(500),
        Curve::EaseOut,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*dome_ramp::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(500),
        Curve::EaseOut,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*arc_ramp::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(500),
        Curve::EaseOut,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*center_orbit::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(500),
        Curve::EaseOut,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*lift_ramp::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(500),
        Curve::EaseOut,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*right_orbit::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(500),
        Curve::EaseOut,
        Cycle::Forever,
      ),
    ])
  }

  fn super_jackpot_attention_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      LedQ::any(vec![
        &captive_ball::LEFT_BOLT.q(),
        &captive_ball::RIGHT_BOLT.q(),
      ]),
      ColorSequence::exact(vec![Rgba::yellow(), Rgba::default()]),
      Duration::from_millis(550),
      Curve::Linear,
      Cycle::Forever,
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
        LEFT_SPEAKER_LEDS.q(),
        ColorSequence::exact(vec![*MODE_COLOR, *MODE_COLOR, *MODE_COLOR, *MODE_COLOR]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        RIGHT_SPEAKER_LEDS.q(),
        ColorSequence::exact(vec![*MODE_COLOR, *MODE_COLOR, *MODE_COLOR, *MODE_COLOR]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
    ])
    .stopped()
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::flash(
        &*left_orbit::HEX_CIRCLE_LEDS,
        ColorSequence::solid(*MODE_COLOR),
        Cycle::Forever,
      ),
      LedProgram1d::flash(
        &*dome_ramp::HEX_CIRCLE_LEDS,
        ColorSequence::solid(*MODE_COLOR),
        Cycle::Forever,
      ),
      LedProgram1d::flash(
        &*arc_ramp::HEX_CIRCLE_LEDS,
        ColorSequence::solid(*MODE_COLOR),
        Cycle::Forever,
      ),
      LedProgram1d::flash(
        &*center_orbit::HEX_CIRCLE_LEDS,
        ColorSequence::solid(*MODE_COLOR),
        Cycle::Forever,
      ),
      LedProgram1d::flash(
        &*lift_ramp::HEX_CIRCLE_LEDS,
        ColorSequence::solid(*MODE_COLOR),
        Cycle::Forever,
      ),
      LedProgram1d::flash(
        &*right_orbit::HEX_CIRCLE_LEDS,
        ColorSequence::solid(*MODE_COLOR),
        Cycle::Forever,
      ),
    ])
  }

  fn super_jackpot_hit_effect() -> LedProgram1d {
    LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::flash(
          LedQ::any(vec![
            &captive_ball::LEFT_BOLT.q(),
            &captive_ball::RIGHT_BOLT.q(),
          ]),
          ColorSequence::solid(Rgba::yellow()),
          Cycle::Times(3),
        ),
      )
      .at(
        Duration::from_millis(185) * 3,
        LedProgram1d::rotating(
          LedQ::any(vec![
            &captive_ball::LEFT_BOLT.q(),
            &arc_ramp::HEX_LINE_LEDS,
            &dome_ramp::HEX_LINE_LEDS,
            &left_orbit::HEX_LINE_LEDS,
          ]),
          ColorSequence::exact(vec![Rgba::orange()]),
          Duration::from_millis(1500),
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .at(
        Duration::from_millis(185) * 3,
        LedProgram1d::rotating(
          LedQ::any(vec![
            &captive_ball::RIGHT_BOLT.q(),
            &lift_ramp::HEX_LINE_LEDS,
            &right_orbit::HEX_LINE_LEDS,
          ]),
          ColorSequence::exact(vec![Rgba::orange()]),
          Duration::from_millis(1500),
          Curve::Linear,
          Cycle::Once,
        ),
      )
  }

  fn progress_effect(duration: Duration) -> LedProgram1d {
    LedProgram1d::progress_time_remaining(
      city_map::SPORE_COUNT_BAR.q().reverse(),
      ColorSequence::fade(Rgba::blue(), *MODE_COLOR),
      duration,
      Curve::Linear,
    )
  }

  fn jackpot_hit(&mut self, ctx: &SystemContext) {
    self.hit_effect.reset();
    self.hit_effect.play();
    self.hits += 1;
    ctx.add_points(5_000_000);

    // super jackpot
    if self.hits % 5 == 0 {
      self.start_super_jackpot(ctx);
    }
  }

  fn super_jackpot_hit(&mut self, ctx: &SystemContext) {
    ctx.add_points(25_000_000);
    self.super_jackpot_hit_effect.play();
    self.end_super_jackpot(ctx);
  }

  fn start_super_jackpot(&mut self, ctx: &SystemContext) {
    // restart timer if another one is triggered
    if let Some(cue_id) = self.cue_id {
      ctx.cancel_cue(cue_id);
    }
    self.cue_id = Some(ctx.cue(SuperJackpotOver, Duration::from_secs(20).once()));

    self.super_jackpot_active = true;
    self.super_jackpot_attention_effect.play();
  }

  fn end_super_jackpot(&mut self, ctx: &SystemContext) {
    self.super_jackpot_active = false;
    self.super_jackpot_attention_effect.stop(ctx);
  }
}

impl System for SporeMultiballMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SporeMultiball)
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);
    self.super_jackpot_attention_effect.apply(delta, ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftOrbitHit>()
      || event.is::<DomeRampHit>()
      || event.is::<ArcRampHit>()
      || event.is::<CenterOrbitHit>()
      || event.is::<LiftRampHit>()
      || event.is::<RightOrbitHit>()
    {
      self.jackpot_hit(ctx)
    } else if event.is::<SuperJackpotOver>() && self.super_jackpot_active {
      self.end_super_jackpot(ctx);
    } else if event.is::<PlayerTurnEnding>() {
      ctx.despawn_self();
    }
  }
}

#[derive(serde::Serialize, Event)]
struct SuperJackpotOver;
