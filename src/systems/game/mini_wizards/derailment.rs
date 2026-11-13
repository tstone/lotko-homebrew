use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::drop_bank::{self, DropBankCleared, DropBankTargetHit};
use crate::hardware::lift_ramp::{self, LiftRampSystem};
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, RootLashed, flash_effect, glow_effect, points,
};
use crate::systems::sounds;

/// Bank clears that complete it (start value)
const CLEARS: u8 = 3;

/// Skyrail's mini-wizard (DESIGN.md, Mini-wizards), 3-ball. The lift ramp stays up, so the ramp feeds the rear scoop and
/// the wireform down to the upper flipper, which aims at the drops. Each drop target is a jackpot and each bank clear a
/// super jackpot, with the bank raising itself after a clear.
///
/// There are no locks yet, so there are none to keep as virtual locks. With Groundswell they'd be kept here.
pub struct DerailmentMode {
  wizard: MiniWizardRun,
  clears: u8,
  attention_effect: LedPlayback,
}

impl DerailmentMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::Derailment, CLEARS * 3),
      clears: 0,
      attention_effect: Self::attention_effect(),
    }
  }

  /// The drop strips and the lift ramp, in the line's blue
  fn attention_effect() -> LedPlayback {
    let leds = LedQ::Or(vec![
      drop_bank::TARGET1_LEDS.q(),
      drop_bank::TARGET2_LEDS.q(),
      drop_bank::TARGET3_LEDS.q(),
      lift_ramp::HEX_CENTER_LED.clone(),
    ]);
    glow_effect(&leds, Mode::Derailment.color())
  }

  fn jackpot(&mut self, target: Option<u8>, ctx: &SystemContext) {
    log::info!("Derailment: jackpot");
    self
      .wizard
      .score(points::DERAIL_JACKPOT * (self.clears as u64 + 1), ctx);
    ctx.play_sfx(sounds::LATCH_DOUBLE);
    let strip = match target {
      Some(1) => Some(&drop_bank::TARGET1_LEDS),
      Some(2) => Some(&drop_bank::TARGET2_LEDS),
      Some(3) => Some(&drop_bank::TARGET3_LEDS),
      _ => None,
    };
    if let Some(strip) = strip {
      ctx
        .systems
        .expect::<EffectsSystem>()
        .play(flash_effect(&strip.q(), Rgba::green()));
    }
  }

  fn cleared(&mut self, ctx: &SystemContext) {
    self.clears += 1;
    log::info!("Derailment: bank cleared {} of {CLEARS}", self.clears);
    self
      .wizard
      .score(points::DERAIL_SUPER_JACKPOT * self.clears as u64, ctx);
    self.wizard.progress(self.clears * 3);

    if self.clears == CLEARS {
      self.attention_effect.stop(ctx);
      self.release_ramp(ctx);
      self.wizard.complete(
        led_playback!("startable/hit.scene").param("color", Rgba::green()),
        ctx,
      );
      return;
    }
    ctx.play_sfx(sounds::IMPACT_CINEMATIC_HEAVY);
  }

  fn request_ramp(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LiftRampSystem>()
      .request_up(ctx.id(), ctx.into());
  }

  fn release_ramp(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LiftRampSystem>()
      .release(ctx.id(), ctx.into());
  }
}

impl System for DerailmentMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::Derailment)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.request_ramp(ctx);
    self.wizard.begin(ctx);
  }

  /// A resume keeps the clears
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.request_ramp(ctx);
    self.wizard.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.release_ramp(ctx);
    self.wizard.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.wizard.on_event(event, ctx);

    if let Some(DropBankTargetHit(target)) = event.downcast_ref::<DropBankTargetHit>() {
      self.jackpot(Some(*target), ctx);
    } else if event.is::<DropBankCleared>() {
      self.cleared(ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      for _ in 0..lash.hits {
        self.jackpot(None, ctx);
      }
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
    DerailmentMode::attention_effect();
  }
}
