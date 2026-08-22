use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Hex;

const NAME: &'static str = "l_ramp";

hardware_defs! {

  pub OPTO: SwitchDefinition = SwitchDefinition::new(NAME);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Insert)
    .tag(Hex)
    .tag(Lane);
}

pub static HEX_CENTER_LED: LazyLock<HardwareQuery> =
  LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

pub static HEX_LINE_LEDS: LazyLock<HardwareQuery> = LazyLock::new(|| {
  Q::names(vec![
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
});

pub static HEX_CIRCLE_LEDS: LazyLock<HardwareQuery> = LazyLock::new(|| {
  // TODO: verify order
  Q::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
});
