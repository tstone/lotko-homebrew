use std::ops::Range;

use frontbox_core::prelude::*;
use frontbox_gfx::animation::Curve;
use frontbox_lighting::prelude::*;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::city_map;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, RootLashed, SporesSystem};
use crate::systems::{Callout, SayExt, sounds};

/// Cars the train starts with, each lighting one main shot
const CARS: usize = 3;

/// Hits to knock a car off: the first turns it cyan
const HITS_PER_CAR: u8 = 2;

/// Hits to the end of the mode
const REQUIRED_HITS: u8 = CARS as u8 * HITS_PER_CAR;

/// How long the train waits at a station
const STOP: Duration = Duration::from_secs(5);

/// How long it waits once the player has gone [`SLOW_AFTER`] without a hit
const SLOW_STOP: Duration = Duration::from_secs(6);

/// Time without a hit before the stops get longer
const SLOW_AFTER: Duration = Duration::from_secs(20);

/// How long the cars take to close up after one comes off
const CLOSE_UP: Duration = Duration::from_millis(200);

/// The z of the station marker on its hex ring, the mode attention z. The train scene draws just above it.
const STATION_Z: i8 = 10;

/// How far below the car its debris is thrown, toward the flippers
const DEBRIS_DROP: f32 = 10.0;

/// Where the train can stop on the left, picked at random each trip
const LEFT_STATIONS: [MainShot; 2] = [MainShot::LeftOrbit, MainShot::DomeRamp];

/// Where the train can stop on the right, picked at random each trip
const RIGHT_STATIONS: [MainShot; 2] = [MainShot::LiftRamp, MainShot::RightOrbit];

/// The monorail hub (design/city/skyrail-station.md). The train is a block of three neighboring main shots lit blue.
/// It runs back and forth along the shots and stops for a few seconds at a station: either of the two outer shots at
/// each end, picked at random each trip. The front car stops on the station.
///
/// While it's stopped, each car takes two hits: the first turns it cyan and the second knocks it off, and the train
/// closes up behind its front car. Knocking off the last car completes the mode. Each car lost makes it move faster.
/// The stops are 5 seconds, or 6 once the player goes 20 seconds without a hit.
///
/// The game logic jumps the train straight to its station when it leaves, and the train scene sweeps the cars there
/// along a path through the hex centers, easing in and out. Hits only count once it has arrived.
pub struct SkyrailStationMode {
  run: ModeRun,
  train: Train,
  /// The train has left its first station, once the mode's intro is over
  running: bool,
  stop: Duration,
  slow_timer: Option<u64>,
  /// How far along the train scene's path each hex center is, 0 to 1, in [`MainShot::ALL`] order
  path: [f32; 6],
  train_scene: ScenePlayer,
  /// The flashers alternating like crossing lights while the train is stopped
  crossing_scene: Option<ScenePlayer>,
}

impl SkyrailStationMode {
  pub fn new() -> Self {
    let train = Train::new(random_station(Heading::Right));
    let path = path_positions();
    let mut scene = scene!("skyrail_station/train.scene");
    for (index, shot) in MainShot::ALL.into_iter().enumerate() {
      scene = scene.variable(format!("p{index}"), shot.origin());
    }
    for (shot, car) in train.shots().zip(&train.cars) {
      scene = scene.variable(car.position_variable(), path[shot]);
    }
    Self {
      run: ModeRun::new(Mode::SkyrailStation, REQUIRED_HITS),
      train,
      running: false,
      stop: STOP,
      slow_timer: None,
      path,
      train_scene: ScenePlayer::new(scene),
      crossing_scene: None,
    }
  }

  fn knock_off_scene(shot: MainShot) -> PlayableScene<Completes> {
    let origin = shot.origin();
    let debris_to = Vec2::new(origin.x, (origin.y + DEBRIS_DROP).min(40.0));
    completing_scene!("skyrail_station/car_burst.scene")
      .variable("origin", origin)
      .variable("debris_to", debris_to)
  }

  fn crossing_scene(stop: Duration) -> PlayableScene<Completes> {
    completing_scene!("skyrail_station/crossing.scene").over(stop)
  }

  fn complete_scene(shot: MainShot) -> PlayableScene<Completes> {
    completing_scene!("skyrail_station/complete.scene").variable("origin", shot.origin())
  }

