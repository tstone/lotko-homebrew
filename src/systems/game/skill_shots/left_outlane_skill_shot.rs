use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::{GameManagementExt, GameManager, TurnState};

use crate::hardware::left_outlane::{self, LeftOutlaneRollover};
use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::hardware::right_pass_lane::RightPassLane;
use crate::hardware::{left_inlane, right_inlane, right_outlane};

// Secret skill shot to hit the left outlane without hitting anything else
#[derive(Clone)]
pub struct LeftOutlaneSkillShot {
  hit_effect: LedProgram1d,
  hit: bool,
}

impl LeftOutlaneSkillShot {
  pub fn new() -> Self {
    Self {
      hit_effect: Self::hit_effect().stopped(),
      hit: false,
    }
  }

  fn on_skill_shot(&mut self, ctx: &SystemContext) {
    self.hit = true;
    self.hit_effect.play();

    // TODO: play sfx
    ctx.add_points(500_000);
    ctx.despawn_self();
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::flash(
      LedQ::any(vec![
        &left_outlane::LED.q(),
        &left_inlane::TARGET_LED.q(),
        &right_inlane::ENTRANCE_LED.q(),
        &right_outlane::LED.q(),
      ]),
      ColorSequence::solid(Rgba::purple()),
      Cycle::Times(5),
    )
  }
}

impl System for LeftOutlaneSkillShot {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftOutlaneRollover>() {
      self.on_skill_shot(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<DoesNotCancelSkillshot>())
      && let Some(game_state) = ctx.expect::<GameManager>().game_state()
      && game_state.current_player_turn_state() == &TurnState::Active
    {
      ctx.despawn_self();
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.hit_effect.apply(delta, ctx);

    if self.hit && self.hit_effect.is_complete() {
      ctx.despawn_self();
    }
  }
}
