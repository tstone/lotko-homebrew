use frontbox_core::prelude::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;

use crate::hardware::drop_bank::{self, DropBankSystem, DropBankTargetHit};
use crate::hardware::flashers::{self, FlashersSystem};
use crate::hardware::lift_ramp::{LiftRampHit, LiftRampScoopBallEnter, LiftRampSystem};
use crate::hardware::{backbox, city_map, lift_ramp};
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::skyrail_station::mode::State::*;
use crate::systems::game::{
  self, ExclusiveMode, LiftRampStartable, MULTIBALL_BALL_SAVE, ModeManager, SporeApplied, points,
};

pub struct SkyrailStationMode {
  attention_effect: LedScene,
  hit_effect: LedScene,
  intensity_effect: LedScene,
  progress_effect: LedScene,
  target_hits: u8,
  ramp_hits: u8,
  state: State,
  ramp_up: bool,
  eject_id: Option<u64>,
}

impl SkyrailStationMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect_ramp(),
      hit_effect: Self::hit_effect(),
      intensity_effect: Self::intensity_effect(),
      progress_effect: Self::progress_effect(0),
      target_hits: 0,
      ramp_hits: 0,
      state: HitRamp,
      ramp_up: false,
      eject_id: None,
    }
  }

  fn attention_effect_ramp() -> LedScene {
    LedScene::new(
      &*lift_ramp::HEX_CENTER_LED,
      led_scene!("skyrail_station/attention_ramp.ron"),
    )
  }

  fn attention_effect_target() -> LedScene {
    LedScene::unbound(led_scene!("skyrail_station/attention_target.ron"))
      .on(
        "targets",
        LedQ::any(vec![
          &drop_bank::TARGET1_LEDS.q().skip(3).take(1),
          &drop_bank::TARGET2_LEDS.q().skip(3).take(1),
          &drop_bank::TARGET3_LEDS.q().skip(3).take(1),
        ]),
      )
      .unwrap()
      .on("hex", &*lift_ramp::HEX_LINE_LEDS)
      .unwrap()
  }

  fn attention_effect_target_final() -> LedScene {
    LedScene::unbound(led_scene!("skyrail_station/attention_target_final.ron"))
      .on("target1", drop_bank::TARGET1_LEDS.q())
      .unwrap()
      .on("target2", drop_bank::TARGET2_LEDS.q())
      .unwrap()
      .on("target3", drop_bank::TARGET3_LEDS.q())
      .unwrap()
      .on("hex", &*lift_ramp::HEX_LINE_LEDS)
      .unwrap()
  }

  fn hit_effect() -> LedScene {
    LedScene::new(
      LedQ::tag::<Playfield>().at_z(-1),
      led_scene!("skyrail_station/hit.ron"),
    )
    .stopped()
  }

  fn intensity_effect() -> LedScene {
    LedScene::unbound(led_scene!("skyrail_station/intensity.ron"))
      .on(
        "flashers",
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
      )
      .unwrap()
      .on("left_speaker", backbox::LEFT_SPEAKER_LEDS.q().reverse())
      .unwrap()
      .on("right_speaker", backbox::RIGHT_SPEAKER_LEDS.q().reverse())
      .unwrap()
      .stopped()
  }

  fn progress_effect(hits: u8) -> LedScene {
    city_map::spore_count_effect(*MODE_COLOR, hits as f32 / 8.0)
  }

  fn advance(&mut self, ctx: &SystemContext) {
    ctx.add_points(points::EXL_MODE_HIT);
    ctx.emit(SporeApplied);
    self.hit_effect.play();
    ctx
      .systems
      .expect::<FlashersSystem>()
      .play(led_scene!("skyrail_station/flashers.ron"));

    match self.state {
      Final => {
        self.target_hits += 1;
        log::info!("Skyrail: Final - target hits {}", self.target_hits);

        // check for completion
        if self.target_hits == 5 {
          self.state = Complete;
          self.ramp_down(ctx);
          ctx.add_points(game::points::EXL_COMPLETION);

          self.hit_effect.stop(ctx);
          self.hit_effect = LedScene::new(LedQ::Every, led_scene!("skyrail_station/complete.ron"));
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

        if self.ramp_hits == 3 {
          // on the final round all 3 targets must be hit with a 2 ball multiball
          self.eject_id = ctx.eject_balls(1, Launch::Auto);
          ctx.request_ball_save_until_released();
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
      _ => {}
    }

    self.progress_effect = Self::progress_effect(self.target_hits + self.ramp_hits);
  }

  fn ramp_up(&mut self, delay: Duration, ctx: &SystemContext) {
    if !self.ramp_up {
      if delay > Duration::ZERO {
        ctx.cues.start(RampUp, delay.once());
      } else {
        ctx.systems.expect::<LiftRampSystem>().lift_up(ctx.into());
        self.ramp_up = true;
      }
    }
  }

  fn ramp_down(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.systems.expect::<LiftRampSystem>().lift_down(ctx.into());
      self.ramp_up = false;
    }
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SkyrailStation, ctx.into());
    self.ramp_down(ctx);
    ctx.systems.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::SkyrailStation,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.systems.despawn_self();
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::SkyrailStation, ctx.into());
    // a city mode can only be completed once per game
    ctx.systems.despawn_self();
  }
}

impl System for SkyrailStationMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SkyrailStation)
  }

  fn on_spawn(&mut self, _ctx: &SystemContext) {
    log::info!("SkyrailStation mode started");
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LiftRampScoopBallEnter>() {
      ctx.add_points(500);
      ctx.systems.expect::<LiftRampSystem>().eject(ctx.into());
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
    } else if event.is::<RoutineStarted<TurnStarting>>() {
      self.revert_to_startable(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.systems.expect::<LiftRampSystem>().lift_up(ctx.into());
    }
    ctx.activate_led_declarations();
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.systems.expect::<LiftRampSystem>().lift_down(ctx.into());
    }
    ctx.deactivate_led_declarations();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);

    if self.state == Complete && self.hit_effect.is_complete() {
      self.complete(ctx);
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
  HitRamp,
  HitTarget,
  Final,
  Complete,
}

#[derive(serde::Serialize, Event)]
struct RampUp;
