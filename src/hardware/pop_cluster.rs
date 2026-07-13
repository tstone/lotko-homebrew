use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

pub mod left {
  use super::*;

  hardware_defs! {
    pub COIL: DriverDefinition = DriverDefinition::new("l_pop")
      .tag(Playfield);

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("l_spoon");
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("l_target");

    pub POP_LED: LedDefinition = LedDefinition::single("l_pop_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Target);

    pub TARGET_LED: LedDefinition = LedDefinition::single("l_target_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Target);
  }
}

pub mod upper_right {
  use super::*;

  hardware_defs! {
    pub COIL: DriverDefinition = DriverDefinition::new("ur_pop")
      .tag(Playfield);

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("ur_spoon");
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("ur_target");

    pub POP_LED: LedDefinition = LedDefinition::single("ur_pop_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Target);

    pub TARGET_LED: LedDefinition = LedDefinition::single("ur_target_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Target);
  }
}

pub mod lower_right {
  use super::*;

  hardware_defs! {
    pub COIL: DriverDefinition = DriverDefinition::new("lr_pop")
      .tag(Playfield);

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("lr_spoon");
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("lr_target");

    pub POP_LED: LedDefinition = LedDefinition::single("lr_pop_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Target);

    pub TARGET_LED: LedDefinition = LedDefinition::single("lr_target_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Target);
  }
}
