use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

pub const NAME: &'static str = "center_orbit";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Lane)
    .tag(Playfield);

  pub SPINNER_OPTO: SwitchDefinition = SwitchDefinition::new("center_spinner")
    .inverted()
    .debounce_close(Duration::from_millis(10))
    .tag(Spinner)
    .tag(Playfield);

  pub SPINNER_LED: LedDefinition = LedDefinition::single("center_spinner")
    .tag(Circle)
    .tag(Insert)
    .tag(Playfield);

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
pub struct CenterOrbitSystem;

impl CenterOrbitSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for CenterOrbitSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      ctx.emit(CenterOrbitHit);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct CenterOrbitHit;
