use frontbox::prelude::*;
use frontbox_canvas::*;
use frontbox_pin2dmd::*;
use frontbox_pinball::{GameManagementExt, TurnEnding};

use crate::systems::dmd::TURN_END_DMD_LAYER;
use crate::systems::game::{ModeManager, points};

/// A participant in `TurnEnding`: awards the end-of-ball bonus, and shows it on the DMD
#[derive(Clone)]
pub struct BonusSystem;

impl BonusSystem {
  pub fn new() -> Self {
    Self
  }

  fn on_turn_ending(&self, ctx: &SystemContext) {
    let completed = ctx.expect::<ModeManager>().completed_city_mode_count() as u32;
    let bonus = completed * points::BONUS_PER_CITY_MODE;
    log::info!(
      "Bonus: {} city modes completed, awarding {}",
      completed,
      bonus
    );
    ctx.add_points(bonus);

    if let Some(mut dmd) = ctx.get::<DmdSystem>() {
      dmd.insert_layer(TURN_END_DMD_LAYER, Self::draw(bonus).default_position());
    }
    ctx.cue(BonusShown, Duration::from_secs(2).once());
  }

  fn draw(bonus: u32) -> Container {
    let mut frame = Container::new(Fill2d::Solid(Rgba::black()));
    frame.add(
      BOLD_PIXELS_10PX_REGULAR_FONT
        .center_aligned(TextFormatting::number(bonus), Rgba::white())
        .top_offset(6),
    );
    frame.add(
      SIGI_BOLD_7PX_FONT
        .center_aligned("BONUS", Rgba::yellow().lighten(0.3))
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
    if event.is::<BonusShown>() {
      self.clear_screen(ctx);
      ctx.complete_routine_step::<TurnEnding>();
    }
  }

  fn on_despawn(&mut self, ctx: &SystemContext) {
    self.clear_screen(ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct BonusShown;
