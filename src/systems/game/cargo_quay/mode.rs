use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::city_map;
use crate::hardware::lift_ramp::{self, LiftRampSystem};
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::cargo_quay::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, RootLashed, SporesSystem};
use crate::systems::sounds;

/// Where the crane is. A container that gets here is loaded.
const CRANE: MainShot = MainShot::LiftRamp;

/// The two containers' starting shots. Each layout takes 5 pushes to load both.
const LAYOUTS: [[MainShot; 2]; 3] = [
  [MainShot::LeftOrbit, MainShot::CenterOrbit],
  [MainShot::LeftOrbit, MainShot::RightOrbit],
  [MainShot::DomeRamp, MainShot::ArcRamp],
];

/// Pushes to load both containers, whichever layout
const PUSHES: u8 = 5;

/// The chance a hit pushes a container two shots instead of one, until it happens once
const JUMP_CHANCE: f64 = 1.0 / 3.0;

/// The z of a container on its hex, the mode attention z
const CONTAINER_Z: i8 = 10;

/// The river freight terminal (design/city/cargo-quay.md). The lift ramp is the crane. Two containers sit on main
/// shots, and hitting a container's shot pushes it one shot along toward the crane. Containers pushed onto the same
/// shot stack, and move together from then on. Once a mode, a hit has a third of a chance to push two shots instead.
/// Once both are in, the crane flashes and the ramp goes up, and hitting it
/// completes the mode. The ramp stays down until then, as its coil buzzes while it holds the ramp up.
pub struct CargoQuayMode {
  run: ModeRun,
  /// Where each container is. One at [`CRANE`] is loaded.
  containers: [MainShot; 2],
  /// A hit has pushed two shots, which happens once a mode
  jumped: bool,
}

impl CargoQuayMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::CargoQuay, PUSHES + 1),
      containers: *LAYOUTS.choose(&mut rand::rng()).unwrap(),
      jumped: false,
    }
  }

  fn push_scene(from: MainShot, to: MainShot) -> PlayableScene<Completes> {
    completing_scene!("cargo_quay/push.scene")
      .variable("from", from.origin())
      .variable("to", to.origin())
  }

  fn complete_scene() -> PlayableScene<Completes> {
    completing_scene!("cargo_quay/complete.scene")
  }

  fn progress_scene(pushes: u8) -> PlayableScene<Completes> {
    city_map::mode_progress_scene(*MODE_COLOR, pushes as f32 / PUSHES as f32)
  }

  fn is_loaded(&self) -> bool {
    self.containers.iter().all(|shot| *shot == CRANE)
  }

  /// Pushes made so far: the shots between each container and the crane, taken off the start
  fn pushes(&self) -> u8 {
    PUSHES
      - self
        .containers
        .iter()
        .map(|shot| distance_to_crane(*shot))
        .sum::<u8>()
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if self.run.is_starting() {
      return;
    }
    if shot == CRANE {
      if self.is_loaded() {
        self.complete(ctx);
      }
      return;
    }
    if !self.containers.contains(&shot) {
      return;
    }
    let mut times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    // only rolled where a jump can move it two, so it isn't spent next to the crane
    if !self.jumped && distance_to_crane(shot) >= 2 && rand::random_bool(JUMP_CHANCE) {
      log::info!("CargoQuay: {shot:?} jumps two");
      self.jumped = true;
      times += 1;
    }
    self.push_from(shot, times, ctx);
  }

  /// Each of a Root Lash's hits pushes a container at random, or lifts them once both are loaded
  fn on_lash(&mut self, hits: u8, ctx: &SystemContext) {
    for _ in 0..hits {
      if self.is_loaded() {
        self.complete(ctx);
        return;
      }
      let waiting: Vec<MainShot> = self
        .containers
        .into_iter()
        .filter(|shot| *shot != CRANE)
        .collect();
      if let Some(shot) = waiting.choose(&mut rand::rng()).copied() {
        self.push_from(shot, 1, ctx);
      }
    }
  }

  /// Pushes whatever is on `shot` `times` shots toward the crane, scoring each container moved each time
  fn push_from(&mut self, shot: MainShot, times: u8, ctx: &SystemContext) {
    let mut from = shot;
    for _ in 0..times {
      if from == CRANE {
        break;
      }
      let to = step_toward_crane(from);
      let moved = push(&mut self.containers, from);
      log::info!("CargoQuay: pushed {moved} from {from:?} to {to:?}");
      self.run.score_hit(moved, ctx);
      ctx.play_scene(&Self::push_scene(from, to));
      from = to;
    }

    let pushes = self.pushes();
    self.run.progress(pushes);
    ctx.play_scene(&Self::progress_scene(pushes));
    if self.is_loaded() {
      log::info!("CargoQuay: both containers loaded, the crane is ready");
      self.request_ramp(ctx);
      ctx.play_sfx(sounds::MOTOR1);
    } else {
      ctx.play_sfx(sounds::LATCH_HEAVY1);
    }
    self.render(ctx);
  }

  fn complete(&mut self, ctx: &SystemContext) {
    log::info!("CargoQuay: complete");
    self.run.score_hit(1, ctx);
    self.release_ramp(ctx);
    self.run.complete(Self::complete_scene(), ctx);
  }

  /// A container lights its hex center, and a stack the whole hex. Once both are loaded, the crane and the bolt under
  /// it flash.
  fn render(&self, ctx: &SystemContext) {
    for shot in MainShot::ALL.into_iter().filter(|shot| *shot != CRANE) {
      let here = self
        .containers
        .iter()
        .filter(|container| **container == shot)
        .count();
      ctx.undeclare_lights(&shot.hex_leds().q(), CONTAINER_Z);
      match here {
        0 => {}
        1 => ctx.declare_lights(shot.hex_center(), CONTAINER_Z, *MODE_COLOR),
        _ => ctx.declare_lights(&shot.hex_leds().q(), CONTAINER_Z, *MODE_COLOR),
      }
    }
    let crane = LightQ::any(vec![CRANE.hex_center(), &lift_ramp::BOLT_LED.q()]);
    if self.is_loaded() {
      ctx.declare_lights_flashing(&crane, CONTAINER_Z, *MODE_COLOR);
    } else {
      ctx.undeclare_lights(&crane, CONTAINER_Z);
    }
  }

  fn request_ramp(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LiftRampSystem>()
      .request_up(ctx.id(), ctx.into());
  }

  fn release_ramp(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LiftRampSystem>()
      .release(ctx.id(), ctx.into());
  }
}

