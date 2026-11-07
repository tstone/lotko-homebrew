use std::collections::HashSet;

use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::lower_scoop::{LowerScoopBallEnter, LowerScoopEjected};
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{EffectsSystem, flash_effect, points};
use crate::systems::sounds;

/// How long after a shot the next one continues a route
const WINDOW: Duration = Duration::from_secs(4);

/// Traffic routes (DESIGN.md, Combos): 2 and 3 shot routes made in a row, each paying once a game. Making all 8 is
/// Gridlock.
///
/// Each shot must come within [`WINDOW`] of the last, except that time in the left scoop doesn't count. A lift ramp
/// shot with the ramp up goes to the wireform, which breaks the route. Runs alongside modes and multiballs.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct ComboSystem {
  routes: Routes,
  window_cue: Option<u64>,
  /// The ball went into the scoop during a route, which carries on once it's ejected
  in_scoop: bool,
}

impl ComboSystem {
  pub fn new() -> Self {
    Self {
      routes: Routes::new(),
      window_cue: None,
      in_scoop: false,
    }
  }

  fn on_shot(&mut self, hit: &MainShotHit, ctx: &SystemContext) {
    self.cancel_window(ctx);
    if hit.ramp_up {
      self.routes.break_route();
      return;
    }

    let made = self.routes.shot(hit.shot);
    self.window_cue = Some(ctx.cues.start(WindowClosed, WINDOW.once()));
    for route in made {
      self.pay(route, ctx);
    }
    if self.routes.take_gridlock() {
      log::info!("Combos: gridlock");
      ctx.add_points(points::GRIDLOCK);
      ctx.play_sfx(sounds::BRAAM_BIO2);
      let shots: Vec<&LedQ> = MainShot::ALL.iter().map(|shot| shot.hex_center()).collect();
      ctx
        .systems
        .expect::<EffectsSystem>()
        .play(flash_effect(&LedQ::any(shots), Rgba::green()));
    }
  }

  fn pay(&mut self, route: Route, ctx: &SystemContext) {
    log::info!("Combos: {route:?}");
    ctx.add_points(if route.shots().len() > 2 {
      points::COMBO_LONG
    } else {
      points::COMBO
    });
    ctx.play_sfx(sounds::ARP_HIT1);
    let shots: Vec<&LedQ> = route.shots().iter().map(|shot| shot.hex_center()).collect();
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(&LedQ::any(shots), Rgba::green()));
  }

  fn cancel_window(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.window_cue.take() {
      ctx.cues.cancel(cue);
    }
  }
}

impl System for ComboSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.cancel_window(ctx);
    self.in_scoop = false;
    self.routes.break_route();
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot(hit, ctx);
    } else if event.is::<WindowClosed>() {
      self.window_cue = None;
      self.routes.break_route();
    } else if event.is::<LowerScoopBallEnter>() && self.window_cue.is_some() {
      // the window waits while the ball is held
      self.cancel_window(ctx);
      self.in_scoop = true;
    } else if event.is::<LowerScoopEjected>() && self.in_scoop {
      self.in_scoop = false;
      self.window_cue = Some(ctx.cues.start(WindowClosed, WINDOW.once()));
    }
  }
}

#[derive(serde::Serialize, Event)]
struct WindowClosed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Route {
  Interchange,
  ReturnTrip,
  ExpressLine,
  Roundabout,
  RingRoad,
  Underpass,
  Cloverleaf,
  GrandTour,
}

impl Route {
  const ALL: [Route; 8] = [
    Self::Interchange,
    Self::ReturnTrip,
    Self::ExpressLine,
    Self::Roundabout,
    Self::RingRoad,
    Self::Underpass,
    Self::Cloverleaf,
    Self::GrandTour,
  ];

