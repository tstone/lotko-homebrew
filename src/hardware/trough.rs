use frontbox::prelude::*;
use frontbox::provided::Trough;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub DRAIN_LED: LedDefinition = LedDefinition::single("drain")
    .tag(Circle)
    .tag(Playfield);

  pub SWITCH1: SwitchDefinition = Trough::switch_definition("trough_1");
  pub SWITCH2: SwitchDefinition = Trough::switch_definition("trough_2");
  pub SWITCH3: SwitchDefinition = Trough::switch_definition("trough_3");
  pub SWITCH4: SwitchDefinition = Trough::switch_definition("trough_4");
  pub SWITCH5: SwitchDefinition = Trough::switch_definition("trough_5");
  pub SWITCH6: SwitchDefinition = Trough::switch_definition("trough_6");

  pub COIL: DriverDefinition = Trough::eject_coil_definition("trough_coil");
}

pub fn system() -> Trough {
  Trough::new(
    COIL.name,
    vec![
      SWITCH1.name,
      SWITCH2.name,
      SWITCH3.name,
      // Only using 3 balls for testing
      // SWITCH4.name,
      // SWITCH5.name,
      // SWITCH6.name,
    ],
  )
}
