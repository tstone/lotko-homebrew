use frontbox::prelude::*;
use frontbox_turn_based::PlayerTurnBeginning;

use crate::systems::game::{DropBankSkillShot, LeftOutlaneSkillShot, LeftPopSkillShot};

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
      ctx.spawn_system(LeftPopSkillShot::new());
      ctx.spawn_system(LeftOutlaneSkillShot::new());
      ctx.spawn_system(DropBankSkillShot::new());
    }
  }
}
