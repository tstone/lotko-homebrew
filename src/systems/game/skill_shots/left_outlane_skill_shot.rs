use frontbox_pinball::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};

use crate::hardware::left_outlane::LeftOutlaneRollover;
use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::hardware::right_pass_lane::RightPassLane;
use crate::systems::game;

// Secret skill shot to hit the left outlane without hitting anything else
#[derive(Clone)]
pub struct LeftOutlaneSkillShot;

impl LeftOutlaneSkillShot {
  pub fn new() -> Self {
    Self
  }

  fn on_skill_shot(&mut self, ctx: &SystemContext) {
    ctx.play_scene(&completing_scene!("skill_shots/left_outlane_hit.scene"));

    // TODO: play sfx
    ctx.add_points(game::points::SKILL_SHOT);
    ctx.systems.despawn_self();
  }
}

impl System for LeftOutlaneSkillShot {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftOutlaneRollover>() {
      self.on_skill_shot(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<DoesNotCancelSkillshot>())
      && ctx.states.is(TurnPhase::Active)
    {
      ctx.systems.despawn_self();
    }
  }
}
