use std::collections::VecDeque;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::GameManagementExt;

use crate::hardware::arc_ramp;
use crate::hardware::lower_scoop;
use crate::hardware::lower_scoop::LowerScoopBallEnter;
use crate::systems::game;
use crate::systems::game::ExclusiveMode;
use crate::systems::game::ExclusiveModeEnded;
use crate::systems::game::ModeManager;
use crate::systems::game::left_scoop_startable::State::*;

#[derive(Clone)]
pub struct LeftScoopStartable {
  effects: Option<StartableEffects>,
  state: State,
  // if a current mode is already set, keep track of subsequent startable modes
  additional_modes: VecDeque<ExclusiveMode>,
  current_mode: Option<ExclusiveMode>,
  handle: SystemHandle,
}

impl LeftScoopStartable {
  pub fn new() -> Self {
    Self {
      effects: None,
      state: Startable,
      additional_modes: VecDeque::new(),
      current_mode: None,
      handle: SystemHandle::default(),
    }
  }

  pub fn make_startable(
    &mut self,
    mode: ExclusiveMode,
    activation_delay: Duration,
    ctx: &ServiceContext,
  ) {
    if self.current_mode.is_none() {
      self.set_mode(mode);
      self.state = Startable;

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

  fn set_mode(&mut self, mode: ExclusiveMode) {
    self.current_mode = Some(mode);
    self.effects = Some(StartableEffects {
      hit_effect: Self::hit_effect(&mode),
      attention_effect: Self::attention_effect(&mode),
    });
  }

  fn advance_mode(&mut self) {
    self.current_mode = None;
    let next_mode = self.additional_modes.pop_front();

    if let Some(next_mode) = next_mode {
      self.set_mode(next_mode);
    }
  }

  fn start_mode(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Some(current) = self.current_mode
      && let Ok(..) = ctx.expect::<ModeManager>().take_exclusive(current, ctx)
    {
      if let Some(effects) = self.effects.as_mut() {
        effects.hit_effect.play();
      }

      ctx.play_sfx(current.start_sound());
      ctx.add_points(game::points::EXCL_START);
    }
  }

  fn attention_effect(mode: &ExclusiveMode) -> LedProgram1d {
    LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::flash(
          &*lower_scoop::BOLTS_Q,
          ColorSequence::solid(mode.color()),
          Cycle::Forever,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::flash(
          &*arc_ramp::HEX_CENTER_LED,
          ColorSequence::solid(mode.color()),
          Cycle::Forever,
        ),
      )
      .at(Duration::ZERO, arc_ramp::into_subway_program(mode.color()))
  }

  fn hit_effect(mode: &ExclusiveMode) -> LedProgram1d {
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
}

impl System for LeftScoopStartable {
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
    if self.state == Startable && event.is::<LowerScoopBallEnter>() {
      self.start_mode(ctx);
    } else if event.is::<Resume>() && self.state == Pending {
      self.state = Startable;
    } else if event.is::<ExclusiveModeEnded>() {
      self.state = Startable;
    }
  }
}

#[derive(Clone)]
pub struct StartableEffects {
  pub attention_effect: LedProgram1d,
  pub hit_effect: LedProgram1d,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Pending,
  Startable,
  Inactive,
}

#[derive(serde::Serialize, Event)]
struct Resume;
