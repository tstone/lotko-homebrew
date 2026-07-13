use frontbox::prelude::*;

use crate::hardware::*;

pub fn exp_network() -> Vec<ExpansionBoard> {
  vec![
    ExpansionBoard::neuron()
      .wire_led_port(1, LedPort::ws2812().leds(vec![&backbox::LEFT_SPEAKER_LEDS]))
      .wire_led_port(
        2,
        LedPort::ws2812().leds(vec![&backbox::RIGHT_SPEAKER_LEDS]),
      )
      .wire_led_port(
        3,
        LedPort::ws2812().leds(vec![&cabinet::action_button::LED]),
      ),
    ExpansionBoard::fp_exp0081(JumperState::Open, JumperState::Open)
      .wire_led_port(
        6,
        LedPort::ws2812().leds(vec![
          &pop_cluster::left::POP_LED,
          &pop_cluster::left::TARGET_LED,
          &left_ramp::HEX_LEDS,
          &left_orbit::HEX_LEDS,
          &gi::LOWER_SCOOP_TRIANGLE,
          &arc_ramp::SUBWAY_LEDS,
        ]),
      )
      .wire_led_port(
        7,
        LedPort::ws2812().leds(vec![
          &city_map::SOLARIUM_ATRIUMS,
          &city_map::SKYRAIL_STATION,
          &city_map::NIMBUS_PROMENADE,
          &city_map::MERIDIAN_BASINS,
          &left_inlane::TARGET_LED,
          &lower_scoop::LEFT_BOLT,
          &gi::LOWER_SCOOP_TRIANGLE,
          &lower_scoop::RIGHT_BOLT,
          &left_outlane::LED,
          &gi::LEFT_SLING,
        ]),
      ),
  ]
}
