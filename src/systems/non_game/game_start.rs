use frontbox_core::prelude::*;
use frontbox_gfx::animation::*;
use frontbox_gfx::color_sequence::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;

use crate::hardware::cabinet::*;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .effect(LedProgram1d::tween(
      action_button::LED.q(),
      Duration::from_secs(2),
      Curve::Linear,
      Cycle::Forever,
      vec![
        ColorSequence::solid(Rgba::blue().lighten(0.35)),
        ColorSequence::solid(Rgba::purple().desaturate(0.25)),
      ],
      EndBehavior::Clear,
    ))
}
