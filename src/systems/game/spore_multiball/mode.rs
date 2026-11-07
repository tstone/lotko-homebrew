use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::city_map;
use crate::hardware::main_shot::MainShotHit;
use crate::systems::game::ModeManager;
use crate::systems::game::SporeMultiballQualification;
use crate::systems::game::{MULTIBALL_BALL_SAVE, Multiball};
use crate::systems::sounds;

// TODO: super jackpot

#[derive(Clone)]
pub struct SporeMultiballMode {
  attention_effect: LedPlayback,
  super_jackpot_attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  super_jackpot_hit_effect: LedPlayback,
  progress_effect: LedPlayback,
  hits: u8,
  super_jackpot_active: bool,
  cue_id: Option<u64>,
  eject_id: Option<u64>,
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
      eject_id: None,
    }
  }

  fn attention_effect() -> LedPlayback {
    led_playback!("spore_multiball/attention.scene")
  }

  fn super_jackpot_attention_effect() -> LedPlayback {
    led_playback!("spore_multiball/super_jackpot_attention.scene").stopped()
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("spore_multiball/hit.scene").stopped()
  }

  fn super_jackpot_hit_effect() -> LedPlayback {
    led_playback!("spore_multiball/super_jackpot_hit.scene").stopped()
  }

  fn progress_effect(hits: u8) -> LedPlayback {
    city_map::spore_count_effect(Rgba::yellow(), (hits % 5) as f32 / 5.0)
  }

  fn jackpot_hit(&mut self, ctx: &SystemContext) {
    self.hit_effect.reset();
    self.hit_effect.play();
    self.hits += 1;
    ctx.add_points(5_000_000);
    // rises toward the super jackpot that every 5th hit lights
    ctx.play_sfx(sounds::rising_organic_hit((self.hits - 1) % 5 + 1));

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
      ctx.cues.cancel(cue_id);
    }
    self.cue_id = Some(
      ctx
        .cues
        .start(SuperJackpotOver, Duration::from_secs(20).once()),
    );

    self.super_jackpot_active = true;
    self.super_jackpot_attention_effect.play();
  }

  fn end_super_jackpot(&mut self, ctx: &SystemContext) {
    self.super_jackpot_active = false;
    self.super_jackpot_attention_effect.stop(ctx);
  }

  fn shutdown(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<ModeManager>()
      .release_multiball(Multiball::Heartwood, ctx.into());
    ctx.systems.replace_self(SporeMultiballQualification::new());
  }
}

impl System for SporeMultiballMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_multiball() == Some(Multiball::Heartwood)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.eject_id = ctx.eject_balls(2, Launch::Auto);
    ctx.request_ball_save_until_released();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);
    self.super_jackpot_hit_effect.apply(delta, ctx);
    self.super_jackpot_attention_effect.apply(delta, ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<MainShotHit>() {
      self.jackpot_hit(ctx)
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
