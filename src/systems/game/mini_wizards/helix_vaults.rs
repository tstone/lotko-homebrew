use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::arc_ramp;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{Mode, ModeManager, RootLashed, fill_scene, points};
use crate::systems::{Callout, SayExt, sounds};

/// Orbit hits that drain enough of the vat for a rupture (start value)
const DRAIN_PER_RUPTURE: u8 = 8;

/// Ruptures that empty the vat and complete it
const RUPTURES: u8 = 3;

const ORBITS: [MainShot; 3] = [
  MainShot::LeftOrbit,
  MainShot::CenterOrbit,
  MainShot::RightOrbit,
];

const RAMPS: [MainShot; 3] = [MainShot::DomeRamp, MainShot::ArcRamp, MainShot::LiftRamp];

/// LEDs in the subway, which is the vat
const SUBWAY_LEDS: usize = 11;

/// The z of the shots' statuses on their hex centers, the mode attention z
const SHOT_Z: i8 = 10;

/// A ramp lit for a rupture, lime so it reads apart from the orbits
const RUPTURE_COLOR: Rgba<u8> = Rgba([90, 255, 0, 255]);

/// Meridian's mini-wizard (design/city/helix-vaults.md), 2-ball. The vat, shown in the subway, starts full. Orbit hits
/// drain it, and after enough of them a ramp lights for a rupture, a super jackpot. The third rupture empties it. Each
/// ramp lights once.
pub struct HelixVaultsMode {
  wizard: MiniWizardRun,
  /// Orbit hits since the last rupture
  drained: u8,
  ruptures: u8,
  /// The ramp lit for a rupture, once enough has drained
  lit_ramp: Option<MainShot>,
  /// Ramps that have had their rupture
  used_ramps: Vec<MainShot>,
  vat_scene: ScenePlayer,
}

impl HelixVaultsMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::HelixVaults, DRAIN_PER_RUPTURE * RUPTURES),
      drained: 0,
      ruptures: 0,
      lit_ramp: None,
      used_ramps: Vec::new(),
      vat_scene: ScenePlayer::new(Self::vat_scene(SUBWAY_LEDS)),
    }
  }

  fn vat_scene(lit: usize) -> PlayableScene {
    fill_scene(&arc_ramp::SUBWAY_LEDS.q(), SUBWAY_LEDS, lit, Mode::HelixVaults.color())
  }

  /// A blob of goo from the vat to the shot
  fn discharge_scene(shot: MainShot) -> PlayableScene<Completes> {
    completing_scene!("helix_vaults/discharge.scene").variable("to", shot.origin())
  }

  fn splatter_scene() -> PlayableScene<Completes> {
    completing_scene!("helix_vaults/splatter.scene")
  }

  fn complete_scene() -> PlayableScene<Completes> {
    completing_scene!("helix_vaults/complete.scene")
  }

  /// Orbit hits over the whole run, against [`DRAIN_PER_RUPTURE`] times [`RUPTURES`]
  fn total_drained(&self) -> u8 {
    self.ruptures * DRAIN_PER_RUPTURE + self.drained
  }

  /// LEDs of the subway still full, never fewer than one until the last rupture
  fn vat_level(&self) -> usize {
    let total = (DRAIN_PER_RUPTURE * RUPTURES) as usize;
    let left = total - self.total_drained() as usize;
    (left * SUBWAY_LEDS).div_ceil(total)
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if ORBITS.contains(&shot) {
      self.drain(shot, ctx);
    } else if self.lit_ramp == Some(shot) {
      self.rupture(shot, ctx);
    }
  }

  fn drain(&mut self, shot: MainShot, ctx: &SystemContext) {
    if self.lit_ramp.is_some() {
      return;
    }
    self.drained += 1;
    log::info!("HelixVaults: drained {} of {DRAIN_PER_RUPTURE}", self.drained);
    self.wizard.score(points::HELIX_DRAIN, ctx);
    self.wizard.progress(self.total_drained());
    ctx.play_scene(&Self::discharge_scene(shot));

    if self.drained == DRAIN_PER_RUPTURE {
      let unused: Vec<MainShot> = RAMPS
        .into_iter()
        .filter(|ramp| !self.used_ramps.contains(ramp))
        .collect();
      self.lit_ramp = unused.choose(&mut rand::rng()).copied();
      log::info!("HelixVaults: rupture lit on {:?}", self.lit_ramp);
      ctx.play_sfx(sounds::ARP_HIT1);
      ctx.say(Callout::ShootTheRamps);
    } else {
      ctx.play_sfx(sounds::SLIME2);
    }
    self.render(ctx);
  }

  fn rupture(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.ruptures += 1;
    log::info!("HelixVaults: rupture {}", self.ruptures);
    self.wizard.score(points::HELIX_RUPTURE * self.ruptures as u64, ctx);

    self.drained = 0;
    self.lit_ramp = None;
    self.used_ramps.push(shot);
    if self.ruptures == RUPTURES {
      self.vat_scene.stop(ctx);
      // the last of the vat draining away
      ctx.play_sfx(sounds::SLIME);
      self.wizard.complete(Self::complete_scene(), ctx);
      return;
    }
    ctx.say(Callout::ContainmentBreach);
    ctx.play_scene(&Self::splatter_scene());
    self.render(ctx);
  }

  /// The orbits while the vat drains, or the ramp lit for a rupture, and the vat's level
  fn render(&mut self, ctx: &SystemContext) {
    let color = Mode::HelixVaults.color();
    for shot in ORBITS {
      let leds = shot.hex_center().clone().at_z(SHOT_Z);
      if self.lit_ramp.is_none() {
        ctx.declare_leds(&leds, color);
      } else {
        ctx.undeclare_leds(&leds);
      }
    }
    for shot in RAMPS {
      let leds = shot.hex_center().clone().at_z(SHOT_Z);
      if self.lit_ramp == Some(shot) {
        ctx.declare_leds(&leds, RUPTURE_COLOR);
      } else {
        ctx.undeclare_leds(&leds);
      }
    }
    self
      .vat_scene
      .replace_scene(Self::vat_scene(self.vat_level()), ctx);
  }
}

impl System for HelixVaultsMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::HelixVaults)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.wizard.begin(ctx);
    self.render(ctx);
  }

  /// A resume keeps the vat's level and the ruptures
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.wizard.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.wizard.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.wizard.on_event(event, ctx);

    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      // the roots lash up from the orbits, and each hit drains the vat as an orbit would
      for _ in 0..lash.hits {
        self.drain(MainShot::CenterOrbit, ctx);
      }
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.wizard.apply(ctx) {
      return;
    }

    self.vat_scene.start(ctx);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_vat_starts_full_and_empties_with_the_last_rupture() {
    let mut mode = HelixVaultsMode::new();
    assert_eq!(mode.vat_level(), SUBWAY_LEDS);
    mode.ruptures = RUPTURES - 1;
    mode.drained = DRAIN_PER_RUPTURE - 1;
    assert_eq!(mode.vat_level(), 1);
    mode.ruptures = RUPTURES;
    mode.drained = 0;
    assert_eq!(mode.vat_level(), 0);
  }

  #[test]
  fn the_scenes_load() {
    HelixVaultsMode::vat_scene(SUBWAY_LEDS);
    HelixVaultsMode::discharge_scene(MainShot::LeftOrbit);
    HelixVaultsMode::splatter_scene();
    HelixVaultsMode::complete_scene();
  }
}
