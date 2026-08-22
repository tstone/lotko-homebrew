use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::ArcRamp;

hardware_defs! {
  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("arc_opto")
    .inverted()
    .tag(Playfield);

  /// detects when the ball has entered the arc subway
  pub SUBWAY_OPTO: SwitchDefinition = SwitchDefinition::new("arc_subway")
    .inverted()
    .tag(Playfield);

  pub SUBWAY_LEDS: LedDefinition = LedDefinition::multi("arc_subway", 11)
    .tag(ArcRamp)
    .tag(Playfield);

  pub ARC_LEDS: LedDefinition = LedDefinition::strip("arc", 18)
    .tag(ArcRamp)
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("arc_ramp_lane", 7)
    .tag(Playfield)
    .tag(Insert)
    .tag(Lane);
}

pub static HEX_CENTER_LED: LazyLock<HardwareQuery> =
  LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

pub static HEX_LINE_LEDS: LazyLock<HardwareQuery> = LazyLock::new(|| {
  Q::names(vec![
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(0).unwrap().name(),
  ])
});

pub static HEX_CIRCLE_LEDS: LazyLock<HardwareQuery> = LazyLock::new(|| {
  Q::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
});

#[derive(Clone)]
pub struct ArcRampSystem;

impl ArcRampSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for ArcRampSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if event.switch.name == RAMP_OPTO.name {
        log::info!("Arc ramp hit");
        ctx.emit(ArcRampHit);
      } else if event.switch.name == SUBWAY_OPTO.name {
        log::info!("Arc ramp subway entered");
        ctx.emit(ArcRampSubwayHit);
      }
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct ArcRampHit;

#[derive(serde::Serialize, Event)]
pub struct ArcRampSubwayHit;
