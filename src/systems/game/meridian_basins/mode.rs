use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use rand::prelude::IndexedRandom;

use crate::hardware::arc_ramp;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::meridian_basins::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, RootLashed, SporesSystem, fill_scene};
use crate::systems::sounds;
use frontbox_sound::*;

/// Basins seeded at the start
const SEEDS: usize = 2;

/// Basins to bloom to complete it: every main shot
const REQUIRED_HITS: u8 = MainShot::ALL.len() as u8;

/// Time without a bloom before the last one withers
const WITHER_AFTER: Duration = Duration::from_secs(15);

/// The chance a wither that's due takes the last bloom
const WITHER_CHANCE: f64 = 0.5;

/// LEDs on the arc ramp, which show the progress
const ARC_LEDS: usize = 18;

/// The z of a basin's status on its hex center, the mode attention z
const BASIN_Z: i8 = 10;

/// The hydroponic farms (design/city/meridian-basins.md). Two basins are seeded at random, and they flash. Hitting a
/// flashing basin blooms it, and the growth spreads to the shots on either side, which start flashing. Blooming all six
/// completes it. Going 15 seconds without a bloom has an even chance to wither the last one, along with the spread it
/// gave.
///
/// Every bloom fills a sixth of the arc ramp.
pub struct MeridianBasinsMode {
  run: ModeRun,
  seeds: Vec<MainShot>,
  /// In the order they bloomed, so the last one withers first
  bloomed: Vec<MainShot>,
  wither_timer: Option<u64>,
  progress_scene: ScenePlayer,
}

impl MeridianBasinsMode {
  pub fn new() -> Self {
    let seeds = MainShot::ALL
      .sample(&mut rand::rng(), SEEDS)
      .copied()
      .collect();
    Self {
      run: ModeRun::new(Mode::MeridianBasins, REQUIRED_HITS),
      seeds,
      bloomed: Vec::new(),
      wither_timer: None,
      progress_scene: ScenePlayer::new(Self::progress_scene(0)),
    }
  }

  fn progress_scene(blooms: usize) -> PlayableScene {
    let lit = blooms * ARC_LEDS / REQUIRED_HITS as usize;
    fill_scene(&arc_ramp::ARC_LEDS.q(), ARC_LEDS, lit, *MODE_COLOR)
  }

  fn bloom_scene(shot: MainShot) -> PlayableScene<Completes> {
    completing_scene!("meridian_basins/bloom.scene").variable("origin", shot.origin())
  }

  fn complete_scene() -> PlayableScene<Completes> {
    completing_scene!("meridian_basins/complete.scene")
  }

  /// A seed, or next to a bloom, and not bloomed itself
  fn is_lit(&self, shot: MainShot) -> bool {
    !self.bloomed.contains(&shot)
      && (self.seeds.contains(&shot)
        || shot
          .neighbors()
          .iter()
          .any(|neighbor| self.bloomed.contains(neighbor)))
  }

  fn lit(&self) -> Vec<MainShot> {
    MainShot::ALL
      .into_iter()
      .filter(|shot| self.is_lit(*shot))
      .collect()
  }

