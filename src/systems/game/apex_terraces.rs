use std::collections::HashSet;

use crate::{
  hardware::{
    center_orbit::CenterSpinnerHit,
    left_inlane,
    pop_cluster::{self, PopBumper, match_target_switch},
  },
  systems::{
    game::{ModeManager, NonExclusiveMode},
    sounds,
  },
};
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::*;

/// Lit target hits it takes to charge the center spinner
const REQUIRED_CHARGE: u8 = 6;
/// How long targets stay lit after an unlit target is hit
const LIT_DURATION: Duration = Duration::from_secs(18);
/// How long before lit targets go out that they start flashing as a warning
const LIT_WARNING: Duration = Duration::from_secs(4);
/// How long a charged spinner scores once it's first spun
const SPINNING_DURATION: Duration = Duration::from_secs(15);

mod points {
  pub static UNLIT_TARGET: u32 = 250_000;
  pub static LIT_TARGET: u32 = 1_000_000;
  pub static SPIN: u32 = 1_000_000;
  pub static CHARGED_TARGET: u32 = 50_000;
}

/// Phase 1: hitting an unlit green target lights the others for a while. Hitting a lit one collects it and adds to the
/// center spinner's charge, shown by the LED under it getting brighter.
/// Phase 2: once fully charged, the spinner flashes green. The first spin starts a window where every spin scores big.
/// The targets stay dimly lit and are worth a little.
#[derive(Clone)]
pub struct ApexTerracesMode {
  phase: Phase,
  charge: u8,
  lit: HashSet<Target>,
  lit_warning: bool,
  lit_cue: Option<u64>,
  target_effect: Option<LedPlayback>,
  spinner_effect: Option<LedPlayback>,
  start_effect: Option<LedPlayback>,
}

impl ApexTerracesMode {
  pub fn new() -> Self {
    Self {
      phase: Phase::Charging,
      charge: 0,
      lit: HashSet::new(),
      lit_warning: false,
      lit_cue: None,
      target_effect: None,
      spinner_effect: None,
      start_effect: None,
    }
  }

  fn on_target_hit(&mut self, target: Target, ctx: &SystemContext) {
    if self.phase != Phase::Charging {
      ctx.add_points(points::CHARGED_TARGET);
      ctx.play_sfx(sounds::HIT_ORGANIC_LOW2);
      return;
    }

    if self.lit.remove(&target) {
      ctx.add_points(points::LIT_TARGET);
      self.charge += 1;
      log::info!(
        "ApexTerraces: spinner charge {}/{}",
        self.charge,
        REQUIRED_CHARGE
      );

      if self.charge == REQUIRED_CHARGE {
        self.charge_spinner(ctx);
        return;
      }

      ctx.play_sfx(sounds::COUNTING);
      self.set_spinner_effect(Self::charge_effect(self.charge), ctx);

      if self.lit.is_empty() {
        self.clear_lit(ctx);
      } else {
        self.render_targets(ctx);
      }
    } else {
      ctx.add_points(points::UNLIT_TARGET);
      ctx.play_sfx(sounds::COUNTING);
      self.light_others(&target, ctx);
    }
  }

  fn light_others(&mut self, hit: &Target, ctx: &SystemContext) {
    self.cancel_lit_cue(ctx);
    self.lit = Target::ALL.into_iter().filter(|t| t != hit).collect();
    self.lit_warning = false;
    self.lit_cue = Some(
      ctx
        .cues
        .start(LitTargetsWarned, (LIT_DURATION - LIT_WARNING).once()),
    );
    self.render_targets(ctx);
  }

  fn clear_lit(&mut self, ctx: &SystemContext) {
    self.cancel_lit_cue(ctx);
    self.lit.clear();
    self.lit_warning = false;
    self.render_targets(ctx);
  }

