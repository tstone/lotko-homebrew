use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Hex;
use crate::hardware::planes;

const NAME: &'static str = "dome_ramp";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Ramp)
    .tag(Playfield)
    .location(Vec3::new(2.386, 17.349, 1.75).relative_to(&planes::PLAYFIELD));

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME)
    .locations(LedLayout::ring(6, Vec3::new(5.813, 23.201, 0.0), 0.5, 340.0f32.to_radians(), LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .locations([Vec3::new(5.813, 23.201, 0.0).relative_to(&planes::PLAYFIELD)])
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
