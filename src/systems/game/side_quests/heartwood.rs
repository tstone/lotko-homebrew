use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::captive_ball::{self, CaptiveBallHit};
use crate::systems::game::{
  EffectsSystem, ModeManager, Multiball, Perk, SporeMultiballMode, heartbeat_effect, points,
};
use crate::systems::sounds;

/// Charge for the first multiball (start value)
const FIRST_CHARGE: u8 = 4;

/// How much more each multiball after that needs (start value)
const CHARGE_STEP: u8 = 2;

/// The bolts' beat with no charge, which speeds up to [`FULL_BEAT`]
const EMPTY_BEAT: Duration = Duration::from_millis(1400);
const FULL_BEAT: Duration = Duration::from_millis(500);

/// The captive ball charges the heart of the oak (DESIGN.md, Heartwood), and the bolts beat faster as it does. Full
/// charge starts Heartwood multiball, and each one after needs more. Hits count double with the Solarium perk.
///
/// Waits while any multiball runs, as Heartwood doesn't stack with Groundswell and has the captive ball to itself.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct HeartwoodCharge {
  charge: u8,
  required: u8,
  beat_effect: LedPlayback,
}

impl HeartwoodCharge {
  pub fn new() -> Self {
    Self {
      charge: 0,
      required: FIRST_CHARGE,
      beat_effect: Self::beat_effect(0.0),
    }
  }

  fn beat_effect(charged: f32) -> LedPlayback {
    let beat = EMPTY_BEAT.as_secs_f32() - (EMPTY_BEAT - FULL_BEAT).as_secs_f32() * charged;
    let bolts = LedQ::any(vec![
      &captive_ball::LEFT_BOLT.q(),
      &captive_ball::RIGHT_BOLT.q(),
    ]);
    heartbeat_effect(&bolts, Rgba::lime(), Duration::from_secs_f32(beat))
  }

  fn on_hit(&mut self, ctx: &SystemContext) {
    let perk = ctx
      .systems
      .expect::<ModeManager>()
      .has_perk(Perk::FasterHeartCharge);
    self.charge = (self.charge + if perk { 2 } else { 1 }).min(self.required);
    log::info!("Heartwood: {} of {}", self.charge, self.required);
    ctx.add_points(points::HEARTWOOD_CHARGE);

    if self.charge < self.required {
      ctx.play_sfx(sounds::city_hit(self.charge as f32 / self.required as f32));
      self.beat_effect.stop(ctx);
      self.beat_effect = Self::beat_effect(self.charge as f32 / self.required as f32);
      return;
    }

    if ctx
      .systems
      .expect::<ModeManager>()
      .take_multiball(Multiball::Heartwood, ctx.into())
      .is_ok()
    {
      log::info!("Heartwood: multiball");
      self.charge = 0;
      self.required += CHARGE_STEP;
      self.beat_effect.stop(ctx);
      self.beat_effect = Self::beat_effect(0.0);
      ctx.play_sfx(sounds::BRAAM_BIO2);
      ctx
        .systems
        .expect::<EffectsSystem>()
        .play(led_playback!("spore_multiball/startable_hit.scene"));
      ctx.systems.spawn(SporeMultiballMode::new());
    }
  }
}

impl System for HeartwoodCharge {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
      && ctx
        .systems
        .expect::<ModeManager>()
        .current_multiball()
        .is_none()
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CaptiveBallHit>() {
      self.on_hit(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.beat_effect.apply(delta, ctx);
  }
}
