use std::collections::HashSet;
use std::sync::LazyLock;

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
