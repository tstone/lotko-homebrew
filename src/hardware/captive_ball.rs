use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::captive_ball::State::*;
use crate::hardware::more_tags::*;

hardware_defs! {
  pub TARGET_SWITCH: SwitchDefinition = SwitchDefinition::new("cap_ball_target")
    .tag(Playfield);

  pub REST_SWITCH: SwitchDefinition = SwitchDefinition::new("cap_ball_rest")
    .debounce(Duration::from_millis(50))
    .tag(DoesNotCancelSkillshot);

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("l_cap_ball_bolt")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield);

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("r_cap_ball_bolt")
    .tag(Bolt)
    .tag(Insert)
    .tag(Playfield);
}

#[derive(Clone)]
pub struct CaptiveBallSystem {
  state: State,
}

impl CaptiveBallSystem {
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

impl System for CaptiveBallSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<Reset>() {
      self.reset(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == TARGET_SWITCH.name
      && self.state == AwaitingHit
    {
      ctx.emit(CaptiveBallHit);
      self.transition_to_long_debounce(ctx);
    }
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    self.reset(ctx);
  }
}

#[derive(serde::Serialize, Event)]
pub struct CaptiveBallHit;

#[derive(serde::Serialize, Event)]
struct Reset;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
  AwaitingHit,
  LongDebouncing(u64),
}
