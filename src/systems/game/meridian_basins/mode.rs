use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::*;

use crate::hardware::city_map;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::meridian_basins::MODE_COLOR;
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, ModeRun, SporesSystem, TunnelCollected, points,
};
use crate::systems::{Callout, SayExt, sounds};
use frontbox_sound::*;

static REQUIRED_HITS: u8 = 6;

/// How many neighboring shots are lit at once
const WINDOW: usize = 3;

pub struct MeridianBasinsMode {
  run: ModeRun,
  attention_effect: LedPlayback,
  hit_effect: LedPlayback,
  intensity_effect: LedPlayback,
  hits: u8,
  /// The index in [`MainShot::ALL`] of the window's leftmost shot
  window: usize,
}

impl MeridianBasinsMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::MeridianBasins, REQUIRED_HITS),
      attention_effect: Self::attention_effect(&lit_shots(0)),
      hit_effect: Self::hit_effect(),
      intensity_effect: Self::intensity_effect(),
      hits: 0,
      window: 0,
    }
  }

  fn attention_effect(shots: &[MainShot]) -> LedPlayback {
    let qs: Vec<&LedQ> = shots.iter().map(|shot| shot.hex_center()).collect();
    led_playback!("meridian_basins/attention.scene", LedQ::any(qs))
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("meridian_basins/hit.scene").stopped()
  }

  fn complete_effect() -> LedPlayback {
    led_playback!("meridian_basins/complete.scene").stopped()
  }

  fn intensity_effect() -> LedPlayback {
    led_playback!("meridian_basins/intensity.scene").stopped()
  }

  fn progress_effect(hits: u8) -> LedPlayback {
    city_map::mode_progress_effect(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if !lit_shots(self.window).contains(&shot) {
      return;
    }

    let progress = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.add_hits(progress, ctx);
  }

  /// Hits from a lit shot, or from a root tunnel
  fn add_hits(&mut self, hits: u8, ctx: &SystemContext) {
    self.hits = (self.hits + hits).min(REQUIRED_HITS);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(Self::progress_effect(self.hits));
    log::info!("MeridianBasins: hits = {}", self.hits);
    ctx.add_points(points::EXL_MODE_HIT);

    if self.hits == REQUIRED_HITS {
      log::info!("MeridianBasins: complete");
      self
        .run
        .complete(points::EXL_COMPLETION, Self::complete_effect(), ctx);
      return;
    }

    ctx.play_sfx(sounds::city_hit(self.hits as f32 / REQUIRED_HITS as f32));
    ctx.say(if self.hits == 4 {
      Callout::FourOfSix
    } else {
      Callout::ChannelBreach
    });

    if self.run.progress(self.hits) == Some(50) {
      self.intensity_effect.play();
    }
    self.hit_effect.play();
  }

  /// Move the window one shot to the right, wrapping back to the left orbit after the far right
  fn advance(&mut self, ctx: &SystemContext) {
    self.window = (self.window + 1) % MainShot::ALL.len();

    ctx.play_sfx(sounds::rnd_latch());
    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect(&lit_shots(self.window));
  }
}

impl System for MeridianBasinsMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::MeridianBasins)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("MeridianBasins mode started");
    self.run.begin(ctx);
    ctx
      .cues
      .start(WindowMoveDue, Duration::from_millis(2250).forever());
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<WindowMoveDue>() {
      self.advance(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if event.is::<TunnelCollected>() {
      self.add_hits(1, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct WindowMoveDue;

/// The shots lit by a window starting at `window`
fn lit_shots(window: usize) -> Vec<MainShot> {
  (window..window + WINDOW)
    .map(|idx| MainShot::ALL[idx % MainShot::ALL.len()])
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_first_window_is_the_three_leftmost_shots() {
    assert_eq!(
      lit_shots(0),
      vec![MainShot::LeftOrbit, MainShot::DomeRamp, MainShot::ArcRamp]
    );
  }

  #[test]
  fn the_window_wraps_back_to_the_left() {
    assert_eq!(
      lit_shots(5),
      vec![
        MainShot::RightOrbit,
        MainShot::LeftOrbit,
        MainShot::DomeRamp
      ]
    );
  }
}
