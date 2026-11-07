use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::city_map;
use crate::hardware::left_inlane;
use crate::hardware::pop_cluster;
use crate::hardware::vspinner::VerticalSpinnerHit;
use crate::metrics;
use crate::systems::game::{EffectsSystem, ModeManager, Perk, flash_effect, points};
use crate::systems::{Callout, SayExt, sounds};

/// Charge for the first burst (start value)
const FIRST_CHARGE: u32 = 60;

/// How much more each charge after that needs (start value)
const CHARGE_STEP: u32 = 30;

/// Charge from a spin of the vertical spinner
const SPIN_CHARGE: u32 = 1;

/// Charge from a pop hit, doubled by the Nimbus perk
const POP_CHARGE: u32 = 3;

/// How long a burst doubles mode shots (start value)
const BURST_TIME: Duration = Duration::from_secs(8);

/// The forest's spores, as the Charge side quest (DESIGN.md, Charge). The vertical spinner and the pops fill the spore
/// bar. A full bar lights the burst on the left inlane target, and hitting it fires the burst: for a while every mode
/// shot counts double, or with no mode running it scores big. Each charge after that needs more.
///
/// Also counts the spores applied this game, which every mode shot applies through [`SporesSystem::mode_shot`].
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SporesSystem {
  count: u32,
  charge: u32,
  required: u32,
  bar_effect: LedPlayback,
  /// Shown while a burst is ready to fire
  ready_effect: Option<LedPlayback>,
  /// While a burst is running
  burst_cue: Option<u64>,
}

impl SporesSystem {
  pub fn new() -> Self {
    Self {
      count: 0,
      charge: 0,
      required: FIRST_CHARGE,
      bar_effect: Self::bar_effect(0.0),
      ready_effect: None,
      burst_cue: None,
    }
  }

  fn bar_effect(progress: f32) -> LedPlayback {
    city_map::spore_count_effect(Rgba::green(), progress)
  }

  fn ready_effect() -> LedPlayback {
    led_playback!("side/burst_ready.scene")
  }

  pub fn count(&self) -> u32 {
    self.count
  }

  /// A mode shot was made, which applies a spore, or two during a burst. Returns the progress the shot adds to the
  /// mode, which a burst doubles the same way.
  pub fn mode_shot(&mut self, ctx: &ServiceContext) -> u8 {
    let spores = if self.burst_cue.is_some() { 2 } else { 1 };
    self.count += spores;
    ctx.metrics.add(&metrics::SPORES_APPLIED, spores as u64);
    spores as u8
  }

  fn add_charge(&mut self, amount: u32, ctx: &SystemContext) {
    if self.ready_effect.is_some() {
      return;
    }

    self.charge = (self.charge + amount).min(self.required);
    self.bar_effect = Self::bar_effect(self.charge as f32 / self.required as f32);
    if self.charge >= self.required {
      log::info!("Spores: burst ready");
      self.ready_effect = Some(Self::ready_effect());
      ctx.play_sfx(sounds::ARP_HIT1);
      ctx.say(Callout::BiologicalContaminantIdentified);
    }
  }

  fn on_pop(&mut self, pop: pop_cluster::PopBumper, ctx: &SystemContext) {
    let perk = ctx
      .systems
      .expect::<ModeManager>()
      .has_perk(Perk::FasterPopCharge);
    self.add_charge(if perk { POP_CHARGE * 2 } else { POP_CHARGE }, ctx);

    let mut effects = ctx.systems.expect::<EffectsSystem>();
    effects.play(ray_effect(&pop, Rgba::green()));
    effects.play(flash_effect(
      &pop_cluster::led_for_pop(&pop).q(),
      Rgba::green(),
    ));
  }

  fn fire_burst(&mut self, ctx: &SystemContext) {
    let Some(mut ready) = self.ready_effect.take() else {
      return;
    };
    ready.stop(ctx);

    log::info!("Spores: burst fired");
    self.charge = 0;
    self.required += CHARGE_STEP;
    self.bar_effect = Self::bar_effect(0.0);
    self.burst_cue = Some(ctx.cues.start(BurstEnded, BURST_TIME.once()));

    ctx.play_sfx(sounds::IMPACT_DIRT);
    ctx.say(Callout::AirborneContaminantDetected);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(led_playback!("side/burst.scene"));

    // with no mode to double, the burst is worth points of its own
    if ctx.systems.expect::<ModeManager>().current_mode().is_none() {
      ctx.add_points(points::BURST);
    }
  }
}

impl System for SporesSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  /// A burst doesn't carry over to the next ball, while the charge and a burst that's ready do
  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.burst_cue.take() {
      ctx.cues.cancel(cue);
    }
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<VerticalSpinnerHit>() {
      self.add_charge(SPIN_CHARGE, ctx);
    } else if event.is::<BurstEnded>() {
      log::info!("Spores: burst over");
      self.burst_cue = None;
    } else if let Some(closed) = event.downcast_ref::<SwitchClosed>() {
      if closed.switch.name == left_inlane::TARGET_SWITCH.name {
        self.fire_burst(ctx);
      } else if let Some(pop) = pop_cluster::match_switch(&closed.switch) {
        self.on_pop(pop, ctx);
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.bar_effect.apply(delta, ctx);
    if let Some(effect) = self.ready_effect.as_mut() {
      effect.apply(delta, ctx);
    }
  }
}

/// A packet down a pop's ray from the pop into the roots
pub fn ray_effect(pop: &pop_cluster::PopBumper, color: Rgba<u8>) -> LedPlayback {
  led_playback!(
    "side/charge_ray.scene",
    pop_cluster::led_ray_for_pop(pop).clone()
  )
  .param("color", color)
}

#[derive(serde::Serialize, Event)]
struct BurstEnded;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    SporesSystem::ready_effect();
    ray_effect(&pop_cluster::PopBumper::Left, Rgba::green());
    led_playback!("side/burst.scene");
  }
}
