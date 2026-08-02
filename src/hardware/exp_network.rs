use frontbox::prelude::*;

use crate::hardware::*;

pub fn exp_network() -> ExpNetwork {
  ExpNetwork::new(vec![
    ExpBoard::neuron()
      .wire_led_port(1, LedPort::ws2812().leds(vec![&backbox::LEFT_SPEAKER_LEDS]))
      .wire_led_port(
        2,
        LedPort::ws2812().leds(vec![&backbox::RIGHT_SPEAKER_LEDS]),
      )
      .wire_led_port(
        3,
        LedPort::ws2812().leds(vec![&cabinet::action_button::LED]),
      ),
    ExpBoard::fp_exp0081(JumperState::Open, JumperState::Open)
      .wire_led_port(
        2,
        LedPort::ws2812().leds(vec![
          &vspinner::upper_right_ray::LED3,
          &vspinner::upper_right_ray::LED2,
          &vspinner::upper_right_ray::LED1,
          &pop_cluster::upper_right::TARGET_LED,
        ]),
      )
      .wire_led_port(
        3,
        LedPort::ws2812().leds(vec![
          &city_map::APEX_TERRACES,
          &city_map::HYDRO_CORE,
          &gi::RIGHT_SLING,
          &right_inlane::ENTRANCE_LED,
          &right_outlane::LED,
          &right_pass_lane::ARROW_LED,
          &gi::LOWER_RIGHT_POP,
          &pop_cluster::lower_right::POP_LED,
          &pop_cluster::lower_right::TARGET_LED,
          &vspinner::lower_right_ray::LED1,
          &vspinner::lower_right_ray::LED2,
          &vspinner::lower_right_ray::LED3,
          &vspinner::LEDS,        // 12
          &right_orbit::HEX_LEDS, // 7
          &pop_cluster::upper_right::POP_LED,
        ]),
      )
      .wire_led_port(
        4,
        LedPort::ws2812().leds(vec![
          &city_map::SPORE_COUNT,
          &slingshots::POST_LEDS3,
          &slingshots::POST_LEDS4,
        ]),
      )
      .wire_led_port(
        6,
        LedPort::ws2812().leds(vec![
          &vspinner::left_ray::LED4,
          &vspinner::left_ray::LED3,
          &vspinner::left_ray::LED2,
          &vspinner::left_ray::LED1,
          &upper_pass_lane::ARROW_LED1,
          &upper_pass_lane::ARROW_LED2,
          &captive_ball::RIGHT_BOLT,
          &captive_ball::LEFT_BOLT,
          &gi::CAPTIVE_BALL,
          &center_orbit::SPINNER_LED,
          &lift_ramp::BOLT_LED,
          &arc_ramp::HEX_LEDS,
          &lift_ramp::HEX_LEDS,
          &center_orbit::HEX_LEDS,
        ]),
      )
      .wire_led_port(
        7,
        LedPort::ws2812().leds(vec![
          &pop_cluster::left::POP_LED,
          &pop_cluster::left::TARGET_LED,
          &left_ramp::HEX_LEDS,
          &left_orbit::HEX_LEDS,
          &gi::LOWER_SCOOP_ABOVE,
          &arc_ramp::SUBWAY_LEDS,
          &upper_pass_lane::SPINNER,
          &drop_bank::PADDLE_LED,
          &gi::DROP1,
          &gi::DROP2,
        ]),
      )
      .wire_led_port(
        8,
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
          &lower_thirds::slingshots::POST_LEDS1, // 8
          &lower_thirds::slingshots::POST_LEDS2, // 8
          &trough::DRAIN_LED,
          &right_inlane::LANE_LED1,
          &right_inlane::LANE_LED2,
          &plunge_lane::LED_STRIP,
        ]),
      ),
  ])
}
