use std::collections::HashMap;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_turn_based::{GameManagementExt, GameManager, TurnState};

use crate::hardware::pop_cluster::{self, PopBumper};
use crate::hardware::vspinner;
use crate::systems::game::NimbusPromenadeQualification;
use crate::systems::game::nimbus_promenade::MODE_COLOR;

pub struct NimbusPromenadeMode {
  attention_effect: LedProgram1d,
  hit_effect: Option<LedProgram1d>,
  current_pop: PopBumper,
  hits: u8,
  complete: bool,
}

impl NimbusPromenadeMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(&PopBumper::Left, 0),
      hit_effect: None,
      current_pop: PopBumper::Left,
      hits: 0,
      complete: false,
    }
  }

  fn on_pop_hit(&mut self, pop: &PopBumper, ctx: &SystemContext) {
    if *pop == self.current_pop {
      ctx.add_points(15_000);
      self.hits += 1;
      self.hit_effect = Some(Self::hit_effect());
      self.advance(ctx);
    }

    let required_hits = match self.current_pop {
      PopBumper::Left => 4,
      _ => 3,
    };
    if self.hits == required_hits {
      self.complete(ctx);
    }
  }

  fn advance(&mut self, ctx: &SystemContext) {
    self.current_pop = self.current_pop.next();
    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect(&self.current_pop, self.hits);
  }

  fn complete(&mut self, ctx: &SystemContext) {
    // play sfx
    ctx.add_points(5_000_000);
    self.complete = true;
  }

  fn attention_effect(pop: &PopBumper, hit_count: u8) -> LedProgram1d {
    LedProgram1d::pulse(
      LedQ::any(vec![
        &pop_cluster::led_for_pop(pop).q(),
        &pop_cluster::target_led_for_pop(pop).q(),
        &pop_cluster::led_ray_for_pop(pop)
          .clone()
          .take(hit_count as usize),
      ]),
      (*MODE_COLOR).into(),
      Duration::bpm(83),
      Cycle::Forever,
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      LedQ::any(vec![
        &pop_cluster::left::TARGET_LED.q(),
        &pop_cluster::upper_right::TARGET_LED.q(),
        &pop_cluster::lower_right::TARGET_LED.q(),
        &vspinner::left_ray::Q,
        &vspinner::upper_right_ray::Q,
        &vspinner::lower_right_ray::Q,
      ]),
      ColorSequence::fade(*MODE_COLOR, Rgba::white()),
      Duration::from_millis(1250),
      Curve::BounceInOut,
      Cycle::Times(3),
    )
  }
}

impl System for NimbusPromenadeMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .expect::<GameManager>()
      .game_state()
      .map(|g| *g.current_player_turn_state() == TurnState::Active)
      .unwrap_or(false)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.cue(Next, Cue::Once(Duration::from_millis(1750)));
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Next>() {
      self.advance(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if let Some(pop) = pop_cluster::match_switch(&event.switch) {
        self.on_pop_hit(&pop, ctx);
      } else if let Some(pop) = pop_cluster::match_target_switch(&event.switch) {
        self.on_pop_hit(&pop, ctx);
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);

    if let Some(hit_effect) = self.hit_effect.as_mut() {
      hit_effect.apply(delta, ctx);

      if hit_effect.is_complete() {
        hit_effect.stop(ctx);
        self.hit_effect = None;

        if self.complete {
          ctx.replace_self(NimbusPromenadeQualification::new());
        }
      }
    }
  }
}

#[derive(serde::Serialize, Event)]
struct Next;
