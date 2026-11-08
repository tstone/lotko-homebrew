use std::collections::HashSet;

use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{BallManagementExt, GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{self, ModeManager};
use crate::systems::sounds;

/// Z of the lit shots' hex centers
const LIT_SHOT_Z: i8 = 10;

/// Lights the left scoop (DESIGN.md, Starting modes). All 6 main shots start lit, in white on their hex centers. A hit
/// shot goes out and counts, and enough of them light the scoop. Each time gets harder: 2, then 3, then 4 hits.
/// A mini-wizard lights the scoop just by being reached.
///
/// Stops while a mode runs, so a lit scoop waits for the mode to end. Belongs in the player template.
#[derive(Clone)]
pub struct ScoopQualifier {
  count: QualifyCount,
  /// What the scoop is showing, as the color it's lit in
  lit_effect: Option<LedPlayback>,
  hit_effect: LedPlayback,
  handle: SystemHandle,
}

impl ScoopQualifier {
  pub fn new() -> Self {
    Self {
      count: QualifyCount::new(),
      lit_effect: None,
      hit_effect: led_playback!("exclusive_qualification_hit.scene").stopped(),
      handle: SystemHandle::default(),
    }
  }

  /// Whether a ball in the scoop gets the mode select menu: it's qualified, and no mode or multiball is running
  pub fn opens_menu(&self, mode_manager: &ModeManager, balls_in_play: u8) -> bool {
    (self.count.is_qualified() || mode_manager.mini_wizard_available())
      && mode_manager.current_mode().is_none()
      && balls_in_play <= 1
  }

  /// A mode was chosen, so the scoop goes out and the next qualify needs another hit
  pub fn used(&mut self, ctx: &ServiceContext) {
    self.count.next_round();
    let ctx = &ctx.for_system(self.handle);
    self.stop_lit_effect(ctx);
    self.render_shots(ctx);
  }

  /// Count something other than a main shot as a lit shot, e.g. a Surveillance set
  pub fn add_lit_shot(&mut self, ctx: &ServiceContext) {
    let ctx = &ctx.for_system(self.handle);
    if self.count.add_hit() {
      self.on_hit(ctx);
    }
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if self.count.hit_shot(shot) {
      self.on_hit(ctx);
    }
  }

  fn on_hit(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::EXCL_QUAL_HIT);
    self.hit_effect.reset();
    self.hit_effect.play();
    ctx.play_sfx(if self.count.is_qualified() {
      sounds::ARP_HIT1
    } else {
      sounds::rnd_lane_hit()
    });
    self.render_shots(ctx);
  }

  /// White on what's lit, nothing on what isn't
  fn render_shots(&self, ctx: &SystemContext) {
    for shot in MainShot::ALL {
      let center = shot.hex_center().at_z(LIT_SHOT_Z);
      if self.count.is_lit(shot) {
        ctx.declare_leds(&center, ColorSequence::solid(Rgba::white()));
      } else {
        ctx.undeclare_leds(&center);
      }
    }
  }

  fn update_lit_effect(&mut self, ctx: &SystemContext) {
    let (lit, color) = {
      let mode_manager = ctx.systems.expect::<ModeManager>();
      let color = mode_manager
        .available_modes()
        .first()
        .map(|mode| mode.color());
      (self.opens_menu(&mode_manager, ctx.balls_in_play()), color)
    };

    if !lit {
      self.stop_lit_effect(ctx);
    } else if self.lit_effect.is_none() {
      self.lit_effect = Some(
        led_playback!("startable/left_scoop_attention.scene")
          .param("color", color.unwrap_or(Rgba::white())),
      );
      ctx
        .systems
        .expect::<ModeManager>()
        .set_scoop_lit(true, ctx.into());
    }
  }

  fn stop_lit_effect(&mut self, ctx: &SystemContext) {
    if let Some(mut effect) = self.lit_effect.take() {
      effect.stop(ctx);
      ctx
        .systems
        .expect::<ModeManager>()
        .set_scoop_lit(false, ctx.into());
    }
  }
}

impl System for ScoopQualifier {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    let mode_manager = ctx.systems.expect::<ModeManager>();
    ctx.states.is(TurnPhase::Active)
      && mode_manager.current_mode().is_none()
      && !mode_manager.available_modes().is_empty()
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.update_lit_effect(ctx);
    self.hit_effect.apply(delta, ctx);
    if let Some(effect) = self.lit_effect.as_mut() {
      effect.apply(delta, ctx);
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    ctx.deactivate_led_declarations();
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.render_shots(ctx);
  }
}

/// What's been hit toward lighting the scoop
#[derive(Debug, Clone)]
struct QualifyCount {
  required: u8,
  hits: u8,
  /// Shots that have gone out this round
  out: HashSet<MainShot>,
}

impl QualifyCount {
  fn new() -> Self {
    Self {
      required: 2,
      hits: 0,
      out: HashSet::new(),
    }
  }

  fn is_qualified(&self) -> bool {
    self.hits >= self.required
  }

  fn is_lit(&self, shot: MainShot) -> bool {
    !self.is_qualified() && !self.out.contains(&shot)
  }

  /// Returns whether the shot was lit
  fn hit_shot(&mut self, shot: MainShot) -> bool {
    let lit = self.is_lit(shot);
    if lit {
      self.out.insert(shot);
      self.hits += 1;
    }
    lit
  }

  /// Returns whether it counted, as nothing does once the scoop is lit
  fn add_hit(&mut self) -> bool {
    let counts = !self.is_qualified();
    if counts {
      self.hits += 1;
    }
    counts
  }

  fn next_round(&mut self) {
    self.required += 1;
    self.hits = 0;
    self.out.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_hit_shot_goes_out_and_counts_once() {
    let mut count = QualifyCount::new();
    assert!(count.hit_shot(MainShot::ArcRamp));
    assert!(!count.is_lit(MainShot::ArcRamp));
    assert!(count.is_lit(MainShot::DomeRamp));
    assert!(!count.hit_shot(MainShot::ArcRamp));
    assert!(!count.is_qualified());
  }

  #[test]
  fn the_required_hits_qualify_and_no_shots_stay_lit() {
    let mut count = QualifyCount::new();
    count.hit_shot(MainShot::ArcRamp);
    count.hit_shot(MainShot::DomeRamp);
    assert!(count.is_qualified());
    assert!(MainShot::ALL.iter().all(|shot| !count.is_lit(*shot)));
    assert!(!count.hit_shot(MainShot::LeftOrbit));
  }

  #[test]
  fn each_round_needs_another_hit_with_all_shots_lit_again() {
    let mut count = QualifyCount::new();
    count.hit_shot(MainShot::ArcRamp);
    count.hit_shot(MainShot::DomeRamp);
    count.next_round();

    assert!(MainShot::ALL.iter().all(|shot| count.is_lit(*shot)));
    count.hit_shot(MainShot::ArcRamp);
    count.hit_shot(MainShot::DomeRamp);
    assert!(!count.is_qualified());
    count.hit_shot(MainShot::LeftOrbit);
    assert!(count.is_qualified());
  }

  #[test]
  fn another_source_counts_as_a_lit_shot_without_putting_one_out() {
    let mut count = QualifyCount::new();
    assert!(count.add_hit());
    assert!(MainShot::ALL.iter().all(|shot| count.is_lit(*shot)));
    assert!(count.add_hit());
    assert!(!count.add_hit());
  }
}