  /// The hit shot blooms, and any further hits from a spore burst bloom other lit shots
  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if !self.is_lit(shot) {
      return;
    }
    let hits = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.bloom(shot, ctx);
    self.bloom_random(hits - 1, ctx);
    self.after_blooms(hits, ctx);
  }

  /// A Root Lash's hits bloom lit shots at random
  fn on_lash(&mut self, hits: u8, ctx: &SystemContext) {
    self.bloom_random(hits, ctx);
    self.after_blooms(hits, ctx);
  }

  fn bloom_random(&mut self, times: u8, ctx: &SystemContext) {
    for _ in 0..times {
      if let Some(shot) = self.lit().choose(&mut rand::rng()).copied() {
        self.bloom(shot, ctx);
      }
    }
  }

  fn bloom(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.bloomed.push(shot);
    log::info!(
      "MeridianBasins: {shot:?} bloomed, {} of {REQUIRED_HITS}",
      self.bloomed.len()
    );
    ctx.play_scene(&Self::bloom_scene(shot));
    ctx.play_sfx(sounds::GUITAR_SWELL);
  }

  fn after_blooms(&mut self, hits: u8, ctx: &SystemContext) {
    self.run.score_hit(hits, ctx);
    if self.bloomed.len() >= REQUIRED_HITS as usize {
      log::info!("MeridianBasins: complete");
      ctx.timers.cancel(self.wither_timer.take());
      self.progress_scene.stop(ctx);
      self.run.complete(Self::complete_scene(), ctx);
      return;
    }
    self.restart_wither_timer(ctx);
    self.render(ctx);
  }

  /// Half the time, the last bloom goes back to lit, and the shots it spread to go dark unless something else lights
  /// them. Otherwise it's spared until the next wither is due.
  fn wither(&mut self, ctx: &SystemContext) {
    self.wither_timer = None;
    let Some(shot) = self.bloomed.last().copied() else {
      return;
    };
    if !rand::random_bool(WITHER_CHANCE) {
      log::info!("MeridianBasins: {shot:?} spared from withering");
      self.restart_wither_timer(ctx);
      return;
    }
    self.bloomed.pop();
    log::info!(
      "MeridianBasins: {shot:?} withered, {} left",
      self.bloomed.len()
    );
    if !self.bloomed.is_empty() {
      self.restart_wither_timer(ctx);
    }
    self.render(ctx);
  }

  fn restart_wither_timer(&mut self, ctx: &SystemContext) {
    ctx.timers.cancel(self.wither_timer.take());
    self.wither_timer = Some(ctx.timers.start(WitherDue, WITHER_AFTER.once()));
  }

  /// A basin that can be hit flashes on its hex center, and a bloomed one goes dark as it's done. The arc ramp shows
  /// the progress.
  fn render(&mut self, ctx: &SystemContext) {
    for shot in MainShot::ALL {
      let leds = shot.hex_center();
      if self.is_lit(shot) {
        ctx.declare_lights_flashing(leds, BASIN_Z, *MODE_COLOR);
      } else {
        ctx.undeclare_lights(leds, BASIN_Z);
      }
    }
    self
      .progress_scene
      .replace_scene(Self::progress_scene(self.bloomed.len()), ctx);
  }
}

impl System for MeridianBasinsMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::MeridianBasins)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("MeridianBasins: started, seeded {:?}", self.seeds);
    self.run.begin(ctx);
    self.render(ctx);
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.run.interrupt(ctx);
    ctx.deactivate_light_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<WitherDue>() {
      self.wither(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.on_lash(lash.hits, ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.run.apply(ctx) {
      return;
    }

    self.progress_scene.start(ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct WitherDue;

#[cfg(test)]
mod tests {
  use super::*;

  fn mode(seeds: &[MainShot]) -> MeridianBasinsMode {
    let mut mode = MeridianBasinsMode::new();
    mode.seeds = seeds.to_vec();
    mode
  }

  #[test]
  fn two_different_basins_are_seeded() {
    let mode = MeridianBasinsMode::new();
    assert_eq!(mode.seeds.len(), SEEDS);
    assert_ne!(mode.seeds[0], mode.seeds[1]);
    assert_eq!(mode.lit(), {
      let mut seeds = mode.seeds.clone();
      seeds.sort_by_key(|shot| *shot as usize);
      seeds
    });
  }

  #[test]
  fn a_bloom_spreads_to_the_shots_on_either_side() {
    let mut mode = mode(&[MainShot::ArcRamp, MainShot::RightOrbit]);
    mode.bloomed.push(MainShot::ArcRamp);
    assert_eq!(
      mode.lit(),
      vec![
        MainShot::DomeRamp,
        MainShot::CenterOrbit,
        MainShot::RightOrbit
      ]
    );
  }

  #[test]
  fn there_is_always_a_lit_shot_until_all_have_bloomed() {
    let mut mode = mode(&[MainShot::LeftOrbit, MainShot::DomeRamp]);
    while let Some(shot) = mode.lit().first().copied() {
      mode.bloomed.push(shot);
    }
    assert_eq!(mode.bloomed.len(), REQUIRED_HITS as usize);
  }

  #[test]
  fn withering_takes_back_the_spread() {
    let mut mode = mode(&[MainShot::LeftOrbit, MainShot::RightOrbit]);
    mode.bloomed.push(MainShot::LeftOrbit);
    assert!(mode.is_lit(MainShot::DomeRamp));
    mode.bloomed.pop();
    assert!(!mode.is_lit(MainShot::DomeRamp));
    assert!(mode.is_lit(MainShot::LeftOrbit));
  }

  #[test]
  fn every_bloom_fills_a_sixth_of_the_arc() {
    assert_eq!(REQUIRED_HITS, 6);
    assert_eq!(1 * ARC_LEDS / REQUIRED_HITS as usize, 3);
  }

  #[test]
  fn the_scenes_load() {
    MeridianBasinsMode::progress_scene(REQUIRED_HITS as usize);
    MeridianBasinsMode::bloom_scene(MainShot::DomeRamp);
    MeridianBasinsMode::complete_scene();
  }
}
