use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("arc_opto");

  /// detects when the ball has entered the arc subway
  pub SUBWAY_SWITCH: SwitchDefinition = SwitchDefinition::new("arc_subway");

  pub SUBWAY_LEDS: LedDefinition = LedDefinition::multi("arc_subway", 11)
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("lift_ramp_lane", 7)
    .tag(Playfield)
    .tag(Lane);
}
