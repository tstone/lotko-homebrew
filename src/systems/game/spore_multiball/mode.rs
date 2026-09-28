use frontbox::animation::*;
use frontbox::prelude::*;
use frontbox::provided::MultiballEnded;
use frontbox::provided::MultiballExt;
use frontbox::tags::Playfield;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::*;

use crate::hardware::arc_ramp::{self, ArcRampHit};
use crate::hardware::captive_ball;
use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::center_orbit::{self, CenterOrbitHit};
use crate::hardware::dome_ramp::{self, DomeRampHit};
use crate::hardware::left_orbit::{self, LeftOrbitHit};
use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::hardware::{city_map, lift_ramp, right_orbit};
use crate::systems::game::ExclusiveMode;
use crate::systems::game::ModeManager;
use crate::systems::game::SporeMultiballQualification;
use crate::systems::sounds;

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
      progress_effect: Self::progress_effect(0),
      hits: 0,
      super_jackpot_active: false,
      cue_id: None,
    }
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::pulse(
        &*left_orbit::HEX_LINE_LEDS,
        Rgba::yellow(),
        Duration::from_millis(350),
        Cycle::Forever,
      ),
      LedProgram1d::pulse(
        &*dome_ramp::HEX_LINE_LEDS,
        Rgba::yellow(),
        Duration::from_millis(350),
        Cycle::Forever,
      ),
      LedProgram1d::pulse(
        &*arc_ramp::HEX_LINE_LEDS,
        Rgba::yellow(),
        Duration::from_millis(350),
        Cycle::Forever,
      ),
      LedProgram1d::pulse(
        &*center_orbit::HEX_LINE_LEDS,
        Rgba::yellow(),
        Duration::from_millis(350),
        Cycle::Forever,
      ),
      LedProgram1d::pulse(
        &*lift_ramp::HEX_LINE_LEDS,
        Rgba::yellow(),
        Duration::from_millis(350),
        Cycle::Forever,
      ),
      LedProgram1d::pulse(
        &*right_orbit::HEX_LINE_LEDS,
        Rgba::yellow(),
        Duration::from_millis(350),
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
      ColorSequence::exact(vec![Rgba::orange(), Rgba::default()]),
      Duration::from_millis(550),
      Curve::Linear,
      Cycle::Forever,
    )
    .stopped()
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::flash(
      LedQ::Every,
      ColorSequence::fade(Rgba::yellow(), Rgba::default()),
      Cycle::Times(2),
    )
    .stopped()
  }

  fn super_jackpot_hit_effect() -> LedProgram1d {
    LedProgram1d::flash(
      LedQ::tag::<Playfield>(),
      ColorSequence::solid(Rgba::orange()),
      Cycle::Times(4),
    )
    .stopped()
  }

  fn progress_effect(hits: u8) -> LedProgram1d {
    LedProgram1d::fixed(
      city_map::SPORE_COUNT_BAR.q().reverse(),
      ColorSequence::solid(Rgba::yellow())
        .padding_right(Extent::Relative(1.0 - (hits % 5) as f32 / 5.0)),
    )
  }

  fn jackpot_hit(&mut self, ctx: &SystemContext) {
    self.hit_effect.reset();
    self.hit_effect.play();
    self.hits += 1;
    ctx.add_points(5_000_000);
    ctx.play_sfx(sounds::rnd_lane_hit());

    // super jackpot
    if self.hits % 5 == 0 {
      self.start_super_jackpot(ctx);
    }
  }

  fn super_jackpot_hit(&mut self, ctx: &SystemContext) {
    ctx.add_points(25_000_000);
    ctx.play_sfx(sounds::ARP_HIT1);
    self.super_jackpot_hit_effect.reset();
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

  fn shutdown(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SporeMultiball, ctx.into());
    ctx.replace_self(SporeMultiballQualification::new());
  }
}

impl System for SporeMultiballMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SporeMultiball)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.multiball_add_balls(2);
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);
    self.super_jackpot_hit_effect.apply(delta, ctx);
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
    } else if event.is::<CaptiveBallHit>() && self.super_jackpot_active {
      self.super_jackpot_hit(ctx);
    } else if event.is::<SuperJackpotOver>() && self.super_jackpot_active {
      self.end_super_jackpot(ctx);
    } else if event.is::<MultiballEnded>() || event.is::<PlayerTurnEnding>() {
      self.shutdown(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct SuperJackpotOver;
