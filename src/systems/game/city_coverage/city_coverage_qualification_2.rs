use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_sound::*;
use frontbox_turn_based::*;
use rand::prelude::RngExt;

use crate::hardware::*;
use crate::systems::game::CityCoverageQualification3;
use crate::systems::sounds;

#[derive(Clone)]
pub struct CityCoverageQualification2 {
  left_orbit_effect: Option<LedProgram1d>,
  center_orbit_effect: Option<LedProgram1d>,
  right_orbit_effect: Option<LedProgram1d>,
  shot_hit_effect: LedProgram1d,
}

impl CityCoverageQualification2 {
  pub fn new(left_orbit_hit: bool, center_orbit_hit: bool, right_orbit_hit: bool) -> Self {
    Self {
      left_orbit_effect: Self::hex_led_program(left_orbit::hex_line_leds_q(), left_orbit_hit),
      center_orbit_effect: Self::hex_led_program(center_orbit::hex_line_leds_q(), center_orbit_hit),
      right_orbit_effect: Self::hex_led_program(right_orbit::hex_line_leds_q(), right_orbit_hit),
      shot_hit_effect: LedProgram1d::rotating(
        Q::tag::<more_tags::Circle>(),
        ColorSequence::tile(vec![Rgba::white(), Rgba::black()]),
        Duration::from_millis(90),
        Curve::Steps(3),
        Cycle::Times(3),
      ),
    }
  }

  /// Creates a new Qualification2 instance with 1 shot randomly completed
  pub fn new_rnd() -> Self {
    let mut completed = vec![false; 3];
    completed[rand::rng().random_range(0..3)] = true;
    Self::new(completed[0], completed[1], completed[2])
  }

  fn hex_led_program(query: HardwareQuery, hit: bool) -> Option<LedProgram1d> {
    if !hit {
      Some(
        LedProgram1d::rotating(
          query,
          ColorSequence::exact(vec![Rgba::white(), Rgba::default(), Rgba::default()]),
          Duration::from_millis(520),
          Curve::Linear,
          Cycle::Forever,
        )
        .playing(),
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
