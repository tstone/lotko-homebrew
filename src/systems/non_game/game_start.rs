use frontbox_core::prelude::LightQ;
use frontbox_pinball::*;

use crate::hardware::cabinet::*;

pub fn credits() -> CreditsSystem {
  CreditsSystem::new(start_button::SWITCH.q()).start_lamp(LightQ::name(start_button::LAMP_DRIVER.name))
}
