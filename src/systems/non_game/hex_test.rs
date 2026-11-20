use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;

use crate::hardware::cabinet::RIGHT_FLIPPER_SWITCH1;

pub struct HexTest {
  program: ScenePlayer,
  state: State,
}

impl HexTest {
  pub fn new() -> Self {
    Self {
      state: State::Center,
      program: ScenePlayer::new(Self::center_program()),
    }
  }

  fn center_program() -> PlayableScene {
    scene!("hex_test/center.scene")
  }

  fn line_program() -> PlayableScene {
    scene!("hex_test/line.scene")
  }

  fn circle_program() -> PlayableScene {
    scene!("hex_test/circle.scene")
  }
}

impl System for HexTest {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == RIGHT_FLIPPER_SWITCH1.name
    {
      match self.state {
        State::Center => {
          self.program.replace_scene(Self::line_program(), ctx);
          self.state = State::Line;
        }
        State::Line => {
          self.program.replace_scene(Self::circle_program(), ctx);
          self.state = State::Circle;
        }
        State::Circle => {
          self.program.replace_scene(Self::center_program(), ctx);
          self.state = State::Center;
        }
      }
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    self.program.start(ctx);
  }
}

enum State {
  Center,
  Line,
  Circle,
}
