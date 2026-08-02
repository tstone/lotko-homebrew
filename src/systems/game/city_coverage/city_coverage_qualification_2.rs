use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::*;
use crate::systems::game::CityCoverageQualification3;

#[derive(Clone)]
pub struct CityCoverageQualification2 {
  left_orbit_effect: Option<LedEffect>,
  center_orbit_effect: Option<LedEffect>,
  right_orbit_effect: Option<LedEffect>,
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
    }
  }

  fn create_led_effect(query: HardwareQuery, hit: bool) -> Option<LedEffect> {
    if hit {
      Some(
        LedEffect::initial(
          query,
          ColorSequence::exact(vec![Rgba::white(), Rgba::default(), Rgba::default()]),
        )
        .rotating(Duration::from_millis(750), Curve::Linear),
      )
    } else {
      None
    }
  }

  fn is_complete(&self) -> bool {
    self.left_orbit_effect.is_none()
      && self.center_orbit_effect.is_none()
      && self.right_orbit_effect.is_none()
  }

  fn attempt_complete(&mut self, ctx: &Context) {
    if self.is_complete() {
      ctx.add_points(20000);
      ctx.replace_self(CityCoverageQualification3::new());
    }
  }
}

impl System for CityCoverageQualification2 {
  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    if let Some(effect) = self.left_orbit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
    if let Some(effect) = self.center_orbit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
    if let Some(effect) = self.right_orbit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if event.is::<left_orbit::LeftOrbitHit>() && self.left_orbit_effect.is_none() {
      ctx.add_points(10000);
      self.attempt_complete(ctx);
    } else if event.is::<center_orbit::CenterOrbitHit>() && self.center_orbit_effect.is_none() {
      ctx.add_points(10000);
      self.attempt_complete(ctx);
    } else if event.is::<right_orbit::RightOrbitHit>() && self.right_orbit_effect.is_none() {
      ctx.add_points(10000);
      self.attempt_complete(ctx);
    }
  }
}
