use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::right_orbit::State::*;

const NAME: &'static str = "r_orbit";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .debounce(Duration::from_millis(20))
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Insert)
    .tag(Hex)
    .tag(Lane);
}

pub static HEX_CENTER_LED: LazyLock<LedQ> = LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

pub static HEX_LINE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
  ])
});

pub static HEX_CIRCLE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
  ])
});

#[derive(Clone)]
pub struct RightOrbitSystem {
  state: State,
}

impl RightOrbitSystem {
  pub fn new() -> Self {
    Self { state: AwaitingHit }
  }

  fn reset(&mut self, ctx: &SystemContext) {
    match self.state {
      LongDebouncing(cue_id) => {
        ctx.cancel_cue(cue_id);
        self.state = AwaitingHit;
      }
      AwaitingHit => {}
    }
  }

  fn transition_to_long_debounce(&mut self, ctx: &SystemContext) {
    let cue_id = ctx.cue(Reset, Duration::from_millis(250).once());
    self.state = LongDebouncing(cue_id);
  }
}

impl System for RightOrbitSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Reset>() {
      self.reset(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
      && self.state == AwaitingHit
    {
      ctx.emit(RightOrbitHit);
      self.transition_to_long_debounce(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    self.reset(ctx);
  }
}

#[derive(serde::Serialize, Event)]
pub struct RightOrbitHit;

#[derive(serde::Serialize, Event)]
struct Reset;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
  AwaitingHit,
  LongDebouncing(u64),
}
