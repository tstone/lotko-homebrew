use frontbox_core::prelude::*;

use crate::hardware::arc_ramp::{self, ArcRampHit};
use crate::hardware::center_orbit::{self, CenterOrbitHit};
use crate::hardware::dome_ramp::{self, DomeRampHit};
use crate::hardware::left_orbit::{self, LeftOrbitHit};
use crate::hardware::lift_ramp::{self, LiftRampHit, LiftRampSystem};
use crate::hardware::planes;
use crate::hardware::right_orbit::{self, RightOrbitHit};

/// The six shots with a hex insert, in playfield order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum MainShot {
  LeftOrbit,
  DomeRamp,
  ArcRamp,
  CenterOrbit,
  LiftRamp,
  RightOrbit,
}

impl MainShot {
  pub const ALL: [MainShot; 6] = [
    Self::LeftOrbit,
    Self::DomeRamp,
    Self::ArcRamp,
    Self::CenterOrbit,
    Self::LiftRamp,
    Self::RightOrbit,
  ];

  pub fn hex_center(self) -> &'static LedQ {
    match self {
      Self::LeftOrbit => &*left_orbit::HEX_CENTER_LED,
      Self::DomeRamp => &*dome_ramp::HEX_CENTER_LED,
      Self::ArcRamp => &*arc_ramp::HEX_CENTER_LED,
      Self::CenterOrbit => &*center_orbit::HEX_CENTER_LED,
      Self::LiftRamp => &*lift_ramp::HEX_CENTER_LED,
      Self::RightOrbit => &*right_orbit::HEX_CENTER_LED,
    }
  }

  /// The hex's outer LEDs in chase order, starting from the one nearest the shot
  pub fn hex_ring(self) -> &'static LedQ {
    match self {
      Self::LeftOrbit => &*left_orbit::HEX_CIRCLE_LEDS,
      Self::DomeRamp => &*dome_ramp::HEX_CIRCLE_LEDS,
      Self::ArcRamp => &*arc_ramp::HEX_CIRCLE_LEDS,
      Self::CenterOrbit => &*center_orbit::HEX_CIRCLE_LEDS,
      Self::LiftRamp => &*lift_ramp::HEX_CIRCLE_LEDS,
      Self::RightOrbit => &*right_orbit::HEX_CIRCLE_LEDS,
    }
  }

  /// Where the hex's center is on the playfield, for scene params
  pub fn origin(self) -> Vec2 {
    let center = self
      .hex_leds()
      .child(6)
      .and_then(|led| led.location)
      .expect("hex center LED has a location");
    planes::PLAYFIELD.to_relative(center)
  }

  /// The shots on either side. The outer shots only have one.
  pub fn neighbors(self) -> Vec<MainShot> {
    let idx = self as usize;
    Self::ALL
      .into_iter()
      .filter(|shot| (*shot as usize).abs_diff(idx) == 1)
      .collect()
  }

  /// The shot whose hex is farthest from this one
  pub fn farthest_from(self) -> MainShot {
    let origin = self.origin();
    Self::ALL
      .into_iter()
      .max_by(|a, b| {
        let a = a.origin().distance_squared(origin);
        let b = b.origin().distance_squared(origin);
        a.total_cmp(&b)
      })
      .unwrap_or(self)
  }

  fn hex_leds(self) -> &'static LedDefinition {
    match self {
      Self::LeftOrbit => &*left_orbit::HEX_LEDS,
      Self::DomeRamp => &*dome_ramp::HEX_LEDS,
      Self::ArcRamp => &*arc_ramp::HEX_LEDS,
      Self::CenterOrbit => &*center_orbit::HEX_LEDS,
      Self::LiftRamp => &*lift_ramp::HEX_LEDS,
      Self::RightOrbit => &*right_orbit::HEX_LEDS,
    }
  }
}

/// Reports every main shot as one [`MainShotHit`], so that modes don't match on six hit events.
pub struct ShotSystem;

impl ShotSystem {
  pub fn new() -> Self {
    Self
  }
}

impl System for ShotSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    let shot = if event.is::<LeftOrbitHit>() {
      MainShot::LeftOrbit
    } else if event.is::<DomeRampHit>() {
      MainShot::DomeRamp
    } else if event.is::<ArcRampHit>() {
      MainShot::ArcRamp
    } else if event.is::<CenterOrbitHit>() {
      MainShot::CenterOrbit
    } else if event.is::<LiftRampHit>() {
      MainShot::LiftRamp
    } else if event.is::<RightOrbitHit>() {
      MainShot::RightOrbit
    } else {
      return;
    };

    let ramp_up = shot == MainShot::LiftRamp && ctx.systems.expect::<LiftRampSystem>().is_lifted();
    ctx.emit(MainShotHit { shot, ramp_up });
  }
}

#[derive(serde::Serialize, Event)]
pub struct MainShotHit {
  pub shot: MainShot,
  /// The lift ramp was up, so the ball is going to the scoop and the shot doesn't count for combos
  pub ramp_up: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Where the shots come from, between the flippers
  const FLIPPERS: Vec2 = Vec2::new(9.9, 38.5);

  /// The last LED of each HEX_LINE_LEDS, which is wired differently per hex
  const TIPS: [(MainShot, u16); 6] = [
    (MainShot::LeftOrbit, 5),
    (MainShot::DomeRamp, 2),
    (MainShot::ArcRamp, 3),
    (MainShot::CenterOrbit, 2),
    (MainShot::LiftRamp, 2),
    (MainShot::RightOrbit, 2),
  ];

  #[test]
  fn hex_lines_point_along_their_shots() {
    for (shot, tip) in TIPS {
      let origin = shot.origin();
      let tip = shot
        .hex_leds()
        .child(tip)
        .and_then(|led| led.location)
        .unwrap();
      let pointing = (planes::PLAYFIELD.to_relative(tip) - origin).normalize();
      let shot_axis = (origin - FLIPPERS).normalize();
      let off_by = pointing.dot(shot_axis).acos().to_degrees();
      assert!(
        off_by < 10.0,
        "{shot:?} hex is {off_by} degrees off its shot"
      );
    }
  }

  #[test]
  fn neighbors_are_the_adjacent_shots() {
    assert_eq!(MainShot::LeftOrbit.neighbors(), vec![MainShot::DomeRamp]);
    assert_eq!(MainShot::RightOrbit.neighbors(), vec![MainShot::LiftRamp]);
    assert_eq!(
      MainShot::ArcRamp.neighbors(),
      vec![MainShot::DomeRamp, MainShot::CenterOrbit]
    );
  }

  #[test]
  fn farthest_shot_is_on_the_other_side() {
    assert_eq!(MainShot::LeftOrbit.farthest_from(), MainShot::RightOrbit);
    assert_eq!(MainShot::RightOrbit.farthest_from(), MainShot::LeftOrbit);
  }
}
