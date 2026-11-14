use frontbox_core::prelude::*;
use frontbox_led::prelude::*;

use crate::hardware::city_map;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::meridian_basins::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, RootLashed, SporesSystem};
use crate::systems::{Callout, SayExt, sounds};
use frontbox_sound::*;

static REQUIRED_HITS: u8 = 6;

/// How many neighboring shots are lit at once
const WINDOW: usize = 3;

pub struct MeridianBasinsMode {
  run: ModeRun,
  attention_scene: ScenePlayer,
  hit_scene: ScenePlayer,
  intensity_scene: ScenePlayer,
  hits: u8,
  /// The index in [`MainShot::ALL`] of the window's leftmost shot
  window: usize,
}

impl MeridianBasinsMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::MeridianBasins, REQUIRED_HITS),
      attention_scene: ScenePlayer::new(Self::attention_scene(&lit_shots(0))),
      hit_scene: ScenePlayer::new(Self::hit_scene()),
      intensity_scene: ScenePlayer::new(Self::intensity_scene()),
      hits: 0,
      window: 0,
    }
  }

  fn attention_scene(shots: &[MainShot]) -> PlayableScene {
    let qs: Vec<&LedQ> = shots.iter().map(|shot| shot.hex_center()).collect();
    scene!("meridian_basins/attention.scene", LedQ::any(qs))
  }

  fn hit_scene() -> PlayableScene {
    scene!("meridian_basins/hit.scene")
  }

  fn complete_scene() -> PlayableScene<Completes> {
    completing_scene!("meridian_basins/complete.scene")
  }

  fn intensity_scene() -> PlayableScene {
    scene!("meridian_basins/intensity.scene")
  }

  fn progress_scene(hits: u8) -> PlayableScene<Completes> {
    city_map::mode_progress_scene(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if !lit_shots(self.window).contains(&shot) {
      return;
    }

    let hits = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.add_hits(hits, ctx);
  }

  /// Hits from a lit shot, or from a Root Lash, each scoring
  fn add_hits(&mut self, hits: u8, ctx: &SystemContext) {
    self.hits = (self.hits + hits).min(REQUIRED_HITS);
    ctx.play_scene(&Self::progress_scene(self.hits));
    log::info!("MeridianBasins: hits = {}", self.hits);
    self.run.score_hit(hits, ctx);

    if self.hits == REQUIRED_HITS {
      log::info!("MeridianBasins: complete");
      self.run.complete(Self::complete_scene(), ctx);
      return;
    }

    ctx.play_sfx(sounds::city_hit(self.hits as f32 / REQUIRED_HITS as f32));
    // the first hit gets the mode's line, and after that the Steward counts them
    if self.hits == 1 {
      ctx.say(Callout::ChannelBreach);
    } else {
      ctx.say_count(self.hits, REQUIRED_HITS);
    }

    if self.run.progress(self.hits) == Some(50) {
      self.intensity_scene.start(ctx);
    }
    self.hit_scene.restart(ctx);
  }

  /// Move the window one shot to the right, wrapping back to the left orbit after the far right
  fn advance(&mut self, ctx: &SystemContext) {
    self.window = (self.window + 1) % MainShot::ALL.len();

    // the gate moves quietly while the Steward is talking
    if !ctx.callout_pending() {
      ctx.play_sfx(sounds::rnd_latch());
    }
    self
      .attention_scene
      .replace_scene(Self::attention_scene(&lit_shots(self.window)), ctx);
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
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.add_hits(lash.hits, ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.run.apply(ctx) {
      return;
    }

    self.attention_scene.start(ctx);
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
