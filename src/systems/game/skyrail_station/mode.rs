use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox::provided::MultiballExt;
use frontbox_turn_based::{GameManagementExt, PlayerTurnActive};

use crate::hardware::drop_bank::{self, DropBankSystem, DropBankTargetHit};
use crate::hardware::flashers::{self, FlashersSystem};
use crate::hardware::lift_ramp::{LiftRampHit, LiftRampScoopBallEnter, LiftRampSystem};
use crate::hardware::{backbox, city_map, lift_ramp};
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::skyrail_station::mode::State::*;
use crate::systems::game::{
  self, ExclusiveMode, LiftRampStartable, ModeManager, SkyrailStationQualification, points,
};

pub struct SkyrailStationMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  intensity_effect: LedProgram1d,
  progress_effect: LedProgram1d,
  target_hits: u8,
  ramp_hits: u8,
  state: State,
  ramp_up: bool,
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
    }
  }

  fn attention_effect_ramp() -> LedProgram1d {
    LedProgram1d::pulse(
      &*lift_ramp::HEX_CENTER_LED,
      *MODE_COLOR,
      Duration::bpm(128),
      Cycle::Forever,
    )
  }

  fn attention_effect_target() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::flash(
        LedQ::any(vec![
          &drop_bank::TARGET1_LEDS.q().skip(3).take(1),
          &drop_bank::TARGET2_LEDS.q().skip(3).take(1),
          &drop_bank::TARGET3_LEDS.q().skip(3).take(1),
        ]),
        (*MODE_COLOR).into(),
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*lift_ramp::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
    ])
  }

  fn attention_effect_target_final() -> LedProgram1d {
    LedProgram1d::multi(vec![
      LedProgram1d::rotating(
        drop_bank::TARGET1_LEDS.q(),
        ColorSequence::exact(vec![
          *MODE_COLOR,
          Rgba::default(),
          Rgba::default(),
          Rgba::default(),
        ]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        drop_bank::TARGET2_LEDS.q(),
        ColorSequence::exact(vec![
          Rgba::default(),
          *MODE_COLOR,
          Rgba::default(),
          Rgba::default(),
        ]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        drop_bank::TARGET3_LEDS.q(),
        ColorSequence::exact(vec![
          Rgba::default(),
          Rgba::default(),
          *MODE_COLOR,
          Rgba::default(),
        ]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
      LedProgram1d::rotating(
        &*lift_ramp::HEX_LINE_LEDS,
        ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
        Duration::from_millis(250),
        Curve::Linear,
        Cycle::Forever,
      ),
    ])
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::tween(
      LedQ::tag::<Playfield>().at_z(-1),
      Duration::from_millis(600),
      Curve::ExponentialOut,
      Cycle::Once,
      vec![
        ColorSequence::fade(*MODE_COLOR, Rgba::default()).shuffle(rand::random()),
        ColorSequence::solid(Rgba::default()),
      ],
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
        .padding_right(Extent::Relative(1.0 - (hits as f32 / 8 as f32))),
    )
  }

  fn advance(&mut self, ctx: &SystemContext) {
    ctx.add_points(points::EXL_MODE_HIT);
    self.hit_effect.play();
    ctx
      .expect::<FlashersSystem>()
      .flash(3, ColorSequence::tile(vec![*MODE_COLOR, Rgba::default()]));

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
          self.hit_effect = LedProgram1d::timeline()
            .at(
              Duration::ZERO,
              LedProgram1d::rotating(
                LedQ::Every,
                ColorSequence::fade(*MODE_COLOR, MODE_COLOR.lighten(0.5)),
                Duration::from_millis(1200),
                Curve::Linear,
                Cycle::Times(3),
              ),
            )
            .at(
              Duration::from_millis(3600),
              LedProgram1d::tween(
                LedQ::Every,
                Duration::from_millis(1250),
                Curve::EaseIn,
                Cycle::Once,
                vec![
                  ColorSequence::fade(*MODE_COLOR, MODE_COLOR.lighten(0.5)),
                  ColorSequence::solid(Rgba::default()),
                ],
              ),
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

        if self.ramp_hits == 3 {
          // on the final round all 3 targets must be hit with a 2 ball multiball
          ctx.multiball_add_balls(1);
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

        ctx.expect::<DropBankSystem>().raise_targets(ctx.into());
      }
      _ => {}
    }

    self.progress_effect = Self::progress_effect(self.target_hits + self.ramp_hits);
  }

  fn ramp_up(&mut self, delay: Duration, ctx: &SystemContext) {
    if !self.ramp_up {
      if delay > Duration::ZERO {
        ctx.cue(RampUp, delay.once());
      } else {
        ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
        self.ramp_up = true;
      }
    }
  }

  fn ramp_down(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_down(ctx.into());
      self.ramp_up = false;
    }
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SkyrailStation, ctx.into());
    self.ramp_down(ctx);
    ctx.expect::<LiftRampStartable>().make_startable(
      ExclusiveMode::SkyrailStation,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.despawn_self();
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::SkyrailStation, ctx.into());
    ctx.replace_self(SkyrailStationQualification::new());
  }
}

impl System for SkyrailStationMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SkyrailStation)
  }

  fn on_spawn(&mut self, _ctx: &SystemContext) {
    log::info!("SkyrailStation mode started");
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LiftRampScoopBallEnter>() {
      ctx.add_points(500);
      ctx.expect::<LiftRampSystem>().eject(ctx.into());
    } else if event.is::<RampUp>() {
      self.ramp_up(Duration::ZERO, ctx);
    } else if event.is::<LiftRampHit>() && self.state == HitRamp {
      self.advance(ctx);
    } else if event.is::<DropBankTargetHit>() && (self.state == HitTarget || self.state == Final) {
      self.advance(ctx);
    } else if event.is::<PlayerTurnActive>() {
      self.revert_to_startable(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
    }
    ctx.activate_led_declarations();
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if self.ramp_up {
      ctx.expect::<LiftRampSystem>().lift_down(ctx.into());
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
