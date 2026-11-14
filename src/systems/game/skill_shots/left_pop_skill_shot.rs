use frontbox_core::prelude::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::*;

use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::hardware::pop_cluster;
use crate::systems::game;
use crate::systems::sounds;

#[derive(Clone)]
pub struct LeftPopSkillShot {
  attention_scene: ScenePlayer,
}

impl LeftPopSkillShot {
  pub fn new() -> Self {
    Self {
      attention_scene: ScenePlayer::new(Self::attention_scene()),
    }
  }

  fn on_skill_shot(&mut self, ctx: &SystemContext) {
    ctx.play_scene(&Self::hit_scene());

    ctx.play_sfx(sounds::ARP_HIT1);
    ctx.add_points(game::points::SKILL_SHOT);
    ctx.systems.despawn_self();
  }

  fn attention_scene() -> PlayableScene {
    scene!("skill_shots/left_pop_attention.scene")
  }

  fn hit_scene() -> PlayableScene<Completes> {
    completing_scene!("skill_shots/left_pop_hit.scene")
  }
}

impl System for LeftPopSkillShot {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == pop_cluster::left::SPOON_SWITCH.name
    {
      self.on_skill_shot(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<DoesNotCancelSkillshot>())
      && ctx.states.is(TurnPhase::Active)
    {
      log::info!(
        "LeftPopSkillShot: killed due to switch {}",
        event.switch.name
      );
      ctx.systems.despawn_self();
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    self.attention_scene.start(ctx);
  }
}
