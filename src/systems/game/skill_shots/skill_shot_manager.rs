use frontbox::prelude::*;
use frontbox_turn_based::*;

use crate::{
  hardware::drop_bank::DropBankSystem,
  systems::game::{DropBankSkillShot, LeftOutlaneSkillShot, LeftPopSkillShot},
};

#[derive(Clone)]
pub struct SkillshotManager;

impl SkillshotManager {
  pub fn new() -> Self {
    Self
  }
}

impl System for SkillshotManager {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<PlayerTurnBeginning>() {
      if let Some(drop_bank) = ctx.get::<DropBankSystem>() {
        drop_bank.raise_targets(ctx.into());
      }

      // avoid any switch rattle from the ball entering the plunge lane
      ctx.cue(StartSkillshotModes, Cue::Once(Duration::from_millis(500)));
    } else if event.is::<StartSkillshotModes>() {
      ctx.spawn_system(LeftPopSkillShot::new());
      ctx.spawn_system(LeftOutlaneSkillShot::new());
      ctx.spawn_system(DropBankSkillShot::new());
    }
  }
}

#[derive(serde::Serialize, Event)]
struct StartSkillshotModes;
