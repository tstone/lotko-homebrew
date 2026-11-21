use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::center_orbit::CenterSpinnerHit;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::hardware::pop_cluster;
use crate::hardware::vspinner::VerticalSpinnerHit;
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{Mode, ModeManager, RootLashed, flash_scene, glow_scene, points};
use crate::systems::sounds;

/// Collects that complete it (start value)
const COLLECTS: u8 = 3;

const RAMPS: [MainShot; 3] = [MainShot::DomeRamp, MainShot::ArcRamp, MainShot::LiftRamp];

/// The air plant, in tier 3 (design/city/halcyon-vents.md), 3-ball. The vertical and center spinners and the pops build a value, and
/// the ramps collect it, which starts it building again from the base.
pub struct HalcyonVentsMode {
  wizard: MiniWizardRun,
  value: u64,
  collects: u8,
  attention_scene: ScenePlayer,
}

impl HalcyonVentsMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::HalcyonVents, COLLECTS),
      value: points::HALCYON_BASE,
      collects: 0,
      attention_scene: ScenePlayer::new(Self::attention_scene()),
    }
  }

  fn attention_scene() -> PlayableScene {
    let qs: Vec<&LightQ> = RAMPS.iter().map(|shot| shot.hex_center()).collect();
    glow_scene(&LightQ::any(qs), Mode::HalcyonVents.color())
  }

  fn build(&mut self, amount: u64) {
    self.value += amount;
  }

  fn collect(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.collects += 1;
    log::info!(
      "HalcyonVents: collected {} ({} of {COLLECTS})",
      self.value,
      self.collects
    );
    self.wizard.score(self.value, ctx);
    self.value = points::HALCYON_BASE;
    self.wizard.progress(self.collects);
    ctx.play_scene(&flash_scene(shot.hex_ring(), Rgba::green()));

    if self.collects == COLLECTS {
      self.attention_scene.stop(ctx);
      self.wizard.complete(
        completing_scene!("startable/hit.scene").variable("color", Rgba::green()),
        ctx,
      );
      return;
    }
    ctx.play_sfx(sounds::city_hit(self.collects as f32 / COLLECTS as f32));
  }
}

impl System for HalcyonVentsMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::HalcyonVents)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.wizard.begin(ctx);
  }

  /// A resume keeps the collects and the value built
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    self.wizard.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.wizard.interrupt(ctx);
    ctx.deactivate_light_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.wizard.on_event(event, ctx);

    if event.is::<VerticalSpinnerHit>() || event.is::<CenterSpinnerHit>() {
      self.build(points::HALCYON_SPIN);
    } else if let Some(closed) = event.downcast_ref::<SwitchClosed>()
      && pop_cluster::match_switch(&closed.switch).is_some()
    {
      self.build(points::HALCYON_POP);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.build(points::HALCYON_ROOTS * lash.hits as u64);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>()
      && RAMPS.contains(&hit.shot)
    {
      self.collect(hit.shot, ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.wizard.apply(ctx) {
      return;
    }

    self.attention_scene.start(ctx);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    HalcyonVentsMode::attention_scene();
  }

  #[test]
  fn building_adds_to_the_base_value() {
    let mut mode = HalcyonVentsMode::new();
    mode.build(points::HALCYON_POP);
    assert_eq!(mode.value, points::HALCYON_BASE + points::HALCYON_POP);
  }
}
