use frontbox::animation::*;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::cabinet::*;
use crate::hardware::gi;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .effect(
      LedEffect::new(
        action_button::LED.q(),
        Colors::pattern(vec![Rgba::alice_blue()]),
      )
      .animate(
        |pattern, new_color| pattern.seq[0] = new_color,
        Tween::forever(
          Duration::from_secs(2),
          Curve::Linear,
          vec![Rgba::alice_blue(), Rgba::rebecca_purple()],
        ),
      ),
    )
    // animate GI
    .effect(
      LedEffect::new(
        Q::any_of(vec![gi::LEFT_SLING.q(), gi::RIGHT_SLING.q()]),
        Colors::pattern(vec![Rgba::medium_purple(), Rgba::default()]),
      )
      .animate(
        |pattern, rot| pattern.rotation = rot,
        Tween::forever(Duration::from_millis(200), Curve::Linear, vec![0.0, 360.0]),
      ),
    )
}
