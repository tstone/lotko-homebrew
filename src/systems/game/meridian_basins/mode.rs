use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;

use crate::hardware::captive_ball;
use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::hardware::pop_cluster::PopBumper;
use crate::hardware::{city_map, pop_cluster};
use crate::systems::game::meridian_basins::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, SporeApplied, points};
use crate::systems::{sounds, sounds_bytes};
use frontbox_sound::*;

static REQUIRED_HITS: u8 = 6;

pub struct MeridianBasinsMode {
  run: ModeRun,
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  intensity_effect: LedPlayback,
  progress_effect: LedPlayback,
  hits: u8,
  current_shots: Vec<ModeShots>,
}

impl MeridianBasinsMode {
  pub fn new() -> Self {
    let initial = vec![
      ModeShots::Main(MainShot::LeftOrbit),
      ModeShots::Main(MainShot::DomeRamp),
      ModeShots::Main(MainShot::LeftOrbit),
    ];
    Self {
      run: ModeRun::new(Mode::MeridianBasins, REQUIRED_HITS),
      attention_effect: Self::attention_effect(&initial),
      hit_effect: Self::hit_effect(),
      intensity_effect: Self::intensity_effect(),
      progress_effect: Self::progress_effect(0),
      current_shots: initial,
      hits: 0,
    }
  }

  fn attention_effect(shots: &Vec<ModeShots>) -> LedPlayback {
    let led_qs: Vec<LedQ> = shots.iter().map(|shot| shot.led_q()).collect();
    let qs: Vec<&LedQ> = led_qs.iter().collect();

    led_playback!("meridian_basins/attention.scene", LedQ::any(qs))
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("meridian_basins/hit.scene").stopped()
  }

  fn complete_effect() -> LedPlayback {
    led_playback!("meridian_basins/complete.scene").stopped()
  }

  fn intensity_effect() -> LedPlayback {
    led_playback!("meridian_basins/intensity.scene").stopped()
  }

  fn progress_effect(hits: u8) -> LedPlayback {
    city_map::spore_count_effect(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  fn on_shot_hit(&mut self, shot: ModeShots, ctx: &SystemContext) {
    if self.current_shots.contains(&shot) {
      self.hits += 1;
      self.progress_effect = Self::progress_effect(self.hits);
      log::info!("MeridianBasins: hits = {}", self.hits);
      ctx.play_sfx(sounds::rnd_lane_hit());
      ctx.add_points(points::EXL_MODE_HIT);
      ctx.emit(SporeApplied);

      if self.hits == REQUIRED_HITS {
        log::info!("MeridianBasins: complete");
        ctx.play_sfx(sounds::ARP_HIT1);
        self
          .run
          .complete(points::EXL_COMPLETION, Self::complete_effect(), ctx);
        return;
      }

      if self.run.progress(self.hits) == Some(50) {
        self.intensity_effect.play();
      }
      self.hit_effect.play();
    }
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
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::MeridianBasins)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("MeridianBasins mode started");
    self.run.begin(ctx);
    ctx
      .cues
      .start(NextSet, Duration::from_millis(2250).forever());
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<NextSet>() {
      self.advance(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(ModeShots::Main(hit.shot), ctx);
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
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct NextSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ModeShots {
  Main(MainShot),
  LeftPop,
  CaptiveBall,
  UpperRightPop,
  LowerRightPop,
}

impl ModeShots {
  pub fn led_q(&self) -> LedQ {
    match self {
      Self::Main(shot) => shot.hex_center().clone(),
      Self::LeftPop => pop_cluster::left::POP_LED.q(),
      Self::CaptiveBall => LedQ::any(vec![
        &captive_ball::LEFT_BOLT.q(),
        &captive_ball::RIGHT_BOLT.q(),
      ]),
      Self::UpperRightPop => pop_cluster::upper_right::POP_LED.q(),
      Self::LowerRightPop => pop_cluster::lower_right::POP_LED.q(),
    }
  }

  pub fn ordered() -> Vec<ModeShots> {
    vec![
      Self::Main(MainShot::LeftOrbit),
      Self::Main(MainShot::DomeRamp),
      Self::LeftPop,
      Self::Main(MainShot::ArcRamp),
      Self::CaptiveBall,
      Self::Main(MainShot::CenterOrbit),
      Self::Main(MainShot::LiftRamp),
      Self::UpperRightPop,
      Self::Main(MainShot::RightOrbit),
      Self::LowerRightPop,
    ]
  }
}
