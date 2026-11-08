use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::side_quests::TunnelCollected;
use crate::systems::game::{EffectsSystem, ModeManager, Perk, fill_effect, flash_effect, points};
use crate::systems::sounds;

/// The orbits, which each dig a tunnel. Whether the center orbit counts is still to confirm (PLAN.md, decision 8).
const TUNNELS: [MainShot; 3] = [
  MainShot::LeftOrbit,
  MainShot::CenterOrbit,
  MainShot::RightOrbit,
];

/// Segments in a hex's outer ring
const SEGMENTS: u8 = 6;

/// Orbit hits dig root tunnels (DESIGN.md, Root tunnels), filling the orbit hex's outer ring a segment a hit, or two
/// with the Meridian perk. A full ring collects the tunnel and adds a hit of progress to the current district
/// ([`TunnelCollected`]).
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct RootTunnelsSystem {
  /// By tunnel, in [`TUNNELS`] order
  dug: [u8; 3],
  ring_effects: [LedPlayback; 3],
}

impl RootTunnelsSystem {
  pub fn new() -> Self {
    Self {
      dug: [0; 3],
      ring_effects: TUNNELS.map(|tunnel| Self::ring_effect(tunnel, 0)),
    }
  }

  fn ring_effect(tunnel: MainShot, dug: u8) -> LedPlayback {
    fill_effect(
      tunnel.hex_ring(),
      SEGMENTS as usize,
      dug as usize,
      Rgba::bronze(),
    )
  }

  fn dig(&mut self, shot: MainShot, ctx: &SystemContext) {
    let Some(index) = TUNNELS.iter().position(|tunnel| *tunnel == shot) else {
      return;
    };

    let perk = ctx
      .systems
      .expect::<ModeManager>()
      .has_perk(Perk::DoubleTunnels);
    let dug = (self.dug[index] + if perk { 2 } else { 1 }).min(SEGMENTS);
    ctx.add_points(points::TUNNEL_SEGMENT);

    if dug < SEGMENTS {
      ctx.play_sfx(sounds::HIT_ORGANIC_LOW1);
      self.dug[index] = dug;
    } else {
      log::info!("RootTunnels: {shot:?} collected");
      ctx.add_points(points::TUNNEL_COLLECT);
      ctx.play_sfx(sounds::IMPACT_DIRT);
      ctx
        .systems
        .expect::<EffectsSystem>()
        .play(flash_effect(shot.hex_ring(), Rgba::bronze()));
      ctx.emit(TunnelCollected);
      self.dug[index] = 0;
    }
    self.ring_effects[index] = Self::ring_effect(shot, self.dug[index]);
  }
}

impl System for RootTunnelsSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.dig(hit.shot, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    for effect in self.ring_effects.iter_mut() {
      effect.apply(delta, ctx);
    }
  }
}
