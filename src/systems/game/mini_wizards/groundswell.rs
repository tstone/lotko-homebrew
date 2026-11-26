use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::drop_bank::{self, DropBankCleared, DropBankTargetHit};
use crate::hardware::lift_ramp::{self, LiftRampSystem};
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{Mode, ModeManager, RootLashed, flash_scene, glow_scene, points};
use crate::systems::sounds;

/// Bank clears that complete it, each breaking a foundation (start value)
const CLEARS: u8 = 3;

/// The roots heave the ground under the city (DESIGN.md, Groundswell), 2-ball, lit by spreading roots rather than
/// reached on the route. The lift ramp stays up, so the ramp feeds the rear scoop and the wireform down to the upper
/// flipper, which aims at the drops: the city's foundations. Each drop target is a jackpot and each bank clear breaks a
/// foundation for a super jackpot. The bank raises itself after a clear.
pub struct GroundswellMode {
  wizard: MiniWizardRun,
  clears: u8,
  /// Each drop target's strip, lit while the target is standing, in [`targets`] order
  strip_scenes: [ScenePlayer; 3],
  /// The lift ramp, the way up to the upper flipper
  ramp_scene: ScenePlayer,
}

impl GroundswellMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::Groundswell, CLEARS * 3),
      clears: 0,
      strip_scenes: targets().map(|(_, leds)| ScenePlayer::new(Self::attention_scene(&leds.q()))),
      ramp_scene: ScenePlayer::new(Self::attention_scene(&lift_ramp::HEX_CENTER_LED)),
    }
  }

  /// A drop strip or the lift ramp, in the roots' bronze
  fn attention_scene(leds: &LightQ) -> PlayableScene {
    glow_scene(leds, Mode::Groundswell.color())
  }

  /// Lights the strips of the targets still standing, and the ramp. A knocked down target is done, so its strip goes
  /// dark until the bank comes back up.
  fn render(&mut self, ctx: &SystemContext) {
    for ((switch, _), scene) in targets().iter().zip(self.strip_scenes.iter_mut()) {
      let down = ctx.switches.is_closed(switch.name).unwrap_or(false);
      if down {
        scene.stop(ctx);
      } else {
        scene.start(ctx);
      }
    }
    self.ramp_scene.start(ctx);
  }

  fn stop_scenes(&mut self, ctx: &SystemContext) {
    for scene in &mut self.strip_scenes {
      scene.stop(ctx);
    }
    self.ramp_scene.stop(ctx);
  }

  fn jackpot(&mut self, target: Option<u8>, ctx: &SystemContext) {
    log::info!("Groundswell: jackpot");
    self
      .wizard
      .score(points::GROUNDSWELL_JACKPOT * (self.clears as u64 + 1), ctx);
    ctx.play_sfx(sounds::LATCH_DOUBLE);
    let strip = match target {
      Some(1) => Some(&drop_bank::TARGET1_LEDS),
      Some(2) => Some(&drop_bank::TARGET2_LEDS),
      Some(3) => Some(&drop_bank::TARGET3_LEDS),
      _ => None,
    };
    if let Some(strip) = strip {
      ctx.play_scene(&flash_scene(&strip.q(), Rgba::green()));
    }
  }

  fn cleared(&mut self, ctx: &SystemContext) {
    self.clears += 1;
    log::info!(
      "Groundswell: foundation broken, {} of {CLEARS}",
      self.clears
    );
    self
      .wizard
      .score(points::GROUNDSWELL_SUPER_JACKPOT * self.clears as u64, ctx);
    self.wizard.progress(self.clears * 3);

    if self.clears == CLEARS {
      self.stop_scenes(ctx);
      self.release_ramp(ctx);
      self.wizard.complete(
        completing_scene!("startable/hit.scene").variable("color", Rgba::green()),
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

impl System for GroundswellMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::Groundswell)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.request_ramp(ctx);
    self.wizard.begin(ctx);
  }

  /// A resume keeps the clears
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    self.request_ramp(ctx);
    self.wizard.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.release_ramp(ctx);
    self.wizard.interrupt(ctx);
    ctx.deactivate_light_declarations();
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

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.wizard.apply(ctx) {
      return;
    }

    self.render(ctx);
  }
}

/// The drop targets' switches, closed when the target is down, and their strips
fn targets() -> [(&'static SwitchDefinition, &'static LedDefinition); 3] {
  [
    (&drop_bank::TARGET1, &drop_bank::TARGET1_LEDS),
    (&drop_bank::TARGET2, &drop_bank::TARGET2_LEDS),
    (&drop_bank::TARGET3, &drop_bank::TARGET3_LEDS),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    GroundswellMode::attention_scene(&lift_ramp::HEX_CENTER_LED);
  }
}
