use frontbox::prelude::*;

pub mod leds {
  pub const ACTION_BUTTON: &str = "action_button";
}

pub fn exp_network() -> Vec<ExpansionBoard> {

  let neuron_expansion: ExpansionBoard = ExpansionBoard::neutron()
    .with_led_port(LedPort {
      port: 0,
      leds: vec![leds::ACTION_BUTTON],
      ..Default::default()
    });

  vec![neuron_expansion]
}