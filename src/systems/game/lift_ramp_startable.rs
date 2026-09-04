use std::collections::VecDeque;

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
use crate::systems::game::ExclusiveMode;
use crate::systems::game::ExclusiveModeEnded;
use crate::systems::game::ModeManager;
use crate::systems::game::StartableEffects;
use crate::systems::game::lift_ramp_startable::State::*;

#[derive(Clone)]
pub struct LiftRampStartable {
  startable_modes: Vec<ExclusiveMode>,
  effects: Option<StartableEffects>,
  state: State,
  ramp_up_duration: Duration,
  ramp_downs: u8,
  cue_id: Option<u64>,
  // if a current mode is already set, keep track of subsequent startable modes
  additional_modes: VecDeque<ExclusiveMode>,
  current_mode: Option<ExclusiveMode>,
  handle: SystemHandle,
}

impl Default for LiftRampStartable {
  fn default() -> Self {
    Self::new()
  }
}

impl LiftRampStartable {
  pub fn new() -> Self {
    Self {
      startable_modes: Vec::new(),
      effects: None,
      state: Pending,
      ramp_up_duration: Duration::from_secs(20),
      ramp_downs: 0,
      cue_id: None,
      additional_modes: VecDeque::new(),
      current_mode: None,
      handle: SystemHandle::default(),
    }
  }

  fn begin(&mut self, ctx: &SystemContext) {
    self.state = Startable;
    self.ramp_up(ctx);
  }

  pub fn make_startable(
    &mut self,
    mode: ExclusiveMode,
    activation_delay: Duration,
    ctx: &ServiceContext,
  ) {
    if self.current_mode.is_none() {
      self.current_mode = Some(mode);
      self.state = Startable;
      self.ramp_up(&ctx.for_system(self.handle));

      if activation_delay > Duration::ZERO {
        self.state = Pending;
        ctx
          .for_system(self.handle)
          .cue(Resume, Cue::Once(activation_delay));
      }
    } else {
      self.additional_modes.push_back(mode);
    }
  }

  fn advance_mode(&mut self) {
    self.current_mode = None;
    let next_mode = self.additional_modes.pop_front();

    if let Some(next_mode) = next_mode {
      self.current_mode = Some(next_mode);
    }
  }

  fn queue_start(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Some(current) = self.current_mode
      && let Ok(..) = ctx.expect::<ModeManager>().take_exclusive(current, ctx)
    {
      if let Some(effects) = self.effects.as_mut() {
        effects.hit_effect.play();
      }

      let mut lift_ramp = ctx.expect::<LiftRampSystem>();
      lift_ramp.lift_down(ctx.into());
      lift_ramp.eject(ctx.into());

      ctx.play_sfx(current.start_sound());
      ctx.add_points(game::points::EXCL_START);
    }
  }

  fn ramp_up(&mut self, ctx: &SystemContext) {
    self.state = Startable;

    ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
    self.cue_id = Some(ctx.cue(TimeOut, Cue::Once(self.ramp_up_duration)));

    if let Some(effects) = self.effects.as_mut()
      && let Some(mode) = self.current_mode.as_ref()
    {
      effects.attention_effect.stop(ctx);
      effects.attention_effect = Self::attention_effect_ramp_up(mode);

      effects.hit_effect.stop(ctx);
      effects.hit_effect = Self::hit_effect_ramp_up(mode);
    }
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

    if let Some(effects) = self.effects.as_mut()
      && let Some(mode) = self.current_mode.as_ref()
    {
      effects.attention_effect.stop(ctx);
      effects.attention_effect = Self::attention_effect_ramp_down(mode);

      effects.hit_effect.stop(ctx);
      effects.hit_effect = Self::hit_effect_ramp_down(mode);
    }
  }

  fn attention_effect_ramp_up(mode: &ExclusiveMode) -> LedProgram1d {
    LedProgram1d::flash(lift_ramp::BOLT_LED.q(), mode.color().into(), Cycle::Forever)
  }

  fn attention_effect_ramp_down(mode: &ExclusiveMode) -> LedProgram1d {
    LedProgram1d::flash(
      &*lift_ramp::HEX_CENTER_LED,
      mode.color().into(),
      Cycle::Forever,
    )
  }

  fn hit_effect_ramp_up(mode: &ExclusiveMode) -> LedProgram1d {
    LedProgram1d::tween(
      LedQ::tag::<tags::Playfield>().at_z(-1),
      Duration::from_millis(750),
      Curve::EaseIn,
      Cycle::Once,
      vec![
        ColorSequence::solid(mode.color()),
        ColorSequence::solid(Rgba::default()),
      ],
    )
    .stopped()
  }

  fn hit_effect_ramp_down(mode: &ExclusiveMode) -> LedProgram1d {
    LedProgram1d::tween(
      lift_ramp::HEX_LEDS.q().at_z(-1),
      Duration::from_millis(750),
      Curve::EaseIn,
      Cycle::Once,
      vec![
        ColorSequence::solid(mode.color()),
        ColorSequence::solid(Rgba::default()),
      ],
    )
  }
}

impl System for LiftRampStartable {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    if self.state == Inactive {
      return false;
    }

    let mode_manager = ctx.expect::<ModeManager>();
    let mode = mode_manager.current_mode();

    if let Some(current) = self.current_mode {
      mode.is_none() || *mode == Some(current)
    } else {
      mode.is_none()
    }
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
    self.handle = *ctx.current_handle();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(effects) = self.effects.as_mut() {
      effects.attention_effect.apply(delta, ctx);
      effects.hit_effect.apply(delta, ctx);

      if effects.hit_effect.is_complete()
        && let Some(current) = self.current_mode
      {
        effects.attention_effect.stop(ctx);
        effects.hit_effect.stop(ctx);
        self.advance_mode();
        current.start(ctx);
        self.state = Inactive;
      }
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
    } else if event.is::<ExclusiveModeEnded>() {
      self.state = Startable;
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Pending,
  Startable,
  RampDown,
  Inactive,
}

#[derive(serde::Serialize, Event)]
struct Start;

#[derive(serde::Serialize, Event)]
struct Resume;

#[derive(serde::Serialize, Event)]
struct TimeOut;
