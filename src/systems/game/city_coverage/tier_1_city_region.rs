use std::collections::HashMap;
use std::sync::LazyLock;

use frontbox::prelude::System;

use crate::systems::game::{CityRegion, CityShot};

pub trait Tier1CityRegion {
  fn shot_amounts_src(&self) -> &HashMap<CityShot, f32>;
  fn shots(&self) -> &Vec<CityShot>;
}

impl<T: Tier1CityRegion> CityRegion for T {
  fn apply_biospore(&mut self, shot: CityShot, amount: f32) {
    let shots = self.shots();
    if shots.contains(&shot) {
      self.shot_amounts().insert(shot, amount);
    }
  }

  fn shot_amounts(&self) -> HashMap<CityShot, f32> {
    self.shot_amounts_src().clone()
  }

  fn is_started(&self) -> bool {
    self
      .shot_amounts()
      .iter()
      .find(|(_, v)| **v > 0.0)
      .is_some()
  }

  fn is_complete(&self) -> bool {
    self.shot_amounts().iter().all(|(_, v)| *v == 1.0)
  }
}

// -- Meridian Basins --
// the "orbits" region

#[derive(Clone)]
pub struct MeridianBasins {
  shot_amounts: HashMap<CityShot, f32>,
}

impl MeridianBasins {
  pub fn new() -> Self {
    let mut shot_amounts = HashMap::new();
    for shot in MERIDIAN_BASIN_SHOTS.iter() {
      shot_amounts.insert(*shot, 0f32);
    }
    Self { shot_amounts }
  }
}

static MERIDIAN_BASIN_SHOTS: LazyLock<Vec<CityShot>> = LazyLock::new(|| {
  vec![
    CityShot::LeftOrbit,
    CityShot::CenterOrbit,
    CityShot::RightOrbit,
  ]
});

impl Tier1CityRegion for MeridianBasins {
  fn shot_amounts_src(&self) -> &HashMap<CityShot, f32> {
    &self.shot_amounts
  }

  fn shots(&self) -> &Vec<CityShot> {
    &MERIDIAN_BASIN_SHOTS
  }
}

impl System for MeridianBasins {}

// -- HydroCore --
// the "ramps" region

#[derive(Clone)]
pub struct HydroCore {
  shot_amounts: HashMap<CityShot, f32>,
}

impl HydroCore {
  pub fn new() -> Self {
    let mut shot_amounts = HashMap::new();
    for shot in HYDRO_CORE_SHOTS.iter() {
      shot_amounts.insert(*shot, 0f32);
    }
    Self { shot_amounts }
  }
}

static HYDRO_CORE_SHOTS: LazyLock<Vec<CityShot>> =
  LazyLock::new(|| vec![CityShot::LeftRamp, CityShot::ArcRamp, CityShot::LiftRamp]);

impl Tier1CityRegion for HydroCore {
  fn shot_amounts_src(&self) -> &HashMap<CityShot, f32> {
    &self.shot_amounts
  }

  fn shots(&self) -> &Vec<CityShot> {
    &HYDRO_CORE_SHOTS
  }
}

impl System for HydroCore {}