  fn progress_scene(hits: u8) -> PlayableScene<Completes> {
    city_map::mode_progress_scene(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  /// Sets the train off to its station: the logic jumps there, and the scene sweeps the cars along
  fn leave(&mut self, ctx: &SystemContext) {
    let step_time = self.train.step_time();
    let steps = self.train.trip();
    let duration = step_time * steps;
    log::info!("Skyrail: the train leaves for {:?}", self.train.destination);
    ctx.play_sfx(sounds::WOOSH_LIGHT);
    self.move_cars(duration, Curve::EaseInOut, ctx);
    ctx.timers.start(ArrivalDue, duration.once());
    self.render(ctx);
  }

  fn arrive(&mut self, ctx: &SystemContext) {
    self.train.arrive();
    log::info!("Skyrail: the train stopped at {:?}", self.train.destination);
    ctx.play_sfx(sounds::LATCH_DOUBLE);
    // a car knocked off on the way by a Root Lash leaves a gap to close
    self.move_cars(CLOSE_UP, Curve::EaseOut, ctx);
    self.crossing_scene = Some(ScenePlayer::started(Self::crossing_scene(self.stop), ctx));
    ctx.timers.start(DepartureDue, self.stop.once());
    self.render(ctx);
  }

  fn depart(&mut self, ctx: &SystemContext) {
    self.stop_crossing(ctx);
    self.train.depart(random_station(self.train.heading.reversed()));
    self.leave(ctx);
  }

  /// Counts only while the train is stopped, on one of its cars
  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if !self.train.stopped {
      return;
    }
    let Some(car) = self.train.car_at(shot) else {
      return;
    };
    let times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.hit_car(car, times, ctx);
  }

  /// A Root Lash's hits go to the front car, wherever the train is
  fn on_lash(&mut self, hits: u8, ctx: &SystemContext) {
    let car = self.train.lead_car();
    self.hit_car(car, hits, ctx);
  }

  /// Up to `times` hits on the car, stopping once it's knocked off
  fn hit_car(&mut self, car: usize, times: u8, ctx: &SystemContext) {
    let shot = self.train.shot_of(car);
    let mut knocked_off = false;
    let mut hits = 0;
    while hits < times && !knocked_off {
      hits += 1;
      knocked_off = self.train.hit(car);
    }
    self.run.score_hit(hits, ctx);
    self.stop = STOP;
    self.restart_slow_timer(ctx);

    let done = self.train.hits();
    ctx.play_sfx(sounds::city_hit(done as f32 / REQUIRED_HITS as f32));
    if knocked_off {
      log::info!("Skyrail: knocked a car off at {shot:?}, {} left", self.train.cars.len());
      if self.train.cars.is_empty() {
        self.complete(shot, ctx);
        return;
      }
      ctx.play_scene(&Self::knock_off_scene(shot));
      ctx.play_scene(&completing_scene!("skyrail_station/flashers.scene"));
      if self.train.cars.len() == CARS - 1 {
        ctx.say(Callout::The515IsDelayed);
      }
      if self.train.stopped {
        self.move_cars(CLOSE_UP, Curve::EaseOut, ctx);
      }
    } else {
      log::info!("Skyrail: hit the car at {shot:?}");
    }
    self.run.progress(done);
    ctx.play_scene(&Self::progress_scene(done));
    self.render(ctx);
  }

  /// The last car derails where it was hit
  fn complete(&mut self, shot: MainShot, ctx: &SystemContext) {
    log::info!("Skyrail: derailed");
    self.train_scene.stop(ctx);
    self.stop_crossing(ctx);
    self.clear_station(ctx);
    self.run.complete(Self::complete_scene(shot), ctx);
  }

  fn restart_slow_timer(&mut self, ctx: &SystemContext) {
    ctx.timers.cancel(self.slow_timer.take());
    self.slow_timer = Some(ctx.timers.start(SlowStopsDue, SLOW_AFTER.once()));
  }

  /// Sweeps each car's slot in the train scene to the shot it's on now
  fn move_cars(&mut self, duration: Duration, curve: Curve, ctx: &SystemContext) {
    for (shot, car) in self.train.shots().zip(&self.train.cars) {
      self
        .train_scene
        .ease_variable(car.position_variable(), self.path[shot], duration, curve.clone(), ctx);
    }
  }

  /// Each car slot's color and whether it's moving or pulsing at a stop, or hidden once its car is gone. A moving
  /// train has a bright front car to show its direction, and the station it's heading to has its ring lit.
  fn render(&mut self, ctx: &SystemContext) {
    let stopped = self.train.stopped;
    let lead_car = self.train.lead_car();
    for slot in 0..CARS as u8 {
      let car = self.train.cars.iter().enumerate().find(|(_, car)| car.slot == slot);
      let (moving, pulsing) = match car {
        Some(_) => (!stopped, stopped),
        None => (false, false),
      };
      if let Some((index, car)) = car {
        let color = if !stopped && index == lead_car {
          brighter(car.color())
        } else {
          car.color()
        };
        self.train_scene.set_variable(format!("car{slot}_color"), color, ctx);
      }
      self.train_scene.set_variable(format!("car{slot}_moving"), moving, ctx);
      self.train_scene.set_variable(format!("car{slot}_stopped"), pulsing, ctx);
    }

    self.clear_station(ctx);
    if !stopped {
      ctx.declare_lights(
        self.train.destination.hex_ring(),
        STATION_Z,
        MODE_COLOR.with_alpha(110),
      );
    }
  }

  fn clear_station(&self, ctx: &SystemContext) {
    for shot in MainShot::ALL {
      ctx.undeclare_lights(shot.hex_ring(), STATION_Z);
    }
  }

  fn stop_crossing(&mut self, ctx: &SystemContext) {
    if let Some(mut scene) = self.crossing_scene.take() {
      scene.stop(ctx);
    }
  }
}

impl System for SkyrailStationMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::SkyrailStation)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("Skyrail: started");
    self.run.begin(ctx);
    self.train_scene.start(ctx);
    self.render(ctx);
  }

  /// The train's timers and scenes kept their place, so it carries on from where it was
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_light_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.run.interrupt(ctx);
    ctx.deactivate_light_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<ArrivalDue>() {
      self.arrive(ctx);
    } else if event.is::<DepartureDue>() {
      self.depart(ctx);
    } else if event.is::<SlowStopsDue>() {
      log::info!("Skyrail: no hits for {SLOW_AFTER:?}, the stops get longer");
      self.slow_timer = None;
      self.stop = SLOW_STOP;
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.on_lash(lash.hits, ctx);
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.run.apply(ctx) {
      return;
    }

    if !self.running {
      self.running = true;
      self.restart_slow_timer(ctx);
      self.leave(ctx);
    }
  }
}

