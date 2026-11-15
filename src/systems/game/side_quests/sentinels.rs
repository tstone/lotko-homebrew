use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::drop_bank::{self, DropBankCleared, DropBankSystem, DropBankTargetHit};
use crate::systems::game::side_quests::{LockLit, SentinelsDispatched};
use crate::systems::game::{
  ExtraBallSource, ExtraBallSystem, ModeManager, Perk, flash_scene, points,
};
use crate::systems::{Callout, SayExt, sounds};

/// The city's drones are the drop targets (DESIGN.md, Sentinels). Surveillance dispatches them
/// ([`SentinelsDispatched`]) when the cameras go a minute without one being blinded, and they take over above
/// everything, suspending a district that's running. Each bank clear downs a drone, and the strips show the targets
/// still up, in the Steward's red. Downing all the drones dispatched ends the takeover.
///
/// Each dispatch sends one more drone than the last, counting dispatches rather than defeats, so draining during the
/// Sentinels doesn't make the next one easier. A drain ends them, and a district they suspended is left to resume from
/// the menu.
///
/// Every 2 drones downed lights a lock, or every one with the Skyrail perk, and 5, 10 and 15 drones give awards, 10
/// being an extra ball. Outside the Sentinels the drops only score consolation points.
///
/// Belongs in the player template.
#[derive(Clone)]
pub struct SentinelsSystem {
  /// The Sentinels have taken over
  running: bool,
  /// Times the Sentinels have been dispatched this game, which is how many drones the latest sent
  dispatches: u8,
  /// Drones still to down before the Sentinels are defeated
  remaining: u8,
  /// Drones downed this game
  drones: u8,
  /// Drones toward the next lock
  toward_lock: u8,
}

impl SentinelsSystem {
  pub fn new() -> Self {
    Self {
      running: false,
      dispatches: 0,
      remaining: 0,
      drones: 0,
      toward_lock: 0,
    }
  }

  fn dispatch(&mut self, ctx: &SystemContext) {
    self.dispatches += 1;
    self.remaining = self.dispatches;
    self.running = true;
    log::info!("Sentinels: {} drones dispatched", self.remaining);
    ctx.say(Callout::SentinelsDeployed);
    ctx.play_sfx(sounds::SCAN_HEAVY_STUTTER);
    // every drone starts with the whole bank up
    if targets().iter().any(|(_, switch)| ctx.switches.is_closed(switch).unwrap_or(false)) {
      ctx.systems.expect::<DropBankSystem>().raise_targets(ctx.into());
    }
    self.render(ctx);
  }

  fn drone_down(&mut self, ctx: &SystemContext) {
    self.drones += 1;
    self.remaining = self.remaining.saturating_sub(1);
    log::info!("Sentinels: {} drones down, {} to go", self.drones, self.remaining);
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

    for (target, _) in targets() {
      ctx.play_scene(&flash_scene(&target.q(), Rgba::white()));
    }

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

    if self.remaining == 0 {
      self.defeated(ctx);
    }
  }

  fn defeated(&mut self, ctx: &SystemContext) {
    log::info!("Sentinels: defeated");
    self.running = false;
    ctx.systems.expect::<ModeManager>().end_sentinels(ctx.into());
    self.render(ctx);
  }

  /// Red on the targets still up while the Sentinels are out
  fn render(&self, ctx: &SystemContext) {
    for (target, switch) in targets() {
      let down = ctx.switches.is_closed(switch).unwrap_or(false);
      if !self.running || down {
        ctx.undeclare_leds(&target.q());
      } else {
        ctx.declare_leds(&target.q(), Rgba::red());
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

  /// The turn ended, which ends the Sentinels. The dispatch still counts toward the next one's drones.
  fn on_deactivate(&mut self, ctx: &SystemContext) {
    if self.running {
      log::info!("Sentinels: the ball ended with {} drones still up", self.remaining);
      self.running = false;
      self.remaining = 0;
    }
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<SentinelsDispatched>() {
      self.dispatch(ctx);
      return;
    }
    if event.is::<DropBankCleared>() {
      if self.running {
        self.drone_down(ctx);
      } else {
        ctx.add_points(points::BANK_CLEAR_UNDEPLOYED);
      }
    } else if event.is::<DropBankTargetHit>() && !self.running {
      ctx.add_points(points::DROP_TARGET_UNDEPLOYED);
    }
    if !self.running {
      return;
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
