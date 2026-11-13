use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, RootLashed, fill_effect, flash_effect, glow_effect, points,
};
use crate::systems::{Callout, SayExt, sounds};

/// Orbit hits to high water (start value)
const HIGH_WATER: u8 = 8;

/// Super jackpots that complete it (start value)
const SUPER_JACKPOTS: u8 = 3;

const ORBITS: [MainShot; 3] = [
  MainShot::LeftOrbit,
  MainShot::CenterOrbit,
  MainShot::RightOrbit,
];

const RAMPS: [MainShot; 3] = [MainShot::DomeRamp, MainShot::ArcRamp, MainShot::LiftRamp];

/// LEDs on the arc ramp, which show the water level
const ARC_LEDS: usize = 18;

/// Meridian's mini-wizard (DESIGN.md, Mini-wizards), 2-ball. Orbit hits raise the water, shown on the arc ramp, and at
/// high water the ramps are super jackpots. A super jackpot lets the water out to rise again.
pub struct FloodgatesMode {
  wizard: MiniWizardRun,
  level: u8,
  super_jackpots: u8,
  /// The orbits while the water rises, the ramps at high water
  attention_effect: LedPlayback,
  gauge_effect: LedPlayback,
}

impl FloodgatesMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::Floodgates, HIGH_WATER * SUPER_JACKPOTS),
      level: 0,
      super_jackpots: 0,
      attention_effect: Self::attention_effect(false),
      gauge_effect: Self::gauge_effect(0),
    }
  }

  fn attention_effect(high_water: bool) -> LedPlayback {
    let (shots, color) = if high_water {
      (RAMPS, Rgba::white())
    } else {
      (ORBITS, Mode::Floodgates.color())
    };
    let qs: Vec<&LedQ> = shots.iter().map(|shot| shot.hex_center()).collect();
    glow_effect(&LedQ::any(qs), color)
  }

  fn gauge_effect(level: u8) -> LedPlayback {
    let lit = level as usize * ARC_LEDS / HIGH_WATER as usize;
    fill_effect(&arc_ramp::ARC_LEDS.q(), ARC_LEDS, lit, Rgba::blue())
  }

  fn high_water(&self) -> bool {
    self.level == HIGH_WATER
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if ORBITS.contains(&shot) {
      self.raise(ctx);
    } else if RAMPS.contains(&shot) && self.high_water() {
      self.super_jackpot(shot, ctx);
    }
  }

  fn raise(&mut self, ctx: &SystemContext) {
    if self.high_water() {
      return;
    }
    self.level += 1;
    log::info!("Floodgates: water at {} of {HIGH_WATER}", self.level);
    self.wizard.score(points::FLOOD_HIT, ctx);
    self
      .wizard
      .progress(self.super_jackpots * HIGH_WATER + self.level);
    self.render(ctx);

    if self.high_water() {
      ctx.play_sfx(sounds::ARP_HIT1);
      // the Steward wants the water out, which is what the forest wants too
      ctx.say(Callout::ShootTheRamps);
    } else {
      ctx.play_sfx(sounds::city_hit(self.level as f32 / HIGH_WATER as f32));
    }
  }

  fn super_jackpot(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.super_jackpots += 1;
    log::info!("Floodgates: super jackpot {}", self.super_jackpots);
    self.wizard.score(
      points::FLOOD_SUPER_JACKPOT * self.super_jackpots as u64,
      ctx,
    );
    ctx.play_sfx(sounds::IMPACT_CINEMATIC_HEAVY);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(shot.hex_ring(), Rgba::white()));

    self.level = 0;
    if self.super_jackpots == SUPER_JACKPOTS {
      self.attention_effect.stop(ctx);
      self.gauge_effect.stop(ctx);
      self.wizard.complete(
        led_playback!("startable/hit.scene").param("color", Rgba::blue()),
        ctx,
      );
      return;
    }
    self.render(ctx);
  }

  fn render(&mut self, ctx: &SystemContext) {
    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect(self.high_water());
    self.gauge_effect.stop(ctx);
    self.gauge_effect = Self::gauge_effect(self.level);
  }
}

impl System for FloodgatesMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::Floodgates)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.wizard.begin(ctx);
  }

  /// A resume keeps the water level and the super jackpots
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
      // the roots lash up from the orbits, and each hit is a rush of water of its own
      for _ in 0..lash.hits {
        self.raise(ctx);
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.wizard.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
    self.gauge_effect.apply(delta, ctx);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    FloodgatesMode::attention_effect(true);
    FloodgatesMode::gauge_effect(HIGH_WATER);
  }
}
