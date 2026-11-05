use frontbox_core::prelude::*;
use frontbox_pinball::*;

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
    if event.is::<RoutineStarted<TurnStarting>>() {
      if let Some(drop_bank) = ctx.systems.get::<DropBankSystem>() {
        drop_bank.raise_targets(ctx.into());
      }

      // avoid any switch rattle from the ball entering the plunge lane
      ctx
        .cues
        .start(StartSkillshotModes, Cue::Once(Duration::from_millis(500)));
    } else if event.is::<StartSkillshotModes>() {
      ctx.systems.spawn(LeftPopSkillShot::new());
      ctx.systems.spawn(LeftOutlaneSkillShot::new());
      ctx.systems.spawn(DropBankSkillShot::new());
    }
  }
}

#[derive(serde::Serialize, Event)]
struct StartSkillshotModes;
