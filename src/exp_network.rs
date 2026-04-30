use frontbox::prelude::*;

pub mod leds {
  pub const ACTION_BUTTON: &str = "action_button";
  pub const LEFT_SPEAKER_STRIP: &str = "lstrip";
  pub const RIGHT_SPEAKER_STRIP: &str = "rstrip";
}

pub fn exp_network() -> Vec<ExpansionBoard> {
  let neuron_expansion: ExpansionBoard = ExpansionBoard::neuron()
    .port(
      0,
      LedPort::ws2812().with(led_strip(leds::LEFT_SPEAKER_STRIP, 25)),
    )
    .port(
      1,
      LedPort::ws2812().with(led_strip(leds::RIGHT_SPEAKER_STRIP, 25)),
    )
    .port(
      2,
      LedPort::ws2812().with(led(leds::ACTION_BUTTON).tagged(tags::ActionButton)),
    );

  vec![neuron_expansion]
}
