use frontbox::animation::*;
use frontbox::prelude::color_sequence::*;
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
    // animate GI
    .effect(
      LedEffect::new(
        Q::any_of(vec![gi::LEFT_SLING.q(), gi::RIGHT_SLING.q()]),
        ColorSequence::pattern(vec![Rgba::purple(), Rgba::default()], Cycle::Forever)
          .modify(Modification::rotated(0.0)),
      )
      .animate(
        |seq, degree| {
          if let Some(rotation) = seq.modifications[0].rotation_mut() {
            *rotation = degree;
          }
        },
        Tween::forever(Duration::from_millis(200), Curve::Linear, vec![0.0, 360.0]),
      ),
    )
}
