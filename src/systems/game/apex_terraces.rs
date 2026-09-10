use std::collections::HashMap;

use crate::{
  hardware::{
    flashers::FlashersSystem,
    left_inlane,
    pop_cluster::{self, PopBumper, match_target_switch},
  },
  systems::{
    game::{ModeManager, NonExclusiveMode, apex_terraces::State::*},
    sounds,
  },
};
use frontbox::prelude::*;
use frontbox_sound::*;
use frontbox_turn_based::GameManagementExt;

#[derive(Clone)]
pub struct ApexTerracesMode {
  progress_effect: Option<LedProgram1d>,
  jackpot_effect: Option<LedProgram1d>,
  state: State,
  hits: HashMap<Target, u8>,
}

impl ApexTerracesMode {
  pub fn new() -> Self {
    Self {
      state: AccumulatingTargets,
      progress_effect: None,
      jackpot_effect: None,
      hits: HashMap::new(),
    }
  }

  fn jackpot_effect() -> LedProgram1d {
    LedProgram1d::flash(
      LedQ::any(vec![
        &pop_cluster::left::TARGET_LED.q(),
        &pop_cluster::upper_right::TARGET_LED.q(),
        &pop_cluster::lower_right::TARGET_LED.q(),
        &left_inlane::TARGET_LED.q(),
      ]),
      ColorSequence::solid(Rgba::green()),
      Cycle::Times(3),
    )
  }

  fn on_target_hit(&mut self, target: Target, ctx: &SystemContext) {
    ctx.add_points(75_000);
    *self.hits.entry(target).or_insert(0) += 1;

    if self.jackpot_qualifications_met() {
      ctx.play_sfx(sounds::ARP_HIT1);
      self.start_jackpot(ctx);
    } else {
      ctx.play_sfx(sounds::HIT_ORGANIC2);

      if let Some(effect) = self.progress_effect.as_mut() {
        effect.stop(ctx);
      }

      self.progress_effect = Some(LedProgram1d::multi(
        self
          .hits
          .iter()
          .map(|(target, count)| {
            let q = match target {
              Target::LowerSling => left_inlane::TARGET_LED.q(),
              Target::Pop(PopBumper::Left) => pop_cluster::left::TARGET_LED.q(),
              Target::Pop(PopBumper::UpperRight) => pop_cluster::upper_right::TARGET_LED.q(),
              Target::Pop(PopBumper::LowerRight) => pop_cluster::lower_right::TARGET_LED.q(),
            };
            // pulse speed is relative to hit count
            let duration = Duration::from_millis(1750u64 / *count.min(&5) as u64);
            LedProgram1d::pulse(q, Rgba::green(), duration, Cycle::Forever)
          })
          .collect(),
      ));
    }
  }

  fn jackpot_qualifications_met(&self) -> bool {
    *self.hits.get(&Target::LowerSling).unwrap_or(&0) > 0
      && *self.hits.get(&Target::Pop(PopBumper::Left)).unwrap_or(&0) > 1
      && *self
        .hits
        .get(&Target::Pop(PopBumper::UpperRight))
        .unwrap_or(&0)
        > 1
      && *self
        .hits
        .get(&Target::Pop(PopBumper::LowerRight))
        .unwrap_or(&0)
        > 1
  }

  fn start_jackpot(&mut self, ctx: &SystemContext) {
    log::info!("ApexTerraces: starting jackpot mode");

    ctx
      .expect::<ModeManager>()
      .non_exclusive_active(NonExclusiveMode::ApexTerraces, ctx.into());

    ctx.cue(JackpotOver, Duration::from_secs(45).once());
    self.jackpot_effect = Some(Self::jackpot_effect());
    self.state = Jackpot;
  }

  fn on_jackpot_hit(&mut self, target: Target, ctx: &SystemContext) {
    if let Some(effect) = self.jackpot_effect.as_mut() {
      effect.reset();
      effect.play();
    }

    ctx
      .expect::<FlashersSystem>()
      .flash(2, ColorSequence::solid(Rgba::green()));

    match target {
      Target::LowerSling => {
        ctx.add_points(*self.hits.get(&Target::LowerSling).unwrap().min(&5) as u32 * 500_000);
      }
      Target::Pop(PopBumper::Left) => {
        ctx.add_points(
          *self
            .hits
            .get(&Target::Pop(PopBumper::Left))
            .unwrap()
            .min(&5) as u32
            * 100_000,
        );
      }
      Target::Pop(PopBumper::UpperRight) => {
        ctx.add_points(
          *self
            .hits
            .get(&Target::Pop(PopBumper::UpperRight))
            .unwrap()
            .min(&5) as u32
            * 100_000,
        );
      }
      Target::Pop(PopBumper::LowerRight) => {
        ctx.add_points(
          *self
            .hits
            .get(&Target::Pop(PopBumper::LowerRight))
            .unwrap()
            .min(&5) as u32
            * 250_000,
        );
      }
    }

    if self.jackpot_qualifications_met() {
      ctx.play_sfx(sounds::ARP_HIT1);
    } else {
      ctx.play_sfx(sounds::HIT_ORGANIC2);
    }
  }
}

impl System for ApexTerracesMode {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<JackpotOver>() {
      log::info!("ApexTerraces: jackpot complete");

      ctx
        .expect::<ModeManager>()
        .complete_non_exclusive(NonExclusiveMode::ApexTerraces, ctx.into());
      ctx.replace_self(ApexTerracesMode::new()); // restart
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      let target_hit = if let Some(pop_hit) = match_target_switch(&event.switch) {
        Some(Target::Pop(pop_hit))
      } else if event.switch.name == left_inlane::TARGET_SWITCH.name {
        Some(Target::LowerSling)
      } else {
        None
      };

      if let Some(target) = target_hit {
        match self.state {
          AccumulatingTargets => self.on_target_hit(target, ctx),
          Jackpot => self.on_jackpot_hit(target, ctx),
        }
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(effect) = self.progress_effect.as_mut() {
      effect.apply(delta, ctx);
    }

    if let Some(effect) = self.jackpot_effect.as_mut() {
      effect.apply(delta, ctx);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  AccumulatingTargets,
  Jackpot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Target {
  Pop(PopBumper),
  LowerSling,
}

#[derive(serde::Serialize, Event)]
struct JackpotOver;
