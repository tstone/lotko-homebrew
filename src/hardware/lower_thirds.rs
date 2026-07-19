use crate::hardware::cabinet;
use frontbox::prelude::*;
use frontbox::tags::*;

pub mod left_flipper {
  use super::*;

  hardware_defs! {
    pub EOS_SWITCH: SwitchDefinition = SwitchDefinition::new("l_flipper_eos");

    pub MAIN_COIL: DriverDefinition = DriverDefinition::new("l_flipper_main")
    .mode(FlipperMainDirectMode {
        button_switch: cabinet::LEFT_FLIPPER_SWITCH1.name,
        eos_switch: EOS_SWITCH.name,
        ..Default::default()
      });

    pub HOLD_COIL: DriverDefinition = DriverDefinition::new("l_flipper_hold")
      .mode(FlipperHoldDirectMode {
        button_switch: cabinet::LEFT_FLIPPER_SWITCH1.name,
        ..Default::default()
      });
  }
}

pub mod right_flipper {
  use super::*;

  hardware_defs! {

    pub EOS_SWITCH: SwitchDefinition = SwitchDefinition::new("r_flipper_eos");

    pub MAIN_COIL: DriverDefinition = DriverDefinition::new("r_flipper_main")
    .mode(FlipperMainDirectMode {
        button_switch: cabinet::RIGHT_FLIPPER_SWITCH1.name,
        eos_switch: EOS_SWITCH.name,
        ..Default::default()
      });

    pub HOLD_COIL: DriverDefinition = DriverDefinition::new("r_flipper_hold")
      .mode(FlipperHoldDirectMode {
        button_switch: cabinet::RIGHT_FLIPPER_SWITCH1.name,
        ..Default::default()
      });
  }
}

pub mod slingshots {
  use super::*;

  hardware_defs! {
    pub LEFT_SWITCH: SwitchDefinition = SwitchDefinition::new("l_sling");
    pub RIGHT_SWITCH: SwitchDefinition = SwitchDefinition::new("r_sling");

    // -- Coils --

    pub LEFT_COIL: DriverDefinition = DriverDefinition::new("l_sling_coil")
      .mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(LEFT_SWITCH.name),
        initial_pwm_power: HardwareValue::config("Left Sling Power", "Power of the left slingshot", Power::percent(80), Ranges::full_power()),
        ..Default::default()
      })
      .tag(Playfield)
      .tag(SlingShot);

    pub RIGHT_COIL: DriverDefinition = DriverDefinition::new("r_sling_coil")
      .mode(PulseMode {
        trigger_mode: DriverTriggerMode::Switch(RIGHT_SWITCH.name),
        initial_pwm_power: HardwareValue::config("Right Sling Power", "Power of the right slingshot", Power::percent(80), Ranges::full_power()),
        ..Default::default()
      })
      .tag(Playfield)
      .tag(SlingShot);

    // -- Post LEDs --

    /// Left Sling, left-most post
    pub POST_LEDS1: LedDefinition = LedDefinition::multi("l_sling_left_post", 8)
      .tag(GeneralIllumination)
      .tag(Playfield);

    /// Left Sling, lower post
    pub POST_LEDS2: LedDefinition = LedDefinition::multi("l_sling_lower_post", 8)
      .tag(GeneralIllumination)
      .tag(Playfield);

    /// Right Sling, right-most post
    pub POST_LEDS3: LedDefinition = LedDefinition::multi("r_sling_right_post", 8)
      .tag(GeneralIllumination)
      .tag(Playfield);

    /// Right Sling, lower post
    pub POST_LEDS4: LedDefinition = LedDefinition::multi("r_sling_lower_post", 8)
      .tag(GeneralIllumination)
      .tag(Playfield);
  }
}
