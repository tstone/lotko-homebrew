use std::sync::LazyLock;

use frontbox::prelude::*;
use frontbox_turn_based::GameManager;

use crate::systems::BasicPoints;

pub static manager: LazyLock<GameManager> = LazyLock::new(|| {
  GameManager::competitive(4, systems![BasicPoints::new()], Q::tag::<tags::Playfield>())
});
