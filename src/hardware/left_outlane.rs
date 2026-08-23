use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::Circle;

const NAME: &'static str = "l_outlane";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane);

  pub LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(Lane);
}

pub struct LeftOutlaneSystem;

impl LeftOutlaneSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for LeftOutlaneSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      ctx.emit(LeftOutlaneRollover);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct LeftOutlaneRollover;
