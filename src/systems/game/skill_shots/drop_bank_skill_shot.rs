use frontbox::animation::Curve;
use frontbox::prelude::tags::Playfield;
use frontbox::prelude::*;
use frontbox_sound::*;
use frontbox_turn_based::*;

use crate::hardware::drop_bank::{self, DropBankTarget};
use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::systems::sounds;

#[derive(Clone)]
pub struct DropBankSkillShot {
  attention_effect: LedProgram1d,
  enqueued_target: Option<DropBankTarget>,
  current_target: DropBankTarget,
  hit_effect: Option<LedProgram1d>,
  hit: bool,
}

impl DropBankSkillShot {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(drop_bank::TARGET1_LEDS.q()),
      enqueued_target: None,
      current_target: DropBankTarget::Target1,
      hit_effect: None,
      hit: false,
    }
  }

  fn on_target_hit(&mut self, hit_target: DropBankTarget, ctx: &SystemContext) {
    self.hit = true;

    log::info!(
      "DropbankSkillShot: hit {:?} vs current {:?}",
      hit_target,
      self.current_target
    );

    if hit_target == self.current_target {
      ctx.play_sfx(sounds::ARP_HIT1);
      ctx.add_points(500_000);
      self.hit_effect = Some(Self::hit_effect());
    } else {
      // consolation prize for still hitting the drop bank
      ctx.add_points(50_000);
    }

    ctx.despawn_self();
  }

  fn attention_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedProgram1d {
    LedProgram1d::flash(
      target,
      ColorSequence::fade(Rgba::purple(), Rgba::blue()),
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
    self.enqueued_target = Some(self.current_target.next());
    self.attention_effect.stop(ctx);
    self.attention_effect =
      Self::attention_effect(drop_bank::leds_for_target(&self.current_target).q());

    ctx.cue(NextTarget, Cue::Once(Duration::from_millis(1750)));
  }
}

impl System for DropBankSkillShot {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.next(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && let Some(target) = drop_bank::match_switch(&event.switch)
    {
      self.on_target_hit(target, ctx);
    } else if event.is::<NextTarget>() {
      self.next(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<DoesNotCancelSkillshot>())
      && let Some(game_state) = ctx.expect::<GameManager>().game_state()
      && game_state.current_player_turn_state() == &TurnState::Active
    {
      ctx.despawn_self();
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);

    // Change to the next target on tick so that it always aligns with the display
    if let Some(next) = self.enqueued_target.as_ref() {
      log::info!("DropBankSkillShot: Changing to {:?}", self.current_target);
      self.current_target = next.clone();
      self.enqueued_target = None;
    }

    if let Some(hit_effect) = self.hit_effect.as_mut() {
      if self.hit && hit_effect.is_complete() {
        ctx.despawn_self();
      }

      hit_effect.apply(delta, ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct NextTarget;
