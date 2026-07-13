use std::sync::LazyLock;

use frontbox::plugins::TroughPlugin;
use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub DRAIN_LED: LedDefinition = LedDefinition::single("drain")
    .tag(Playfield);

  pub SWITCH1: SwitchDefinition = SwitchDefinition::new("trough1");
  pub SWITCH2: SwitchDefinition = SwitchDefinition::new("trough2");
  pub SWITCH3: SwitchDefinition = SwitchDefinition::new("trough3");
  pub SWITCH4: SwitchDefinition = SwitchDefinition::new("trough4");
  pub SWITCH5: SwitchDefinition = SwitchDefinition::new("trough5");
  pub SWITCH6: SwitchDefinition = SwitchDefinition::new("trough6");

  pub COIL: DriverDefinition = DriverDefinition::new("trough");
}

pub fn plugin() -> TroughPlugin {
  TroughPlugin::new(vec![&SWITCH1, &SWITCH2], &COIL)
}
