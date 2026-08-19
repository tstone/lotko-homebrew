use std::collections::HashMap;

use frontbox::prelude::Event;

use crate::systems::game::spores::SPORE_UNIT;
use crate::systems::game::*;

/// PinkSpore creates an AoE type effect that also hits nearby shots
#[derive(Clone)]
pub struct PinkSpore {}

impl PinkSpore {
  pub fn new() -> Self {
    Self {}
  }
}

impl Spore for PinkSpore {
  fn apply(
    &mut self,
    target_shot: &CityShot,
    current: &HashMap<CityShot, f32>,
  ) -> HashMap<CityShot, f32> {
    let region_shots = CityShot::ordered_only(current.keys().copied());
    let count = region_shots.len() as isize;
    let hit_idx = region_shots
      .iter()
      .position(|s| s == target_shot)
      .unwrap_or(0);
    let prev_idx = (hit_idx as isize - 1).rem_euclid(count) as usize;
    let next_idx = (hit_idx as isize + 1).rem_euclid(count) as usize;

    let mut results = HashMap::new();

    // main target gets 2, AOE effect spreads to neighbor shots for 1
    results.insert(*target_shot, SPORE_UNIT * 2.0);
    if prev_idx != hit_idx {
      let shot = region_shots[prev_idx];
      results.insert(shot, SPORE_UNIT);
      // TODO: cue decay
    }
    if next_idx != hit_idx && next_idx != prev_idx {
      let shot = region_shots[next_idx];
      results.insert(shot, SPORE_UNIT);
      // TODO: cue decay
    }

    results
  }
}

#[derive(serde::Serialize, Event)]
struct SpreadDecay(CityShot, f32);
