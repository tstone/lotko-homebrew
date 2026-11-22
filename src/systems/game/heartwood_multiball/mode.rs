use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp::ArcRampSubwayHit;
use crate::hardware::captive_ball::CaptiveBallHit;
use crate::hardware::city_map;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{self, MULTIBALL_BALL_SAVE, ModeManager, Multiball};
use crate::systems::{Callout, SayExt, sounds};

/// Hits it takes to light the super jackpot, each time
const JACKPOTS_PER_SUPER: u8 = 5;

/// How long the super jackpot stays lit
const SUPER_JACKPOT_TIME: Duration = Duration::from_secs(20);

/// Arc ramps all the way to the subway that add a ball, once a multiball
const RAMPS_TO_ADD_A_BALL: u8 = 2;

/// How bright a dimmed jackpot is, as a mix toward black
const DIMMED: f32 = 0.7;

/// The Heartwood multiball (DESIGN.md, Multiballs), 2-ball, started by [`HeartwoodCharge`](crate::systems::game::HeartwoodCharge).
/// Every main shot starts as a full jackpot. Hitting it dims it to a half jackpot, and hitting that puts it out. Every
/// 5th jackpot lights the captive ball as the super jackpot for a while, and hitting it lights every jackpot full again.
/// With every jackpot out, the super jackpot stays lit until it's hit. Making the arc ramp to the subway
/// [`RAMPS_TO_ADD_A_BALL`] times adds a ball, once. It ends when a drain that stands leaves one ball or none, or with
/// the turn.
#[derive(Clone)]
pub struct HeartwoodMultiballMode {
  /// The pulse on each full jackpot, in [`MainShot::ALL`] order
  jackpot_scenes: Vec<ScenePlayer>,
  jackpots: [Jackpot; 6],
  super_jackpot_attention_scene: ScenePlayer,
  hit_scene: ScenePlayer,
  super_jackpot_hit_scene: ScenePlayer,
  hits: u8,
  /// Arc ramps made to the subway, toward adding a ball
  subway_ramps: u8,
  super_jackpot_active: bool,
  timer_id: Option<u64>,
  eject_id: Option<u64>,
}

impl HeartwoodMultiballMode {
  pub fn new() -> Self {
    Self {
      jackpot_scenes: MainShot::ALL
        .into_iter()
        .map(|shot| ScenePlayer::new(Self::jackpot_scene(shot)))
        .collect(),
      jackpots: [Jackpot::Full; 6],
      hit_scene: ScenePlayer::new(Self::hit_scene()),
      super_jackpot_attention_scene: ScenePlayer::new(Self::super_jackpot_attention_scene()),
      super_jackpot_hit_scene: ScenePlayer::new(Self::super_jackpot_hit_scene()),
      hits: 0,
      subway_ramps: 0,
      super_jackpot_active: false,
      timer_id: None,
      eject_id: None,
    }
  }

  fn jackpot_scene(shot: MainShot) -> PlayableScene {
    scene!("heartwood_multiball/attention.scene", jackpot_leds(shot))
      .variable("color", Multiball::Heartwood.color())
  }

  fn super_jackpot_attention_scene() -> PlayableScene {
    scene!("heartwood_multiball/super_jackpot_attention.scene")
  }

  fn hit_scene() -> PlayableScene {
    scene!("heartwood_multiball/hit.scene")
  }

  fn super_jackpot_hit_scene() -> PlayableScene {
    scene!("heartwood_multiball/super_jackpot_hit.scene")
  }

  fn progress_scene(hits: u8) -> PlayableScene<Completes> {
    let per = JACKPOTS_PER_SUPER;
    city_map::mode_progress_scene(
      Multiball::Heartwood.color(),
      (hits % per) as f32 / per as f32,
    )
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    let jackpot = &mut self.jackpots[shot as usize];
    let points = match jackpot {
      Jackpot::Full => game::points::HEARTWOOD_JACKPOT,
      Jackpot::Dimmed => game::points::HEARTWOOD_JACKPOT / 2,
      Jackpot::Out => return,
    };
    *jackpot = jackpot.after_hit();
    log::info!("HeartwoodMultiball: {shot:?} jackpot, now {:?}", self.jackpots[shot as usize]);
    self.hit_scene.restart(ctx);
    self.hits += 1;
    ctx.play_scene(&Self::progress_scene(self.hits));
    ctx.add_points(points);
    // rises toward the super jackpot that every 5th hit lights
    ctx.play_sfx(sounds::city_hit(
      ((self.hits - 1) % JACKPOTS_PER_SUPER) as f32 / (JACKPOTS_PER_SUPER - 1) as f32,
    ));

    // super jackpot
    if self.hits % JACKPOTS_PER_SUPER == 0 {
      self.start_super_jackpot(ctx);
    }
    if self.all_out() {
      // nothing left to hit, so the super jackpot waits for the player
      self.cancel_super_jackpot_timer(ctx);
      if !self.super_jackpot_active {
        self.super_jackpot_active = true;
        self.super_jackpot_attention_scene.restart(ctx);
      }
    }
    self.render(ctx);
  }

  fn all_out(&self) -> bool {
    self.jackpots.iter().all(|jackpot| *jackpot == Jackpot::Out)
  }

