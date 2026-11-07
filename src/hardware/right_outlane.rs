use frontbox_core::prelude::*;
use frontbox_core::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

const NAME: &'static str = "r_outlane";

hardware_defs! {
  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME)
    .tag(Playfield)
    .tag(Circle)
    .tag(Lane)
    .location(Vec3::new(17.501, 35.41, 0.0).relative_to(&planes::PLAYFIELD));

  pub LED: LedDefinition = LedDefinition::single(NAME)
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(Lane)
    .location(Vec3::new(17.219, 30.695, 0.0).relative_to(&planes::PLAYFIELD));
}

pub struct RightOutlaneSystem;

impl RightOutlaneSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for RightOutlaneSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      ctx.emit(RightOutlaneRollover);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct RightOutlaneRollover;
