use dyn_clone::DynClone;
use frontbox::prelude::SwitchClosed;
use std::collections::HashMap;
use std::time::Duration;

mod active_spore;
mod pink_spore;

pub use active_spore::*;
pub use pink_spore::*;

use crate::systems::game::CityShot;

static SPORE_UNIT: f32 = 1.0 / 7.0;

pub trait Spore: DynClone {
  fn apply(
    &mut self,
    target_shot: &CityShot,
    current: &HashMap<CityShot, f32>,
  ) -> HashMap<CityShot, f32>;

  fn tick(
    &mut self,
    _delta: Duration,
    _current: &HashMap<CityShot, f32>,
  ) -> Option<HashMap<CityShot, f32>> {
    None
  }

  fn on_switch_closed(&mut self, _event: SwitchClosed) -> Option<HashMap<CityShot, f32>> {
    None
  }
}

dyn_clone::clone_trait_object!(Spore);
