use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp::ArcRampSubwayHit;
use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::city_map;
use crate::hardware::main_shot::MainShotHit;
use crate::systems::game::{self, MULTIBALL_BALL_SAVE, ModeManager, Multiball};
use crate::systems::{Callout, SayExt, sounds};

/// Hits it takes to light the super jackpot, each time
const JACKPOTS_PER_SUPER: u8 = 5;

/// How long the super jackpot stays lit
const SUPER_JACKPOT_TIME: Duration = Duration::from_secs(20);

/// Arc ramps all the way to the subway that add a ball, once a multiball
const RAMPS_TO_ADD_A_BALL: u8 = 2;

/// The Heartwood multiball (DESIGN.md, Multiballs), 2-ball, started by [`HeartwoodCharge`](crate::systems::game::HeartwoodCharge).
/// Every main shot is a jackpot, and every 5th lights the captive ball as the super jackpot for a while. Making the arc
/// ramp to the subway [`RAMPS_TO_ADD_A_BALL`] times adds a ball, once. It ends when a drain that stands leaves one ball
/// or none, or with the turn.
#[derive(Clone)]
pub struct HeartwoodMultiballMode {
  attention_scene: ScenePlayer,
  super_jackpot_attention_scene: ScenePlayer,
  hit_scene: ScenePlayer,
  super_jackpot_hit_scene: ScenePlayer,
  hits: u8,
  /// Arc ramps made to the subway, toward adding a ball
  subway_ramps: u8,
  super_jackpot_active: bool,
  timer_id: Option<u64>,
  eject_id: Option<u64>,
}

impl HeartwoodMultiballMode {
  pub fn new() -> Self {
    Self {
      attention_scene: ScenePlayer::new(Self::attention_scene()),
      hit_scene: ScenePlayer::new(Self::hit_scene()),
      super_jackpot_attention_scene: ScenePlayer::new(Self::super_jackpot_attention_scene()),
      super_jackpot_hit_scene: ScenePlayer::new(Self::super_jackpot_hit_scene()),
      hits: 0,
      subway_ramps: 0,
      super_jackpot_active: false,
      timer_id: None,
      eject_id: None,
    }
  }

  fn attention_scene() -> PlayableScene {
    scene!("heartwood_multiball/attention.scene")
  }

  fn super_jackpot_attention_scene() -> PlayableScene {
    scene!("heartwood_multiball/super_jackpot_attention.scene")
  }

  fn hit_scene() -> PlayableScene {
    scene!("heartwood_multiball/hit.scene")
  }

  fn super_jackpot_hit_scene() -> PlayableScene {
    scene!("heartwood_multiball/super_jackpot_hit.scene")
  }

  fn progress_scene(hits: u8) -> PlayableScene<Completes> {
    let per = JACKPOTS_PER_SUPER;
    city_map::mode_progress_scene(
      Multiball::Heartwood.color(),
      (hits % per) as f32 / per as f32,
    )
  }

  fn jackpot_hit(&mut self, ctx: &SystemContext) {
    self.hit_scene.restart(ctx);
    self.hits += 1;
    ctx.play_scene(&Self::progress_scene(self.hits));
    ctx.add_points(game::points::HEARTWOOD_JACKPOT);
    // rises toward the super jackpot that every 5th hit lights
    ctx.play_sfx(sounds::city_hit(
      ((self.hits - 1) % JACKPOTS_PER_SUPER) as f32 / (JACKPOTS_PER_SUPER - 1) as f32,
    ));

    // super jackpot
    if self.hits % JACKPOTS_PER_SUPER == 0 {
      self.start_super_jackpot(ctx);
    }
  }

  fn super_jackpot_hit(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::HEARTWOOD_SUPER_JACKPOT);
    ctx.play_sfx(sounds::ARP_HIT1);
    self.super_jackpot_hit_scene.restart(ctx);
    self.end_super_jackpot(ctx);
  }

  fn start_super_jackpot(&mut self, ctx: &SystemContext) {
    // restart timer if another one is triggered
    if let Some(timer_id) = self.timer_id {
      ctx.timers.cancel(timer_id);
    }
    self.timer_id = Some(ctx.timers.start(SuperJackpotOver, SUPER_JACKPOT_TIME.once()));

    self.super_jackpot_active = true;
    self.super_jackpot_attention_scene.restart(ctx);
  }

  fn end_super_jackpot(&mut self, ctx: &SystemContext) {
    self.super_jackpot_active = false;
    self.super_jackpot_attention_scene.stop(ctx);
  }

  fn on_subway_ramp(&mut self, ctx: &SystemContext) {
    self.subway_ramps += 1;
    if self.subway_ramps == RAMPS_TO_ADD_A_BALL {
      log::info!("HeartwoodMultiball: ball added");
      ctx.say(Callout::AdditionalDeployment);
      self.eject_id = ctx.eject_balls(1, Launch::Auto);
    }
  }

  fn shutdown(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<ModeManager>()
      .release_multiball(Multiball::Heartwood, ctx.into());
    ctx.systems.despawn_self();
  }
}

impl System for HeartwoodMultiballMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_multiball() == Some(Multiball::Heartwood)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    // on top of the one in play
    self.eject_id = ctx.eject_balls(1, Launch::Auto);
    ctx.say(Callout::OakHeartwood);
    ctx.request_ball_save_until_released();
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    self.attention_scene.start(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<MainShotHit>() {
      self.jackpot_hit(ctx)
    } else if event.is::<ArcRampSubwayHit>() {
      self.on_subway_ramp(ctx);
    } else if event.is::<CaptiveBallHit>() && self.super_jackpot_active {
      self.super_jackpot_hit(ctx);
    } else if event.is::<SuperJackpotOver>() && self.super_jackpot_active {
      ctx.play_sfx(sounds::HIT_ORGANIC_LOW4);
      self.end_super_jackpot(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the balls are in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
    } else if let Some(done) = event.downcast_ref::<RoutineCompleted<BallDraining>>()
      && done.completion == RoutineCompletion::AllSteps
      && done.routine.in_play <= 1
    {
      // a drain that wasn't saved left one ball or none
      self.shutdown(ctx);
    } else if event.is::<RoutineStarted<TurnEnding>>() {
      self.shutdown(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct SuperJackpotOver;
