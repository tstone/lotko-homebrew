use frontbox_core::prelude::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::*;

use crate::hardware::drop_bank::{self, DropBankTarget};
use crate::hardware::more_tags::DoesNotCancelSkillshot;
use crate::systems::sounds;

/// How long after an unlit target drops that the lit one still counts
const GRACE_PERIOD: Duration = Duration::from_millis(100);

#[derive(Clone)]
pub struct DropBankSkillShot {
  attention_effect: LedScene,
  enqueued_target: Option<DropBankTarget>,
  current_target: DropBankTarget,
  hit_effect: Option<LedScene>,
  hit: bool,
  /// An unlit target dropped, and the lit one has until `GracePeriodElapsed` to drop too
  grace_period: bool,
}

impl DropBankSkillShot {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(drop_bank::TARGET1_LEDS.q()),
      enqueued_target: None,
      current_target: DropBankTarget::Target1,
      hit_effect: None,
      hit: false,
      grace_period: false,
    }
  }

  fn on_target_hit(&mut self, hit_target: DropBankTarget, ctx: &SystemContext) {
    log::info!(
      "DropbankSkillShot: hit {:?} vs current {:?}",
      hit_target,
      self.current_target
    );

    if hit_target == self.current_target {
      self.hit = true;
      ctx.play_sfx(sounds::ARP_HIT1);
      ctx.add_points(500_000);
      // despawns from on_tick once the hit effect completes
      self.attention_effect.stop(ctx);
      self.hit_effect = Some(Self::hit_effect());
    } else if !self.grace_period {
      // A ball hitting the lit target near its edge can drop the neighbouring target a few ms first, so give the lit
      // target a moment to drop too before calling it a miss
      self.grace_period = true;
      ctx.cues.start(GracePeriodElapsed, Cue::Once(GRACE_PERIOD));
    }
  }

  fn on_grace_period_elapsed(&mut self, ctx: &SystemContext) {
    log::info!("DropbankSkillShot: missed {:?}", self.current_target);
    // consolation prize for still hitting the drop bank
    ctx.add_points(50_000);
    ctx.systems.despawn_self();
  }

  fn attention_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedScene {
    LedScene::new(target, led_scene!("skill_shots/drop_bank_attention.ron"))
  }

  fn hit_effect() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &drop_bank::TARGET1_LEDS.q(),
        &drop_bank::TARGET2_LEDS.q(),
        &drop_bank::TARGET3_LEDS.q(),
      ]),
      led_scene!("skill_shots/drop_bank_hit.ron"),
    )
  }

  fn next(&mut self, ctx: &SystemContext) {
    let next = self.current_target.next();

    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect(drop_bank::leds_for_target(&next).q());
    self.enqueued_target = Some(next);

    ctx
      .cues
      .start(NextTarget, Cue::Once(Duration::from_millis(1750)));
  }
}

impl System for DropBankSkillShot {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.next(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if self.hit {
      // already awarded, only the hit effect is left to play
      return;
    }

    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && let Some(target) = drop_bank::match_switch(&event.switch)
    {
      self.on_target_hit(target, ctx);
    } else if event.is::<GracePeriodElapsed>() {
      self.on_grace_period_elapsed(ctx);
    } else if self.grace_period {
      // until it elapses, the lit target holds and switches the ball rattles off near the bank don't cancel
    } else if event.is::<NextTarget>() {
      self.next(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && (event.switch.has_tag::<Playfield>() && !event.switch.has_tag::<DoesNotCancelSkillshot>())
      && ctx.states.is(TurnPhase::Active)
    {
      ctx.systems.despawn_self();
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if !self.hit {
      self.attention_effect.apply(delta, ctx);
    }

    // Change to the next target on tick so that it always aligns with the display
    if let Some(next) = self.enqueued_target.take() {
      log::info!("DropBankSkillShot: Changing to {:?}", next);
      self.current_target = next;
    }

    if let Some(hit_effect) = self.hit_effect.as_mut() {
      if self.hit && hit_effect.is_complete() {
        ctx.systems.despawn_self();
      }

      hit_effect.apply(delta, ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct NextTarget;

#[derive(serde::Serialize, Event)]
struct GracePeriodElapsed;
