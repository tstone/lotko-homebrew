use frontbox_core::prelude::LightQ;
use frontbox_pinball::*;

use crate::hardware::cabinet::*;
use crate::led_scenes;

pub fn credits() -> CreditsSystem {
  CreditsSystem::new(start_button::SWITCH.q()).start_lamp(LightQ::name(start_button::LAMP_DRIVER.name))
}

/// Animates the action button while no game is running
pub fn action_button_attract() -> AttractSystem {
  let scene = led_scenes::load!("game_start.scene").unwrap_or_else(|e| panic!("game_start.scene: {e}"));
  AttractSystem::new(vec![scene])
}
