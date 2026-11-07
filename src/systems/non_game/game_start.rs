use frontbox_led::prelude::*;
use frontbox_pinball::*;

use crate::hardware::cabinet::*;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .effect(LedScene::new(
      action_button::LED.q(),
      led_scene!("game_start.ron"),
    ))
}
