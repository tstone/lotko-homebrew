use frontbox::prelude::*;
use frontbox_sound::*;
use frontbox_turn_based::*;

use crate::hardware::*;
use crate::systems::game::CityCoverageQualification2;
use crate::systems::sounds;

#[derive(Clone)]
pub struct CityCoverageQualification1;

impl CityCoverageQualification1 {
  pub fn new() -> Self {
    Self {}
  }
}

impl System for CityCoverageQualification1 {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.declare_leds(
      &left_orbit::hex_center_led_q(),
      ColorSequence::solid(Rgba::white()),
    );
    ctx.declare_leds(
      &center_orbit::hex_center_led_q(),
      ColorSequence::solid(Rgba::white()),
    );
    ctx.declare_leds(
      &right_orbit::hex_center_led_q(),
      ColorSequence::solid(Rgba::white()),
    );
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<left_orbit::LeftOrbitHit>()
      || event.is::<center_orbit::CenterOrbitHit>()
      || event.is::<right_orbit::RightOrbitHit>()
    {
      ctx.play_sfx(sounds::rnd_lane_hit());
      ctx.add_points(10000);
      ctx.replace_self(CityCoverageQualification2::new(
        event.is::<left_orbit::LeftOrbitHit>(),
        event.is::<center_orbit::CenterOrbitHit>(),
        event.is::<right_orbit::RightOrbitHit>(),
      ));
    }
  }
}
