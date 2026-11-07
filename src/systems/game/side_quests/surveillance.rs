use frontbox_core::prelude::*;
use frontbox_core::tags::Playfield;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::pop_cluster::{self, PopBumper};
use crate::systems::game::{
  EffectsSystem, ModeManager, OutlaneSaveSystem, ScoopQualifier, flash_effect, points, ray_effect,
};
use crate::systems::{Callout, SayExt, sounds};

/// Playfield switch hits in a player's game before the cameras come online
const SWITCHES_TO_ENABLE: u32 = 50;

/// Every this many sets lights an outlane save
const SETS_PER_OUTLANE: u8 = 3;

/// The Steward watches the pops through a camera on each pop target (DESIGN.md, Surveillance). The cameras come online
/// after [`SWITCHES_TO_ENABLE`] playfield switch hits, and stay on for the rest of the player's game. A watching
/// camera's target is red, and hitting it enough blinds it and puts it out. Blinding all 3 is a set: the cameras come
/// back on, each needing one more hit than before, and the set counts as a lit shot toward the scoop. Every third set
/// lights an outlane save.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SurveillanceSystem {
  /// Playfield switch hits so far, counting up to [`SWITCHES_TO_ENABLE`]
  switches: u32,
  cameras: Cameras,
}

impl SurveillanceSystem {
  pub fn new() -> Self {
    Self {
      switches: 0,
      cameras: Cameras::new(),
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
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(led_playback!("side/surveillance_enabled.scene"));
    self.render(ctx);
  }

  fn on_target_hit(&mut self, pop: PopBumper, ctx: &SystemContext) {
    let Some(blinded) = self.cameras.hit(pop) else {
      return;
    };
    ctx.add_points(points::SURVEILLANCE_HIT);
    let led = pop_cluster::target_led_for_pop(&pop).q();
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(&led, Rgba::white()));

    if !blinded {
      ctx.play_sfx(sounds::HIT_ORGANIC1);
      return;
    }

    log::info!("Surveillance: {pop:?} camera blinded");
    ctx.play_sfx(sounds::rnd_latch());
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

    let mut effects = ctx.systems.expect::<EffectsSystem>();
    for pop in PopBumper::ALL {
      effects.play(ray_effect(&pop, Rgba::white()));
    }
    drop(effects);

    // the scoop only qualifies between modes
    if ctx.systems.expect::<ModeManager>().current_mode().is_none() {
      ctx
        .systems
        .expect::<ScoopQualifier>()
        .add_lit_shot(ctx.into());
    }

    if sets % SETS_PER_OUTLANE == 0 {
      ctx.systems.expect::<OutlaneSaveSystem>().light(ctx.into());
    }
  }

  /// Red on the cameras still watching, once they're online
  fn render(&self, ctx: &SystemContext) {
    for pop in PopBumper::ALL {
      let led = pop_cluster::target_led_for_pop(&pop).q();
      if self.is_enabled() && self.cameras.is_watching(pop) {
        ctx.declare_leds(&led, ColorSequence::solid(Rgba::red()));
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

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.render(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
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
}

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
    PopBumper::ALL.iter().all(|pop| !self.is_watching(*pop))
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
    led_playback!("side/surveillance_enabled.scene");
  }

  #[test]
  fn blinding_all_three_is_a_set() {
    let mut cameras = Cameras::new();
    assert_eq!(cameras.hit(PopBumper::Left), Some(true));
    assert_eq!(cameras.hit(PopBumper::Left), None);
    cameras.hit(PopBumper::UpperRight);
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
