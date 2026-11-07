use frontbox_core::prelude::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};

use crate::hardware::left_outlane::{self, LeftOutlaneRollover};
use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::hardware::right_pass_lane::RightPassLane;
use crate::hardware::{left_inlane, right_inlane, right_outlane};

// Secret skill shot to hit the left outlane without hitting anything else
#[derive(Clone)]
pub struct LeftOutlaneSkillShot {
  hit_effect: LedScene,
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
    ctx.systems.despawn_self();
  }

  fn hit_effect() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &left_outlane::LED.q(),
        &left_inlane::TARGET_LED.q(),
        &right_inlane::ENTRANCE_LED.q(),
        &right_outlane::LED.q(),
      ]),
      led_scene!("skill_shots/left_outlane_hit.ron"),
    )
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

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.hit_effect.apply(delta, ctx);

    if self.hit && self.hit_effect.is_complete() {
      ctx.systems.despawn_self();
    }
  }
}