impl System for CargoQuayMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::CargoQuay)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("CargoQuay: started, containers on {:?}", self.containers);
    self.run.begin(ctx);
    self.render(ctx);
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    if self.is_loaded() {
      self.request_ramp(ctx);
    }
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.release_ramp(ctx);
    self.run.interrupt(ctx);
    ctx.deactivate_light_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.on_lash(lash.hits, ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    self.run.apply(ctx);
  }
}

/// The next shot toward the crane, along [`MainShot::ALL`]
fn step_toward_crane(shot: MainShot) -> MainShot {
  let (index, crane) = (shot as usize, CRANE as usize);
  match index.cmp(&crane) {
    std::cmp::Ordering::Less => MainShot::ALL[index + 1],
    std::cmp::Ordering::Greater => MainShot::ALL[index - 1],
    std::cmp::Ordering::Equal => shot,
  }
}

fn distance_to_crane(shot: MainShot) -> u8 {
  (shot as usize).abs_diff(CRANE as usize) as u8
}

/// Moves every container on `from` one shot toward the crane, returning how many moved
fn push(containers: &mut [MainShot; 2], from: MainShot) -> u8 {
  let to = step_toward_crane(from);
  let mut moved = 0;
  for container in containers
    .iter_mut()
    .filter(|container| **container == from)
  {
    *container = to;
    moved += 1;
  }
  moved
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn every_layout_takes_five_pushes() {
    for layout in LAYOUTS {
      let total: u8 = layout.iter().map(|shot| distance_to_crane(*shot)).sum();
      assert_eq!(total, PUSHES, "{layout:?}");
    }
  }

  #[test]
  fn containers_move_toward_the_crane_from_either_side() {
    assert_eq!(step_toward_crane(MainShot::LeftOrbit), MainShot::DomeRamp);
    assert_eq!(step_toward_crane(MainShot::CenterOrbit), MainShot::LiftRamp);
    assert_eq!(step_toward_crane(MainShot::RightOrbit), MainShot::LiftRamp);
  }

  #[test]
  fn a_stack_moves_together() {
    let mut containers = [MainShot::DomeRamp, MainShot::ArcRamp];
    assert_eq!(push(&mut containers, MainShot::DomeRamp), 1);
    assert_eq!(containers, [MainShot::ArcRamp, MainShot::ArcRamp]);
    assert_eq!(push(&mut containers, MainShot::ArcRamp), 2);
    assert_eq!(containers, [MainShot::CenterOrbit, MainShot::CenterOrbit]);
  }

  #[test]
  fn pushes_count_up_to_loaded() {
    let mut mode = CargoQuayMode::new();
    mode.containers = [MainShot::LeftOrbit, MainShot::RightOrbit];
    assert_eq!(mode.pushes(), 0);
    push(&mut mode.containers, MainShot::RightOrbit);
    assert_eq!(mode.pushes(), 1);
    assert!(!mode.is_loaded());
    for shot in [
      MainShot::LeftOrbit,
      MainShot::DomeRamp,
      MainShot::ArcRamp,
      MainShot::CenterOrbit,
    ] {
      push(&mut mode.containers, shot);
    }
    assert_eq!(mode.pushes(), PUSHES);
    assert!(mode.is_loaded());
  }

  #[test]
  fn the_scenes_load() {
    CargoQuayMode::push_scene(MainShot::LeftOrbit, MainShot::DomeRamp);
    CargoQuayMode::complete_scene();
    CargoQuayMode::progress_scene(PUSHES);
  }
}
