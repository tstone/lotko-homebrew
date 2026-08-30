use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::vspinner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PopBumper {
  Left,
  UpperRight,
  LowerRight,
}

impl PopBumper {
  pub fn next(&self) -> Self {
    match self {
      Self::Left => Self::UpperRight,
      Self::UpperRight => Self::LowerRight,
      Self::LowerRight => Self::Left,
    }
  }
}

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

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("l_spoon").tag(Playfield);
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("l_target").tag(Playfield);

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

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("ur_spoon")
      .debounce_close(Duration::from_millis(8))
      .tag(Playfield);

    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("ur_target")
      .tag(Playfield);

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

    pub SPOON_SWITCH: SwitchDefinition = SwitchDefinition::new("lr_spoon").tag(Playfield);
    pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("lr_target").tag(Playfield);

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

pub fn led_for_pop(pop: &PopBumper) -> &'static LedDefinition {
  match pop {
    PopBumper::Left => &left::POP_LED,
    PopBumper::UpperRight => &upper_right::POP_LED,
    PopBumper::LowerRight => &lower_right::POP_LED,
  }
}

pub fn target_led_for_pop(pop: &PopBumper) -> &'static LedDefinition {
  match pop {
    PopBumper::Left => &left::TARGET_LED,
    PopBumper::UpperRight => &upper_right::TARGET_LED,
    PopBumper::LowerRight => &lower_right::TARGET_LED,
  }
}

pub fn led_ray_for_pop(pop: &PopBumper) -> &'static LedQ {
  match pop {
    PopBumper::Left => &vspinner::left_ray::Q,
    PopBumper::UpperRight => &vspinner::upper_right_ray::Q,
    PopBumper::LowerRight => &vspinner::lower_right_ray::Q,
  }
}

pub fn match_switch(switch: &Switch) -> Option<PopBumper> {
  if switch.name == left::SPOON_SWITCH.name {
    Some(PopBumper::Left)
  } else if switch.name == upper_right::SPOON_SWITCH.name {
    Some(PopBumper::UpperRight)
  } else if switch.name == lower_right::SPOON_SWITCH.name {
    Some(PopBumper::LowerRight)
  } else {
    None
  }
}

pub fn match_target_switch(switch: &Switch) -> Option<PopBumper> {
  if switch.name == left::TARGET_SWITCH.name {
    Some(PopBumper::Left)
  } else if switch.name == upper_right::TARGET_SWITCH.name {
    Some(PopBumper::UpperRight)
  } else if switch.name == lower_right::TARGET_SWITCH.name {
    Some(PopBumper::LowerRight)
  } else {
    None
  }
}
