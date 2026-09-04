use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Hex;

const NAME: &'static str = "l_ramp";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Ramp)
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Insert)
    .tag(Hex)
    .tag(Lane);
}

pub static HEX_CENTER_LED: LazyLock<LedQ> = LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

pub static HEX_LINE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
  ])
});

pub static HEX_CIRCLE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
  ])
});

pub struct DomeRampSystem;

impl DomeRampSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for DomeRampSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      ctx.emit(DomeRampHit);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct DomeRampHit;
