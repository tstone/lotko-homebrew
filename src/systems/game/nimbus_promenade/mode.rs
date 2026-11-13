use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::ScoopEjected;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::lower_scoop::Lower;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, ModeRun, RootLashed, SporesSystem, flash_effect,
};
use crate::systems::{Callout, SayExt, sounds};

const REQUIRED_CHOKES: u8 = 4;

/// The time between seals, before chokes speed them up (start value)
const SEAL_INTERVAL: Duration = Duration::from_secs(6);

/// How much sooner each choke brings the next seal
const SEAL_SPEEDUP: Duration = Duration::from_secs(1);

/// How long a seal is telegraphed before it comes down
const SEAL_WARNING: Duration = Duration::from_secs(1);

/// Choke points (DESIGN.md). The Steward seals the open walkways one at a time, and the forest chokes them. Each of the
/// 6 main shots is a walkway.
pub struct NimbusPromenadeMode {
  run: ModeRun,
  /// By shot, in [`MainShot::ALL`] order
  walkways: [Walkway; 6],
  /// The open walkways, other than one about to be sealed
  attention_effect: LedPlayback,
  /// The walkway about to be sealed, and its warning
  sealing: Option<(MainShot, LedPlayback)>,
  /// The next seal or its warning, while they're running
  seal_cue: Option<u64>,
}

impl NimbusPromenadeMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::NimbusPromenade, REQUIRED_CHOKES),
      walkways: [Walkway::Open; 6],
      attention_effect: Self::attention_effect(&MainShot::ALL),
      sealing: None,
      seal_cue: None,
    }
  }

  fn attention_effect(shots: &[MainShot]) -> LedPlayback {
    let qs: Vec<&LedQ> = shots.iter().map(|shot| shot.hex_center()).collect();
    led_playback!("nimbus_promenade/attention.scene", LedQ::any(qs))
  }

  fn seal_warning_effect(shot: MainShot) -> LedPlayback {
    led_playback!(
      "nimbus_promenade/seal_warning.scene",
      shot.hex_center().clone()
    )
  }

  fn walkway(&self, shot: MainShot) -> Walkway {
    self.walkways[shot as usize]
  }

  fn chokes(&self) -> u8 {
    self
      .walkways
      .iter()
      .filter(|walkway| **walkway == Walkway::Choked)
      .count() as u8
  }

  fn open_shots(&self) -> Vec<MainShot> {
    MainShot::ALL
      .into_iter()
      .filter(|shot| self.walkway(*shot) == Walkway::Open)
      .collect()
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    match self.walkway(shot) {
      Walkway::Open => {
        // one walkway a hit, so a burst doubles the points rather than choking two
        let times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
        self.choke(shot, times, ctx);
      }
      Walkway::Sealed => self.reopen(shot, ctx),
      Walkway::Choked => {}
    }
  }

  /// Each hit of a Root Lash chokes an open walkway of its own
  fn on_lash_hit(&mut self, ctx: &SystemContext) {
    if let Some(shot) = self.open_shots().choose(&mut rand::rng()).copied() {
      self.choke(shot, 1, ctx);
    }
  }

  /// Choke a walkway for good, scoring `times` over for a burst
  fn choke(&mut self, shot: MainShot, times: u8, ctx: &SystemContext) {
    self.run.score_hit(times, ctx);

    self.walkways[shot as usize] = Walkway::Choked;
    if self
      .sealing
      .as_ref()
      .is_some_and(|(sealing, _)| *sealing == shot)
    {
      self.stop_warning(ctx);
    }
    self.refresh(ctx);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(shot.hex_ring(), Rgba::green()));

    let chokes = self.chokes();
    log::info!("NimbusPromenade: {shot:?} choked, {chokes} of {REQUIRED_CHOKES}");
    if chokes >= REQUIRED_CHOKES {
      self.cancel_seal(ctx);
      self
        .run
        .complete(led_playback!("nimbus_promenade/complete.scene"), ctx);
      return;
    }

    ctx.play_sfx(sounds::city_hit(chokes as f32 / REQUIRED_CHOKES as f32));
    self.run.progress(chokes);
  }

  fn reopen(&mut self, shot: MainShot, ctx: &SystemContext) {
    log::info!("NimbusPromenade: {shot:?} reopened");
    self.walkways[shot as usize] = Walkway::Open;
    self.refresh(ctx);
    ctx.play_sfx(sounds::WOOSH_LIGHT);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(shot.hex_ring(), Rgba::white()));
  }

  /// Telegraph a seal on a random open walkway, as long as it wouldn't be the last one open
  fn warn_seal(&mut self, ctx: &SystemContext) {
    let open = self.open_shots();
    let target = open.choose(&mut rand::rng()).copied();
    let Some(shot) = target.filter(|_| open.len() > 1) else {
      self.schedule_seal(ctx);
      return;
    };

    self.sealing = Some((shot, Self::seal_warning_effect(shot)));
    self.refresh(ctx);
    self.seal_cue = Some(ctx.cues.start(SealLanded, SEAL_WARNING.once()));
  }

  fn seal(&mut self, ctx: &SystemContext) {
    // the warned walkway may have been choked in the meantime
    if let Some((shot, _)) = self.sealing
      && self.walkway(shot) == Walkway::Open
      && self.open_shots().len() > 1
    {
      log::info!("NimbusPromenade: {shot:?} sealed");
      self.walkways[shot as usize] = Walkway::Sealed;
      ctx.play_sfx(sounds::rnd_latch());
      // walkways are numbered in playfield order, and only the fourth has its line recorded
      if shot == MainShot::CenterOrbit {
        ctx.say(Callout::WalkwayFourSealed);
      }
    }
    self.stop_warning(ctx);
    self.refresh(ctx);
    self.schedule_seal(ctx);
  }

  /// Seals come faster as walkways are choked
  fn schedule_seal(&mut self, ctx: &SystemContext) {
    let interval = SEAL_INTERVAL.saturating_sub(SEAL_SPEEDUP * self.chokes() as u32);
    self.seal_cue = Some(ctx.cues.start(SealDue, interval.once()));
  }

  fn cancel_seal(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.seal_cue.take() {
      ctx.cues.cancel(cue);
    }
    self.stop_warning(ctx);
  }

  fn stop_warning(&mut self, ctx: &SystemContext) {
    if let Some((_, mut warning)) = self.sealing.take() {
      warning.stop(ctx);
    }
  }

  /// Show the walkways as they are: open ones in magenta and choked ones green. Sealed ones are dark.
  fn refresh(&mut self, ctx: &SystemContext) {
    let sealing = self.sealing.as_ref().map(|(shot, _)| *shot);
    let open: Vec<MainShot> = self
      .open_shots()
      .into_iter()
      .filter(|shot| Some(*shot) != sealing)
      .collect();
    self.attention_effect.stop(ctx);
    self.attention_effect = Self::attention_effect(&open);

    // after the attention stops, as that gives up every LED it had, which a newly choked one was among
    for shot in MainShot::ALL {
      if self.walkway(shot) == Walkway::Choked {
        ctx.declare_leds(shot.hex_center(), Rgba::green());
      }
    }
  }
}

