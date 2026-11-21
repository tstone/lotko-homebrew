use frontbox_core::prelude::*;
use frontbox_pinball::tags::Playfield;
use frontbox_lighting::prelude::*;
use frontbox_pinball::{GameManagerExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::cabinet::action_button;
use crate::hardware::drop_bank::{self, DropBankCleared, DropBankTargetHit};
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::hardware::plunge_lane;
use crate::systems::game::side_quests::RootLashed;
use crate::systems::game::{
  ExtraBallSource, ExtraBallSystem, Mode, ModeManager, ModeSelectSystem, ModeSlotsChanged, Perk,
  fill_scene, flash_scene, glow_scene, points,
};
use crate::systems::{Callout, SayExt, sounds};

/// The orbits, which each grow roots (TODO.md, decision 8)
const ORBITS: [MainShot; 3] = [
  MainShot::LeftOrbit,
  MainShot::CenterOrbit,
  MainShot::RightOrbit,
];

/// Segments in a hex's outer ring
const SEGMENTS: u8 = 6;

/// Mode shots a Root Lash counts as
const LASH_HITS: u8 = 2;

/// Playfield switch hits after a Root Lash before the roots grow again
const LASH_COOLDOWN_SWITCHES: u32 = 50;

/// LEDs on a drop target's strip, which show its orbit's spreads toward Groundswell
const STRIP_LEDS: usize = 4;

/// Orbit hits grow roots (DESIGN.md, Spread roots), filling the orbit hex's outer ring a segment a hit, or two with the
/// Meridian perk. A full ring spreads the roots, which arms Root Lash on the action button. Pressing it while a mode is
/// running counts as [`LASH_HITS`] shots in that mode ([`RootLashed`]). The roots then stop growing until
/// [`LASH_COOLDOWN_SWITCHES`] playfield switch hits. A spread while a lash is armed only scores. The rings start over
/// each ball.
///
/// An armed lash is bronze on the action button, and pulses lime while there's a mode to strike in.
///
/// The drop targets feed the orbits, one each, in [`ORBITS`] order: knocking one down grows its orbit's ring a segment,
/// and clearing the bank ends a Root Lash's cooldown. Each target's strip shows its orbit's spreads toward Groundswell
/// while it's standing. The Sentinels and Groundswell have the drops to themselves while they run.
///
/// Spreading the roots on all three orbits in one ball lights an extra ball, once a game. Spreading them on all three
/// over the game lights Groundswell at the scoop, and each time after needs one more spread on every orbit.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SpreadRootsSystem {
  /// By orbit, in [`ORBITS`] order
  grown: [u8; 3],
  /// Orbits spread this ball, in [`ORBITS`] order
  spread_this_ball: [bool; 3],
  /// Spreads on each orbit toward lighting Groundswell, in [`ORBITS`] order. Kept for the whole game.
  toward_groundswell: [u8; 3],
  /// Times Groundswell has been lit this game
  groundswell_lit: u8,
  lash: Lash,
  ring_scenes: [ScenePlayer; 3],
  /// Each drop target's strip, showing its orbit's spreads toward Groundswell, in [`ORBITS`] order
  strip_scenes: [ScenePlayer; 3],
  /// On the action button while a lash is armed and a mode is running
  lash_glow: Option<ScenePlayer>,
}

impl SpreadRootsSystem {
  pub fn new() -> Self {
    Self {
      grown: [0; 3],
      spread_this_ball: [false; 3],
      toward_groundswell: [0; 3],
      groundswell_lit: 0,
      lash: Lash::Ready,
      ring_scenes: ORBITS.map(|orbit| ScenePlayer::new(Self::ring_scene(orbit, 0))),
      strip_scenes: [0, 1, 2].map(|index| ScenePlayer::new(Self::strip_scene(index, 0, 0))),
      lash_glow: None,
    }
  }

  /// A drop target's strip, filled by how far its orbit is toward the `needed` spreads that light Groundswell
  fn strip_scene(index: usize, spreads: u8, needed: u8) -> PlayableScene {
    fill_scene(&target_strip(index).q(), STRIP_LEDS, strip_lit(spreads, needed), Rgba::bronze())
  }

  fn ring_scene(orbit: MainShot, grown: u8) -> PlayableScene {
    fill_scene(
      orbit.hex_ring(),
      SEGMENTS as usize,
      grown as usize,
      Rgba::bronze(),
    )
  }

  fn grow(&mut self, shot: MainShot, ctx: &SystemContext) {
    let Some(index) = ORBITS.iter().position(|orbit| *orbit == shot) else {
      return;
    };
    if matches!(self.lash, Lash::CoolingDown(_)) {
      return;
    }

    let perk = ctx
      .systems
      .expect::<ModeManager>()
      .has_perk(Perk::DoubleRoots);
    let grown = (self.grown[index] + if perk { 2 } else { 1 }).min(SEGMENTS);
    ctx.add_points(points::ROOT_SEGMENT);

    if grown < SEGMENTS {
      ctx.play_sfx(sounds::HIT_ORGANIC_LOW1);
      self.grown[index] = grown;
    } else {
      self.spread(index, ctx);
      self.grown[index] = 0;
    }
    self.ring_scenes[index].replace_scene(Self::ring_scene(shot, self.grown[index]), ctx);
  }

