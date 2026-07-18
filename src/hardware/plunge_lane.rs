use frontbox::prelude::*;
use frontbox::provided::AutoPlunger;
use frontbox::tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = AutoPlunger::coil_definition("plunge_coil");
  pub SWITCH: SwitchDefinition = AutoPlunger::switch_definition("plunge_lane_sw");

  pub LED_STRIP: LedDefinition = LedDefinition::multi("plunge", 4)
    .tag(Playfield);
}

pub fn system() -> AutoPlunger {
  AutoPlunger::new(COIL.name, SWITCH.name)
}