  fn cancel_lit_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.lit_cue.take() {
      ctx.cues.cancel(cue_id);
    }
  }

  fn render_targets(&mut self, ctx: &SystemContext) {
    if let Some(effect) = self.target_effect.as_mut() {
      effect.stop(ctx);
    }

    self.target_effect = if self.phase != Phase::Charging {
      // once charged, the targets stay dimly lit since they're still worth a little
      Some(led_playback!("apex_terraces/targets_charged.scene"))
    } else if self.lit.is_empty() {
      None
    } else {
      let q = LedQ::Or(self.lit.iter().map(Target::led).collect());
      Some(if self.lit_warning {
        led_playback!("apex_terraces/targets_warning.scene", q)
      } else {
        led_playback!("apex_terraces/targets_lit.scene", q)
      })
    };
  }

  fn set_spinner_effect(&mut self, effect: LedPlayback, ctx: &SystemContext) {
    if let Some(effect) = self.spinner_effect.as_mut() {
      effect.stop(ctx);
    }
    self.spinner_effect = Some(effect);
  }

  /// The LED under the spinner gets brighter with each charge
  fn charge_effect(charge: u8) -> LedPlayback {
    let dim = 0.4 * (1.0 - charge as f32 / REQUIRED_CHARGE as f32);
    led_playback!("apex_terraces/spinner_charging.scene").param("brightness", 1.0 - 2.0 * dim)
  }

  fn charge_spinner(&mut self, ctx: &SystemContext) {
    log::info!("ApexTerraces: spinner charged");
    self.phase = Phase::Charged;
    self.clear_lit(ctx);

    ctx.play_sfx(sounds::EVOLUTION_COMPLETE);
    self.start_effect = Some(led_playback!("apex_terraces/start.scene"));
    self.set_spinner_effect(led_playback!("apex_terraces/spinner_charged.scene"), ctx);

    ctx
      .systems
      .expect::<ModeManager>()
      .non_exclusive_active(NonExclusiveMode::ApexTerraces, ctx.into());
  }

  fn on_spin(&mut self, ctx: &SystemContext) {
    match self.phase {
      Phase::Charging => {}
      Phase::Charged => {
        log::info!("ApexTerraces: spinning started");
        self.phase = Phase::Spinning;
        ctx.cues.start(SpinningCompleted, SPINNING_DURATION.once());
        ctx.add_points(points::SPIN);
      }
      Phase::Spinning => ctx.add_points(points::SPIN),
    }
  }

  fn complete(&mut self, ctx: &SystemContext) {
    log::info!("ApexTerraces: complete");
    for effect in [&mut self.target_effect, &mut self.spinner_effect]
      .into_iter()
      .flatten()
    {
      effect.stop(ctx);
    }

    ctx
      .systems
      .expect::<ModeManager>()
      .complete_non_exclusive(NonExclusiveMode::ApexTerraces, ctx.into());
    // a city mode can only be completed once per game
    ctx.systems.despawn_self();
  }
}

impl System for ApexTerracesMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LitTargetsWarned>() {
      self.lit_warning = true;
      self.lit_cue = Some(ctx.cues.start(LitTargetsTimedOut, LIT_WARNING.once()));
      self.render_targets(ctx);
    } else if event.is::<LitTargetsTimedOut>() {
      self.lit_cue = None;
      ctx.play_sfx(sounds::HIT_ORGANIC_LOW3);
      self.clear_lit(ctx);
    } else if event.is::<SpinningCompleted>() {
      self.complete(ctx);
    } else if event.is::<CenterSpinnerHit>() {
      self.on_spin(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      let target = if let Some(pop) = match_target_switch(&event.switch) {
        Some(Target::Pop(pop))
      } else if event.switch.name == left_inlane::TARGET_SWITCH.name {
        Some(Target::LowerSling)
      } else {
        None
      };

      if let Some(target) = target {
        self.on_target_hit(target, ctx);
      }
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    // lit targets and the spinning window end with the ball; a charged spinner waits for the next one
    self.clear_lit(ctx);
    if self.phase == Phase::Spinning {
      self.complete(ctx);
    }
    ctx.deactivate_led_declarations();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    for effect in [
      &mut self.target_effect,
      &mut self.spinner_effect,
      &mut self.start_effect,
    ]
    .into_iter()
    .flatten()
    {
      effect.apply(delta, ctx);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Phase {
  /// Hitting targets to charge the spinner
  Charging,
  /// The spinner is charged and waiting for its first spin
  Charged,
  /// Every spin scores until the window runs out
  Spinning,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Target {
  Pop(PopBumper),
  LowerSling,
}

impl Target {
  const ALL: [Target; 4] = [
    Target::Pop(PopBumper::Left),
    Target::Pop(PopBumper::UpperRight),
    Target::Pop(PopBumper::LowerRight),
    Target::LowerSling,
  ];

  fn led(&self) -> LedQ {
    match self {
      Target::Pop(pop) => pop_cluster::target_led_for_pop(pop).q(),
      Target::LowerSling => left_inlane::TARGET_LED.q(),
    }
  }
}

#[derive(serde::Serialize, Event)]
struct LitTargetsWarned;

#[derive(serde::Serialize, Event)]
struct LitTargetsTimedOut;

#[derive(serde::Serialize, Event)]
struct SpinningCompleted;
