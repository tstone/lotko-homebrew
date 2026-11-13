use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::city_map;
use crate::hardware::drop_bank::{DropBankSystem, DropBankTargetHit};
use crate::hardware::lift_ramp::{LiftRampHit, LiftRampSystem};
use crate::hardware::lower_scoop::LowerScoopEjected;
use crate::hardware::rear_scoop::RearScoopBallEnter;
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::skyrail_station::mode::State::*;
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, ModeRun, RootLashed, SporesSystem, points,
};
use crate::systems::{Callout, SayExt, sounds};

/// Drop target hits in the finale that complete the mode
const FINALE_HITS: u8 = 5;

/// Hits to the end of the mode: 3 ramp shots, a drop after each of the first two, then the finale
const REQUIRED_HITS: u8 = 3 + 2 + FINALE_HITS;

/// Signal failure (DESIGN.md). The shots go lift ramp, drops, lift ramp, drops, lift ramp, and the third ramp shot adds
/// a ball for a 2-ball finale that's saved until the mode ends. The finale completes on 5 drop target hits, and the bank
/// raises itself each time it's cleared.
pub struct SkyrailStationMode {
  run: ModeRun,
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  intensity_effect: LedPlayback,
  /// Drop target hits before the finale
  target_hits: u8,
  ramp_hits: u8,
  finale_hits: u8,
  state: State,
  ramp_up: bool,
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
      finale_hits: 0,
      state: HitRamp,
      ramp_up: false,
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

  fn hits(&self) -> u8 {
    self.ramp_hits + self.target_hits + self.finale_hits
  }

  fn advance(&mut self, ctx: &SystemContext) {
    // each shot is a stage of its own, so a burst doubles the points rather than skipping one
    let times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.run.score_hit(times, ctx);
    self.hit_effect.play();
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(led_playback!("skyrail_station/flashers.scene"));
    ctx.play_sfx(sounds::city_hit(self.hits() as f32 / REQUIRED_HITS as f32));
    ctx.play_sfx(if self.state == HitRamp {
      sounds::WOOSH_LIGHT
    } else {
      sounds::LATCH_DOUBLE
    });

    match self.state {
      Final => {
        self.finale_hits += 1;
        log::info!("Skyrail: Final - target hits {}", self.finale_hits);

        if self.finale_hits == FINALE_HITS {
          self.ramp_down(ctx);
          self
            .run
            .complete(led_playback!("skyrail_station/complete.scene"), ctx);
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
          // the finale is on the drops with a 2 ball multiball
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

    self.run.progress(self.hits());
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(Self::progress_effect(self.hits()));
  }

  /// The finale's second ball. It's saved until the mode ends, and completing despawns the mode, which takes the save
  /// with it.
  fn add_ball(&mut self, ctx: &SystemContext) {
    ctx.eject_balls(1, Launch::Auto);
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

    if event.is::<RearScoopBallEnter>() {
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
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      // each hit of a lash makes whichever shot is next
      for _ in 0..lash.hits {
        self.advance(ctx);
      }
    }
  }

  /// A resume picks up at the shot it was on, with the bank back up for a drop shot. The finale gets its second ball back
  /// once the ball leaves the scoop.
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    if self.state != HitRamp {
      ctx
        .systems
        .expect::<DropBankSystem>()
        .raise_targets(ctx.into());
    }
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
