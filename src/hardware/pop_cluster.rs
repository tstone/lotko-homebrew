use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

pub mod left {
  use super::*;

  hardware_defs! {
    pub COIL: DriverDefinition = DriverDefinition::new("l_pop")
      .mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(SPOON_SWITCH.name),
        initial_pwm_power: HardwareValue::config("Left Pop Power", "Power of the left pop bumper", Power::FULL, Ranges::full_power()),
        initial_pwm_length: HardwareValue::Fixed(Duration::from_millis(30)),
        ..Default::default()
      })
      .tag(Playfield);

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("l_spoon");
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("l_target");

    pub POP_LED: LedDefinition = LedDefinition::single("l_pop_led")
      .tag(Playfield)
      .tag(SmallArrow)
      .tag(Insert)
      .tag(Target);

    pub TARGET_LED: LedDefinition = LedDefinition::single("l_target_led")
      .tag(Playfield)
      .tag(Insert)
      .tag(SmallArrow)
      .tag(Target);
  }
}

pub mod upper_right {
  use super::*;

  hardware_defs! {
    pub COIL: DriverDefinition = DriverDefinition::new("ur_pop")
      .mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(SPOON_SWITCH.name),
        initial_pwm_power: HardwareValue::config("Upper Right Pop Power", "Power of the upper right pop bumper", Power::FULL, Ranges::full_power()),
        initial_pwm_length: HardwareValue::Fixed(Duration::from_millis(30)),
        ..Default::default()
      })
      .tag(Playfield);

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("ur_spoon").debounce_close(Duration::from_millis(8));
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("ur_target");

    pub POP_LED: LedDefinition = LedDefinition::single("ur_pop_led")
      .tag(Playfield)
      .tag(Insert)
      .tag(SmallArrow)
      .tag(Target);

    pub TARGET_LED: LedDefinition = LedDefinition::single("ur_target_led")
      .tag(Playfield)
      .tag(Insert)
      .tag(SmallArrow)
      .tag(Target);
  }
}

pub mod lower_right {
  use super::*;

  hardware_defs! {
    pub COIL: DriverDefinition = DriverDefinition::new("lr_pop")
      .mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(SPOON_SWITCH.name),
        initial_pwm_power: HardwareValue::config("Lower Right Pop Power", "Power of the lower right pop bumper", Power::FULL, Ranges::full_power()),
        initial_pwm_length: HardwareValue::Fixed(Duration::from_millis(30)),
        ..Default::default()
      })
      .tag(Playfield);

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("lr_spoon");
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("lr_target");

    pub POP_LED: LedDefinition = LedDefinition::single("lr_pop_led")
      .tag(Playfield)
      .tag(Insert)
      .tag(SmallArrow)
      .tag(Target);

    pub TARGET_LED: LedDefinition = LedDefinition::single("lr_target_led")
      .tag(Playfield)
      .tag(Insert)
      .tag(SmallArrow)
      .tag(Target);
  }
}
