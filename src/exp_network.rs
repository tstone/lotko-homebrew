use frontbox::prelude::*;

pub mod leds {
  pub const ACTION_BUTTON: &str = "action_button";
}

pub fn exp_network() -> Vec<ExpansionBoardSpec> {

  let neuron_expansion: ExpansionBoardSpec = ExpansionBoardSpec::neutron()
    .with_led_port(LedPortSpec {
      port: 0,
      leds: vec![leds::ACTION_BUTTON],
      ..Default::default()
    });

  vec![neuron_expansion]
}