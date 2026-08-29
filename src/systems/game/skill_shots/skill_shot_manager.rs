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
      // avoid any switch rattle from the ball entering the plunge lane
      ctx.cue(StartSkillshotModes, Cue::Once(Duration::from_millis(200)));
    } else if event.is::<StartSkillshotModes>() {
      ctx.spawn_system(LeftPopSkillShot::new());
      ctx.spawn_system(LeftOutlaneSkillShot::new());
      ctx.spawn_system(DropBankSkillShot::new());
    }
  }
}

#[derive(serde::Serialize, Event)]
struct StartSkillshotModes;
