use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{ScoopBallEntered, ScoopEjected};
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::lower_scoop::{self, Lower};
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::hardware::vspinner;
use crate::systems::game::mini_wizards::MiniWizardRun;
use crate::systems::game::{Mode, ModeManager, RootLashed, points};
use crate::systems::{Callout, SayExt, sounds};

/// Over everything on the playfield, so it all goes dark
const DARK_Z: i8 = 40;

/// What the dark leaves lit: the roots and the lit shots
const LIT_Z: i8 = 41;

/// Dark shots at which the Steward starts to break up
const STEWARD_FRAGMENTS: u8 = 3;

/// Hydro's mini-wizard (DESIGN.md, Mini-wizards), 2-ball. The playfield goes dark but for the roots and the lit shots,
/// and each lit shot hit goes dark too. Once they're all dark the left scoop is the super jackpot, which completes it.
///
/// The time from the start to the super jackpot is Fastest Blackout, counting only the time it was running.
pub struct BlackoutMode {
  wizard: MiniWizardRun,
  /// By shot, in [`MainShot::ALL`] order
  dark: [bool; 6],
  /// The lit shots, or the scoop once they're all dark
  lit_effect: Option<LedPlayback>,
  /// The dark is down, which it is from the ball leaving the scoop to the end of the run
  darkened: bool,
  elapsed: Duration,
}

impl BlackoutMode {
  pub fn new() -> Self {
    Self {
      wizard: MiniWizardRun::new(Mode::Blackout, MainShot::ALL.len() as u8 + 1),
      dark: [false; 6],
      lit_effect: None,
      darkened: false,
      elapsed: Duration::ZERO,
    }
  }

  fn lit_effect(leds: LedQ) -> LedPlayback {
    led_playback!("blackout/lit.scene", leds).param("color", Rgba::white())
  }

  fn lit_shots(&self) -> Vec<MainShot> {
    MainShot::ALL
      .into_iter()
      .filter(|shot| !self.dark[*shot as usize])
      .collect()
  }

  fn super_jackpot_lit(&self) -> bool {
    self.lit_shots().is_empty()
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if !self.dark[shot as usize] {
      self.darken(shot, ctx);
    }
  }

  fn darken(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.dark[shot as usize] = true;
    let dark = self.dark.iter().filter(|dark| **dark).count() as u8;
    log::info!("Blackout: {shot:?} dark, {dark} of 6");
    self.wizard.score(points::BLACKOUT_HIT, ctx);
    self.wizard.progress(dark);
    ctx.play_sfx(sounds::rnd_latch());
    if dark == STEWARD_FRAGMENTS {
      ctx.say(Callout::System67Invalid);
    }
    if self.super_jackpot_lit() {
      ctx.play_sfx(sounds::ARP_HIT1);
    }
    self.render(ctx);
  }

  fn super_jackpot(&mut self, ctx: &SystemContext) {
    log::info!(
      "Blackout: super jackpot, Fastest Blackout {:?}",
      self.elapsed
    );
    self.wizard.score(points::BLACKOUT_SUPER_JACKPOT, ctx);
    self.lift_dark(ctx);
    self.wizard.complete(
      led_playback!("startable/hit.scene").param("color", Rgba::white()),
      ctx,
    );
  }

  fn render(&mut self, ctx: &SystemContext) {
    ctx.declare_leds(&LedQ::tag::<tags::Playfield>().at_z(DARK_Z), Rgba::black());
    ctx.declare_leds(
      &roots().at_z(LIT_Z),
      Rgba::green().mix_with(Rgba::black(), 0.6),
    );

    if let Some(mut effect) = self.lit_effect.take() {
      effect.stop(ctx);
    }
    let leds = if self.super_jackpot_lit() {
      lower_scoop::BOLTS_Q.clone()
    } else {
      let qs: Vec<&LedQ> = self
        .lit_shots()
        .into_iter()
        .map(|shot| shot.hex_center())
        .collect();
      LedQ::any(qs)
    };
    self.lit_effect = Some(Self::lit_effect(leds));
    self.darkened = true;
  }

  fn lift_dark(&mut self, ctx: &SystemContext) {
    if let Some(mut effect) = self.lit_effect.take() {
      effect.stop(ctx);
    }
    ctx.undeclare_leds(&LedQ::tag::<tags::Playfield>().at_z(DARK_Z));
    ctx.undeclare_leds(&roots().at_z(LIT_Z));
    self.darkened = false;
  }
}

impl System for BlackoutMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::Blackout)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.wizard.begin(ctx);
  }

  /// A resume keeps the dark shots and the time so far
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.wizard.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.lift_dark(ctx);
    self.wizard.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.wizard.on_event(event, ctx);

    if event.is::<ScoopEjected<Lower>>() && !self.darkened {
      // the lights go out as the ball leaves the scoop
      self.render(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      for _ in 0..lash.hits {
        if let Some(shot) = self.lit_shots().choose(&mut rand::rng()).copied() {
          self.darken(shot, ctx);
        }
      }
    } else if event.is::<ScoopBallEntered<Lower>>() && self.super_jackpot_lit() {
      self.super_jackpot(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.wizard.apply(delta, ctx) {
      return;
    }

    if self.darkened {
      self.elapsed += delta;
    }
    if let Some(effect) = self.lit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
  }
}

/// The roots around the vertical spinner, with the pop rays running into them
fn roots() -> LedQ {
  LedQ::Or(vec![
    vspinner::LEDS.q(),
    vspinner::left_ray::Q.clone(),
    vspinner::upper_right_ray::Q.clone(),
    vspinner::lower_right_ray::Q.clone(),
  ])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    BlackoutMode::lit_effect(lower_scoop::BOLTS_Q.clone());
  }

  #[test]
  fn the_scoop_is_lit_once_every_shot_is_dark() {
    let mut mode = BlackoutMode::new();
    mode.dark = [true, true, true, true, true, false];
    assert!(!mode.super_jackpot_lit());
    mode.dark[5] = true;
    assert!(mode.super_jackpot_lit());
  }
}
