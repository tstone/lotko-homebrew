use frontbox::prelude::*;
use frontbox::tags::*;
use frontbox_pinball::TroughSystem;

use crate::hardware::{more_tags::*, planes};

hardware_defs! {
  pub DRAIN_LED: LedDefinition = LedDefinition::single("drain")
    .tag(Circle)
    .tag(Playfield)
    .location(Vec3::new(9.143, 39.724, 0.0).relative_to(&planes::PLAYFIELD));

  pub SWITCH1: SwitchDefinition = TroughSystem::switch_definition("trough_1")
    .location(Vec3::new(16.029, 41.306, 0.0).relative_to(&planes::PLAYFIELD));

  pub SWITCH2: SwitchDefinition = TroughSystem::switch_definition("trough_2")
    .location(Vec3::new(15.321, 41.842, 0.0).relative_to(&planes::PLAYFIELD));

  pub SWITCH3: SwitchDefinition = TroughSystem::switch_definition("trough_3")
    .location(Vec3::new(14.578, 42.299, 0.0).relative_to(&planes::PLAYFIELD));

  pub SWITCH4: SwitchDefinition = TroughSystem::switch_definition("trough_4")
    .location(Vec3::new(13.834, 42.758, 0.0).relative_to(&planes::PLAYFIELD));

  pub SWITCH5: SwitchDefinition = TroughSystem::switch_definition("trough_5")
    .location(Vec3::new(13.152, 43.313, 0.0).relative_to(&planes::PLAYFIELD));

  pub SWITCH6: SwitchDefinition = TroughSystem::switch_definition("trough_6")
    .location(Vec3::new(12.436, 43.87, 0.0).relative_to(&planes::PLAYFIELD));

  pub COIL: DriverDefinition = TroughSystem::eject_coil_definition("trough_coil")
    .location(Vec3::new(17.377, 40.193, 0.0).relative_to(&planes::PLAYFIELD));
}

pub fn system() -> TroughSystem {
  TroughSystem::new(
    COIL.name,
    vec![
      SWITCH1.name,
      SWITCH2.name,
      SWITCH3.name,
      SWITCH4.name,
      SWITCH5.name,
      // SWITCH6.name, -- off by one ala opto
    ],
  )
}
