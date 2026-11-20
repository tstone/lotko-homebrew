use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::left_outlane::{self, LeftOutlaneRollover};
use crate::hardware::right_outlane::{self, RightOutlaneRollover};
use crate::hardware::right_pass_lane::RightPassLaneHit;
use crate::hardware::slingshots::{self, LeftSlingHit, RightSlingHit};
use crate::systems::game::{fill_scene, heartbeat_scene};
use crate::systems::{Callout, SayExt, sounds};

/// Sling hits to light a side, for its first save and each after (start values). Past the last, it stays there.
const REQUIRED_HITS: [u8; 4] = [8, 12, 18, 24];

/// How long after a ball goes through an outlane a drain counts as through it
const OUTLANE_DRAIN: Duration = Duration::from_secs(3);

/// How long the hidden save lasts after the right pass lane
const PASS_LANE_SAVE: Duration = Duration::from_secs(3);

/// LEDs in each post ring
const RING_LEDS: usize = 8;

/// The roots catch the ball (DESIGN.md, Outlane saves). Sling hits grow the roots up that side's two post rings, and
/// with both full that side's outlane is lit. A drain through a lit outlane is saved, relaunching the ball, and the
/// side then needs more hits to light again. Surveillance can light a side too ([`OutlaneSaveSystem::light`]).
///
/// A lit side stays lit until it's used, from ball to ball. It isn't used in multiball, which has its own save.
///
/// The Oaks speak for every save here, including the timed ball save's in single-ball play.
///
/// The right pass lane's lower switch also gives a short, hidden save on the right outlane, for a ball that doesn't make
/// it through.
///
/// A participant in the `BallDraining` routine, after `BallSaveSystem` so a timed save is used first. Belongs in the
/// player template.
#[derive(Clone)]
pub struct OutlaneSaveSystem {
  sides: [Outlane; 2],
  /// The outlane the ball last went through, while a drain still counts as through it
  recent: Option<(Side, u64)>,
  /// While the hidden pass lane save lasts
  pass_lane_timer: Option<u64>,
  ring_scenes: [[ScenePlayer; 2]; 2],
  lit_scenes: [Option<ScenePlayer>; 2],
  handle: SystemHandle,
}

impl OutlaneSaveSystem {
  pub fn new() -> Self {
    Self {
      sides: [Outlane::new(), Outlane::new()],
      recent: None,
      pass_lane_timer: None,
      ring_scenes: Side::ALL
        .map(|side| Self::ring_scenes(side, &Outlane::new()).map(ScenePlayer::new)),
      lit_scenes: [None, None],
      handle: SystemHandle::default(),
    }
  }

  fn ring_scenes(side: Side, outlane: &Outlane) -> [PlayableScene; 2] {
    let lit = outlane.grown(RING_LEDS);
    side
      .rings()
      .map(|ring| fill_scene(&ring.q(), RING_LEDS, lit, Rgba::lime()))
  }

  /// Light a side that isn't lit, if there is one. Returns whether one was.
  pub fn light(&mut self, ctx: &ServiceContext) -> bool {
    let ctx = &ctx.for_system(self.handle);
    let unlit: Vec<Side> = Side::ALL
      .into_iter()
      .filter(|side| !self.sides[*side as usize].lit)
      .collect();
    let Some(side) = unlit.choose(&mut rand::rng()).copied() else {
      return false;
    };
    self.sides[side as usize].lit = true;
    self.on_lit(side, ctx);
    true
  }

  fn on_sling(&mut self, side: Side, ctx: &SystemContext) {
    if self.sides[side as usize].grow() {
      self.on_lit(side, ctx);
    }
    self.render(side, ctx);
  }

  fn on_lit(&mut self, side: Side, ctx: &SystemContext) {
    log::info!("OutlaneSave: {side:?} lit");
    ctx.play_sfx(sounds::ARP_HIT1);
    ctx.say(Callout::OakHold);
    self.render(side, ctx);
  }

  fn on_outlane(&mut self, side: Side, ctx: &SystemContext) {
    if let Some((_, timer)) = self.recent.take() {
      ctx.timers.cancel(timer);
    }
    self.recent = Some((side, ctx.timers.start(OutlaneDrainOver, OUTLANE_DRAIN.once())));
  }

  fn on_pass_lane(&mut self, ctx: &SystemContext) {
    if let Some(timer) = self.pass_lane_timer.take() {
      ctx.timers.cancel(timer);
    }
    self.pass_lane_timer = Some(ctx.timers.start(PassLaneSaveOver, PASS_LANE_SAVE.once()));
  }

  /// Save the drain if it came through a lit outlane. Returns whether it did.
  fn try_save(&mut self, draining: &BallDraining, ctx: &SystemContext) -> bool {
    // multiball has its own save, and the outlane stays lit for after
    if draining.in_play > 0 {
      return false;
    }
    let Some((side, timer)) = self.recent.take() else {
      return false;
    };
    ctx.timers.cancel(timer);

    if side == Side::Right && self.pass_lane_timer.is_some() {
      log::info!("OutlaneSave: pass lane save");
    } else if self.sides[side as usize].use_save() {
      log::info!("OutlaneSave: {side:?} save used");
      ctx.say(Callout::OakNotYet);
      ctx.say(Callout::RetrievalFailed);
      self.render(side, ctx);
    } else {
      return false;
    }

    ctx.eject_balls(draining.count, Launch::Auto);
    true
  }

