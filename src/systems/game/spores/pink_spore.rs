use itertools::Itertools;
use std::collections::HashMap;
use std::sync::LazyLock;

use frontbox::animation::*;
use frontbox::prelude::*;

use crate::systems::game::spores::*;
use crate::systems::game::*;

static AOE_DECAY: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs(45));
static COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::magenta());

/// PinkSpore creates an AoE type effect that also hits nearby shots, but those extra hits decay
#[derive(Clone)]
pub struct PinkSpore {
  handle: SystemHandle,
  nodes: HashMap<CityShot, Vec<SporeNode>>,
}

impl PinkSpore {
  pub fn new() -> Self {
    Self {
      handle: SystemHandle::default(),
      nodes: HashMap::new(),
    }
  }

  fn push_next_locked(&mut self, shot: &CityShot) {
    if let Some(idx) = self.next_idx_for_shot(shot) {
      self.nodes.get_mut(shot).unwrap()[idx] = SporeNode::Locked;
    }
  }

  fn push_next_decay(&mut self, shot: &CityShot) {
    if let Some(idx) = self.next_idx_for_shot(shot) {
      self.nodes.get_mut(shot).unwrap()[idx] = SporeNode::Decaying(Tween::new(
        AOE_DECAY.clone(),
        Curve::Linear,
        vec![*COLOR, Rgba::default()],
        Cycle::Once,
      ));
    }
  }

  fn next_idx_for_shot(&self, shot: &CityShot) -> Option<usize> {
    if let Some(shot_decay) = self.nodes.get(&shot) {
      let next = shot_decay.len() + 1;
      if next < SPORE_COUNT as usize {
        Some(next)
      } else {
        // there aren't any free ones left
        // find the most depleted and use that
        shot_decay
          .iter()
          .enumerate()
          .filter_map(|(idx, node)| match node {
            SporeNode::Decaying(tween) => Some((idx, tween)),
            _ => None,
          })
          .sorted_by(|(_, a), (_, b)| a.remaining().cmp(&b.remaining()))
          .map(|(idx, _)| idx)
          .next()
      }
    } else {
      Some(0)
    }
  }
}

impl Spore for PinkSpore {
  fn apply(
    &mut self,
    target_shot: &CityShot,
    current: &HashMap<CityShot, f32>,
    _ctx: &ServiceContext,
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

    // main target gets 1 locked, 1 decaying
    results.insert(*target_shot, SPORE_UNIT * 2.0);
    self.push_next_locked(target_shot);
    self.push_next_decay(target_shot);

    if prev_idx != hit_idx {
      let shot = region_shots[prev_idx];
      results.insert(shot, SPORE_UNIT);
      self.push_next_decay(target_shot);
    }
    if next_idx != hit_idx && next_idx != prev_idx {
      let shot = region_shots[next_idx];
      results.insert(shot, SPORE_UNIT);
      self.push_next_decay(target_shot);
    }

    results
  }
}

impl System for PinkSpore {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = *ctx.current_handle();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    // process decay
    for (shot, nodes) in &mut self.nodes {
      let hex = shot.to_hex_leds();
      let mut reapply_spore = false;

      for (idx, node) in nodes.iter_mut().enumerate() {
        let child = &hex.child(idx as u16).unwrap().q();

        match node {
          SporeNode::Locked => {
            ctx.declare_leds(child, ColorSequence::solid(*COLOR));
          }
          SporeNode::Decaying(tween) if !tween.is_complete() => {
            tween.accumulate(delta);

            if tween.is_complete() {
              ctx.undeclare_leds(child);
              reapply_spore = true;
            } else {
              ctx.declare_leds(child, ColorSequence::solid(tween.sample()));
            }
          }
          _ => {}
        }
      }

      if reapply_spore {
        let count = nodes
          .iter()
          .filter(|n| match n {
            SporeNode::Locked => true,
            SporeNode::Decaying(tween) => tween.is_complete(),
          })
          .count();
        let city_manager = ctx.expect::<CityManager>();
        city_manager.apply_biospore(*shot, count as f32 * SPORE_UNIT, ctx.into());
      }
    }
  }
}

#[derive(Clone)]
enum SporeNode {
  Locked,
  Decaying(Tween<Duration, Rgba<u8>>),
}

#[derive(serde::Serialize, Event)]
struct SpreadDecay(pub CityShot, pub f32);
