use std::collections::VecDeque;

use frontbox::prelude::*;
use frontbox_canvas::*;
use frontbox_pin2dmd::*;
use frontbox_pinball::{GameManagementExt, TurnEnding};

use crate::hardware::cabinet::DoubleFlipped;
use crate::metrics;
use crate::systems::dmd::TURN_END_DMD_LAYER;
use crate::systems::game::{ModeManager, SporesSystem, points};

/// How long each bonus screen is shown
const SCREEN_DURATION: Duration = Duration::from_millis(1750);

/// A participant in `TurnEnding`: awards the end-of-ball bonus, and shows its breakdown then its total on the DMD. A
/// double flip skips the rest of the screens.
#[derive(Clone)]
pub struct BonusSystem {
  /// The screens still to show for the current bonus, each an amount over its label
  screens: VecDeque<(String, &'static str)>,
  /// The cue for moving off the screen being shown, while one is
  screen_cue: Option<u64>,
}

impl BonusSystem {
  pub fn new() -> Self {
    Self {
      screens: VecDeque::new(),
      screen_cue: None,
    }
  }

  fn on_turn_ending(&mut self, ctx: &SystemContext) {
    let spores = ctx.expect::<SporesSystem>().count();
    let completed = ctx.expect::<ModeManager>().completed_city_mode_count() as u32;
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
      (TextFormatting::number(bonus), "BONUS"),
    ]);
    self.show_next_screen(ctx);
  }

  fn multiplied(count: u32, value: u32) -> String {
    format!("{} x {}", count, TextFormatting::number(value))
  }

  /// Shows the next screen, or once they've all been shown, completes this step of the turn ending
  fn show_next_screen(&mut self, ctx: &SystemContext) {
    if let Some((amount, label)) = self.screens.pop_front() {
      if let Some(mut dmd) = ctx.get::<DmdSystem>() {
        dmd.insert_layer(
          TURN_END_DMD_LAYER,
          Self::draw(amount, label).default_position(),
        );
      }
      self.screen_cue = Some(ctx.cue(BonusScreenShown, SCREEN_DURATION.once()));
    } else {
      self.screen_cue = None;
      self.clear_screen(ctx);
      ctx.complete_routine_step::<TurnEnding>();
    }
  }

  /// Drops the remaining screens and completes this step of the turn ending. The bonus has already been awarded.
  fn skip(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.screen_cue.take() {
      log::info!("Bonus: skipped");
      ctx.metrics.inc(&metrics::BONUS_SKIPPED);
      ctx.cancel_cue(cue_id);
      self.screens.clear();
      self.show_next_screen(ctx);
    }
  }

  fn draw(amount: String, label: &'static str) -> Container {
    let mut frame = Container::new(Fill2d::Solid(Rgba::black()));
    frame.add(
      BOLD_PIXELS_10PX_REGULAR_FONT
        .center_aligned(amount, Rgba::white())
        .top_offset(6),
    );
    frame.add(
      SIGI_BOLD_7PX_FONT
        .center_aligned(label, Rgba::yellow().lighten(0.3))
        .top_offset(20),
    );
    frame
  }

  fn clear_screen(&self, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.get::<DmdSystem>() {
      dmd.remove_layer(TURN_END_DMD_LAYER);
    }
  }
}

impl System for BonusSystem {
  fn on_routine(&mut self, routine: &dyn Event, ctx: &SystemContext) {
    if routine.is::<TurnEnding>() {
      self.on_turn_ending(ctx);
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

#[derive(serde::Serialize, Event)]
struct BonusScreenShown;
