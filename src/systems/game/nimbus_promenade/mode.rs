use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox::provided::{MultiballExt, MultiballSystem};
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::{GameManagementExt, GameManager, TurnState};

use crate::hardware::pop_cluster::{self, PopBumper};
use crate::hardware::vspinner;
use crate::systems::game::nimbus_promenade::MODE_COLOR;
use crate::systems::game::{ModeManager, NimbusPromenadeQualification, NonExclusiveMode};
use crate::systems::sounds;

pub struct NimbusPromenadeMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  cycle_time: Duration,
  current_pop: PopBumper,
  hits: u8,
  complete: bool,
  cue_id: Option<u64>,
}

impl NimbusPromenadeMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(&PopBumper::Left, 0),
      cycle_time: Duration::from_millis(1500),
      hit_effect: Self::hit_effect(),
      current_pop: PopBumper::Left,
      hits: 0,
      complete: false,
      cue_id: None,
    }
  }

  fn on_pop_hit(&mut self, pop: &PopBumper, ctx: &SystemContext) {
    if self.cycle_time > Duration::from_millis(600) {
      self.cycle_time -= Duration::from_millis(200);
    }

    if *pop == self.current_pop {
      ctx.add_points(50_000);

      self.hits += 1;
      self.hit_effect.reset();
      self.hit_effect.play();
      self.advance(ctx);

      // check for completion
      let required_hits = match self.current_pop {
        PopBumper::Left => 4,
        _ => 3,
      };

      if self.hits == required_hits {
        self.complete(ctx);
        ctx
          .expect::<ModeManager>()
          .complete_non_exclusive(NonExclusiveMode::NimbusPromenade, ctx);
      }
    }
  }

  fn clear_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.cue_id.as_mut() {
      ctx.cancel_cue(*cue_id);
      self.cue_id = None;
    }
  }

  fn advance(&mut self, ctx: &SystemContext) {
    self.clear_cue(ctx);

    if !self.complete {
      self.cue_id = Some(ctx.cue(Next, Cue::Once(self.cycle_time)));

      self.current_pop = self.current_pop.next();
      self.attention_effect.stop(ctx);
      self.attention_effect = Self::attention_effect(&self.current_pop, self.hits);
    }
  }

  fn complete(&mut self, ctx: &SystemContext) {
    // play sfx
    ctx.add_points(15_000_000);
    ctx.play_sfx(sounds::LANE_HIT_COMPLETE);

    self.hit_effect.stop(ctx);
    self.hit_effect = LedProgram1d::flash(
      LedQ::Every,
      ColorSequence::fade(*MODE_COLOR, Rgba::red()),
      Cycle::Times(2),
    );

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
      Duration::from_millis(450),
      Curve::EaseOut,
      Cycle::Once,
    )
    .stopped()
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
    self.cue_id = Some(ctx.cue(Next, Cue::Once(Duration::from_millis(1750))));
    ctx
      .expect::<ModeManager>()
      .non_exclusive_active(NonExclusiveMode::NimbusPromenade, ctx);
    ctx.multiball_add_balls(1);
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

    if self.hit_effect.is_complete() && self.complete {
      ctx.replace_self(NimbusPromenadeQualification::new());
    }
    self.hit_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct Next;
