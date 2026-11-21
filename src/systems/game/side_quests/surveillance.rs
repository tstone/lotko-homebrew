use frontbox_core::prelude::*;
use frontbox_pinball::tags::Playfield;
use frontbox_lighting::prelude::*;
use frontbox_pinball::{BallManagerExt, GameManagerExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::main_shot::MainShotHit;
use crate::hardware::pop_cluster::{self, PopBumper};
use crate::systems::game::side_quests::SentinelsDispatched;
use crate::systems::game::{
  ModeManager, ModeSelectSystem, OutlaneSaveSystem, ScoopQualifier, flash_scene, points, ray_scene,
};
use crate::systems::{Callout, SayExt, sounds};

/// Playfield switch hits in a player's game before the cameras come online
const SWITCHES_TO_ENABLE: u32 = 40;

/// Pop hits that add up to one hit on the pop's camera
const POPS_PER_HIT: u8 = 3;

/// Every this many sets lights an outlane save
const SETS_PER_OUTLANE: u8 = 3;

/// Main shots the cameras can go without one being blinded before the Steward dispatches the Sentinels (start value)
const DEADLINE: u8 = 12;

/// The Steward watches the pops through a camera on each pop target (DESIGN.md, Surveillance). The cameras come online
/// after [`SWITCHES_TO_ENABLE`] playfield switch hits, and stay on for the rest of the player's game. A watching
/// camera's target is red, and hitting it enough blinds it and puts it out. While the cameras are on, each pop hit
/// flashes the pop's insert, and every [`POPS_PER_HIT`] hits on a pop count as one hit on its camera. Blinding all 3 is a set: the cameras come
/// back on, each needing one more hit than before, and the set counts as a lit shot toward the scoop. Every third set
/// lights an outlane save.
///
/// Once online, a camera has to be blinded at least every [`DEADLINE`] main shots. If none is, the Steward dispatches
/// the Sentinels ([`SentinelsDispatched`]), as soon as nothing that uses the drops or adds balls is running. The deadline
/// starts over with each ball, and once the Sentinels are gone. Counting shots rather than time means a ball held in a
/// scoop or cradled doesn't count against the player.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SurveillanceSystem {
  /// Playfield switch hits so far, counting up to [`SWITCHES_TO_ENABLE`]
  switches: u32,
  cameras: Cameras,
  /// Main shots left until the deadline, while it's counting down
  deadline: Option<u8>,
  /// The deadline passed, and the Sentinels go out once they can
  dispatch_pending: bool,
}

impl SurveillanceSystem {
  pub fn new() -> Self {
    Self {
      switches: 0,
      cameras: Cameras::new(),
      deadline: None,
      dispatch_pending: false,
    }
  }

  fn is_enabled(&self) -> bool {
    self.switches >= SWITCHES_TO_ENABLE
  }

  fn on_switch_hit(&mut self, ctx: &SystemContext) {
    self.switches += 1;
    if self.is_enabled() {
      self.enable(ctx);
    }
  }

  fn enable(&mut self, ctx: &SystemContext) {
    log::info!("Surveillance: enabled");
    ctx.say(Callout::SurveillanceEnabled);
    ctx.play_scene(&completing_scene!("side/surveillance_enabled.scene"));
    self.restart_deadline();
    self.render(ctx);
  }

  fn on_pop_hit(&mut self, pop: PopBumper, ctx: &SystemContext) {
    let led = pop_cluster::led_for_pop(&pop).q();
    ctx.play_scene(&flash_scene(&led, Rgba::red()));
    if self.cameras.pop(pop) {
      self.on_camera_hit(pop, ctx);
    }
  }

