use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::hardware::drop_bank;
use crate::hardware::right_pass_lane::RightPassLane;

#[derive(Clone)]
pub struct DropBankSkillShot {
  attention_effect: LedProgram1d,
  target: u8,
  hit_effect: LedProgram1d,
  hit: bool,
}

impl DropBankSkillShot {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(drop_bank::TARGET1_LEDS.q()),
      target: 0,
      hit_effect: Self::hit_effect().stopped(),
      hit: false,
    }
  }

  fn on_skill_shot(&mut self, ctx: &SystemContext) {
    self.hit = true;
    self.hit_effect.play();

    // TODO: play sfx
    ctx.add_points(50_000);
    ctx.despawn_self();
  }

  fn attention_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedProgram1d {
    LedProgram1d::flash(
      target,
      ColorSequence::fade(Rgba::red(), Rgba::blue()),
      Cycle::Forever,
    )
  }

  fn hit_effect() -> LedProgram1d {
    LedProgram1d::rotating(
      LedQ::any(vec![
        &drop_bank::TARGET1_LEDS.q(),
        &drop_bank::TARGET2_LEDS.q(),
        &drop_bank::TARGET3_LEDS.q(),
      ]),
      ColorSequence::fade(Rgba::red(), Rgba::blue()),
      Duration::from_millis(100),
      Curve::Linear,
      Cycle::Times(3),
    )
  }

  fn next(&mut self, ctx: &SystemContext) {
    self.attention_effect.stop(ctx);

    self.target += 1;
    if self.target == 4 {
      self.target = 1;
    }

    self.attention_effect = match self.target {
      1 => Self::attention_effect(drop_bank::TARGET1_LEDS.q()),
      2 => Self::attention_effect(drop_bank::TARGET2_LEDS.q()),
      3 => Self::attention_effect(drop_bank::TARGET1_LEDS.q()),
      _ => panic!("Cannot switch to unknown target"),
    };

    ctx.cue(Next, Cue::Once(Duration::from_millis(1000)));
  }
}

impl System for DropBankSkillShot {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.next(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && ((event.switch.name == drop_bank::TARGET1.name && self.target == 1)
        || (event.switch.name == drop_bank::TARGET2.name && self.target == 2)
        || (event.switch.name == drop_bank::TARGET3.name && self.target == 3))
    {
      self.on_skill_shot(ctx);
    } else if event.is::<Next>() {
      self.next(ctx);
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

#[derive(serde::Serialize, Event)]
struct Next;