impl System for NimbusPromenadeMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::NimbusPromenade)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
  }

  /// A resume keeps the choked walkways and reopens the rest
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    for walkway in self.walkways.iter_mut() {
      if *walkway == Walkway::Sealed {
        *walkway = Walkway::Open;
      }
    }
    self.refresh(ctx);
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.cancel_seal(ctx);
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<SealDue>() {
      self.warn_seal(ctx);
    } else if event.is::<SealLanded>() {
      self.seal(ctx);
    } else if event.is::<ScoopEjected<Lower>>() && self.seal_cue.is_none() {
      // the Steward starts sealing once the ball is in play
      self.schedule_seal(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      for _ in 0..lash.hits {
        self.on_lash_hit(ctx);
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
    if let Some((_, warning)) = self.sealing.as_mut() {
      warning.apply(delta, ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct SealDue;

#[derive(serde::Serialize, Event)]
struct SealLanded;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Walkway {
  Open,
  /// Dark. A hit reopens it.
  Sealed,
  /// Green for the rest of the mode
  Choked,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    NimbusPromenadeMode::seal_warning_effect(MainShot::LeftOrbit);
  }

  #[test]
  fn sealed_walkways_are_not_open_or_choked() {
    let mut mode = NimbusPromenadeMode::new();
    mode.walkways[MainShot::DomeRamp as usize] = Walkway::Choked;
    mode.walkways[MainShot::ArcRamp as usize] = Walkway::Sealed;
    assert_eq!(mode.chokes(), 1);
    assert_eq!(mode.open_shots().len(), 4);
  }
}
