use frontbox_pinball::*;

use crate::hardware::cabinet::*;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .effect(led_playback!("game_start.scene"))
}
