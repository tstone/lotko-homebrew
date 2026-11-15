use frontbox_core::prelude::*;
use frontbox_core::tags::Playfield;
use frontbox_led::prelude::*;
use frontbox_pinball::{BallManagementExt, BallManager, GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::pop_cluster::{self, PopBumper};
use crate::systems::game::side_quests::SentinelsDispatched;
use crate::systems::game::{
  ModeManager, ModeSelectSystem, OutlaneSaveSystem, ScoopQualifier, flash_scene, points, ray_scene,
};
use crate::systems::{Callout, SayExt, sounds};

/// Playfield switch hits in a player's game before the cameras come online
const SWITCHES_TO_ENABLE: u32 = 40;

/// Every this many sets lights an outlane save
const SETS_PER_OUTLANE: u8 = 3;

/// How long the cameras can go without one being blinded before the Steward dispatches the Sentinels (start value)
const DEADLINE: Duration = Duration::from_secs(60);

/// The Steward watches the pops through a camera on each pop target (DESIGN.md, Surveillance). The cameras come online
/// after [`SWITCHES_TO_ENABLE`] playfield switch hits, and stay on for the rest of the player's game. A watching
/// camera's target is red, and hitting it enough blinds it and puts it out. Blinding all 3 is a set: the cameras come
/// back on, each needing one more hit than before, and the set counts as a lit shot toward the scoop. Every third set
/// lights an outlane save.
///
/// Once online, a camera has to be blinded at least every [`DEADLINE`]. If none is, the Steward dispatches the
/// Sentinels ([`SentinelsDispatched`]), as soon as nothing that uses the drops or adds balls is running. The deadline
/// starts over with each ball, and once the Sentinels are gone.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SurveillanceSystem {
  /// Playfield switch hits so far, counting up to [`SWITCHES_TO_ENABLE`]
  switches: u32,
  cameras: Cameras,
  /// Runs out at the deadline, while it's counting down
  deadline_cue: Option<u64>,
  /// The deadline passed, and the Sentinels go out once they can
  dispatch_pending: bool,
}

impl SurveillanceSystem {
  pub fn new() -> Self {
    Self {
      switches: 0,
      cameras: Cameras::new(),
      deadline_cue: None,
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
    self.restart_deadline(ctx);
    self.render(ctx);
  }

  fn on_target_hit(&mut self, pop: PopBumper, ctx: &SystemContext) {
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
    if self.deadline_cue.is_some() {
      self.restart_deadline(ctx);
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

  fn restart_deadline(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.deadline_cue.take() {
      ctx.cues.cancel(cue);
    }
    self.deadline_cue = Some(ctx.cues.start(DeadlinePassed, DEADLINE.once()));
  }

  fn on_deadline_passed(&mut self) {
    self.deadline_cue = None;
    log::info!("Surveillance: no camera blinded in {DEADLINE:?}, the Sentinels are coming");
    self.dispatch_pending = true;
  }

  /// The Sentinels wait for a single ball loose on the playfield, and nothing running that they'd get in the way of
  fn can_dispatch(&self, ctx: &SystemContext) -> bool {
    let held = ctx
      .systems
      .get::<BallManager>()
      .is_some_and(|ball_manager| ball_manager.balls_held() > 0);
    let menu_open = ctx
      .systems
      .get::<ModeSelectSystem>()
      .is_some_and(|menu| menu.is_menu_open());
    ctx.systems.expect::<ModeManager>().sentinels_can_take_over()
      && ctx.balls_in_play() <= 1
      && !held
      && !menu_open
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
      if let Some(cue) = self.deadline_cue.take() {
        ctx.cues.cancel(cue);
      }
      ctx.emit(SentinelsDispatched);
    }
  }

  /// Red on the cameras still watching, once they're online
  fn render(&self, ctx: &SystemContext) {
    for pop in PopBumper::ALL {
      let led = pop_cluster::target_led_for_pop(&pop).q();
      if self.is_enabled() && self.cameras.is_watching(pop) {
        ctx.declare_leds(&led, Rgba::red());
      } else {
        ctx.undeclare_leds(&led);
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
    ctx.activate_led_declarations();
    if self.is_enabled() {
      self.restart_deadline(ctx);
    }
    self.render(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<DeadlinePassed>() {
      self.on_deadline_passed();
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
      self.on_target_hit(pop, ctx);
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
    } else if self.deadline_cue.is_none()
      && !ctx.systems.expect::<ModeManager>().sentinels_running()
    {
      // the Sentinels are gone, so the cameras get a full deadline again
      self.restart_deadline(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct DeadlinePassed;

/// The hits on each camera this set, in [`PopBumper::ALL`] order
#[derive(Debug, Clone)]
struct Cameras {
  hits: [u8; 3],
  /// Hits that blind a camera this set
  required: u8,
  sets: u8,
}

impl Cameras {
  fn new() -> Self {
    Self {
      hits: [0; 3],
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

  /// The cameras come back on, each needing another hit. Returns the sets made.
  fn next_set(&mut self) -> u8 {
    self.sets += 1;
    self.required += 1;
    self.hits = [0; 3];
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
  fn each_set_needs_another_hit_per_camera() {
    let mut cameras = Cameras::new();
    assert_eq!(cameras.next_set(), 1);
    assert!(PopBumper::ALL.iter().all(|pop| cameras.is_watching(*pop)));
    assert_eq!(cameras.hit(PopBumper::Left), Some(false));
    assert_eq!(cameras.hit(PopBumper::Left), Some(true));
  }
}
