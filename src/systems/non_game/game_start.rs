use frontbox::animation::*;
use frontbox::prelude::color_sequence::*;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::cabinet::*;
use crate::hardware::slingshots::*;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .effect(
      LedEffect::new(
        action_button::LED.q(),
        ColorSequence::solid(Rgba::blue().lighten(0.35)),
      )
      .animate(
        |seq, new_color| seq.fill.recolor(new_color),
        Tween::forever(
          Duration::from_secs(2),
          Curve::Linear,
          vec![Rgba::blue().lighten(0.35), Rgba::purple().desaturate(0.25)],
        ),
      ),
    )
    // animate GI posts
    .effect(
      LedEffect::rotate(
        POST_LEDS1.q(),
        ColorSequence::fade(Rgba::cyan(), Rgba::pink()),
        Duration::from_millis(750),
        RotationDirection::CounterClockwise,
      )
      .cycle(
        vec![ColorSequence::off()],
        Duration::from_millis(1500),
        Curve::EaseInOut,
      ),
    )
    .effect(
      LedEffect::rotate(
        POST_LEDS4.q(),
        ColorSequence::fade(Rgba::cyan(), Rgba::pink()),
        Duration::from_millis(750),
        RotationDirection::Clockwise,
      )
      .cycle(
        vec![ColorSequence::off()],
        Duration::from_millis(1500),
        Curve::EaseInOut,
      ),
    )
}
