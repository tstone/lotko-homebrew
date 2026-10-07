use crate::hardware::cabinet;
use frontbox::prelude::*;
use frontbox::tags::*;

pub mod left_flipper {
  use super::*;

  hardware_defs! {
    pub EOS_SWITCH: SwitchDefinition = SwitchDefinition::new("l_flipper_eos");

    pub MAIN_COIL: DriverDefinition = DriverDefinition::new("l_flipper_main")
      .mode(
        DriverMode::flipper_main_direct(cabinet::LEFT_FLIPPER_SWITCH1.name, EOS_SWITCH.name)
          .initial_pwm_power(HardwareValue::config("Initial Power", "", Power::FULL, Ranges::full_power()))
          .secondary_pwm_power(HardwareValue::config("Secondary Power", "", Power::THREE_QUARTERS, Ranges::full_power()))
          .build(),
      );

    pub HOLD_COIL: DriverDefinition = DriverDefinition::new("l_flipper_hold")
      .mode(DriverMode::flipper_hold_direct(cabinet::LEFT_FLIPPER_SWITCH1.name).build());
  }
}

pub mod right_flipper {
  use super::*;

  hardware_defs! {

    pub EOS_SWITCH: SwitchDefinition = SwitchDefinition::new("r_flipper_eos")
      .debounce_close(Duration::from_millis(8));

    pub MAIN_COIL: DriverDefinition = DriverDefinition::new("r_flipper_main")
      .mode(
        DriverMode::flipper_main_direct(cabinet::RIGHT_FLIPPER_SWITCH1.name, EOS_SWITCH.name)
          .initial_pwm_power(HardwareValue::config("Initial Power", "", Power::FULL, Ranges::full_power()))
          .secondary_pwm_power(HardwareValue::config("Secondary Power", "", Power::FULL, Ranges::full_power()))
          .build(),
      );

    pub HOLD_COIL: DriverDefinition = DriverDefinition::new("r_flipper_hold")
      .mode(DriverMode::flipper_hold_direct(cabinet::RIGHT_FLIPPER_SWITCH1.name).build());
  }
}

pub mod slingshots {
  use crate::hardware::planes;

  use super::*;

  hardware_defs! {
    pub LEFT_SWITCH: SwitchDefinition = SwitchDefinition::new("l_sling")
      .debounce_close(Duration::from_millis(15))
      .location(Vec3::new(3.49, 35.403, 0.0).relative_to(&planes::PLAYFIELD));

    pub RIGHT_SWITCH: SwitchDefinition = SwitchDefinition::new("r_sling")
      .location(Vec3::new(13.857, 33.802, 0.0).relative_to(&planes::PLAYFIELD));

    // -- Coils --

    pub LEFT_COIL: DriverDefinition = DriverDefinition::new("l_sling_coil")
      .mode(
        DriverMode::pulse()
          .trigger_mode(DriverTriggerMode::Switch(LEFT_SWITCH.name))
          .initial_pwm_power(HardwareValue::config("Left Sling Power", "Power of the left slingshot", Power::THREE_QUARTERS, Ranges::full_power()))
          .build(),
      )
      .tag(Playfield);

    pub RIGHT_COIL: DriverDefinition = DriverDefinition::new("r_sling_coil")
      .mode(
        DriverMode::pulse()
          .trigger_mode(DriverTriggerMode::Switch(RIGHT_SWITCH.name))
          .initial_pwm_power(HardwareValue::config("Right Sling Power", "Power of the right slingshot", Power::THREE_QUARTERS, Ranges::full_power()))
          .build(),
      )
      .tag(Playfield);

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
