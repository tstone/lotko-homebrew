use frontbox_core::prelude::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::*;

use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::hardware::{pop_cluster, vspinner};
use crate::systems::sounds;

#[derive(Clone)]
pub struct LeftPopSkillShot {
  attention_effect: LedScene,
  hit_effect: Option<LedScene>,
  hit: bool,
}

impl LeftPopSkillShot {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: None,
      hit: false,
    }
  }

  fn on_skill_shot(&mut self, ctx: &SystemContext) {
    self.hit = true;
    self.hit_effect = Some(Self::hit_effect());

    ctx.play_sfx(sounds::ARP_HIT1);
    ctx.add_points(500_000);
    ctx.systems.despawn_self();
  }

  fn attention_effect() -> LedScene {
    LedScene::unbound(led_scene!("skill_shots/left_pop_attention.ron"))
      .on("ray", vspinner::left_ray::Q.clone().reverse())
      .unwrap()
      .on("pop", pop_cluster::left::POP_LED.q())
      .unwrap()
  }

  fn hit_effect() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &*vspinner::left_ray::Q,
        &*vspinner::lower_right_ray::Q,
        &*vspinner::upper_right_ray::Q,
        &pop_cluster::left::POP_LED.q(),
        &pop_cluster::lower_right::POP_LED.q(),
        &pop_cluster::upper_right::POP_LED.q(),
      ]),
      led_scene!("skill_shots/left_pop_hit.ron"),
    )
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

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);

    if let Some(hit_effect) = self.hit_effect.as_mut() {
      hit_effect.apply(delta, ctx);

      if self.hit && hit_effect.is_complete() {
        ctx.systems.despawn_self();
      }
    }
  }
}
