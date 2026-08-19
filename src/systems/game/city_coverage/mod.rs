mod city_coverage_qualification_1;
mod city_coverage_qualification_2;
mod city_coverage_qualification_3;
mod city_manager;
mod city_shot;
mod tier_1_city_region;

use std::collections::HashMap;

pub use city_coverage_qualification_1::*;
pub use city_coverage_qualification_2::*;
pub use city_coverage_qualification_3::*;
pub use city_manager::*;
pub use city_shot::*;
pub use tier_1_city_region::*;

pub trait CityRegion {
  fn apply_biospore(&mut self, shot: CityShot, amount: f32);
  fn shot_amounts(&self) -> HashMap<CityShot, f32>;
  fn is_started(&self) -> bool;
  fn is_complete(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CityRegions {
  MeridianBasins,
  HydroCore,
  SkyrailStation,
  NimbusPromenade,
  ApexTerraces,
  SolariumAtriums,
}
