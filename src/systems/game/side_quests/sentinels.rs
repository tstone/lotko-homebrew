use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::drop_bank::{self, DropBankCleared};
use crate::systems::game::side_quests::LockLit;
use crate::systems::game::{
  EffectsSystem, ExtraBallSource, ExtraBallSystem, ModeManager, Perk, flash_effect, points,
};
use crate::systems::{Callout, SayExt, sounds};

/// The city's drones are the drop targets (DESIGN.md, Sentinels). Each bank clear downs a drone, and the strips show the
/// targets still up toward the next. Every 2 drones lights a lock, or every one with the Skyrail perk, and 5, 10 and 15
/// drones give awards, 10 being an extra ball.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SentinelsSystem {
  drones: u8,
  /// Drones toward the next lock
  toward_lock: u8,
}

impl SentinelsSystem {
  pub fn new() -> Self {
    Self {
      drones: 0,
      toward_lock: 0,
    }
  }

  fn drone_down(&mut self, ctx: &SystemContext) {
    self.drones += 1;
    log::info!("Sentinels: {} drones down", self.drones);
    ctx.add_points(points::DRONE_DOWN);
    ctx.play_sfx(sounds::IMPACT_DIRT);
    match self.drones {
      1 => ctx.say(Callout::DecommissioningDroneOne),
      2 => ctx.say(Callout::DecommissioningDroneTwo),
      3 => ctx.say(Callout::DecommissioningDroneThree),
      4 => ctx.say(Callout::DecommissioningDroneFour),
      drones => match Callout::number(drones) {
        Some(number) => ctx.say_all(&[Callout::DecommissioningDrones, number]),
        None => ctx.say(Callout::DroneDisengaged),
      },
    }

    let mut effects = ctx.systems.expect::<EffectsSystem>();
    for (target, _) in targets() {
      effects.play(flash_effect(&target.q(), Rgba::white()));
    }
    drop(effects);

    self.toward_lock += 1;
    let per_lock = if ctx
      .systems
      .expect::<ModeManager>()
      .has_perk(Perk::QuickLocks)
    {
      1
    } else {
      2
    };
    if self.toward_lock >= per_lock {
      self.toward_lock = 0;
      log::info!("Sentinels: lock lit");
      ctx.emit(LockLit);
    }

    match self.drones {
      5 => ctx.add_points(points::SENTINELS_FIRST_AWARD),
      10 => {
        ctx
          .systems
          .expect::<ExtraBallSystem>()
          .light(ExtraBallSource::Sentinels, ctx.into());
      }
      // DESIGN.md leaves the third award open
      15 => ctx.add_points(points::SENTINELS_THIRD_AWARD),
      _ => {}
    }
  }

  /// Peach on the targets still up
  fn render(&self, ctx: &SystemContext) {
    for (target, switch) in targets() {
      let down = ctx.switches.is_closed(switch).unwrap_or(false);
      if down {
        ctx.undeclare_leds(&target.q());
      } else {
        ctx.declare_leds(&target.q(), ColorSequence::solid(Rgba::peach()));
      }
    }
  }
}

impl System for SentinelsSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.render(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<DropBankCleared>() {
      self.drone_down(ctx);
    }

    // the bank raises itself, so follow the targets rather than the hits
    let switch = if let Some(closed) = event.downcast_ref::<SwitchClosed>() {
      Some(&closed.switch)
    } else {
      event
        .downcast_ref::<SwitchOpened>()
        .map(|opened| &opened.switch)
    };
    if switch.is_some_and(|switch| drop_bank::match_switch(switch).is_some()) {
      self.render(ctx);
    }
  }
}

/// Each target's strip and switch
fn targets() -> [(&'static LedDefinition, &'static str); 3] {
  [
    (&drop_bank::TARGET1_LEDS, drop_bank::TARGET1.name),
    (&drop_bank::TARGET2_LEDS, drop_bank::TARGET2.name),
    (&drop_bank::TARGET3_LEDS, drop_bank::TARGET3.name),
  ]
}
