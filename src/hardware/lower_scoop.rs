use frontbox::prelude::DriverTriggerMode::*;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lower_scoop")
    .tag(Playfield)
    .mode(PulseKickMode {
      trigger_mode: VirtualSwitchTrue,
      initial_pwm_length: Duration::from_millis(7),
      secondary_pwm_length: Duration::from_millis(40),
      ..Default::default()
    });

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lower_scoop");

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt1")
    .tag(Bolt)
    .tag(Playfield);

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt2")
    .tag(Bolt)
    .tag(Playfield);
}
