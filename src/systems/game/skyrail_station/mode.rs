use frontbox_core::prelude::*;
use frontbox_gfx::animation::Curve;
use frontbox_lighting::prelude::*;
use frontbox_pinball::GameManagerExt;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::hardware::city_map;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::skyrail_station::MODE_COLOR;
use crate::systems::game::{Mode, ModeManager, ModeRun, RootLashed, SporesSystem, points};
use crate::systems::{Callout, SayExt, sounds};

/// Stations to destroy to complete it
const STATIONS: u8 = 4;

/// How long a station flashes before the train leaves for it
const ANNOUNCE: Duration = Duration::from_secs(3);

/// How long the train's head takes to reach the station, whatever the distance
const TRAVEL: Duration = Duration::from_secs(3);

/// How far behind the head each tail segment arrives
const TAIL_LAG: Duration = Duration::from_millis(120);

/// The train's segments in the train scene: the head, then the tail
const SEGMENTS: usize = 5;

/// How long the train takes to fade once a station it's heading to is destroyed
const FADE: Duration = Duration::from_millis(250);

/// A station destroyed before the train gets there, as a percent of a mode hit
const EARLY_PERCENT: u64 = 200;

/// A station destroyed once the train is there, as a percent of a mode hit
const LATE_PERCENT: u64 = 75;

/// The z of the station on its hex center, the mode attention z. The train scene draws just above it.
const STATION_Z: i8 = 10;

/// How far below the station its debris is thrown, toward the flippers
const DEBRIS_DROP: f32 = 10.0;

/// The monorail hub (design/city/skyrail-station.md). The game picks a station, one of the main shots, and it flashes
/// as the train is announced. Then the train leaves for it, a comet with a fading tail running along the hex centers,
/// and fades out once it's there, leaving the station lit. Destroying the station before the train gets there is worth
/// double, and after, three quarters. Then the next station is picked, and the train leaves from the last one.
///
/// Destroying four completes the mode, with a bonus if every one was destroyed before the train got there.
pub struct SkyrailStationMode {
  run: ModeRun,
  /// Where the train leaves from: the last station
  from: MainShot,
  station: MainShot,
  phase: Phase,
  destroyed: u8,
  /// Every station so far was destroyed before the train got there
  perfect: bool,
  /// The timer for the next phase, cancelled when the station is destroyed early
  timer: Option<u64>,
  /// How far along the train scene's path each hex center is, 0 to 1, in [`MainShot::ALL`] order
  path: [f32; 6],
  train_scene: ScenePlayer,
  /// The flashers alternating like crossing lights while the train is announced
  crossing_scene: Option<ScenePlayer>,
  /// The first station is picked once the mode's intro is over
  running: bool,
}

impl SkyrailStationMode {
  pub fn new() -> Self {
    let from = *MainShot::ALL.choose(&mut rand::rng()).unwrap();
    let mut scene = scene!("skyrail_station/train.scene");
    for (index, shot) in MainShot::ALL.into_iter().enumerate() {
      scene = scene.variable(format!("p{index}"), shot.origin());
    }
    Self {
      run: ModeRun::new(Mode::SkyrailStation, STATIONS),
      from,
      station: next_station(from),
      phase: Phase::Announced,
      destroyed: 0,
      perfect: true,
      timer: None,
      path: path_positions(),
      train_scene: ScenePlayer::new(scene),
      crossing_scene: None,
      running: false,
    }
  }

  fn station_burst_scene(shot: MainShot) -> PlayableScene<Completes> {
    let origin = shot.origin();
    let debris_to = Vec2::new(origin.x, (origin.y + DEBRIS_DROP).min(40.0));
    completing_scene!("skyrail_station/station_burst.scene")
      .variable("origin", origin)
      .variable("debris_to", debris_to)
  }

  fn crossing_scene() -> PlayableScene<Completes> {
    completing_scene!("skyrail_station/crossing.scene").over(ANNOUNCE)
  }

  fn complete_scene(shot: MainShot) -> PlayableScene<Completes> {
    completing_scene!("skyrail_station/complete.scene").variable("origin", shot.origin())
  }

  fn progress_scene(destroyed: u8) -> PlayableScene<Completes> {
    city_map::mode_progress_scene(*MODE_COLOR, destroyed as f32 / STATIONS as f32)
  }

  /// The station flashes, and the crossing lights count down to the train leaving
  fn announce(&mut self, ctx: &SystemContext) {
    log::info!("Skyrail: the train is announced for {:?}", self.station);
    self.phase = Phase::Announced;
    self.crossing_scene = Some(ScenePlayer::started(Self::crossing_scene(), ctx));
    self.timer = Some(ctx.timers.start(DepartureDue, ANNOUNCE.once()));
    self.render(ctx);
  }

  /// Sends the train from the last station to this one, each tail segment a little behind the one before
  fn depart(&mut self, ctx: &SystemContext) {
    log::info!(
      "Skyrail: the train leaves {:?} for {:?}",
      self.from,
      self.station
    );
    self.phase = Phase::Travelling;
    self.stop_crossing(ctx);
    ctx.play_sfx(sounds::WOOSH_LIGHT);
    self.train_scene.set_variable("opacity", 1.0, ctx);
    for segment in 0..SEGMENTS {
      let name = segment_variable(segment);
      self
        .train_scene
        .set_variable(&name, self.path[self.from as usize], ctx);
      let duration = TRAVEL + TAIL_LAG * segment as u32;
      self.train_scene.ease_variable(
        name,
        self.path[self.station as usize],
        duration,
        Curve::EaseInOut,
        ctx,
      );
    }
    self.timer = Some(ctx.timers.start(ArrivalDue, TRAVEL.once()));
  }

