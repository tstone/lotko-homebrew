use frontbox::plugins::AutoplungerPlugin;
use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("autoplunge");
  pub SWITCH: SwitchDefinition = SwitchDefinition::new("plunge");

  pub LED_STRIP: LedDefinition = LedDefinition::multi("plunge", 4)
    .tag(Playfield);
}

pub fn plugin() -> AutoplungerPlugin {
  AutoplungerPlugin::new(&SWITCH, &COIL)
}
