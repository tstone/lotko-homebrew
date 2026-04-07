use frontbox::prelude::*;

pub mod leds {
  pub const ACTION_BUTTON: &str = "action_button";
  pub const TEST_STRIP: &str = "test_strip";
  pub const SEG1: &str = "seg1";
}

pub fn exp_network() -> Vec<ExpansionBoard> {
  let neuron_expansion: ExpansionBoard = ExpansionBoard::neuron()
    .port(
      0,
      LedPort::ws2812().with(led(leds::ACTION_BUTTON).tagged(tags::ActionButton)),
    )
    .port(1, LedPort::ws2812().with(led_strip(leds::TEST_STRIP, 8)));

  vec![neuron_expansion]
}
