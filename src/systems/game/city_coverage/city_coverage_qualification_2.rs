use frontbox::animation::{Accumulator, Curve};
use frontbox::prelude::*;
use frontbox_sound::*;
use frontbox_turn_based::*;

use crate::hardware::*;
use crate::systems::game::CityCoverageQualification3;
use crate::systems::sounds;

#[derive(Clone)]
pub struct CityCoverageQualification2 {
  left_orbit_effect: Option<LedEffect>,
  center_orbit_effect: Option<LedEffect>,
  right_orbit_effect: Option<LedEffect>,
  shot_hit_effect: LedEffect,
}

impl CityCoverageQualification2 {
  pub fn new(left_orbit_hit: bool, center_orbit_hit: bool, right_orbit_hit: bool) -> Self {
    Self {
      left_orbit_effect: Self::create_led_effect(left_orbit::hex_line_leds_q(), left_orbit_hit),
      center_orbit_effect: Self::create_led_effect(
        center_orbit::hex_line_leds_q(),
        center_orbit_hit,
      ),
      right_orbit_effect: Self::create_led_effect(right_orbit::hex_line_leds_q(), right_orbit_hit),
      shot_hit_effect: LedEffect::cycle(
        Q::tag::<more_tags::Circle>(),
        Duration::from_millis(90),
        Curve::Steps(3),
        Cycle::Times(3),
        vec![
          ColorSequence::fade(Rgba::white(), Rgba::black()),
          ColorSequence::fade(Rgba::black(), Rgba::white()),
        ],
      )
      .shuffled(rand::random())
      .rotating(Duration::from_millis(30), Curve::Linear, Cycle::Forever)
      .stopped(),
    }
  }

  fn create_led_effect(query: HardwareQuery, hit: bool) -> Option<LedEffect> {
    if !hit {
      Some(
        LedEffect::initial(
          query,
          ColorSequence::exact(vec![Rgba::white(), Rgba::default(), Rgba::default()]),
        )
        .rotating(Duration::from_millis(520), Curve::Linear, Cycle::Forever),
      )
    } else {
      None
    }
  }

  fn is_complete(&self) -> bool {
    self.left_orbit_effect.is_none()
      && self.center_orbit_effect.is_none()
      && self.right_orbit_effect.is_none()
      // let animation play out before swapping modes
      && self.shot_hit_effect.is_complete()
  }

  fn attempt_complete(&mut self, ctx: &SystemContext) {
    if self.is_complete() {
      ctx.add_points(20000);
      ctx.replace_self(CityCoverageQualification3::new());
    }
  }
}

impl System for CityCoverageQualification2 {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .get::<GameManager>()
      .map(|game| game.turn_state() == Some(&TurnState::Active))
      .unwrap_or(false)
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(effect) = self.left_orbit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
    if let Some(effect) = self.center_orbit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
    if let Some(effect) = self.right_orbit_effect.as_mut() {
      effect.apply(delta, ctx);
    }

    self.shot_hit_effect.apply(delta, ctx);
    self.attempt_complete(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<left_orbit::LeftOrbitHit>() && self.left_orbit_effect.is_some() {
      log::info!("Coverage Qualification: Left orbit hit");
      self.shot_hit_effect.play();
      ctx.play_sfx(sounds::rnd_lane_hit());
      ctx.add_points(10000);
      self.left_orbit_effect.as_mut().unwrap().stop(ctx);
      self.left_orbit_effect = None;
      self.attempt_complete(ctx);
    } else if event.is::<center_orbit::CenterOrbitHit>() && self.center_orbit_effect.is_some() {
      log::info!("Coverage Qualification: Center orbit hit");
      self.shot_hit_effect.play();
      ctx.play_sfx(sounds::rnd_lane_hit());
      ctx.add_points(10000);
      self.center_orbit_effect.as_mut().unwrap().stop(ctx);
      self.center_orbit_effect = None;
      self.attempt_complete(ctx);
    } else if event.is::<right_orbit::RightOrbitHit>() && self.right_orbit_effect.is_some() {
      log::info!("Coverage Qualification: Right orbit hit");
      self.shot_hit_effect.play();
      ctx.play_sfx(sounds::rnd_lane_hit());
      ctx.add_points(10000);
      self.right_orbit_effect.as_mut().unwrap().stop(ctx);
      self.right_orbit_effect = None;
      self.attempt_complete(ctx);
    }
  }
}
