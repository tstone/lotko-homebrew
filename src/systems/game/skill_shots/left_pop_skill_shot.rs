use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::right_pass_lane::RightPassLane;
use crate::hardware::{pop_cluster, vspinner};

#[derive(Clone)]
pub struct LeftPopSkillShot {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  hit: bool,
}

impl LeftPopSkillShot {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect().stopped(),
      hit: false,
    }
  }

  fn on_skill_shot(&mut self, ctx: &SystemContext) {
    self.hit = true;
    self.hit_effect.play();

    // TODO: play sfx
    ctx.add_points(150_000);
    ctx.despawn_self();
  }

  fn attention_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      &*vspinner::left_ray::Q,
      ColorSequence::exact(vec![Rgba::red()]),
      Duration::from_millis(250),
      Curve::Linear,
      Cycle::Forever,
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      LedQ::any(vec![
        &*vspinner::left_ray::Q,
        &*vspinner::lower_right_ray::Q,
        &*vspinner::upper_right_ray::Q,
        &pop_cluster::left::POP_LED.q(),
        &pop_cluster::lower_right::POP_LED.q(),
        &pop_cluster::upper_right::POP_LED.q(),
      ]),
      ColorSequence::exact(vec![Rgba::red(), Rgba::default()]),
      Duration::from_millis(100),
      Curve::Linear,
      Cycle::Times(5),
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
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<RightPassLane>())
      && let Some(game_state) = ctx.expect::<GameManager>().game_state()
      && game_state.current_player_turn_state() == &TurnState::Active
    {
      // if any other switch that isn't the pass lane switches is hit, then this shot is no longer valid
      ctx.despawn_self();
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);

    if self.hit && self.hit_effect.is_complete() {
      ctx.despawn_self();
    }
  }
}
