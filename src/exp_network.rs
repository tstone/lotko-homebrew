use frontbox::prelude::*;

pub mod leds {
  pub const ACTION_BUTTON: &str = "action_button";
  pub const LEFT_SPEAKER_STRIP: &str = "lstrip";
  pub const RIGHT_SPEAKER_STRIP: &str = "rstrip";
  pub const V_SPINNER: &str = "vspinner";

  // lower thirds
  pub const LEFT_INLANE_TARGET: &str = "left_inlane";
  pub const LEFT_OUTLANE: &str = "left_outlane";
  pub const RIGHT_INLANE: &str = "right_inlane";
  pub const RIGHT_OUTLANE: &str = "right_outlane";
  pub const LOWER_SCOOP_LEFT_BOLT: &str = "lower_scoop_lbolt";
  pub const LOWER_SCOOP_RIGHT_BOLT: &str = "lower_scoop_rbolt";

  pub mod city {
    pub const SOLARIUM_ATRIUMS: &str = "solariaum_atriums";
    pub const SKYRAIL_STATION: &str = "skyrail_station";
    pub const NIMBUS_PROMENADE: &str = "nimbus_promenade";
    pub const MERIDIAN_BASINS: &str = "meridian_basins";
  }

  pub mod GI {
    pub const BOTTOM_LEFT_TRIANGLE: &str = "bottom_left_tri";
  }

  // mid-field
  pub const LEFT_POP: &str = "lpop";
  pub const LEFT_POP_TARGET: &str = "lpop_target";
  pub const LEFT_RAMP: &str = "lramp";
  pub const LEFT_ORBIT: &str = "lorbit";
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

  let playfield_0081 = ExpansionBoard::fp_exp0081(JumperState::Open, JumperState::Open)
    .port(
      6,
      LedPort::ws2812()
        .with(led(leds::LEFT_POP).tagged(tags::Playfield))
        .with(led(leds::LEFT_POP_TARGET).tagged(tags::Playfield))
        .with(led_strip(leds::LEFT_RAMP, 7))
        .with(led_strip(leds::LEFT_ORBIT, 7)),
    )
    .port(
      7,
      LedPort::ws2812()
        .with(led(leds::city::SOLARIUM_ATRIUMS).tagged(tags::Playfield))
        .with(led(leds::city::SKYRAIL_STATION).tagged(tags::Playfield))
        .with(led(leds::city::NIMBUS_PROMENADE).tagged(tags::Playfield))
        .with(led(leds::city::MERIDIAN_BASINS).tagged(tags::Playfield))
        .with(
          led(leds::LEFT_INLANE_TARGET)
            .tagged(tags::Playfield)
            .tagged(tags::Target),
        )
        .with(led(leds::LOWER_SCOOP_LEFT_BOLT).tagged(tags::Playfield))
        .with(
          led(leds::GI::BOTTOM_LEFT_TRIANGLE)
            .tagged(tags::Playfield)
            .tagged(tags::GI),
        )
        .with(led(leds::LOWER_SCOOP_RIGHT_BOLT).tagged(tags::Playfield)),
    );

  vec![neuron_expansion, playfield_0081]
}
