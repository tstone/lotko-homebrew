use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox::tags::*;

hardware_defs! {
  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("arc_opto");

  /// detects when the ball has entered the arc subway
  pub SUBWAY_SWITCH: SwitchDefinition = SwitchDefinition::new("arc_subway")
    .tag(Playfield);

  pub SUBWAY_LEDS: LedDefinition = LedDefinition::multi("arc_subway", 11)
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("arc_ramp_lane", 7)
    .tag(Playfield)
    .tag(Lane);
}

static HEX_CENTER_LED_Q: LazyLock<HardwareQuery> = LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

// TODO: not sure of the exact indexes
static HEX_LINE_LEDS: LazyLock<HardwareQuery> = LazyLock::new(|| {
  Q::names(vec![
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
  ])
});

static HEX_CIRCLE_LEDS: LazyLock<HardwareQuery> = LazyLock::new(|| {
  Q::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
});
