use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

use crate::hardware::{
  arc_ramp, cabinet::RIGHT_FLIPPER_SWITCH1, center_orbit, dome_ramp, left_orbit, lift_ramp,
  right_orbit,
};

pub struct HexTest {
  program: LedScene,
  state: State,
}

impl HexTest {
  pub fn new() -> Self {
    Self {
      state: State::Center,
      program: Self::center_program(),
    }
  }

  fn center_program() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &left_orbit::HEX_CENTER_LED,
        &dome_ramp::HEX_CENTER_LED,
        &arc_ramp::HEX_CENTER_LED,
        &center_orbit::HEX_CENTER_LED,
        &lift_ramp::HEX_CENTER_LED,
        &right_orbit::HEX_CENTER_LED,
      ]),
      led_scene!("hex_test/center.ron"),
    )
  }

  fn line_program() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &left_orbit::HEX_LINE_LEDS,
        &dome_ramp::HEX_LINE_LEDS,
        &arc_ramp::HEX_LINE_LEDS,
        &center_orbit::HEX_LINE_LEDS,
        &lift_ramp::HEX_LINE_LEDS,
        &right_orbit::HEX_LINE_LEDS,
      ]),
      led_scene!("hex_test/line.ron"),
    )
  }

  fn circle_program() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &left_orbit::HEX_CIRCLE_LEDS,
        &dome_ramp::HEX_CIRCLE_LEDS,
        &arc_ramp::HEX_CIRCLE_LEDS,
        &center_orbit::HEX_CIRCLE_LEDS,
        &lift_ramp::HEX_CIRCLE_LEDS,
        &right_orbit::HEX_CIRCLE_LEDS,
      ]),
      led_scene!("hex_test/circle.ron"),
    )
  }
}

impl System for HexTest {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == RIGHT_FLIPPER_SWITCH1.name
    {
      match self.state {
        State::Center => {
          self.program.stop(ctx);
          self.program = Self::line_program();
          self.state = State::Line;
        }
        State::Line => {
          self.program.stop(ctx);
          self.program = Self::circle_program();
          self.state = State::Circle;
        }
        State::Circle => {
          self.program.stop(ctx);
          self.program = Self::center_program();
          self.state = State::Center;
        }
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.program.apply(delta, ctx);
  }
}

enum State {
  Center,
  Line,
  Circle,
}