/// The train's cars and where it is along [`MainShot::ALL`]
struct Train {
  /// Left to right along the shots
  cars: Vec<Car>,
  /// The index in [`MainShot::ALL`] of the leftmost car
  left: usize,
  /// The way it's moving, or the way it was moving when it stopped
  heading: Heading,
  /// The station its front car is heading to or stopped at
  destination: MainShot,
  stopped: bool,
}

impl Train {
  /// At the left end, leaving for `destination`
  fn new(destination: MainShot) -> Self {
    Self {
      cars: (0..CARS as u8).map(|slot| Car { hits: 0, slot }).collect(),
      left: 0,
      heading: Heading::Right,
      destination,
      stopped: false,
    }
  }

  /// The indexes in [`MainShot::ALL`] of its cars, left to right
  fn shots(&self) -> Range<usize> {
    self.left..self.left + self.cars.len()
  }

  /// The index in [`MainShot::ALL`] of the front car
  fn lead(&self) -> usize {
    match self.heading {
      Heading::Left => self.left,
      Heading::Right => self.left + self.cars.len() - 1,
    }
  }

  fn lead_car(&self) -> usize {
    self.lead() - self.left
  }

  fn car_at(&self, shot: MainShot) -> Option<usize> {
    self.shots().contains(&(shot as usize)).then(|| shot as usize - self.left)
  }

  fn shot_of(&self, car: usize) -> MainShot {
    MainShot::ALL[self.left + car]
  }

  /// Moves it to its station, where it's due to arrive. Returns how many shots that took it along.
  fn trip(&mut self) -> u32 {
    let mut steps = 0;
    while self.lead() != self.destination as usize {
      match self.heading {
        Heading::Left => self.left -= 1,
        Heading::Right => self.left += 1,
      }
      steps += 1;
    }
    steps
  }

  fn arrive(&mut self) {
    self.stopped = true;
  }

  /// Turns around and leaves for `destination`
  fn depart(&mut self, destination: MainShot) {
    self.heading = self.heading.reversed();
    self.destination = destination;
    self.stopped = false;
  }

  /// Hits the car. Returns whether that knocked it off, in which case the train closes up behind the front car, so the
  /// front car stays where it is.
  fn hit(&mut self, car: usize) -> bool {
    self.cars[car].hits += 1;
    if self.cars[car].hits < HITS_PER_CAR {
      return false;
    }
    self.cars.remove(car);
    if self.heading == Heading::Right {
      self.left += 1;
    }
    true
  }

  /// Hits made on the cars, counting each car knocked off as all of its hits
  fn hits(&self) -> u8 {
    let knocked_off = (CARS - self.cars.len()) as u8 * HITS_PER_CAR;
    knocked_off + self.cars.iter().map(|car| car.hits).sum::<u8>()
  }

