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
  effects: Option<StartableEffects>,
  state: State,
  ramp_up_duration: Duration,
  ramp_downs: u8,
  cue_id: Option<u64>,
  // if a current mode is already set, keep track of subsequent startable modes
  additional_modes: VecDeque<ExclusiveMode>,
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
      effects: None,
      state: OpenForStarting,
      ramp_up_duration: Duration::from_secs(20),
      ramp_downs: 0,
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
        if activation_delay > Duration::ZERO {
          log::info!("LiftRampStartable: Scheduling ramp lift for {:?}", mode);
          self.state = WaitingForRampUp(mode);
          ctx
            .for_system(self.handle)
            .cue(RampUp, activation_delay.once());
        } else {
          log::info!("LiftRampStartable: Starting {:?}", mode);
          self.state = Startable(mode);
          self.ramp_up(&ctx.for_system(self.handle));
        }
      }
      _ => {
        log::info!("LiftRampStartable: Enqueuing {:?}", mode);
        self.additional_modes.push_back(mode);
      }
    }
  }

  fn advance_mode(&mut self) {
    if let Some(next_mode) = self.additional_modes.pop_front() {
      self.state = Startable(next_mode);
    } else {
      self.state = OpenForStarting;
    }
  }

  fn start(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Startable(mode) = self.state
      && let Ok(..) = ctx.expect::<ModeManager>().take_exclusive(mode, ctx)
    {
      self.state = Starting(mode);
      self.effects.as_mut().unwrap().hit_effect.play();

      let mut lift_ramp = ctx.expect::<LiftRampSystem>();
      lift_ramp.lift_down(ctx.into());
      lift_ramp.eject(ctx.into());

      ctx.play_sfx(mode.start_sound());
      ctx.add_points(game::points::EXCL_START);
    }
  }

  fn clear_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.cue_id {
      ctx.cancel_cue(cue_id);
      self.cue_id = None;
    }
  }

  fn ramp_up(&mut self, ctx: &SystemContext) {
    self.clear_cue(ctx);
    ctx.expect::<LiftRampSystem>().lift_up(ctx.into());
    self.cue_id = Some(ctx.cue(TimeOut, self.ramp_up_duration.once()));

    if let Some(effects) = self.effects.as_mut() {
      effects.attention_effect.stop(ctx);
      effects.hit_effect.stop(ctx);
    }

    if let Startable(mode) = self.state {
      self.effects = Some(StartableEffects {
        attention_effect: Self::attention_effect_ramp_up(&mode),
        hit_effect: Self::hit_effect_ramp_up(&mode),
      });
    } else {
      log::warn!("LiftRampStartable: lifting ramp up but mode is not startable so LEDs not set");
    }
  }

  fn ramp_down(&mut self, forced: bool, ctx: &SystemContext) {
    if !forced {
      self.ramp_downs += 1;
      self.ramp_up_duration = self.ramp_up_duration + Duration::from_secs(5);
    }

    self.state = match self.state {
      Startable(mode) => RampDown(mode),
      Starting(mode) => RampDown(mode),
      _ => OpenForStarting,
    };

    self.clear_cue(ctx);
    ctx.expect::<LiftRampSystem>().lift_down(ctx.into());

    if let Some(effects) = self.effects.as_mut() {
      effects.attention_effect.stop(ctx);
      effects.hit_effect.stop(ctx);
    }

    if !forced && let Startable(mode) = self.state {
      self.effects = Some(StartableEffects {
        attention_effect: Self::attention_effect_ramp_down(&mode),
        hit_effect: Self::hit_effect_ramp_down(&mode),
      });
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
    let mode_manager = ctx.expect::<ModeManager>();
    match (&self.state, mode_manager.current_mode()) {
      (Starting(startable_mode), Some(active_mode)) => active_mode == startable_mode,
      (_, Some(_)) => false,
      _ => true,
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    ctx.deactivate_led_declarations();
    if matches!(&self.state, Startable(_)) {
      self.ramp_down(true, ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    if let Startable(_) = self.state {
      self.ramp_up(ctx);
      log::info!("LiftRampStartable: Reactivate => ramp up");
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
    if event.is::<LiftRampScoopBallEnter>() {
      self.start(ctx);
    } else if event.is::<LiftRampHit>()
      && let RampDown(_) = self.state
    {
      log::info!("LiftRampStartable: Lift ramp hit => ramp up");
      self.ramp_up(ctx);
    } else if event.is::<TimeOut>() {
      self.ramp_down(false, ctx);
    } else if event.is::<PlayerTurnEnding>() {
      self.ramp_down(true, ctx);
    } else if event.is::<RampUp>()
      && let WaitingForRampUp(mode) = self.state
    {
      log::info!("LiftRampStartable: Starting {:?}", mode);
      self.state = Startable(mode);
      self.ramp_up(ctx);
    } else if event.is::<ExclusiveModeEnded>() {
      self.advance_mode();
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  /// no modes in queue
  OpenForStarting,
  /// mode will be startable but ramp needs to come upfirst
  WaitingForRampUp(ExclusiveMode),
  /// mode can be started (listening for ball in scoop)
  Startable(ExclusiveMode),
  /// ball landed in scoope, waiting for hit animation to finish
  Starting(ExclusiveMode),
  /// failed to start, hit ramp to open again
  RampDown(ExclusiveMode),
}

#[derive(serde::Serialize, Event)]
struct RampUp;

#[derive(serde::Serialize, Event)]
struct TimeOut;
