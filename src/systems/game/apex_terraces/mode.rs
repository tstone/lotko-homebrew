use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::GameManagementExt;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, ModeRun, RootLashed, SporesSystem, flash_effect, glow_effect,
  points,
};
use crate::systems::{Callout, SayExt, sounds};

/// Hits each terrace takes to burst, in order. Bursting them all completes the mode.
const TERRACE_HITS: [u8; 3] = [3, 4, 5];

/// How long the ring holds a segment without a hit before losing it (start value)
const DRAIN_TIME: Duration = Duration::from_secs(6);

/// LEDs in a hex's outer ring
const RING_LEDS: usize = 6;

/// Overload (DESIGN.md). One main shot is lit gold, and each hit on it fills a segment of its hex's outer ring. A full
/// ring bursts the terrace and the lit shot moves to the shot farthest away. The ring loses a segment after a while
/// without a hit.
///
/// While it runs, the lit shot's ring shows overload over Spread roots, a z above them.
pub struct ApexTerracesMode {
  run: ModeRun,
  shot: MainShot,
  /// Terraces burst, which a resume keeps
  bursts: u8,
  /// Segments filled on the current terrace's ring
  filled: u8,
  center_effect: LedPlayback,
  ring_effect: LedPlayback,
  drain_cue: Option<u64>,
}

impl ApexTerracesMode {
  pub fn new() -> Self {
    let shot = *MainShot::ALL
      .choose(&mut rand::rng())
      .unwrap_or(&MainShot::CenterOrbit);
    Self {
      run: ModeRun::new(Mode::ApexTerraces, TERRACE_HITS.iter().sum()),
      shot,
      bursts: 0,
      filled: 0,
      center_effect: Self::center_effect(shot),
      ring_effect: Self::ring_effect(shot, 0, TERRACE_HITS[0]),
      drain_cue: None,
    }
  }

  fn center_effect(shot: MainShot) -> LedPlayback {
    glow_effect(shot.hex_center(), Mode::ApexTerraces.color())
  }

  /// The ring in dim gold, with `filled` of `needed` segments green
  fn ring_effect(shot: MainShot, filled: u8, needed: u8) -> LedPlayback {
    let lit = ring_leds(filled, needed);
    let width = 2.0 * (lit as f32 - 0.5) / (RING_LEDS as f32 - 1.0);
    led_playback!("apex_terraces/ring.scene", shot.hex_ring().clone())
      .param("color", Rgba::green())
      .param("width", width.max(0.0))
  }

  fn needed(&self) -> u8 {
    TERRACE_HITS[(self.bursts as usize).min(TERRACE_HITS.len() - 1)]
  }

  /// Hits so far toward the end of the mode
  fn done(&self) -> u8 {
    TERRACE_HITS[..self.bursts as usize].iter().sum::<u8>() + self.filled
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if shot == self.shot {
      let hits = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
      self.add_hits(hits, ctx);
    }
  }

  /// Hits on the lit shot, or from a Root Lash, each scoring
  fn add_hits(&mut self, hits: u8, ctx: &SystemContext) {
    if self.done() == 0 {
      ctx.say(Callout::ThisIsAnExclusiveArea);
    }
    ctx.add_points(points::APEX_HIT * hits as u32);
    self.filled = (self.filled + hits).min(self.needed());
    log::info!(
      "ApexTerraces: terrace {} at {} of {}",
      self.bursts + 1,
      self.filled,
      self.needed()
    );

    if self.filled == self.needed() {
      self.burst(ctx);
      return;
    }

    ctx.play_sfx(sounds::city_hit(self.filled as f32 / self.needed() as f32));
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(self.shot.hex_center(), Rgba::white()));
    self.run.progress(self.done());
    self.render_ring(ctx);
    self.restart_drain(ctx);
  }

  fn burst(&mut self, ctx: &SystemContext) {
    self.bursts += 1;
    self.filled = 0;
    log::info!("ApexTerraces: terrace {} burst", self.bursts);
    ctx.add_points(points::APEX_BURST * self.bursts as u32);
    ctx.play_sfx(sounds::IMPACT_DIRT);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(self.shot.hex_ring(), Rgba::green()));

    if self.bursts as usize == TERRACE_HITS.len() {
      self.cancel_drain(ctx);
      self.center_effect.stop(ctx);
      self.ring_effect.stop(ctx);
      self
        .run
        .complete(led_playback!("apex_terraces/start.scene"), ctx);
      return;
    }

    ctx.say(Callout::WarningStructuralConcern);
    self.run.progress(self.done());
    self.move_to(self.shot.farthest_from(), ctx);
    self.cancel_drain(ctx);
  }

  fn move_to(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.center_effect.stop(ctx);
    self.ring_effect.stop(ctx);
    self.shot = shot;
    self.center_effect = Self::center_effect(shot);
    self.render_ring(ctx);
  }

  fn render_ring(&mut self, ctx: &SystemContext) {
    self.ring_effect.stop(ctx);
    self.ring_effect = Self::ring_effect(self.shot, self.filled, self.needed());
  }

  fn drain(&mut self, ctx: &SystemContext) {
    self.drain_cue = None;
    if self.filled == 0 {
      return;
    }
    self.filled -= 1;
    log::info!("ApexTerraces: ring drained to {}", self.filled);
    ctx.play_sfx(sounds::HIT_ORGANIC_LOW3);
    self.render_ring(ctx);
    if self.filled > 0 {
      self.restart_drain(ctx);
    }
  }

  fn restart_drain(&mut self, ctx: &SystemContext) {
    self.cancel_drain(ctx);
    self.drain_cue = Some(ctx.cues.start(DrainDue, DRAIN_TIME.once()));
  }

  fn cancel_drain(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.drain_cue.take() {
      ctx.cues.cancel(cue);
    }
  }
}

impl System for ApexTerracesMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::ApexTerraces)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
  }

  /// A resume keeps the terraces burst, and starts the current one's ring over
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.filled = 0;
    self.run.reset_progress();
    self.render_ring(ctx);
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.cancel_drain(ctx);
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<DrainDue>() {
      self.drain(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.add_hits(lash.hits, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }

    self.center_effect.apply(delta, ctx);
    self.ring_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct DrainDue;

/// How many of the ring's LEDs show `filled` of `needed` segments
fn ring_leds(filled: u8, needed: u8) -> usize {
  (filled as usize * RING_LEDS + needed as usize / 2) / (needed as usize).max(1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    ApexTerracesMode::ring_effect(MainShot::LeftOrbit, 2, 4);
  }

  #[test]
  fn the_ring_fills_in_proportion_to_the_terrace() {
    assert_eq!(ring_leds(0, 3), 0);
    assert_eq!(ring_leds(1, 3), 2);
    assert_eq!(ring_leds(2, 4), 3);
    assert_eq!(ring_leds(4, 5), 5);
    assert_eq!(ring_leds(5, 5), 6);
  }

  #[test]
  fn progress_counts_burst_terraces_and_the_current_ring() {
    let mut mode = ApexTerracesMode::new();
    mode.bursts = 2;
    mode.filled = 1;
    assert_eq!(mode.done(), 8);
    assert_eq!(mode.needed(), 5);
  }
}
