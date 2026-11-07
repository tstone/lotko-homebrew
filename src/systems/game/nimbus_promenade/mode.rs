use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::pop_cluster::{self, PopBumper};
use crate::systems::game::{MULTIBALL_BALL_SAVE, ModeManager, NonExclusiveMode};
use crate::systems::sounds;

pub struct NimbusPromenadeMode {
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  cycle_time: Duration,
  current_pop: PopBumper,
  hits: u8,
  complete: bool,
  cue_id: Option<u64>,
  eject_id: Option<u64>,
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
      eject_id: None,
    }
  }

  fn on_pop_hit(&mut self, pop: &PopBumper, ctx: &SystemContext) {
    if self.cycle_time > Duration::from_millis(600) {
      self.cycle_time -= Duration::from_millis(350);
    }

    if *pop == self.current_pop {
      ctx.add_points(50_000);

      self.hits += 1;
      self.hit_effect.reset();
      self.hit_effect.play();
      self.advance(ctx);

      // check for completion
      let required_hits = match self.current_pop {
        PopBumper::Left => 8,
        _ => 6,
      };

      if self.hits == required_hits {
        self.complete(ctx);
        ctx
          .systems
          .expect::<ModeManager>()
          .complete_non_exclusive(NonExclusiveMode::NimbusPromenade, ctx.into());
      } else {
        // rises every other hit to spread the four pitches across the mode
        ctx.play_sfx(sounds::rising_organic_hit((self.hits + 1) / 2));
      }
    }
  }

  fn clear_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.cue_id.as_mut() {
      ctx.cues.cancel(*cue_id);
      self.cue_id = None;
    }
  }

  fn advance(&mut self, ctx: &SystemContext) {
    self.clear_cue(ctx);

    if !self.complete {
      self.cue_id = Some(ctx.cues.start(Next, Cue::Once(self.cycle_time)));

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
    self.hit_effect = led_playback!("nimbus_promenade/complete.scene");

    self.complete = true;
  }

  fn attention_effect(pop: &PopBumper, hit_count: u8) -> LedPlayback {
    led_playback!(
      "nimbus_promenade/attention.scene",
      LedQ::any(vec![
        &pop_cluster::led_for_pop(pop).q(),
        &pop_cluster::target_led_for_pop(pop).q(),
        &pop_cluster::led_ray_for_pop(pop)
          .clone()
          .take((hit_count / 2) as usize)
      ])
    )
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("nimbus_promenade/hit.scene").stopped()
  }
}

impl System for NimbusPromenadeMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.cue_id = Some(ctx.cues.start(Next, Cue::Once(Duration::from_millis(1750))));
    ctx
      .systems
      .expect::<ModeManager>()
      .non_exclusive_active(NonExclusiveMode::NimbusPromenade, ctx.into());
    self.eject_id = ctx.eject_balls(1, Launch::Auto);
    ctx.request_ball_save_until_released();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Next>() {
      self.advance(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the ball is in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
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
      // a city mode can only be completed once per game
      ctx.systems.despawn_self();
    }
    self.hit_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct Next;
