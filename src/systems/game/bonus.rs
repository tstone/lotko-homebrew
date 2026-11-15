use std::collections::VecDeque;

use frontbox_core::prelude::*;
use frontbox_gfx::scene::NumberFormat;
use frontbox_pinball::{GameManagementExt, TurnEndReason, TurnEnding};

use crate::hardware::cabinet::DoubleFlipped;
use crate::metrics;
use crate::systems::dmd::{DmdScreen, TURN_END_DMD_LAYER};
use crate::systems::game::{ModeManager, SporesSystem, points};

/// How long each bonus screen is shown
const SCREEN_DURATION: Duration = Duration::from_millis(1750);

/// A participant in `TurnEnding`: awards the end-of-ball bonus, and shows its breakdown then its total on the DMD. A
/// double flip skips the rest of the screens. A tilted turn gets no bonus.
#[derive(Clone)]
pub struct BonusSystem {
  /// The screens still to show for the current bonus, each an amount over its label
  screens: VecDeque<(String, &'static str)>,
  /// The cue for moving off the screen being shown, while one is
  screen_cue: Option<u64>,
  screen: DmdScreen,
}

impl BonusSystem {
  pub fn new() -> Self {
    Self {
      screens: VecDeque::new(),
      screen_cue: None,
      screen: DmdScreen::new("dmd/bonus.scene", TURN_END_DMD_LAYER),
    }
  }

  fn on_turn_ending(&mut self, ctx: &SystemContext) {
    let spores = ctx.systems.expect::<SporesSystem>().count() as u64;
    let completed = ctx.systems.expect::<ModeManager>().districts_completed() as u64;
    let bonus = spores * points::BONUS_PER_SPORE + completed * points::BONUS_PER_CITY_MODE;
    log::info!(
      "Bonus: {} spores applied, {} city modes completed, awarding {}",
      spores,
      completed,
      bonus
    );
    ctx.add_points(bonus);

    self.screens = VecDeque::from([
      (
        Self::multiplied(spores, points::BONUS_PER_SPORE),
        "SPORES APPLIED",
      ),
      (
        Self::multiplied(completed, points::BONUS_PER_CITY_MODE),
        "REGIONS RECLAIMED",
      ),
      (grouped(bonus), "BONUS"),
    ]);
    self.show_next_screen(ctx);
  }

  fn multiplied(count: u64, value: u64) -> String {
    format!("{} x {}", count, grouped(value))
  }

  /// Shows the next screen, or once they've all been shown, completes this step of the turn ending. The last screen is
  /// left up for `TurnEndScoreSystem` to replace: the next step starts after this tick renders, so clearing it here
  /// would flash the game score in between.
  fn show_next_screen(&mut self, ctx: &SystemContext) {
    if let Some((amount, label)) = self.screens.pop_front() {
      self.screen.show([("amount", amount.into()), ("label", label.into())], ctx);
      self.screen_cue = Some(ctx.cues.start(BonusScreenShown, SCREEN_DURATION.once()));
    } else {
      self.screen_cue = None;
      ctx.routines.complete_step::<TurnEnding>();
    }
  }

  /// Drops the remaining screens and completes this step of the turn ending. The bonus has already been awarded.
  fn skip(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.screen_cue.take() {
      log::info!("Bonus: skipped");
      ctx.metrics.inc(&metrics::BONUS_SKIPPED);
      ctx.cues.cancel(cue_id);
      self.screens.clear();
      self.show_next_screen(ctx);
    }
  }

  fn clear_screen(&mut self, ctx: &SystemContext) {
    self.screen.clear(ctx);
  }
}

impl System for BonusSystem {
  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    match routine.downcast_ref::<TurnEnding>() {
      Some(TurnEnding {
        reason: TurnEndReason::Tilted,
        ..
      }) => {
        log::info!("Bonus: none, the turn was tilted");
        ctx.routines.complete_step::<TurnEnding>();
      }
      Some(_) => self.on_turn_ending(ctx),
      None => {}
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<BonusScreenShown>() {
      self.show_next_screen(ctx);
    } else if event.is::<DoubleFlipped>() {
      self.skip(ctx);
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.clear_screen(ctx);
  }
}

/// With commas between the thousands
fn grouped(number: u64) -> String {
  NumberFormat::Grouped.write(number as i64)
}

#[derive(serde::Serialize, Event)]
struct BonusScreenShown;
