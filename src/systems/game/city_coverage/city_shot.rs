use std::collections::HashSet;
use std::sync::LazyLock;

use frontbox::prelude::LedDefinition;

use crate::hardware::{arc_ramp, center_orbit, left_orbit, left_ramp, lift_ramp, right_orbit};

#[derive(serde::Serialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CityShot {
  LeftOrbit,
  LeftRamp,
  ArcRamp,
  CenterOrbit,
  LiftRamp,
  RightOrbit,
}

impl CityShot {
  pub fn ordered() -> &'static Vec<CityShot> {
    &ORDERED_CITY_SHOTS
  }

  /// Returns city shots, in order, but only the ones given
  pub fn ordered_only(shots: impl IntoIterator<Item = CityShot>) -> Vec<CityShot> {
    let shots: HashSet<CityShot> = shots.into_iter().collect();
    let mut ordered = Self::ordered().clone();
    ordered.retain(|s| shots.contains(s));
    ordered
  }

  pub fn to_hex_leds(&self) -> &'static LedDefinition {
    match self {
      CityShot::ArcRamp => &arc_ramp::HEX_LEDS,
      CityShot::CenterOrbit => &center_orbit::HEX_LEDS,
      CityShot::LeftOrbit => &left_orbit::HEX_LEDS,
      CityShot::LeftRamp => &left_ramp::HEX_LEDS,
      CityShot::LiftRamp => &lift_ramp::HEX_LEDS,
      CityShot::RightOrbit => &right_orbit::HEX_LEDS,
    }
  }
}

static ORDERED_CITY_SHOTS: LazyLock<Vec<CityShot>> = LazyLock::new(|| {
  vec![
    CityShot::LeftOrbit,
    CityShot::LeftRamp,
    CityShot::ArcRamp,
    CityShot::CenterOrbit,
    CityShot::LiftRamp,
    CityShot::RightOrbit,
  ]
});
