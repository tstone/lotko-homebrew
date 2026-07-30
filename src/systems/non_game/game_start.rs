use frontbox::animation::*;
use frontbox::prelude::color_sequence::*;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::cabinet::*;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .effect(LedEffect::cycle(
      action_button::LED.q(),
      Duration::from_secs(2),
      Curve::Linear,
      Cycle::Forever,
      vec![
        ColorSequence::solid(Rgba::blue().lighten(0.35)),
        ColorSequence::solid(Rgba::purple().desaturate(0.25)),
      ],
    ))
}
