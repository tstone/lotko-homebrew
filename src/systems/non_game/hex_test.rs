use frontbox::prelude::*;

use crate::hardware::{arc_ramp, center_orbit, left_orbit, left_ramp, lift_ramp, right_orbit};

pub struct HexTest {
  program: LedProgram1d,
}

impl HexTest {
  pub fn new() -> Self {
    Self {
      program: Self::center_program(),
    }
  }

  fn center_program() -> LedProgram1d {
    LedProgram1d::fixed(
      Q::any_of(vec![
        left_orbit::HEX_CENTER_LED.clone(),
        left_ramp::HEX_CENTER_LED.clone(),
        arc_ramp::HEX_CENTER_LED.clone(),
        center_orbit::HEX_CENTER_LED.clone(),
        lift_ramp::HEX_CENTER_LED.clone(),
        right_orbit::HEX_CENTER_LED.clone(),
      ]),
      ColorSequence::solid(Rgba::white()),
    )
    .playing()
  }

  fn line_program() -> LedProgram1d {
    LedProgram1d::fixed(
      Q::any_of(vec![
        left_orbit::HEX_LINE_LEDS.clone(),
        left_ramp::HEX_LINE_LEDS.clone(),
        arc_ramp::HEX_LINE_LEDS.clone(),
        center_orbit::HEX_LINE_LEDS.clone(),
        lift_ramp::HEX_LINE_LEDS.clone(),
        right_orbit::HEX_LINE_LEDS.clone(),
      ]),
      ColorSequence::fade(Rgba::yellow(), Rgba::blue()),
    )
    .playing()
  }

  fn circle_program() -> LedProgram1d {
    LedProgram1d::fixed(
      Q::any_of(vec![
        left_orbit::HEX_CIRCLE_LEDS.clone(),
        left_ramp::HEX_CIRCLE_LEDS.clone(),
        arc_ramp::HEX_CIRCLE_LEDS.clone(),
        center_orbit::HEX_CIRCLE_LEDS.clone(),
        lift_ramp::HEX_CIRCLE_LEDS.clone(),
        right_orbit::HEX_CIRCLE_LEDS.clone(),
      ]),
      ColorSequence::fade(Rgba::yellow(), Rgba::blue()),
    )
    .playing()
  }
}

impl System for HexTest {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.cue(ToLine, Cue::Once(Duration::from_secs(6)));
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<ToCenter>() {
      self.program = Self::center_program();
      ctx.cue(ToLine, Cue::Once(Duration::from_secs(6)));
    } else if event.is::<ToLine>() {
      self.program = Self::line_program();
      ctx.cue(ToCircle, Cue::Once(Duration::from_secs(6)));
    } else if event.is::<ToCircle>() {
      self.program = Self::circle_program();
      ctx.cue(ToCenter, Cue::Once(Duration::from_secs(6)));
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ToCenter;
#[derive(serde::Serialize, Event)]
struct ToLine;
#[derive(serde::Serialize, Event)]
struct ToCircle;
