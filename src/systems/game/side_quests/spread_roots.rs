use frontbox_core::prelude::*;
use frontbox_core::tags::Playfield;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::cabinet::action_button;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::hardware::plunge_lane;
use crate::systems::game::side_quests::RootLashed;
use crate::systems::game::{
  ExtraBallSource, ExtraBallSystem, ModeManager, ModeSelectSystem, ModeSlotsChanged, Perk,
  fill_scene, flash_scene, glow_scene, points,
};
use crate::systems::sounds;

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

/// Orbit hits grow roots (DESIGN.md, Spread roots), filling the orbit hex's outer ring a segment a hit, or two with the
/// Meridian perk. A full ring spreads the roots, which arms Root Lash on the action button. Pressing it while a mode is
/// running counts as [`LASH_HITS`] shots in that mode ([`RootLashed`]). The roots then stop growing until
/// [`LASH_COOLDOWN_SWITCHES`] playfield switch hits. A spread while a lash is armed only scores. The rings start over
/// each ball.
///
/// An armed lash is bronze on the action button, and pulses lime while there's a mode to strike in.
///
/// Spreading the roots on all three orbits in one ball lights an extra ball, once a game.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SpreadRootsSystem {
  /// By orbit, in [`ORBITS`] order
  grown: [u8; 3],
  /// Orbits spread this ball, in [`ORBITS`] order
  spread_this_ball: [bool; 3],
  lash: Lash,
  ring_scenes: [ScenePlayer; 3],
  /// On the action button while a lash is armed and a mode is running
  lash_glow: Option<ScenePlayer>,
}

impl SpreadRootsSystem {
  pub fn new() -> Self {
    Self {
      grown: [0; 3],
      spread_this_ball: [false; 3],
      lash: Lash::Ready,
      ring_scenes: ORBITS.map(|orbit| ScenePlayer::new(Self::ring_scene(orbit, 0))),
      lash_glow: None,
    }
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
      ctx.undeclare_leds(&led);
      self
        .lash_glow
        .get_or_insert_with(|| ScenePlayer::started(glow_scene(&led, Rgba::lime()), ctx));
      return;
    }
    if let Some(mut glow) = self.lash_glow.take() {
      glow.stop(ctx);
    }
    if armed {
      ctx.declare_leds(&led, Rgba::bronze());
    } else {
      ctx.undeclare_leds(&led);
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
    ctx.activate_led_declarations();
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
    } else if event.is::<ModeSlotsChanged>() {
      self.render(ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    for scene in self.ring_scenes.iter_mut() {
      scene.start(ctx);
    }
  }

  /// The ball is over, and the rings and the network start over in the next
  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.spread_this_ball = [false; 3];
    self.reset_rings(ctx);
    if let Some(mut glow) = self.lash_glow.take() {
      glow.stop(ctx);
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
