use frontbox::prelude::DriverTriggerMode::VirtualSwitchTrue;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub RAMP_COIL: DriverDefinition = DriverDefinition::new("lift_ramp")
    .tag(Playfield)
    .mode(PulseMode {
      trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
      initial_pwm_power: Power::percent(100),
      ..Default::default()
    });

  pub EJECT_COIL: DriverDefinition = DriverDefinition::new("lift_ramp_eject")
    .tag(Playfield)
    .mode(PulseKickMode {
        trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
        initial_pwm_length: Duration::from_millis(7),
        secondary_pwm_length: Duration::from_millis(35),
        secondary_pwm_power: Power::FULL,
        ..Default::default()
      });

  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_opto");
  pub SUBWAY_SWITCH: SwitchDefinition = SwitchDefinition::new("lift_ramp_subway");

  pub BOLT_LED: LedDefinition = LedDefinition::single("lift_ramp_bolt")
    .tag(Playfield)
    .tag(Bolt)
    .tag(Lane);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("lift_ramp_lane", 7)
    .tag(Playfield)
    .tag(Hex)
    .tag(Lane);
}
