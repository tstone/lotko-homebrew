use std::marker::PhantomData;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::GameManagementExt;
use frontbox_turn_based::PlayerTurnEnding;

use crate::hardware::lift_ramp;
use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::lift_ramp::LiftRampScoopBallEnter;
use crate::hardware::lift_ramp::LiftRampSystem;
use crate::systems::game;
use crate::systems::game::ExclusiveModeStarter;
use crate::systems::game::ModeManager;
use crate::systems::game::lift_ramp_startable::State::*;

#[derive(Clone)]
pub struct LiftRampStartable<T: ExclusiveModeStarter + Clone> {
  activation_delay: Duration,
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  state: State,
  ramp_up_duration: Duration,
  ramp_downs: u8,
  cue_id: Option<u64>,
  _t: PhantomData<T>,
}

impl<T> Default for LiftRampStartable<T>
where
  T: ExclusiveModeStarter + Clone,
{
  fn default() -> Self {
    Self::new(Duration::ZERO)
  }
}

impl<T> LiftRampStartable<T>
where
  T: ExclusiveModeStarter + Clone,
{
  pub fn new(activation_delay: Duration) -> Self {
    Self {
      activation_delay,
      attention_effect: Self::attention_effect_ramp_up(),
      hit_effect: Self::hit_effect_ramp_up(),
      state: Pending,
      ramp_up_duration: Duration::from_secs(20),
      ramp_downs: 0,
      cue_id: None,
      _t: PhantomData,
    }
  }

  fn begin(&mut self, ctx: &SystemContext) {
    self.state = Startable;
    self.ramp_up(ctx);
  }

  fn queue_start(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Ok(..) = ctx.expect::<ModeManager>().take_exclusive(T::MODE, ctx) {
      let mut lift_ramp = ctx.expect::<LiftRampSystem>();
      lift_ramp.lift_down(ctx.into());
      lift_ramp.eject(ctx.into());

      self.state = Shutdown;
      self.hit_effect.play();

      ctx.play_sfx(T::START_SND_KEY);
      ctx.add_points(game::points::EXCL_START);
    }
  }

  fn ramp_up(&mut self, ctx: &SystemContext) {
    self.state = Startable;

    ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
    self.cue_id = Some(ctx.cue(TimeOut, Cue::Once(self.ramp_up_duration)));

    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect_ramp_up();

    self.hit_effect.stop(ctx);
    self.hit_effect = Self::hit_effect_ramp_up();
  }

  fn ramp_down(&mut self, ctx: &SystemContext) {
    self.ramp_downs += 1;
    self.ramp_up_duration = self.ramp_up_duration + Duration::from_secs(5);
    self.state = RampDown;

    if let Some(cue_id) = self.cue_id {
      ctx.cancel_cue(cue_id);
      self.cue_id = None;
    }

    ctx.expect::<LiftRampSystem>().lift_down(ctx.into());

    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect_ramp_down();

    self.hit_effect.stop(ctx);
    self.hit_effect = Self::hit_effect_ramp_down();
  }

  fn attention_effect_ramp_up() -> LedProgram1d {
    LedProgram1d::flash(
      lift_ramp::BOLT_LED.q(),
      T::mode_color().into(),
      Cycle::Forever,
    )
  }

  fn attention_effect_ramp_down() -> LedProgram1d {
    LedProgram1d::flash(
      &*lift_ramp::HEX_CENTER_LED,
      T::mode_color().into(),
      Cycle::Forever,
    )
  }

  fn hit_effect_ramp_up() -> LedProgram1d {
    LedProgram1d::tween(
      LedQ::tag::<tags::Playfield>().at_z(-1),
      Duration::from_millis(750),
      Curve::EaseIn,
      Cycle::Once,
      vec![
        ColorSequence::solid(T::mode_color()),
        ColorSequence::solid(Rgba::default()),
      ],
    )
    .stopped()
  }

  fn hit_effect_ramp_down() -> LedProgram1d {
    LedProgram1d::tween(
      lift_ramp::HEX_LEDS.q().at_z(-1),
      Duration::from_millis(750),
      Curve::EaseIn,
      Cycle::Once,
      vec![
        ColorSequence::solid(T::mode_color()),
        ColorSequence::solid(Rgba::default()),
      ],
    )
  }
}

impl<T> System for LiftRampStartable<T>
where
  T: ExclusiveModeStarter + Clone,
{
  fn is_active(&self, ctx: &SystemContext) -> bool {
    let mode_manager = ctx.expect::<ModeManager>();
    let mode = mode_manager.current_mode();
    mode.is_none() || mode == &Some(T::MODE)
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    ctx.deactivate_led_declarations();
    if self.state == Startable {
      self.ramp_down(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    if self.state == Startable {
      self.ramp_up(ctx);
    }
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    if self.activation_delay == Duration::ZERO {
      self.begin(ctx);
    } else {
      ctx.cue(Start, Cue::Once(self.activation_delay));
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);

    if self.state == Shutdown && self.hit_effect.is_complete() {
      self.attention_effect.stop(ctx);
      T::on_start(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if self.state == Startable && event.is::<LiftRampScoopBallEnter>() {
      self.queue_start(ctx);
    } else if self.state == RampDown && event.is::<LiftRampHit>() {
      self.ramp_up(ctx);
    } else if event.is::<TimeOut>() || event.is::<PlayerTurnEnding>() {
      self.ramp_down(ctx);
    } else if event.is::<Start>() {
      self.begin(ctx);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Pending,
  Startable,
  RampDown,
  Shutdown,
}

#[derive(serde::Serialize, Event)]
struct Start;

#[derive(serde::Serialize, Event)]
struct TimeOut;
