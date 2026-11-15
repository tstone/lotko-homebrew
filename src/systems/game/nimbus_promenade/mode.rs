use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::ScoopEjected;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::lower_scoop::Lower;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{
  Mode, ModeManager, ModeRun, RootLashed, SporesSystem, flash_scene, glow_scene,
};
use crate::systems::{Callout, SayExt, sounds};

const REQUIRED_CLOSURES: u8 = 3;

/// A walkway hit once, between open (magenta) and closed (dark)
const WEAKENED_COLOR: Rgba<u8> = Rgba([120, 0, 255, 255]);

/// The time between blocks, before closures speed them up (start value)
const BLOCK_INTERVAL: Duration = Duration::from_secs(6);

/// How much sooner each closure brings the next block
const BLOCK_SPEEDUP: Duration = Duration::from_secs(1);

/// How long a block is telegraphed before it comes down
const BLOCK_WARNING: Duration = Duration::from_secs(1);

/// How long a walkway stays blocked (start value)
const BLOCK_DURATION: Duration = Duration::from_secs(8);

/// Choke points (DESIGN.md). Each of the 6 main shots is a walkway the forest closes in two hits: the first weakens it
/// (magenta to purple) and the second closes it (dark). The Steward blocks walkways one at a time for a while, lit red,
/// and a blocked walkway can't be hit.
pub struct NimbusPromenadeMode {
  run: ModeRun,
  /// By shot, in [`MainShot::ALL`] order
  walkways: [Walkway; 6],
  /// By shot, in [`MainShot::ALL`] order: the cue that lifts the block, while it's blocked. A blocked walkway keeps its
  /// progress for when the block lifts.
  blocks: [Option<u64>; 6],
  /// Open walkways, other than one about to be blocked
  open_scene: ScenePlayer,
  /// Weakened walkways, other than one about to be blocked
  weakened_scene: ScenePlayer,
  /// The walkway about to be blocked, and its warning
  blocking: Option<(MainShot, ScenePlayer)>,
  /// The next block or its warning, while they're running
  block_cue: Option<u64>,
  /// The block the Steward last complained about being hit, so a run of hits on it says it once
  complained_about: Option<u64>,
}