  /// The train fades out as its tail catches up, and the station stays lit until it's destroyed
  fn arrive(&mut self, ctx: &SystemContext) {
    log::info!("Skyrail: the train reached {:?}", self.station);
    self.phase = Phase::Arrived;
    self.timer = None;
    ctx.play_sfx(sounds::LATCH_DOUBLE);
    self.fade_train(TAIL_LAG * (SEGMENTS - 1) as u32, ctx);
    self.render(ctx);
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if !self.running || shot != self.station {
      return;
    }
    let times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
    self.destroy_station(times, ctx);
  }

  /// Each of a Root Lash's hits destroys a station, the next one as soon as it's announced
  fn on_lash(&mut self, hits: u8, ctx: &SystemContext) {
    for _ in 0..hits {
      if self.destroyed == STATIONS {
        return;
      }
      self.destroy_station(1, ctx);
    }
  }

  /// Scores `times` hits on the station at its rate, destroys it, and announces the next
  fn destroy_station(&mut self, times: u8, ctx: &SystemContext) {
    let station = self.station;
    let early = self.phase != Phase::Arrived;
    let percent = if early { EARLY_PERCENT } else { LATE_PERCENT };
    self.run.score_hit_percent(times, percent, ctx);
    self.perfect &= early;
    self.destroyed += 1;
    log::info!(
      "Skyrail: destroyed {station:?} {}, {} of {STATIONS}",
      if early {
        "before the train"
      } else {
        "after the train"
      },
      self.destroyed
    );

    ctx.timers.cancel(self.timer.take());
    self.stop_crossing(ctx);
    if self.phase == Phase::Travelling {
      self.fade_train(FADE, ctx);
    }
    self.clear_stations(ctx);
    ctx.play_sfx(sounds::city_hit(self.destroyed as f32 / STATIONS as f32));

    if self.destroyed == STATIONS {
      self.complete(station, ctx);
      return;
    }
    ctx.play_scene(&Self::station_burst_scene(station));
    ctx.play_scene(&completing_scene!("skyrail_station/flashers.scene"));
    if self.destroyed == 1 {
      ctx.say(Callout::The515IsDelayed);
    }
    self.run.progress(self.destroyed);
    ctx.play_scene(&Self::progress_scene(self.destroyed));

    self.from = station;
    self.station = next_station(station);
    self.announce(ctx);
  }

  /// The last station derails the line, with a bonus for destroying every one before the train got there
  fn complete(&mut self, station: MainShot, ctx: &SystemContext) {
    log::info!("Skyrail: derailed, perfect: {}", self.perfect);
    self.train_scene.stop(ctx);
    if self.perfect {
      ctx.add_points(perfect_bonus());
      ctx.say(Callout::SkyrailUnavailable);
    }
    self.run.complete(Self::complete_scene(station), ctx);
  }

  fn fade_train(&mut self, duration: Duration, ctx: &SystemContext) {
    self
      .train_scene
      .ease_variable("opacity", 0.0, duration, Curve::EaseOut, ctx);
  }

  /// The station flashes until the train gets there, then it's lit
  fn render(&self, ctx: &SystemContext) {
    let leds = self.station.hex_center();
    match self.phase {
      Phase::Announced | Phase::Travelling => {
        ctx.declare_lights_flashing(leds, STATION_Z, *MODE_COLOR)
      }
      Phase::Arrived => ctx.declare_lights(leds, STATION_Z, *MODE_COLOR),
    }
  }

  fn clear_stations(&self, ctx: &SystemContext) {
    for shot in MainShot::ALL {
      ctx.undeclare_lights(shot.hex_center(), STATION_Z);
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
  }

  /// The timers and scenes kept their place, so it carries on from where it was
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

    if event.is::<DepartureDue>() {
      self.depart(ctx);
    } else if event.is::<ArrivalDue>() {
      self.arrive(ctx);
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
      self.announce(ctx);
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
  /// The station flashes before the train leaves
  Announced,
  /// The train is on its way, and the station still flashes
  Travelling,
  /// The train is there, and the station is lit
  Arrived,
}

#[derive(serde::Serialize, Event)]
struct DepartureDue;

#[derive(serde::Serialize, Event)]
struct ArrivalDue;

/// Any main shot but the one the train is leaving from
fn next_station(from: MainShot) -> MainShot {
  let others: Vec<_> = MainShot::ALL
    .into_iter()
    .filter(|shot| *shot != from)
    .collect();
  *others.choose(&mut rand::rng()).unwrap()
}

/// Half the completion, on top of it
fn perfect_bonus() -> u64 {
  points::mode_completion(Mode::SkyrailStation) / 2
}

/// The train scene variable for how far along the path a segment is, the head being 0
fn segment_variable(segment: usize) -> String {
  format!("seg{segment}_pos")
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_next_station_is_never_the_last() {
    for shot in MainShot::ALL {
      for _ in 0..20 {
        assert_ne!(next_station(shot), shot);
      }
    }
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
    SkyrailStationMode::station_burst_scene(MainShot::ArcRamp);
    SkyrailStationMode::crossing_scene();
    SkyrailStationMode::complete_scene(MainShot::ArcRamp);
    completing_scene!("skyrail_station/flashers.scene");
  }
}
