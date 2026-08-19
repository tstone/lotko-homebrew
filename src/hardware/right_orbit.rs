use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "r_orbit";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    // this is a swinging roll-under and can activate multiple times
    .debounce_open(Duration::from_millis(20))
    .debounce_close(Duration::from_millis(20));

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Insert)
    .tag(Hex)
    .tag(Lane);
}

pub fn hex_center_led_q() -> HardwareQuery {
  HEX_LEDS.child(6).unwrap().q()
}

pub fn hex_line_leds_q() -> HardwareQuery {
  Q::names(vec![
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
  ])
}

pub fn hex_circle_leds_q() -> HardwareQuery {
  // TODO: verify order
  Q::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
}

#[derive(Clone)]
pub struct RightOrbitSystem;

impl RightOrbitSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for RightOrbitSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      ctx.emit(RightOrbitHit);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct RightOrbitHit;
