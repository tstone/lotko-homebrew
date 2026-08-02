use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "r_orbit";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
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
  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      ctx.emit(RightOrbitHit);
    }
  }
}

pub struct RightOrbitHit;
