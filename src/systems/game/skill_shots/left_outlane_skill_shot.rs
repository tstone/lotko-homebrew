use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::{GameManagementExt, GameManager, TurnState};

use crate::hardware::left_outlane::LeftOutlaneRollover;
use crate::hardware::right_pass_lane::RightPassLane;

// Secret skill shot to hit the left outlane without hitting anything else
pub struct LeftOutlaneSkillShot {}

// TODO
impl LeftOutlaneSkillShot {
  pub fn new() -> Self {
    Self {}
  }

  fn on_skill_shot(&self, ctx: &SystemContext) {
    // TODO: play sfx
    ctx.add_points(500_000);
    // TODO: lighting effect
  }
}

impl System for LeftOutlaneSkillShot {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftOutlaneRollover>() {
      self.on_skill_shot(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<RightPassLane>())
      && let Some(game_state) = ctx.expect::<GameManager>().game_state()
      && game_state.current_player_turn_state() == &TurnState::Active
    {
      // if any other switch that isn't the pass lane switches is hit, then this shot is no longer valid
      ctx.despawn_self();
    }
  }
}
