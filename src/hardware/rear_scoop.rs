use frontbox_core::prelude::DriverTriggerMode::VirtualSwitchTrue;
use frontbox_core::prelude::*;
use frontbox_core::tags::*;
use frontbox_pinball::ScoopSystem;

use crate::hardware::planes;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lift_ramp_eject")
    .tag(Playfield)
    .mode(
      DriverMode::pulse_kick()
        .trigger_mode(VirtualSwitchTrue)
        .initial_pwm_length(HardwareValue::config(
          "Rear Scoop Touch Time",
          "Duration by which the eject plunger is brought into contact with the ball, before full eject",
          Duration::from_millis(2),
          Ranges::duration(0, 100),
        ))
        .initial_pwm_power(HardwareValue::fixed(Power::HALF))
        .secondary_pwm_power(HardwareValue::Fixed(Power::ZERO))
        .secondary_pwm_length(HardwareValue::Fixed(Duration::ZERO))
        .kick_length(HardwareValue::config(
          "Rear Scoop Eject Time",
          "Duration that the plunger exert full power onto the ball (kick)",
          Duration::from_millis(12),
          Ranges::duration(10, 100),
        ))
        .build(),
    )
    .location(Vec3::new(17.424, 1.428, -2.5).relative_to(&planes::PLAYFIELD));

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lift_ramp_scoop_opto")
    .inverted()
    .debounce_close(Duration::from_millis(10))
    .tag(Playfield)
    .location(Vec3::new(17.424, 1.428, -1.5).relative_to(&planes::PLAYFIELD));
}

/// Names the scoop behind the lift ramp, for `ScoopSystem<Rear>` and its events
pub struct Rear;

/// Ejects any ball that lands in the scoop behind the lift ramp
pub fn system() -> ScoopSystem<Rear> {
  ScoopSystem::new(OPTO.name, COIL.name).verify_delay(Duration::from_millis(1200))
}