impl NimbusPromenadeMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::NimbusPromenade, REQUIRED_CLOSURES),
      walkways: [Walkway::Open; 6],
      blocks: [None; 6],
      open_scene: ScenePlayer::new(Self::open_scene(&MainShot::ALL)),
      weakened_scene: ScenePlayer::new(Self::weakened_scene(&[])),
      blocking: None,
      block_cue: None,
      complained_about: None,
    }
  }

  fn open_scene(shots: &[MainShot]) -> PlayableScene {
    scene!("nimbus_promenade/attention.scene", Self::hex_centers(shots))
  }

  fn weakened_scene(shots: &[MainShot]) -> PlayableScene {
    glow_scene(&Self::hex_centers(shots), WEAKENED_COLOR)
  }

  fn hex_centers(shots: &[MainShot]) -> LedQ {
    LedQ::any(shots.iter().map(|shot| shot.hex_center()).collect())
  }

  fn block_warning_scene(shot: MainShot) -> PlayableScene {
    scene!(
      "nimbus_promenade/block_warning.scene",
      shot.hex_center().clone()
    )
  }

  fn walkway(&self, shot: MainShot) -> Walkway {
    self.walkways[shot as usize]
  }

  fn is_blocked(&self, shot: MainShot) -> bool {
    self.blocks[shot as usize].is_some()
  }

  fn closures(&self) -> u8 {
    self
      .walkways
      .iter()
      .filter(|walkway| **walkway == Walkway::Closed)
      .count() as u8
  }

  /// Walkways that a hit would move on: not closed, and not blocked
  fn hittable_shots(&self) -> Vec<MainShot> {
    MainShot::ALL
      .into_iter()
      .filter(|shot| self.walkway(*shot) != Walkway::Closed && !self.is_blocked(*shot))
      .collect()
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if self.is_blocked(shot) {
      self.on_blocked_hit(shot, ctx);
    } else if self.walkway(shot) != Walkway::Closed {
      // one step a hit, so a burst doubles the points rather than closing a walkway at once
      let times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
      self.advance(shot, times, ctx);
    }
  }

  /// Each hit of a Root Lash moves a walkway of its own on a step
  fn on_lash_hit(&mut self, ctx: &SystemContext) {
    if let Some(shot) = self.hittable_shots().choose(&mut rand::rng()).copied() {
      self.advance(shot, 1, ctx);
    }
  }

  /// Weaken an open walkway or close a weakened one, scoring `times` over for a burst
  fn advance(&mut self, shot: MainShot, times: u8, ctx: &SystemContext) {
    self.run.score_hit(times, ctx);
    if self
      .blocking
      .as_ref()
      .is_some_and(|(blocking, _)| *blocking == shot)
    {
      self.stop_warning(ctx);
    }

    match self.walkway(shot) {
      Walkway::Open => {
        log::info!("NimbusPromenade: {shot:?} weakened");
        self.walkways[shot as usize] = Walkway::Weakened;
        self.refresh(ctx);
        ctx.play_scene(&flash_scene(shot.hex_ring(), WEAKENED_COLOR));
        let closures = self.closures();
        ctx.play_sfx(sounds::city_hit(closures as f32 / REQUIRED_CLOSURES as f32));
      }
      Walkway::Weakened => self.close(shot, ctx),
      Walkway::Closed => {}
    }
  }

  fn close(&mut self, shot: MainShot, ctx: &SystemContext) {
    self.walkways[shot as usize] = Walkway::Closed;
    self.refresh(ctx);
    ctx.play_scene(&flash_scene(shot.hex_ring(), Rgba::green()));

    let closures = self.closures();
    log::info!("NimbusPromenade: {shot:?} closed, {closures} of {REQUIRED_CLOSURES}");
    if closures >= REQUIRED_CLOSURES {
      self.cancel_blocks(ctx);
      self
        .run
        .complete(completing_scene!("nimbus_promenade/complete.scene"), ctx);
      return;
    }

    ctx.play_sfx(sounds::city_hit(closures as f32 / REQUIRED_CLOSURES as f32));
    self.run.progress(closures);
  }

  /// A blocked walkway can't be hit
  fn on_blocked_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    log::info!("NimbusPromenade: {shot:?} hit while blocked");
    ctx.play_sfx(sounds::ERROR1);
    ctx.play_scene(&flash_scene(shot.hex_ring(), Rgba::red()));
    let block = self.blocks[shot as usize];
    if self.complained_about != block {
      self.complained_about = block;
      ctx.say(Callout::RestrictedArea);
    }
  }

  /// Telegraph a block on a random walkway that can still be hit, as long as it wouldn't be the last one
  fn warn_block(&mut self, ctx: &SystemContext) {
    let hittable = self.hittable_shots();
    let target = hittable.choose(&mut rand::rng()).copied();
    let Some(shot) = target.filter(|_| hittable.len() > 1) else {
      self.schedule_block(ctx);
      return;
    };

    self.blocking = Some((shot, ScenePlayer::started(Self::block_warning_scene(shot), ctx)));
    self.refresh(ctx);
    self.block_cue = Some(ctx.cues.start(BlockLanded, BLOCK_WARNING.once()));
  }

  fn block(&mut self, ctx: &SystemContext) {
    // the warned walkway may have been closed in the meantime
    if let Some((shot, _)) = self.blocking
      && self.walkway(shot) != Walkway::Closed
      && self.hittable_shots().len() > 1
    {
      log::info!("NimbusPromenade: {shot:?} blocked");
      self.blocks[shot as usize] = Some(ctx.cues.start(BlockLifted { shot }, BLOCK_DURATION.once()));
      ctx.play_sfx(sounds::rnd_latch());
      // walkways are numbered in playfield order, and only the fourth has its line recorded
      if shot == MainShot::CenterOrbit {
        ctx.say(Callout::WalkwayFourSealed);
      }
    }
    self.stop_warning(ctx);
    self.refresh(ctx);
    self.schedule_block(ctx);
  }

  fn lift_block(&mut self, shot: MainShot, ctx: &SystemContext) {
    log::info!("NimbusPromenade: {shot:?} unblocked");
    self.blocks[shot as usize] = None;
    ctx.undeclare_leds(shot.hex_center());
    self.refresh(ctx);
  }

  /// Blocks come faster as walkways are closed
  fn schedule_block(&mut self, ctx: &SystemContext) {
    let interval = BLOCK_INTERVAL.saturating_sub(BLOCK_SPEEDUP * self.closures() as u32);
    self.block_cue = Some(ctx.cues.start(BlockDue, interval.once()));
  }

  /// Stop blocking, and lift the blocks there are
  fn cancel_blocks(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.block_cue.take() {
      ctx.cues.cancel(cue);
    }
    self.stop_warning(ctx);
    for shot in MainShot::ALL {
      if let Some(cue) = self.blocks[shot as usize].take() {
        ctx.cues.cancel(cue);
        ctx.undeclare_leds(shot.hex_center());
      }
    }
  }

  fn stop_warning(&mut self, ctx: &SystemContext) {
    if let Some((_, mut warning)) = self.blocking.take() {
      warning.stop(ctx);
    }
  }

  /// Show the walkways as they are: open ones magenta, weakened ones purple, and blocked ones red. Closed ones are dark.
  fn refresh(&mut self, ctx: &SystemContext) {
    let blocking = self.blocking.as_ref().map(|(shot, _)| *shot);
    let shown = |state: Walkway| -> Vec<MainShot> {
      self
        .hittable_shots()
        .into_iter()
        .filter(|shot| self.walkway(*shot) == state && Some(*shot) != blocking)
        .collect()
    };
    let (open, weakened) = (shown(Walkway::Open), shown(Walkway::Weakened));
    self.open_scene.replace_scene(Self::open_scene(&open), ctx);
    self
      .weakened_scene
      .replace_scene(Self::weakened_scene(&weakened), ctx);

    for shot in MainShot::ALL {
      if self.is_blocked(shot) {
        ctx.declare_leds(shot.hex_center(), Rgba::red());
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

  /// A resume keeps each walkway's progress, with no blocks
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.refresh(ctx);
    self.run.begin(ctx);
    // picking up in play after the Sentinels, with no scoop eject to wait for
    if !self.run.is_starting() {
      self.schedule_block(ctx);
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.cancel_blocks(ctx);
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<BlockDue>() {
      self.warn_block(ctx);
    } else if event.is::<BlockLanded>() {
      self.block(ctx);
    } else if let Some(BlockLifted { shot }) = event.downcast_ref::<BlockLifted>() {
      self.lift_block(*shot, ctx);
    } else if event.is::<ScoopEjected<Lower>>() && self.block_cue.is_none() {
      // the Steward starts blocking once the ball is in play
      self.schedule_block(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      for _ in 0..lash.hits {
        self.on_lash_hit(ctx);
      }
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.run.apply(ctx) {
      return;
    }

    self.open_scene.start(ctx);
    self.weakened_scene.start(ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct BlockDue;

#[derive(serde::Serialize, Event)]
struct BlockLanded;

#[derive(serde::Serialize, Event)]
struct BlockLifted {
  shot: MainShot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Walkway {
  /// Magenta
  Open,
  /// Purple. Hit once, and closed by the next.
  Weakened,
  /// Dark for the rest of the mode
  Closed,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    NimbusPromenadeMode::block_warning_scene(MainShot::LeftOrbit);
    NimbusPromenadeMode::weakened_scene(&[MainShot::LeftOrbit]);
  }

  #[test]
  fn closed_and_blocked_walkways_cannot_be_hit() {
    let mut mode = NimbusPromenadeMode::new();
    mode.walkways[MainShot::DomeRamp as usize] = Walkway::Closed;
    mode.walkways[MainShot::LeftOrbit as usize] = Walkway::Weakened;
    mode.blocks[MainShot::ArcRamp as usize] = Some(1);
    assert_eq!(mode.closures(), 1);
    assert_eq!(mode.hittable_shots().len(), 4);
  }
}