  fn on_camera_hit(&mut self, pop: PopBumper, ctx: &SystemContext) {
    let Some(blinded) = self.cameras.hit(pop) else {
      return;
    };
    ctx.add_points(points::SURVEILLANCE_HIT);
    let led = pop_cluster::target_led_for_pop(&pop).q();
    ctx.play_scene(&flash_scene(&led, Rgba::white()));

    if !blinded {
      ctx.play_sfx(sounds::HIT_ORGANIC1);
      return;
    }

    log::info!("Surveillance: {pop:?} camera blinded");
    ctx.play_sfx(sounds::rnd_latch());
    if self.deadline.is_some() {
      self.restart_deadline();
    }
    if self.cameras.all_blinded() {
      self.on_set(ctx);
    }
    self.render(ctx);
  }

  fn on_set(&mut self, ctx: &SystemContext) {
    let sets = self.cameras.next_set();
    log::info!("Surveillance: set {sets}");
    ctx.add_points(points::SURVEILLANCE_SET);
    ctx.play_sfx(sounds::SCAN_HEAVY_STUTTER);
    ctx.say(Callout::CameraNetworkOffline);

    for pop in PopBumper::ALL {
      ctx.play_scene(&ray_scene(&pop, Rgba::white()));
    }

    // the scoop only qualifies between modes
    if ctx.systems.expect::<ModeManager>().is_between_modes() {
      ctx
        .systems
        .expect::<ScoopQualifier>()
        .add_lit_shot(ctx.into());
    }

    if sets % SETS_PER_OUTLANE == 0 {
      ctx.systems.expect::<OutlaneSaveSystem>().light(ctx.into());
    }
  }

  fn restart_deadline(&mut self) {
    self.deadline = Some(DEADLINE);
  }

  /// A main shot brings the deadline one closer
  fn on_main_shot(&mut self) {
    let Some(left) = self.deadline else {
      return;
    };
    if left > 1 {
      self.deadline = Some(left - 1);
    } else {
      self.on_deadline_passed();
    }
  }

  fn on_deadline_passed(&mut self) {
    self.deadline = None;
    log::info!("Surveillance: no camera blinded in {DEADLINE} shots, the Sentinels are coming");
    self.dispatch_pending = true;
  }

  /// The Sentinels wait for a single ball loose on the playfield, and nothing running that they'd get in the way of
  fn can_dispatch(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().sentinels_can_take_over()
      && ctx.balls_in_play() <= 1
      && !ball_waiting(ctx)
  }

  fn dispatch(&mut self, ctx: &SystemContext) {
    if ctx
      .systems
      .expect::<ModeManager>()
      .start_sentinels(ctx.into())
      .is_ok()
    {
      self.dispatch_pending = false;
      // the cameras get a full deadline once the Sentinels are gone
      self.deadline = None;
      ctx.emit(SentinelsDispatched);
    }
  }

  /// Red on the cameras still watching, once they're online
  fn render(&self, ctx: &SystemContext) {
    for pop in PopBumper::ALL {
      let led = pop_cluster::target_led_for_pop(&pop).q();
      if self.is_enabled() && self.cameras.is_watching(pop) {
        ctx.declare_lights(&led, 0, Rgba::red());
      } else {
        ctx.undeclare_lights(&led, 0);
      }
    }
  }
}

