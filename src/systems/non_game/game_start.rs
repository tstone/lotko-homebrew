use frontbox_pinball::*;

use crate::hardware::cabinet::*;

pub fn game_startable() -> GameStartable {
  GameStartable::new()
    .flash_lamp(start_button::LAMP_DRIVER.name)
    // animate action button
    .scene(scene!("game_start.scene"))
}
