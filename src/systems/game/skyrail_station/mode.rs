use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::city_map;
use crate::hardware::drop_bank::{DropBankSystem, DropBankTargetHit};
use crate::hardware::lift_ramp::{LiftRampHit, LiftRampScoopBallEnter, LiftRampSystem};
use crate::hardware::lower_scoop::LowerScoopEjected;
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::skyrail_station::mode::State::*;
use crate::systems::game::{
  self, EffectsSystem, MULTIBALL_BALL_SAVE, Mode, ModeManager, ModeRun, SporesSystem, points,
};
use crate::systems::{Callout, SayExt, sounds};

/// Ramp shots and drop targets to the end of the mode
const REQUIRED_HITS: u8 = 8;

pub struct SkyrailStationMode {
  run: ModeRun,
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  intensity_effect: LedPlayback,
  target_hits: u8,
  ramp_hits: u8,
  state: State,
  ramp_up: bool,
  eject_id: Option<u64>,
  /// The finale lost its second ball with the turn, so it needs it back on resuming
  needs_ball: bool,
}

impl SkyrailStationMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::SkyrailStation, REQUIRED_HITS),
      attention_effect: Self::attention_effect_ramp(),
      hit_effect: Self::hit_effect(),
      intensity_effect: Self::intensity_effect(),
      target_hits: 0,
      ramp_hits: 0,
      state: HitRamp,
      ramp_up: false,
      eject_id: None,
      needs_ball: false,
    }
  }

  fn attention_effect_ramp() -> LedPlayback {
    led_playback!("skyrail_station/attention_ramp.scene")
  }

  fn attention_effect_target() -> LedPlayback {
    led_playback!("skyrail_station/attention_target.scene")
  }

  fn attention_effect_target_final() -> LedPlayback {
    led_playback!("skyrail_station/attention_target_final.scene")
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("skyrail_station/hit.scene").stopped()
  }

  fn intensity_effect() -> LedPlayback {
    led_playback!("skyrail_station/intensity.scene").stopped()
  }

  fn progress_effect(hits: u8) -> LedPlayback {
    city_map::mode_progress_effect(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  fn advance(&mut self, ctx: &SystemContext) {
    ctx.add_points(points::EXL_MODE_HIT);
    // each shot is a stage of its own, so a burst can't skip one
    ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.hit_effect.play();
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(led_playback!("skyrail_station/flashers.scene"));
    ctx.play_sfx(sounds::city_hit(
      (self.target_hits + self.ramp_hits) as f32 / REQUIRED_HITS as f32,
    ));
    ctx.play_sfx(if self.state == HitRamp {
      sounds::WOOSH_LIGHT
    } else {
      sounds::LATCH_DOUBLE
    });

    match self.state {
      Final => {
        self.target_hits += 1;
        log::info!("Skyrail: Final - target hits {}", self.target_hits);

        // check for completion
        if self.target_hits == 5 {
          self.ramp_down(ctx);
          self.run.complete(
            game::points::EXL_COMPLETION,
            led_playback!("skyrail_station/complete.scene"),
            ctx,
          );
          return;
        }
      }
      HitTarget => {
        log::info!("Skyrail: HitTarget");
        self.target_hits += 1;
        self.state = HitRamp;
        self.attention_effect.stop(ctx);
        self.attention_effect = Self::attention_effect_ramp();
        self.ramp_down(ctx);
        log::info!("SkyrailStation: hit target => ramp down");
      }
      HitRamp => {
        log::info!("Skyrail: HitRamp");
        self.ramp_hits += 1;
        if self.ramp_hits == 1 {
          ctx.say(Callout::The515IsDelayed);
        }

        if self.ramp_hits == 3 {
          // on the final round all 3 targets must be hit with a 2 ball multiball
          self.add_ball(ctx);
          self.state = Final;
          self.intensity_effect.play();
          self.attention_effect.stop(ctx);
          self.attention_effect = Self::attention_effect_target_final();
        } else {
          self.state = HitTarget;
          self.attention_effect.stop(ctx);
          self.attention_effect = Self::attention_effect_target();
        }

        self.ramp_up(Duration::from_millis(250), ctx);
        log::info!("SkyrailStation: hit ramp => ramp up");

        ctx
          .systems
          .expect::<DropBankSystem>()
          .raise_targets(ctx.into());
      }
    }

    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(Self::progress_effect(self.target_hits + self.ramp_hits));
  }

  fn add_ball(&mut self, ctx: &SystemContext) {
    self.eject_id = ctx.eject_balls(1, Launch::Auto);
    ctx.request_ball_save_until_released();
  }

  fn ramp_up(&mut self, delay: Duration, ctx: &SystemContext) {
    if !self.ramp_up {
      if delay > Duration::ZERO {
        ctx.cues.start(RampUp, delay.once());
      } else {
        ctx
          .systems
          .expect::<LiftRampSystem>()
          .request_up(ctx.id(), ctx.into());
        self.ramp_up = true;
      }
    }
  }

  fn ramp_down(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx
        .systems
        .expect::<LiftRampSystem>()
        .release(ctx.id(), ctx.into());
      self.ramp_up = false;
    }
  }
}

impl System for SkyrailStationMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::SkyrailStation)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("SkyrailStation mode started");
    self.run.begin(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<LiftRampScoopBallEnter>() {
      ctx.add_points(points::SKYRAIL_SCOOP_ENTER);
    } else if event.is::<LowerScoopEjected>() && self.needs_ball {
      self.needs_ball = false;
      self.add_ball(ctx);
    } else if event.is::<RampUp>() {
      self.ramp_up(Duration::ZERO, ctx);
    } else if event.is::<LiftRampHit>() && self.state == HitRamp {
      self.advance(ctx);
    } else if event.is::<DropBankTargetHit>() && (self.state == HitTarget || self.state == Final) {
      self.advance(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the ball is in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx
        .systems
        .expect::<LiftRampSystem>()
        .request_up(ctx.id(), ctx.into());
    }
    ctx.activate_led_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx
        .systems
        .expect::<LiftRampSystem>()
        .release(ctx.id(), ctx.into());
    }
    self.needs_ball = self.state == Final;
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
  HitRamp,
  HitTarget,
  Final,
}

#[derive(serde::Serialize, Event)]
struct RampUp;