  fn shots(self) -> &'static [MainShot] {
    use MainShot::*;
    match self {
      Self::Interchange => &[DomeRamp, LiftRamp],
      Self::ReturnTrip => &[LiftRamp, DomeRamp],
      Self::ExpressLine => &[LiftRamp, ArcRamp],
      Self::Roundabout => &[RightOrbit, RightOrbit],
      Self::RingRoad => &[RightOrbit, LiftRamp],
      Self::Underpass => &[ArcRamp, LiftRamp],
      Self::Cloverleaf => &[DomeRamp, LiftRamp, DomeRamp],
      Self::GrandTour => &[DomeRamp, RightOrbit, LiftRamp],
    }
  }

  /// Whether `shots` is this route, or contains it as shots in a row
  fn is_within(self, shots: &[MainShot]) -> bool {
    shots
      .windows(self.shots().len())
      .any(|window| window == self.shots())
  }
}

/// The route being driven, and the ones made this game
#[derive(Debug, Clone)]
struct Routes {
  /// The shots made in a row, the latest last, as far back as the longest route
  driven: Vec<MainShot>,
  made: HashSet<Route>,
  gridlock: bool,
}

impl Routes {
  fn new() -> Self {
    Self {
      driven: Vec::new(),
      made: HashSet::new(),
      gridlock: false,
    }
  }

  /// A shot continuing the route. Returns the routes it made for the first time, along with any 2 shot routes inside a
  /// 3 shot one that it made.
  fn shot(&mut self, shot: MainShot) -> Vec<Route> {
    self.driven.push(shot);
    if self.driven.len() > 3 {
      self.driven.remove(0);
    }

    let completed: Vec<Route> = Route::ALL
      .into_iter()
      .filter(|route| self.driven.ends_with(route.shots()))
      .collect();
    let made: Vec<Route> = Route::ALL
      .into_iter()
      .filter(|route| {
        !self.made.contains(route)
          && completed
            .iter()
            .any(|completed| route.is_within(completed.shots()))
      })
      .collect();
    self.made.extend(made.iter().copied());
    made
  }

  fn break_route(&mut self) {
    self.driven.clear();
  }

  /// Whether every route has been made, the first time it's asked once they are
  fn take_gridlock(&mut self) -> bool {
    let gridlock = !self.gridlock && self.made.len() == Route::ALL.len();
    self.gridlock |= gridlock;
    gridlock
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use MainShot::*;

  #[test]
  fn two_shots_in_a_row_make_a_route_once() {
    let mut routes = Routes::new();
    assert!(routes.shot(DomeRamp).is_empty());
    assert_eq!(routes.shot(LiftRamp), vec![Route::Interchange]);
    routes.break_route();
    routes.shot(DomeRamp);
    assert!(routes.shot(LiftRamp).is_empty());
  }

  #[test]
  fn a_three_shot_route_pays_the_two_shot_routes_inside_it() {
    let mut routes = Routes::new();
    routes.shot(DomeRamp);
    routes.shot(RightOrbit);
    let made = routes.shot(LiftRamp);
    assert!(made.contains(&Route::GrandTour));
    assert!(made.contains(&Route::RingRoad));
  }

  #[test]
  fn cloverleaf_also_makes_both_ramp_routes() {
    let mut routes = Routes::new();
    routes.shot(DomeRamp);
    assert_eq!(routes.shot(LiftRamp), vec![Route::Interchange]);
    let made = routes.shot(DomeRamp);
    assert!(made.contains(&Route::Cloverleaf));
    assert!(made.contains(&Route::ReturnTrip));
  }

  #[test]
  fn a_broken_route_starts_over() {
    let mut routes = Routes::new();
    routes.shot(DomeRamp);
    routes.break_route();
    assert!(routes.shot(LiftRamp).is_empty());
  }

  #[test]
  fn making_every_route_is_gridlock_once() {
    let mut routes = Routes::new();
    for route in Route::ALL {
      routes.break_route();
      for shot in route.shots() {
        routes.shot(*shot);
      }
    }
    assert!(routes.take_gridlock());
    assert!(!routes.take_gridlock());
  }
}