impl System for SurveillanceSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  /// A new ball gets a full deadline. One that passed last ball still dispatches the Sentinels.
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    if self.is_enabled() {
      self.restart_deadline();
    }
    self.render(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<MainShotHit>() {
      if self.is_enabled() {
        self.on_main_shot();
      }
      return;
    }
    let Some(closed) = event.downcast_ref::<SwitchClosed>() else {
      return;
    };

    if !self.is_enabled() {
      if closed.switch.has_tag::<Playfield>() {
        self.on_switch_hit(ctx);
      }
      return;
    }

    if let Some(pop) = pop_cluster::match_target_switch(&closed.switch) {
      self.on_camera_hit(pop, ctx);
    } else if let Some(pop) = pop_cluster::match_switch(&closed.switch) {
      self.on_pop_hit(pop, ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if !self.is_enabled() {
      return;
    }
    if self.dispatch_pending {
      if self.can_dispatch(ctx) {
        self.dispatch(ctx);
      }
    } else if self.deadline.is_none()
      && !ctx.systems.expect::<ModeManager>().sentinels_running()
    {
      // the Sentinels are gone, so the cameras get a full deadline again
      self.restart_deadline();
    }
  }
}

/// A ball held in a scoop, or the mode select menu up: play is waiting on it
fn ball_waiting(ctx: &SystemContext) -> bool {
  let held = ctx
    .ball_manager()
    .is_some_and(|ball_manager| ball_manager.balls_held() > 0);
  let menu_open = ctx
    .systems
    .get::<ModeSelectSystem>()
    .is_some_and(|menu| menu.is_menu_open());
  held || menu_open
}

/// The hits on each camera this set, in [`PopBumper::ALL`] order
#[derive(Debug, Clone)]
struct Cameras {
  hits: [u8; 3],
  /// Pop hits toward each camera's next hit
  pops: [u8; 3],
  /// Hits that blind a camera this set
  required: u8,
  sets: u8,
}

impl Cameras {
  fn new() -> Self {
    Self {
      hits: [0; 3],
      pops: [0; 3],
      required: 1,
      sets: 0,
    }
  }

  fn is_watching(&self, pop: PopBumper) -> bool {
    self.hits[pop as usize] < self.required
  }

  fn all_blinded(&self) -> bool {
    self.blinded() == PopBumper::ALL.len()
  }

  /// Cameras blinded this set
  fn blinded(&self) -> usize {
    PopBumper::ALL
      .iter()
      .filter(|pop| !self.is_watching(**pop))
      .count()
  }

  /// Returns whether the hit blinded the camera, or `None` if it was already blind
  fn hit(&mut self, pop: PopBumper) -> Option<bool> {
    if !self.is_watching(pop) {
      return None;
    }
    self.hits[pop as usize] += 1;
    Some(!self.is_watching(pop))
  }

  /// Returns whether the pop hit adds up to a hit on its camera. Pops don't count toward a blind camera.
  fn pop(&mut self, pop: PopBumper) -> bool {
    if !self.is_watching(pop) {
      return false;
    }
    let pops = &mut self.pops[pop as usize];
    *pops += 1;
    if *pops < POPS_PER_HIT {
      return false;
    }
    *pops = 0;
    true
  }

  /// The cameras come back on, each needing another hit. Returns the sets made.
  fn next_set(&mut self) -> u8 {
    self.sets += 1;
    self.required += 1;
    self.hits = [0; 3];
    self.pops = [0; 3];
    self.sets
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_scenes_load() {
    scene!("side/surveillance_enabled.scene");
  }

  #[test]
  fn blinding_all_three_is_a_set() {
    let mut cameras = Cameras::new();
    assert_eq!(cameras.hit(PopBumper::Left), Some(true));
    assert_eq!(cameras.hit(PopBumper::Left), None);
    cameras.hit(PopBumper::UpperRight);
    assert_eq!(cameras.blinded(), 2);
    assert!(!cameras.all_blinded());
    cameras.hit(PopBumper::LowerRight);
    assert!(cameras.all_blinded());
  }

  #[test]
  fn three_pops_are_one_camera_hit() {
    let mut cameras = Cameras::new();
    assert!(!cameras.pop(PopBumper::Left));
    assert!(!cameras.pop(PopBumper::Left));
    assert!(cameras.pop(PopBumper::Left));
    assert_eq!(cameras.hit(PopBumper::Left), Some(true));
    // a blind camera takes no pops
    assert!(!cameras.pop(PopBumper::Left));
    assert!(!cameras.pop(PopBumper::Left));
    assert!(!cameras.pop(PopBumper::Left));
  }

  #[test]
  fn each_set_needs_another_hit_per_camera() {
    let mut cameras = Cameras::new();
    assert_eq!(cameras.next_set(), 1);
    assert!(PopBumper::ALL.iter().all(|pop| cameras.is_watching(*pop)));
    assert_eq!(cameras.hit(PopBumper::Left), Some(false));
    assert_eq!(cameras.hit(PopBumper::Left), Some(true));
  }
}
