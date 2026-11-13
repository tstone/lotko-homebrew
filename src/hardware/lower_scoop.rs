use std::sync::LazyLock;

use frontbox_core::prelude::*;
use frontbox_core::tags::*;
use frontbox_pinball::{
  GameManagementExt, ScoopBallEntered, ScoopEjectStarted, ScoopEntry, ScoopSystem,
};
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp;
use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lower_scoop")
    .tag(Playfield)
    .mode(
      DriverMode::pulse_kick()
        .trigger_mode(DriverTriggerMode::VirtualSwitchTrue)
        .initial_pwm_length(HardwareValue::config(
          "Lower Scoop Touch Time",
          "Duration by which the eject plunger is brought into contact with the ball, before full eject",
          Duration::from_millis(7),
          Ranges::duration(0, 100),
        ))
        .initial_pwm_power(HardwareValue::fixed(Power::THREE_QUARTERS))
        .secondary_pwm_power(HardwareValue::Fixed(Power::ZERO))
        .secondary_pwm_length(HardwareValue::Fixed(Duration::ZERO))
        .kick_length(HardwareValue::config(
          "Lower Scoop Eject Time",
          "Duration that the plunger exert full power onto the ball (kick)",
          Duration::from_millis(35),
          Ranges::duration(10, 300),
        ))
        .build(),
    )
    .location(Vec3::new(1.675, 25.596, -2.0).relative_to(&planes::PLAYFIELD));

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lower_scoop")
    .inverted()
    .debounce(Duration::from_millis(50))
    .tag(Playfield)
    .location(Vec3::new(1.675, 25.596, -1.5).relative_to(&planes::PLAYFIELD));

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt_l")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(2.393, 28.11, 0.0).relative_to(&planes::PLAYFIELD));

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt_r")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield)
    .location(Vec3::new(3.339, 27.357, 0.0).relative_to(&planes::PLAYFIELD));
}

pub static BOLTS_Q: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    LEFT_BOLT.names()[0].clone(),
    RIGHT_BOLT.names()[0].clone(),
  ])
});

// -- System --

pub const LOWER_SCOOP_EJECT: &'static str = "lower_scoop_eject";

/// Names the lower scoop, for `ScoopSystem<Lower>` and its events
pub struct Lower;

/// Ejects a ball that lands in the scoop, unless something is holding it there (the mode select menu, a mode's intro,
/// the extra ball animation). A ball that came through the arc ramp's subway is a `ScoopEntry::LowerEntrance`.
pub fn system() -> ScoopSystem<Lower> {
  ScoopSystem::new(OPTO.name, COIL.name)
    .lower_entrance_switch(arc_ramp::SUBWAY_OPTO.name)
    .verify_delay(Duration::from_millis(250))
    .before_eject_effect(led_playback!("lower_scoop/eject.scene"))
    .after_eject_effect(led_playback!("lower_scoop/after_eject.scene"))
}

/// Plays the eject sound as the lower scoop's eject starts, and gives points for a ball that got into the scoop
/// without using the subway
#[derive(Clone)]
pub struct LowerScoopFeedback;

impl System for LowerScoopFeedback {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<ScoopEjectStarted<Lower>>() {
      ctx.play_sfx(LOWER_SCOOP_EJECT);
    } else if let Some(enter) = event.downcast_ref::<ScoopBallEntered<Lower>>()
      && enter.entry == ScoopEntry::Direct
    {
      ctx.add_points(500);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    system();
  }
}