  /// Full jackpots pulse, dimmed ones glow steady and dim, and those out are dark
  fn render(&mut self, ctx: &SystemContext) {
    let dimmed = Multiball::Heartwood.color().mix_with(Rgba::black(), DIMMED);
    for (index, shot) in MainShot::ALL.into_iter().enumerate() {
      let leds = jackpot_leds(shot);
      match self.jackpots[index] {
        Jackpot::Full => {
          ctx.undeclare_lights(&leds, 0);
          self.jackpot_scenes[index].start(ctx);
        }
        Jackpot::Dimmed => {
          self.jackpot_scenes[index].stop(ctx);
          ctx.declare_lights(&leds, 0, dimmed);
        }
        Jackpot::Out => {
          self.jackpot_scenes[index].stop(ctx);
          ctx.undeclare_lights(&leds, 0);
        }
      }
    }
  }

  fn super_jackpot_hit(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::HEARTWOOD_SUPER_JACKPOT);
    ctx.play_sfx(sounds::ARP_HIT1);
    self.super_jackpot_hit_scene.restart(ctx);
    self.cancel_super_jackpot_timer(ctx);
    self.end_super_jackpot(ctx);
    log::info!("HeartwoodMultiball: super jackpot, every jackpot lit again");
    self.jackpots = [Jackpot::Full; 6];
    self.render(ctx);
  }

  fn start_super_jackpot(&mut self, ctx: &SystemContext) {
    // restart timer if another one is triggered
    self.cancel_super_jackpot_timer(ctx);
    self.timer_id = Some(ctx.timers.start(SuperJackpotOver, SUPER_JACKPOT_TIME.once()));

    self.super_jackpot_active = true;
    self.super_jackpot_attention_scene.restart(ctx);
  }

  fn end_super_jackpot(&mut self, ctx: &SystemContext) {
    self.super_jackpot_active = false;
    self.super_jackpot_attention_scene.stop(ctx);
  }

  fn cancel_super_jackpot_timer(&mut self, ctx: &SystemContext) {
    if let Some(timer_id) = self.timer_id.take() {
      ctx.timers.cancel(timer_id);
    }
  }

  fn on_subway_ramp(&mut self, ctx: &SystemContext) {
    self.subway_ramps += 1;
    if self.subway_ramps == RAMPS_TO_ADD_A_BALL {
      log::info!("HeartwoodMultiball: ball added");
      ctx.say(Callout::AdditionalDeployment);
      self.eject_id = ctx.eject_balls(1, Launch::Auto);
    }
  }

  fn shutdown(&mut self, ctx: &SystemContext) {
    for (index, shot) in MainShot::ALL.into_iter().enumerate() {
      self.jackpot_scenes[index].stop(ctx);
      ctx.undeclare_lights(&jackpot_leds(shot), 0);
    }
    ctx
      .systems
      .expect::<ModeManager>()
      .release_multiball(Multiball::Heartwood, ctx.into());
    ctx.systems.despawn_self();
  }
}

impl System for HeartwoodMultiballMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_multiball() == Some(Multiball::Heartwood)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    // on top of the one in play
    self.eject_id = ctx.eject_balls(1, Launch::Auto);
    ctx.say(Callout::OakHeartwood);
    ctx.request_ball_save_until_released();
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    for (index, jackpot) in self.jackpots.iter().enumerate() {
      if *jackpot == Jackpot::Full {
        self.jackpot_scenes[index].start(ctx);
      }
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx)
    } else if event.is::<ArcRampSubwayHit>() {
      self.on_subway_ramp(ctx);
    } else if event.is::<CaptiveBallHit>() && self.super_jackpot_active {
      self.super_jackpot_hit(ctx);
    } else if event.is::<SuperJackpotOver>() && self.super_jackpot_active {
      self.timer_id = None;
      ctx.play_sfx(sounds::HIT_ORGANIC_LOW4);
      self.end_super_jackpot(ctx);
    } else if let Some(EjectCompleted { id, .. }) = event.downcast_ref::<EjectCompleted>()
      && Some(*id) == self.eject_id
    {
      // time the ball save from when the balls are in play
      ctx.request_ball_save(MULTIBALL_BALL_SAVE);
    } else if let Some(done) = event.downcast_ref::<RoutineCompleted<BallDraining>>()
      && done.completion == RoutineCompletion::AllSteps
      && done.routine.in_play <= 1
    {
      // a drain that wasn't saved left one ball or none
      self.shutdown(ctx);
    } else if event.is::<RoutineStarted<TurnEnding>>() {
      self.shutdown(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct SuperJackpotOver;

/// What hitting a main shot is worth now
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Jackpot {
  Full,
  /// Half a jackpot
  Dimmed,
  Out,
}

impl Jackpot {
  fn after_hit(self) -> Self {
    match self {
      Self::Full => Self::Dimmed,
      Self::Dimmed | Self::Out => Self::Out,
    }
  }
}

/// The three LEDs of a shot's hex that light as its jackpot
fn jackpot_leds(shot: MainShot) -> LightQ {
  let children: [u16; 3] = if shot == MainShot::ArcRamp { [0, 6, 3] } else { [2, 6, 5] };
  let leds = shot.hex_leds();
  LightQ::any(
    children
      .iter()
      .filter_map(|index| leds.child(*index))
      .map(|led| led.q())
      .collect::<Vec<_>>()
      .iter()
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_jackpot_dims_then_goes_out() {
    assert_eq!(Jackpot::Full.after_hit(), Jackpot::Dimmed);
    assert_eq!(Jackpot::Dimmed.after_hit(), Jackpot::Out);
    assert_eq!(Jackpot::Out.after_hit(), Jackpot::Out);
  }

  #[test]
  fn the_jackpot_scenes_load() {
    for shot in MainShot::ALL {
      HeartwoodMultiballMode::jackpot_scene(shot);
    }
  }
}
