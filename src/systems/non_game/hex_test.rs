use frontbox::prelude::*;

use crate::hardware::{
  arc_ramp, cabinet::RIGHT_FLIPPER_SWITCH1, center_orbit, left_orbit, left_ramp, lift_ramp,
  right_orbit,
};

pub struct HexTest {
  program: LedProgram1d,
  state: State,
}

impl HexTest {
  pub fn new() -> Self {
    Self {
      state: State::Center,
      program: Self::center_program(),
    }
  }

  fn center_program() -> LedProgram1d {
    LedProgram1d::fixed(
      LedQ::any(vec![
        &left_orbit::HEX_CENTER_LED,
        &left_ramp::HEX_CENTER_LED,
        &arc_ramp::HEX_CENTER_LED,
        &center_orbit::HEX_CENTER_LED,
        &lift_ramp::HEX_CENTER_LED,
        &right_orbit::HEX_CENTER_LED,
      ]),
      ColorSequence::solid(Rgba::white()),
    )
  }

  fn line_program() -> LedProgram1d {
    let pattern = ColorSequence::fade(Rgba::blue(), Rgba::yellow()).generate(3);
    LedProgram1d::fixed(
      LedQ::any(vec![
        &left_orbit::HEX_LINE_LEDS,
        &left_ramp::HEX_LINE_LEDS,
        &arc_ramp::HEX_LINE_LEDS,
        &center_orbit::HEX_LINE_LEDS,
        &lift_ramp::HEX_LINE_LEDS,
        &right_orbit::HEX_LINE_LEDS,
      ]),
      ColorSequence::tile(pattern),
    )
  }

  fn circle_program() -> LedProgram1d {
    let pattern = ColorSequence::fade(Rgba::blue(), Rgba::red()).generate(6);
    LedProgram1d::fixed(
      LedQ::any(vec![
        &left_orbit::HEX_CIRCLE_LEDS,
        &left_ramp::HEX_CIRCLE_LEDS,
        &arc_ramp::HEX_CIRCLE_LEDS,
        &center_orbit::HEX_CIRCLE_LEDS,
        &lift_ramp::HEX_CIRCLE_LEDS,
        &right_orbit::HEX_CIRCLE_LEDS,
      ]),
      ColorSequence::tile(pattern),
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
