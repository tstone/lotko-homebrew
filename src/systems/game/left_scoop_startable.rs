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
use crate::systems::game::ModeManager;
use crate::systems::game::left_scoop_startable::State::*;

#[derive(Clone)]
pub struct LeftScoopStartable {
  effects: Option<StartableEffects>,
  state: State,
  cue_id: Option<u64>,
  // if a current mode is already set, keep track of subsequent startable modes
  additional_modes: VecDeque<ExclusiveMode>,
  handle: SystemHandle,
}

impl LeftScoopStartable {
  pub fn new() -> Self {
    Self {
      effects: None,
      state: OpenForStarting,
      cue_id: None,
      additional_modes: VecDeque::new(),
      handle: SystemHandle::default(),
    }
  }

  pub fn make_startable(
    &mut self,
    mode: ExclusiveMode,
    activation_delay: Duration,
    ctx: &ServiceContext,
  ) {
    match self.state {
      OpenForStarting => {
        log::info!("LeftScoopStartable: Scheduling startable for {:?}", mode);
        self.state = Pending(mode);
        // start effects now do it doesn't seem like a weird delay
        self.effects = Some(StartableEffects {
          hit_effect: Self::hit_effect(&mode),
          attention_effect: Self::attention_effect(&mode),
        });
        self.cue_id = Some(
          ctx
            .for_system(self.handle)
            .cue(BecomeStartable, activation_delay.once()),
        );
      }
      _ => {
        log::info!("LeftScoopStartable: Enqueuing {:?}", mode);
        self.additional_modes.push_back(mode);
      }
    }
  }

  fn transition_to_startable(&mut self, mode: ExclusiveMode) {
    self.state = Startable(mode);

    if self.effects.is_none() {
      self.effects = Some(StartableEffects {
        hit_effect: Self::hit_effect(&mode),
        attention_effect: Self::attention_effect(&mode),
      });
    }
  }

  fn advance_mode(&mut self) {
    if let Some(mode) = self.additional_modes.pop_front() {
      self.transition_to_startable(mode);
    } else {
      self.state = OpenForStarting;
    }
  }

  fn start(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Startable(mode) = self.state
      && let Ok(..) = ctx.expect::<ModeManager>().take_exclusive(mode, ctx)
    {
      log::info!("LiftRampStartable: Starting mode {:?}", mode);
      self.state = Starting(mode);
      self.effects.as_mut().unwrap().hit_effect.play();

      ctx.play_sfx(mode.start_sound());
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
    let mode_manager = ctx.expect::<ModeManager>();
    match (&self.state, mode_manager.current_mode()) {
      (Starting(startable_mode), Some(active_mode)) => active_mode == startable_mode,
      (_, Some(_)) => false,
      _ => true,
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
        && let Starting(mode) = self.state
      {
        effects.attention_effect.stop(ctx);
        effects.hit_effect.stop(ctx);
        mode.start(ctx);
        self.advance_mode();
      }
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if matches!(self.state, Startable(_)) && event.is::<LowerScoopBallEnter>() {
      self.start(ctx);
    } else if event.is::<BecomeStartable>()
      && let Pending(mode) = self.state
    {
      self.transition_to_startable(mode);
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
  OpenForStarting,
  /// waiting for the activation delay to transition into startable
  Pending(ExclusiveMode),
  /// mode can be started (listening for ball in scoop)
  Startable(ExclusiveMode),
  /// ball landed in scoop, waiting for hit animation to finish
  Starting(ExclusiveMode),
}

#[derive(serde::Serialize, Event)]
struct BecomeStartable;
