use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_pinball::{GameManagerExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::captive_ball::{self, CaptiveBallHit};
use crate::systems::game::{ModeManager, Multiball, Perk, heartbeat_scene, points};
use crate::systems::sounds;

/// Charge for the first multiball (start value)
const FIRST_CHARGE: u8 = 4;

/// Charge for the second multiball, a bigger jump than the ones after (start value)
const SECOND_CHARGE: u8 = 8;

/// How much more each multiball after the second needs (start value)
const CHARGE_STEP: u8 = 2;

/// The bolts' beat with no charge, which speeds up to [`FULL_BEAT`]
const EMPTY_BEAT: Duration = Duration::from_millis(1400);
const FULL_BEAT: Duration = Duration::from_millis(500);

/// The captive ball charges the heart of the oak (DESIGN.md, Heartwood), and the bolts beat faster as it does. Full
/// charge starts Heartwood multiball, and each one after needs more: 4, 8, 10, 12, and so on. Hits count double with the Solarium perk.
///
/// Waits while another multiball runs, as Heartwood has the captive ball to itself.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct HeartwoodCharge {
  charge: u8,
  required: u8,
  beat_scene: ScenePlayer,
}

impl HeartwoodCharge {
  pub fn new() -> Self {
    Self {
      charge: 0,
      required: FIRST_CHARGE,
      beat_scene: ScenePlayer::new(Self::beat_scene(0.0)),
    }
  }

  fn beat_scene(charged: f32) -> PlayableScene {
    let beat = EMPTY_BEAT.as_secs_f32() - (EMPTY_BEAT - FULL_BEAT).as_secs_f32() * charged;
    let bolts = LightQ::any(vec![
      &captive_ball::LEFT_BOLT.q(),
      &captive_ball::RIGHT_BOLT.q(),
    ]);
    heartbeat_scene(&bolts, Rgba::lime(), Duration::from_secs_f32(beat))
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
      self.beat_scene.replace_scene(
        Self::beat_scene(self.charge as f32 / self.required as f32),
        ctx,
      );
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
      self.required = next_required(self.required);
      self.beat_scene.replace_scene(Self::beat_scene(0.0), ctx);
      ctx.play_scene(&completing_scene!(
        "heartwood_multiball/startable_hit.scene"
      ));
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

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    self.beat_scene.start(ctx);
  }
}

/// The charge the next multiball needs, after one that needed `required`
fn next_required(required: u8) -> u8 {
  if required == FIRST_CHARGE {
    SECOND_CHARGE
  } else {
    required.saturating_add(CHARGE_STEP)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn each_multiball_needs_more_charge_skipping_six() {
    let mut required = FIRST_CHARGE;
    let mut seen = vec![required];
    for _ in 0..3 {
      required = next_required(required);
      seen.push(required);
    }
    assert_eq!(seen, [4, 8, 10, 12]);
  }
}
