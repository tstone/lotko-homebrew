use frontbox_core::prelude::*;
use frontbox_core::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub UPPER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_upper")
    .tag(Playfield)
    .tag(RightPassLane)
    .tag(DoesNotCancelSkillshot)
    .tag(Lane)
    .location(Vec3::new(19.109, 25.405, 0.0).relative_to(&planes::PLAYFIELD));

  pub LOWER_SWITCH: SwitchDefinition = SwitchDefinition::new("r_pass_lane_lower")
    .tag(Playfield)
    .tag(RightPassLane)
    .tag(DoesNotCancelSkillshot)
    .tag(Lane)
    .location(Vec3::new(19.11, 30.29, 0.0).relative_to(&planes::PLAYFIELD));

  pub ARROW_LED: LedDefinition = LedDefinition::single("r_pass_lane_arr")
    .tag(Playfield)
    .tag(Insert)
    .tag(SmallArrow)
    .tag(Lane)
    .location(Vec3::new(19.099, 27.495, 0.0).relative_to(&planes::PLAYFIELD));
}

#[derive(Tag)]
pub struct RightPassLane;

pub struct RightPassLaneSystem;

impl RightPassLaneSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for RightPassLaneSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if event.switch.name == UPPER_SWITCH.name {
        ctx.emit(RightPassLaneHit { upper: true });
      } else if event.switch.name == LOWER_SWITCH.name {
        ctx.emit(RightPassLaneHit { upper: false });
      }
    }
  }
}

/// A ball crossed one of the lane's switches, which it does on its way through in either direction
#[derive(serde::Serialize, Event)]
pub struct RightPassLaneHit {
  pub upper: bool,
}