  fn spread(&mut self, index: usize, ctx: &SystemContext) {
    let shot = ORBITS[index];
    log::info!("SpreadRoots: {shot:?} spread");
    ctx.add_points(points::ROOTS_SPREAD);
    ctx.play_sfx(sounds::IMPACT_DIRT);
    ctx.play_scene(&flash_scene(shot.hex_ring(), Rgba::bronze()));

    if self.lash == Lash::Ready {
      log::info!("SpreadRoots: root lash armed");
      self.lash = Lash::Armed;
      self.render(ctx);
    }

    self.spread_this_ball[index] = true;
    if self.spread_this_ball.iter().all(|spread| *spread) {
      ctx
        .systems
        .expect::<ExtraBallSystem>()
        .light(ExtraBallSource::RootNetwork, ctx.into());
    }

    self.toward_groundswell[index] = self.toward_groundswell[index].saturating_add(1);
    self.update_strips(ctx);
    if groundswell_due(&self.toward_groundswell, self.groundswell_lit) {
      self.light_groundswell(ctx);
    }
  }

  /// Lights Groundswell at the scoop, unless it's already lit or waiting to be resumed. Then the spreads count again
  /// from none.
  fn light_groundswell(&mut self, ctx: &SystemContext) {
    let lit = ctx
      .systems
      .expect::<ModeManager>()
      .light_earned(Mode::Groundswell);
    if !lit {
      return;
    }
    log::info!("SpreadRoots: Groundswell lit, {} times this game", self.groundswell_lit + 1);
    self.toward_groundswell = [0; 3];
    self.groundswell_lit += 1;
    self.update_strips(ctx);
    ctx.say(Callout::SeismicActivityAlert);
  }

  /// The Sentinels (the drops are their drones) or Groundswell (the drops are the foundations) have the drops
  fn drops_taken(ctx: &SystemContext) -> bool {
    let mode_manager = ctx.systems.expect::<ModeManager>();
    mode_manager.sentinels_running() || mode_manager.current_mode() == Some(Mode::Groundswell)
  }

  /// A drop target grows its orbit's ring, like a hit to that orbit
  fn on_target_down(&mut self, target: u8, ctx: &SystemContext) {
    let Some(orbit) = ORBITS.get(target as usize - 1).copied() else {
      return;
    };
    if Self::drops_taken(ctx) {
      return;
    }
    self.grow(orbit, ctx);
  }

  /// A cleared bank ends a Root Lash's cooldown, so the roots grow again
  fn on_bank_cleared(&mut self, ctx: &SystemContext) {
    if Self::drops_taken(ctx) || !matches!(self.lash, Lash::CoolingDown(_)) {
      return;
    }
    log::info!("SpreadRoots: drop bank cleared, growing again");
    self.lash = Lash::Ready;
  }

  /// Refills the strips from the spreads toward Groundswell
  fn update_strips(&mut self, ctx: &SystemContext) {
    let needed = self.groundswell_lit + 1;
    for (index, scene) in self.strip_scenes.iter_mut().enumerate() {
      scene.replace_scene(Self::strip_scene(index, self.toward_groundswell[index], needed), ctx);
    }
  }

  /// A strip is lit while its target is standing, as that's when hitting it does something, and not while another
  /// system has the drops
  fn render_strips(&mut self, ctx: &SystemContext) {
    let taken = Self::drops_taken(ctx);
    for (index, scene) in self.strip_scenes.iter_mut().enumerate() {
      let down = ctx.switches.is_closed(target_switch(index).name).unwrap_or(false);
      if taken || down {
        scene.stop(ctx);
      } else {
        scene.start(ctx);
      }
    }
  }

  /// Strike with an armed lash, if there's a mode for it to count in. Not while the action button is for launching a
  /// ball or choosing in the mode select menu.
  fn strike(&mut self, ctx: &SystemContext) {
    if self.lash != Lash::Armed
      || ctx
        .switches
        .is_closed(plunge_lane::SWITCH.name)
        .unwrap_or(false)
      || ctx.systems.expect::<ModeSelectSystem>().is_menu_open()
      || ctx.systems.expect::<ModeManager>().current_mode().is_none()
    {
      return;
    }
    log::info!("SpreadRoots: root lash");
    self.lash = Lash::CoolingDown(0);
    ctx.play_sfx(sounds::IMPACT_DIRT);
    for orbit in ORBITS {
      ctx.play_scene(&flash_scene(orbit.hex_ring(), Rgba::bronze()));
    }
    ctx.emit(RootLashed { hits: LASH_HITS });
    self.render(ctx);
  }