  fn render(&mut self, side: Side, ctx: &SystemContext) {
    let outlane = &self.sides[side as usize];
    for (player, scene) in self.ring_scenes[side as usize]
      .iter_mut()
      .zip(Self::ring_scenes(side, outlane))
    {
      player.replace_scene(scene, ctx);
    }
    let lit_scene = &mut self.lit_scenes[side as usize];
    if outlane.lit && lit_scene.is_none() {
      *lit_scene = Some(ScenePlayer::started(
        heartbeat_scene(&side.led().q(), Rgba::lime(), Duration::from_millis(1600)),
        ctx,
      ));
    } else if !outlane.lit
      && let Some(mut scene) = lit_scene.take()
    {
      scene.stop(ctx);
    }
  }
}

impl System for OutlaneSaveSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is_any(&[TurnPhase::Starting, TurnPhase::Active])
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = ctx.handle();
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if let Some((_, timer)) = self.recent.take() {
      ctx.timers.cancel(timer);
    }
    if let Some(timer) = self.pass_lane_timer.take() {
      ctx.timers.cancel(timer);
    }
    ctx.deactivate_light_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftSlingHit>() {
      self.on_sling(Side::Left, ctx);
    } else if event.is::<RightSlingHit>() {
      self.on_sling(Side::Right, ctx);
    } else if event.is::<LeftOutlaneRollover>() {
      self.on_outlane(Side::Left, ctx);
    } else if event.is::<RightOutlaneRollover>() {
      self.on_outlane(Side::Right, ctx);
    } else if let Some(hit) = event.downcast_ref::<RightPassLaneHit>()
      && !hit.upper
    {
      self.on_pass_lane(ctx);
    } else if event.is::<BallSaved>() && ctx.balls_in_play() == 0 {
      // the timed save is the roots catching the ball too. In multiball they're kept quiet, as they'd say it every drain.
      ctx.say(Callout::OakAgain);
    } else if event.is::<OutlaneDrainOver>() {
      self.recent = None;
    } else if event.is::<PassLaneSaveOver>() {
      self.pass_lane_timer = None;
    }
  }

  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    let Some(draining) = routine.downcast_ref::<BallDraining>() else {
      return;
    };
    if self.try_save(draining, ctx) {
      ctx.routines.halt::<BallDraining>();
    } else {
      ctx.routines.complete_step::<BallDraining>();
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    for scene in self.ring_scenes.iter_mut().flatten() {
      scene.start(ctx);
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
  Left,
  Right,
}

impl Side {
  const ALL: [Side; 2] = [Self::Left, Self::Right];

  /// The side's two sling post rings
  fn rings(self) -> [&'static LedDefinition; 2] {
    match self {
      Self::Left => [&slingshots::POST_LEDS1, &slingshots::POST_LEDS2],
      Self::Right => [&slingshots::POST_LEDS3, &slingshots::POST_LEDS4],
    }
  }

  fn led(self) -> &'static LedDefinition {
    match self {
      Self::Left => &left_outlane::LED,
      Self::Right => &right_outlane::LED,
    }
  }
}

/// One side's roots
#[derive(Debug, Clone)]
struct Outlane {
  hits: u8,
  /// Saves used so far, each making the next harder to light
  used: usize,
  lit: bool,
}

impl Outlane {
  fn new() -> Self {
    Self {
      hits: 0,
      used: 0,
      lit: false,
    }
  }

  fn required(&self) -> u8 {
    REQUIRED_HITS[self.used.min(REQUIRED_HITS.len() - 1)]
  }

  /// A sling hit. Returns whether it lit the side.
  fn grow(&mut self) -> bool {
    if self.lit {
      return false;
    }
    self.hits += 1;
    self.lit = self.hits >= self.required();
    self.lit
  }

  /// Returns whether the side was lit, in which case the save is used
  fn use_save(&mut self) -> bool {
    if !self.lit {
      return false;
    }
    self.lit = false;
    self.hits = 0;
    self.used += 1;
    true
  }

  /// How many of a ring's `leds` the roots have grown up
  fn grown(&self, leds: usize) -> usize {
    if self.lit {
      leds
    } else {
      self.hits as usize * leds / self.required() as usize
    }
  }
}

#[derive(serde::Serialize, Event)]
struct OutlaneDrainOver;

#[derive(serde::Serialize, Event)]
struct PassLaneSaveOver;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn enough_sling_hits_light_the_side() {
    let mut outlane = Outlane::new();
    for _ in 0..7 {
      assert!(!outlane.grow());
    }
    assert_eq!(outlane.grown(RING_LEDS), 7);
    assert!(outlane.grow());
    assert!(!outlane.grow());
  }

  #[test]
  fn each_save_used_needs_more_hits_to_light_again() {
    let mut outlane = Outlane::new();
    outlane.lit = true;
    assert!(outlane.use_save());
    assert!(!outlane.use_save());
    assert_eq!(outlane.required(), 12);
    outlane.used = 10;
    assert_eq!(outlane.required(), 24);
  }
}
