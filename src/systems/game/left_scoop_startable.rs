use std::marker::PhantomData;

use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::GameManagementExt;

use crate::systems::game;
use crate::systems::game::ExclusiveMode;
use crate::systems::game::ModeManager;
use crate::systems::game::left_scoop_startable::State::*;

#[derive(Clone)]
pub struct LeftScoopStartable<T: ExclusiveModeStarter + Clone> {
  activation_delay: Duration,
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  state: State,
  _t: PhantomData<T>,
}

impl<T> LeftScoopStartable<T>
where
  T: ExclusiveModeStarter + Clone,
{
  pub fn new(activation_delay: Duration) -> Self {
    Self {
      activation_delay,
      attention_effect: T::attention_effect(),
      hit_effect: T::hit_effect(),
      state: Startable,
      _t: PhantomData,
    }
  }

  fn start(&mut self, ctx: &SystemContext) {
    // Ensure that exclusive mode rights can be taken
    if let Ok(..) = ctx.expect::<ModeManager>().take_exclusive(T::MODE, ctx) {
      self.state = Shutdown;
      self.hit_effect.play();

      ctx.play_sfx(T::START_SND_KEY);
      ctx.add_points(game::points::EXCL_START);
    }
  }
}

impl<T> System for LeftScoopStartable<T>
where
  T: ExclusiveModeStarter + Clone,
{
  fn is_active(&self, ctx: &SystemContext) -> bool {
    let mode_manager = ctx.expect::<ModeManager>();
    let mode = mode_manager.current_mode();
    mode.is_none() || mode == &Some(T::MODE)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    if self.activation_delay > Duration::ZERO {
      self.state = Pending;
      ctx.cue(Resume, Cue::Once(self.activation_delay));
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
    if self.state == Startable && T::is_startable_event(event) {
      self.start(ctx);
    } else if event.is::<Resume>() && self.state == Pending {
      self.state = Startable;
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Pending,
  Startable,
  Shutdown,
}

#[derive(serde::Serialize, Event)]
struct Resume;

pub trait ExclusiveModeStarter: Send + Sync + 'static {
  const START_SND_KEY: &'static str;
  const MODE: ExclusiveMode;

  fn is_startable_event(event: &dyn Event) -> bool;
  fn hit_effect() -> LedProgram1d;
  fn attention_effect() -> LedProgram1d;
  fn on_start(ctx: &SystemContext);
}
