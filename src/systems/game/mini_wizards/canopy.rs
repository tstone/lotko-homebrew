use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::center_orbit::CenterSpinnerHit;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::hardware::pop_cluster;
use crate::hardware::vspinner::VerticalSpinnerHit;
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, RootLashed, flash_effect, glow_effect, points,
};
use crate::systems::sounds;

/// Collects that complete it (start value)
const COLLECTS: u8 = 3;

const RAMPS: [MainShot; 3] = [MainShot::DomeRamp, MainShot::ArcRamp, MainShot::LiftRamp];

/// Apex's mini-wizard (DESIGN.md, Mini-wizards), 3-ball. The vertical and center spinners and the pops build a value, and
/// the ramps collect it, which starts it building again from the base.
pub struct CanopyMode {
  wizard: MiniWizardRun,
  value: u32,
  collects: u8,
  attention_effect: LedPlayback,
}

impl CanopyMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::Canopy, COLLECTS),
      value: points::CANOPY_BASE,
      collects: 0,
      attention_effect: Self::attention_effect(),
    }
  }

  fn attention_effect() -> LedPlayback {
    let qs: Vec<&LedQ> = RAMPS.iter().map(|shot| shot.hex_center()).collect();
    glow_effect(&LedQ::any(qs), Mode::Canopy.color())
  }

  fn build(&mut self, amount: u32) {
    self.value += amount;
  }

  fn collect(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.collects += 1;
    log::info!(
      "Canopy: collected {} ({} of {COLLECTS})",
      self.value,
      self.collects
    );
    self.wizard.score(self.value, ctx);
    self.value = points::CANOPY_BASE;
    self.wizard.progress(self.collects);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(shot.hex_ring(), Rgba::green()));

    if self.collects == COLLECTS {
      self.attention_effect.stop(ctx);
      self.wizard.complete(
        led_playback!("startable/hit.scene").param("color", Rgba::green()),
        ctx,
      );
      return;
    }
    ctx.play_sfx(sounds::city_hit(self.collects as f32 / COLLECTS as f32));
  }
}

impl System for CanopyMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::Canopy)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.wizard.begin(ctx);
  }

  /// A resume keeps the collects and the value built
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

    if event.is::<VerticalSpinnerHit>() || event.is::<CenterSpinnerHit>() {
      self.build(points::CANOPY_SPIN);
    } else if let Some(closed) = event.downcast_ref::<SwitchClosed>()
      && pop_cluster::match_switch(&closed.switch).is_some()
    {
      self.build(points::CANOPY_POP);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.build(points::CANOPY_ROOTS * lash.hits as u32);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>()
      && RAMPS.contains(&hit.shot)
    {
      self.collect(hit.shot, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.wizard.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    CanopyMode::attention_effect();
  }

  #[test]
  fn building_adds_to_the_base_value() {
    let mut mode = CanopyMode::new();
    mode.build(points::CANOPY_POP);
    assert_eq!(mode.value, points::CANOPY_BASE + points::CANOPY_POP);
  }
}
