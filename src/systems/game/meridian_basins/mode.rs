use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::arc_ramp::ArcRampHit;
use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::dome_ramp::DomeRampHit;
use crate::hardware::left_orbit::LeftOrbitHit;
use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::pop_cluster::PopBumper;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::hardware::{
  arc_ramp, backbox, center_orbit, city_map, dome_ramp, left_orbit, lift_ramp, pop_cluster,
  right_orbit,
};
use crate::hardware::{captive_ball, flashers};
use crate::systems::game::meridian_basins::MODE_COLOR;
use crate::systems::game::{
  ExclusiveMode, LiftRampStartable, MeridianBasinsQualification, ModeManager, points,
};
use crate::systems::{sounds, sounds_bytes};
use frontbox_sound::*;

static REQUIRED_HITS: u8 = 6;

pub struct MeridianBasinsMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  intensity_effect: LedProgram1d,
  progress_effect: LedProgram1d,
  hits: u8,
  current_shots: Vec<ModeShots>,
}

impl MeridianBasinsMode {
  pub fn new() -> Self {
    let initial = vec![
      ModeShots::LeftOrbit,
      ModeShots::DomeRamp,
      ModeShots::LeftOrbit,
    ];
    Self {
      attention_effect: Self::attention_effect(&initial),
      hit_effect: Self::hit_effect(2),
      intensity_effect: Self::intensity_effect(),
      progress_effect: Self::progress_effect(0),
      current_shots: initial,
      hits: 0,
    }
  }

  fn attention_effect(shots: &Vec<ModeShots>) -> LedProgram1d {
    let led_qs: Vec<LedQ> = shots.iter().map(|shot| shot.led_q()).collect();
    let qs: Vec<&LedQ> = led_qs.iter().collect();

    LedProgram1d::pulse(
      LedQ::any(qs),
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
      .release_exclusive(&ExclusiveMode::MeridianBasins, ctx.into());
    ctx.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::MeridianBasins,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn on_shot_hit(&mut self, shot: ModeShots, ctx: &SystemContext) {
    if self.current_shots.contains(&shot) {
      self.hits += 1;
      self.progress_effect = Self::progress_effect(self.hits);
      log::info!("MeridianBasins: hits = {}", self.hits);
      ctx.play_sfx(sounds::rnd_lane_hit());
      ctx.add_points(points::EXL_MODE_HIT);

      if self.hits == 4 {
        self.intensity_effect.play();
      } else if self.is_complete() {
        log::info!("MeridianBasins: complete");
        ctx.play_sfx(sounds::ARP_HIT1);
        self.hit_effect = Self::hit_effect(4);
        ctx.add_points(points::EXL_COMPLETION);
      }

      self.hit_effect.play();
    }
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::MeridianBasins, ctx.into());
    ctx.replace_self(MeridianBasinsQualification::new());
  }

  fn is_complete(&self) -> bool {
    self.hits == REQUIRED_HITS
  }

  fn advance(&mut self, ctx: &SystemContext) {
    let ordered = ModeShots::ordered();
    let current_start_idx = ordered
      .iter()
      .position(|s| *s == self.current_shots[0])
      .unwrap_or(0);

    self.current_shots = vec![
      ordered[Self::wrap_shot_idx(current_start_idx + 1)],
      ordered[Self::wrap_shot_idx(current_start_idx + 2)],
      ordered[Self::wrap_shot_idx(current_start_idx + 3)],
    ];

    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect(&self.current_shots);
  }

  fn wrap_shot_idx(idx: usize) -> usize {
    let len = ModeShots::ordered().len();
    if idx >= len {
      Self::wrap_shot_idx(idx - len)
    } else {
      idx
    }
  }
}

impl System for MeridianBasinsMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::MeridianBasins)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("MeridianBasins mode started");
    ctx.cue(NextSet, Duration::from_millis(2250).forever());
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<NextSet>() {
      self.advance(ctx);
    } else if event.is::<LeftOrbitHit>() {
      self.on_shot_hit(ModeShots::LeftOrbit, ctx);
    } else if event.is::<DomeRampHit>() {
      self.on_shot_hit(ModeShots::DomeRamp, ctx);
    } else if event.is::<ArcRampHit>() {
      self.on_shot_hit(ModeShots::ArcRamp, ctx);
    } else if event.is::<CenterOrbitHit>() {
      self.on_shot_hit(ModeShots::CenterOrbit, ctx);
    } else if event.is::<LiftRampHit>() {
      self.on_shot_hit(ModeShots::LiftRamp, ctx);
    } else if event.is::<RightOrbitHit>() {
      self.on_shot_hit(ModeShots::RightOrbit, ctx);
    } else if event.is::<CaptiveBallHit>() {
      self.on_shot_hit(ModeShots::CaptiveBall, ctx)
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && let Some(pop) = pop_cluster::match_switch(&event.switch)
    {
      match pop {
        PopBumper::Left => self.on_shot_hit(ModeShots::LeftPop, ctx),
        PopBumper::UpperRight => self.on_shot_hit(ModeShots::UpperRightPop, ctx),
        PopBumper::LowerRight => self.on_shot_hit(ModeShots::LowerRightPop, ctx),
      }
    } else if event.is::<PlayerTurnBeginning>() {
      self.revert_to_startable(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.is_complete() && self.hit_effect.is_complete() {
      self.complete(ctx);
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct NextSet;

// TODO: this feels like it has more general purpose application
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ModeShots {
  LeftOrbit,
  DomeRamp,
  LeftPop,
  ArcRamp,
  CaptiveBall,
  CenterOrbit,
  LiftRamp,
  UpperRightPop,
  RightOrbit,
  LowerRightPop,
}

impl ModeShots {
  pub fn led_q(&self) -> LedQ {
    match self {
      Self::LeftOrbit => left_orbit::HEX_CENTER_LED.clone(),
      Self::DomeRamp => dome_ramp::HEX_CENTER_LED.clone(),
      Self::LeftPop => pop_cluster::left::POP_LED.q(),
      Self::ArcRamp => arc_ramp::HEX_CENTER_LED.clone(),
      Self::CaptiveBall => LedQ::any(vec![
        &captive_ball::LEFT_BOLT.q(),
        &captive_ball::RIGHT_BOLT.q(),
      ]),
      Self::CenterOrbit => center_orbit::HEX_CENTER_LED.clone(),
      Self::LiftRamp => lift_ramp::HEX_CENTER_LED.clone(),
      Self::UpperRightPop => pop_cluster::upper_right::POP_LED.q(),
      Self::RightOrbit => right_orbit::HEX_CENTER_LED.clone(),
      Self::LowerRightPop => pop_cluster::lower_right::POP_LED.q(),
    }
  }

  pub fn ordered() -> Vec<ModeShots> {
    vec![
      Self::LeftOrbit,
      Self::DomeRamp,
      Self::LeftPop,
      Self::ArcRamp,
      Self::CaptiveBall,
      Self::CenterOrbit,
      Self::LiftRamp,
      Self::UpperRightPop,
      Self::RightOrbit,
      Self::LowerRightPop,
    ]
  }
}