  fn on_playfield_switch(&mut self) {
    if let Lash::CoolingDown(switches) = &mut self.lash {
      *switches += 1;
      if *switches >= LASH_COOLDOWN_SWITCHES {
        log::info!("SpreadRoots: growing again");
        self.lash = Lash::Ready;
      }
    }
  }

  /// The action button while a lash is armed: lime and pulsing with a mode to strike in, and bronze until there is one
  fn render(&mut self, ctx: &SystemContext) {
    let led = action_button::LED.q();
    let armed = self.lash == Lash::Armed;
    let in_mode = ctx.systems.expect::<ModeManager>().current_mode().is_some();

    if armed && in_mode {
      ctx.undeclare_lights(&led, 0);
      self
        .lash_glow
        .get_or_insert_with(|| ScenePlayer::started(glow_scene(&led, Rgba::lime()), ctx));
      return;
    }
    if let Some(mut glow) = self.lash_glow.take() {
      glow.stop(ctx);
    }
    if armed {
      ctx.declare_lights(&led, 0, Rgba::bronze());
    } else {
      ctx.undeclare_lights(&led, 0);
    }
  }

  /// Stopped, to start again on the next tick it's active
  fn reset_rings(&mut self, ctx: &SystemContext) {
    self.grown = [0; 3];
    for (scene, orbit) in self.ring_scenes.iter_mut().zip(ORBITS) {
      scene.stop(ctx);
      scene.replace_scene(Self::ring_scene(orbit, 0), ctx);
    }
  }
}

impl System for SpreadRootsSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    self.render(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.grow(hit.shot, ctx);
    } else if let Some(closed) = event.downcast_ref::<SwitchClosed>() {
      if closed.switch.name == action_button::SWITCH.name {
        self.strike(ctx);
      } else if closed.switch.has_tag::<Playfield>() {
        self.on_playfield_switch();
      }
    } else if let Some(DropBankTargetHit(target)) = event.downcast_ref::<DropBankTargetHit>() {
      self.on_target_down(*target, ctx);
    } else if event.is::<DropBankCleared>() {
      self.on_bank_cleared(ctx);
    } else if event.is::<ModeSlotsChanged>() {
      self.render(ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    for scene in self.ring_scenes.iter_mut() {
      scene.start(ctx);
    }
    self.render_strips(ctx);
  }

  /// The ball is over, and the rings and the network start over in the next
  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.spread_this_ball = [false; 3];
    self.reset_rings(ctx);
    if let Some(mut glow) = self.lash_glow.take() {
      glow.stop(ctx);
    }
    for scene in self.strip_scenes.iter_mut() {
      scene.stop(ctx);
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lash {
  /// Growing, and the next spread arms one
  Ready,
  /// On the action button, waiting to be struck
  Armed,
  /// Struck, and not growing, with the playfield switch hits since
  CoolingDown(u32),
}

/// LEDs lit on a strip for `spreads` of the `needed` that light Groundswell
fn strip_lit(spreads: u8, needed: u8) -> usize {
  if needed == 0 {
    return 0;
  }
  spreads.min(needed) as usize * STRIP_LEDS / needed as usize
}

/// The strip of the drop target that feeds the orbit at `index` in [`ORBITS`]
fn target_strip(index: usize) -> &'static LedDefinition {
  match index {
    0 => &drop_bank::TARGET1_LEDS,
    1 => &drop_bank::TARGET2_LEDS,
    _ => &drop_bank::TARGET3_LEDS,
  }
}

/// The switch of the drop target that feeds the orbit at `index`, closed while the target is down
fn target_switch(index: usize) -> &'static SwitchDefinition {
  match index {
    0 => &drop_bank::TARGET1,
    1 => &drop_bank::TARGET2,
    _ => &drop_bank::TARGET3,
  }
}

/// Every orbit has spread enough to light Groundswell: once each the first time, and one more each time after
fn groundswell_due(spreads: &[u8; 3], times_lit: u8) -> bool {
  spreads.iter().all(|spread| *spread > times_lit)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn groundswell_needs_every_orbit_and_one_more_each_time() {
    assert!(!groundswell_due(&[1, 1, 0], 0));
    assert!(groundswell_due(&[1, 1, 1], 0));
    assert!(!groundswell_due(&[2, 2, 1], 1));
    assert!(groundswell_due(&[2, 3, 2], 1));
  }

  #[test]
  fn a_strip_fills_toward_the_spreads_groundswell_needs() {
    assert_eq!(strip_lit(0, 1), 0);
    assert_eq!(strip_lit(1, 1), STRIP_LEDS);
    assert_eq!(strip_lit(1, 2), 2);
    assert_eq!(strip_lit(1, 3), 1);
    assert_eq!(strip_lit(5, 3), STRIP_LEDS);
  }

  #[test]
  fn the_strip_scenes_load() {
    SpreadRootsSystem::strip_scene(0, 1, 2);
  }
}
