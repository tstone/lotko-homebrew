use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use rand::prelude::IndexedRandom;

use crate::hardware::arc_ramp;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::meridian_basins::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, RootLashed, SporesSystem, fill_scene};
use crate::systems::sounds;
use frontbox_sound::*;

/// Basins lit at the start
const BASINS: usize = 4;

/// Basins to burst to complete it
const BURSTS: usize = 3;

/// Hits to complete it: a plant and a burst for each basin
const REQUIRED_HITS: u8 = 2 * BURSTS as u8;

/// LEDs on the arc ramp, which show the progress
const ARC_LEDS: usize = 18;

/// The z of a basin's status on its hex center, the mode attention z
const BASIN_Z: i8 = 10;

/// The hydroponic farms (design/city/meridian-basins.md). Four basins light at random, and each takes two hits: one to
/// plant a seed, which sets it flashing, and one to burst it. Bursting three completes it. Every hit fills a sixth of the
/// arc ramp.
pub struct MeridianBasinsMode {
  run: ModeRun,
  basins: Vec<Basin>,
  progress_scene: ScenePlayer,
}

impl MeridianBasinsMode {
  pub fn new() -> Self {
    let basins = MainShot::ALL
      .sample(&mut rand::rng(), BASINS)
      .map(|shot| Basin { shot: *shot, state: BasinState::Lit })
      .collect();
    Self {
      run: ModeRun::new(Mode::MeridianBasins, REQUIRED_HITS),
      basins,
      progress_scene: ScenePlayer::new(Self::progress_scene(0)),
    }
  }

  fn progress_scene(hits: u8) -> PlayableScene {
    let lit = hits as usize * ARC_LEDS / REQUIRED_HITS as usize;
    fill_scene(&arc_ramp::ARC_LEDS.q(), ARC_LEDS, lit, *MODE_COLOR)
  }

  fn burst_scene(shot: MainShot) -> PlayableScene<Completes> {
    completing_scene!("meridian_basins/burst.scene").variable("origin", shot.origin())
  }

  fn complete_scene() -> PlayableScene<Completes> {
    completing_scene!("meridian_basins/complete.scene")
  }

  fn hits(&self) -> u8 {
    self.basins.iter().map(|basin| basin.state.hits()).sum()
  }

  fn bursts(&self) -> usize {
    self
      .basins
      .iter()
      .filter(|basin| basin.state == BasinState::Burst)
      .count()
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    let Some(index) = self
      .basins
      .iter()
      .position(|basin| basin.shot == shot && basin.state != BasinState::Burst)
    else {
      return;
    };
    let hits = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    for _ in 0..hits {
      self.grow(index, ctx);
    }
    self.after_hits(hits, ctx);
  }

  /// A Root Lash's hits go to the basins furthest along first
  fn on_lash(&mut self, hits: u8, ctx: &SystemContext) {
    for _ in 0..hits {
      let next = self
        .basins
        .iter()
        .position(|basin| basin.state == BasinState::Planted)
        .or_else(|| self.basins.iter().position(|basin| basin.state == BasinState::Lit));
      if let Some(index) = next {
        self.grow(index, ctx);
      }
    }
    self.after_hits(hits, ctx);
  }

  /// Takes the basin a step: a lit one is planted, and a planted one bursts
  fn grow(&mut self, index: usize, ctx: &SystemContext) {
    let basin = &mut self.basins[index];
    match basin.state {
      BasinState::Lit => {
        basin.state = BasinState::Planted;
        log::info!("MeridianBasins: {:?} planted", basin.shot);
      }
      BasinState::Planted => {
        basin.state = BasinState::Burst;
        log::info!("MeridianBasins: {:?} burst", basin.shot);
        ctx.play_scene(&Self::burst_scene(basin.shot));
        ctx.play_sfx(sounds::GUITAR_SWELL);
      }
      BasinState::Burst => {}
    }
    self.render(ctx);
  }

  fn after_hits(&mut self, hits: u8, ctx: &SystemContext) {
    self.run.score_hit(hits, ctx);
    if self.bursts() >= BURSTS {
      log::info!("MeridianBasins: complete");
      self.progress_scene.stop(ctx);
      self.run.complete(Self::complete_scene(), ctx);
      return;
    }
    self
      .progress_scene
      .replace_scene(Self::progress_scene(self.hits()), ctx);
  }

  /// Each basin's status on its hex center
  fn render(&self, ctx: &SystemContext) {
    for basin in &self.basins {
      let leds = basin.shot.hex_center().clone().at_z(BASIN_Z);
      match basin.state {
        BasinState::Lit => ctx.declare_leds(&leds, *MODE_COLOR),
        BasinState::Planted => ctx.declare_leds_flashing(&leds, *MODE_COLOR),
        BasinState::Burst => ctx.undeclare_leds(&leds),
      }
    }
  }
}

impl System for MeridianBasinsMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::MeridianBasins)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("MeridianBasins: started on {:?}", self.basins);
    self.run.begin(ctx);
    self.render(ctx);
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

    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
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

#[derive(Debug)]
struct Basin {
  shot: MainShot,
  state: BasinState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BasinState {
  Lit,
  /// Hit once: a seed is planted, and it flashes
  Planted,
  /// Hit twice: done
  Burst,
}

impl BasinState {
  /// Hits it's had
  fn hits(self) -> u8 {
    match self {
      Self::Lit => 0,
      Self::Planted => 1,
      Self::Burst => 2,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn four_different_basins_light() {
    let mode = MeridianBasinsMode::new();
    let shots: std::collections::HashSet<_> = mode.basins.iter().map(|basin| basin.shot).collect();
    assert_eq!(shots.len(), BASINS);
  }

  #[test]
  fn every_hit_fills_a_sixth_of_the_arc() {
    assert_eq!(REQUIRED_HITS, 6);
    assert_eq!(1 * ARC_LEDS / REQUIRED_HITS as usize, 3);
  }

  #[test]
  fn the_scenes_load() {
    MeridianBasinsMode::progress_scene(REQUIRED_HITS);
    MeridianBasinsMode::burst_scene(MainShot::DomeRamp);
    MeridianBasinsMode::complete_scene();
  }
}