  /// The time it takes to pass each shot, shorter with each car lost
  fn step_time(&self) -> Duration {
    match self.cars.len() {
      3 => Duration::from_millis(250),
      2 => Duration::from_millis(200),
      _ => Duration::from_millis(150),
    }
  }
}

#[derive(Clone)]
struct Car {
  hits: u8,
  /// Its layers in the train scene, which it keeps as the train closes up around it
  slot: u8,
}

impl Car {
  /// Blue, then cyan once it's been hit
  fn color(&self) -> Rgba<u8> {
    if self.hits > 0 {
      Rgba::cyan()
    } else {
      *MODE_COLOR
    }
  }

  /// The train scene variable for how far along the path it is
  fn position_variable(&self) -> String {
    format!("car{}_pos", self.slot)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Heading {
  Left,
  Right,
}

impl Heading {
  fn reversed(self) -> Self {
    match self {
      Self::Left => Self::Right,
      Self::Right => Self::Left,
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ArrivalDue;

#[derive(serde::Serialize, Event)]
struct DepartureDue;

#[derive(serde::Serialize, Event)]
struct SlowStopsDue;

/// A random station at the end the train is heading to
fn random_station(heading: Heading) -> MainShot {
  let stations = match heading {
    Heading::Left => &LEFT_STATIONS,
    Heading::Right => &RIGHT_STATIONS,
  };
  *stations.choose(&mut rand::rng()).unwrap()
}

/// How far along a line through the hex centers each one is, 0 to 1. The train scene's path is that line, and a sweep
/// along it goes by distance.
fn path_positions() -> [f32; 6] {
  let points = MainShot::ALL.map(MainShot::origin);
  let mut lengths = [0.0; 6];
  for index in 1..points.len() {
    lengths[index] = lengths[index - 1] + points[index].distance(points[index - 1]);
  }
  let total = lengths[points.len() - 1];
  lengths.map(|length| length / total)
}

/// Halfway to white, for the front car
fn brighter(color: Rgba<u8>) -> Rgba<u8> {
  let channel = |c: u8| c + (255 - c) / 2;
  Rgba([channel(color.0[0]), channel(color.0[1]), channel(color.0[2]), color.0[3]])
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run_to_station(train: &mut Train) {
    train.trip();
    train.arrive();
  }

  #[test]
  fn the_front_car_stops_on_the_station() {
    let mut train = Train::new(MainShot::RightOrbit);
    assert_eq!(train.trip(), 3);
    assert_eq!(train.shots(), 3..6);

    train.depart(MainShot::DomeRamp);
    run_to_station(&mut train);
    assert_eq!(train.shots(), 1..4);
    assert_eq!(train.lead(), MainShot::DomeRamp as usize);
  }

  #[test]
  fn a_car_takes_two_hits() {
    let mut train = Train::new(MainShot::LiftRamp);
    assert!(!train.hit(1));
    assert_eq!(train.cars.len(), 3);
    assert!(train.hit(1));
    assert_eq!(train.cars.len(), 2);
    assert_eq!(train.hits(), 2);
  }

  #[test]
  fn the_train_closes_up_behind_the_front_car() {
    // stopped on the right orbit heading right: cars on the center orbit, lift ramp and right orbit
    let mut train = Train::new(MainShot::RightOrbit);
    run_to_station(&mut train);
    train.cars[2].hits = 1;
    train.hit(1);
    train.hit(1);
    assert_eq!(train.shots(), 4..6);
    assert_eq!(train.lead(), MainShot::RightOrbit as usize);
    // the hit front car moved along with the rest, keeping its slot
    assert_eq!(train.cars[1].hits, 1);
    assert_eq!(train.cars[1].slot, 2);

    // heading left, the left end stays put
    train.depart(MainShot::LeftOrbit);
    run_to_station(&mut train);
    assert!(train.hit(1));
    assert_eq!(train.shots(), 0..1);
  }

  #[test]
  fn the_path_runs_from_the_left_orbit_to_the_right_orbit() {
    let path = path_positions();
    assert_eq!(path[0], 0.0);
    assert_eq!(path[5], 1.0);
    assert!(path.windows(2).all(|pair| pair[0] < pair[1]));
  }

  #[test]
  fn the_scenes_load() {
    SkyrailStationMode::new();
    SkyrailStationMode::knock_off_scene(MainShot::ArcRamp);
    SkyrailStationMode::crossing_scene(STOP);
    SkyrailStationMode::complete_scene(MainShot::ArcRamp);
    completing_scene!("skyrail_station/flashers.scene");
  }
}
