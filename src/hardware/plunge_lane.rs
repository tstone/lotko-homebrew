use frontbox::prelude::*;
use frontbox::provided::{AutoPlungerSystem, PlungeLaneSystem};
use frontbox::tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = AutoPlungerSystem::coil_definition("plunge_coil");
  pub SWITCH: SwitchDefinition = PlungeLaneSystem::switch_definition("plunge_lane_sw");

  pub LED_STRIP: LedDefinition = LedDefinition::multi("plunge", 4)
    .tag(Playfield);
}
