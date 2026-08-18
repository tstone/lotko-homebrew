use frontbox::prelude::*;

use crate::hardware::{lift_ramp, lower_scoop};

pub struct StartupEject;

impl StartupEject {
  pub fn new() -> Self {
    Self
  }
}

impl System for StartupEject {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    // lift ramp
    if ctx
      .switches
      .is_closed(lift_ramp::SCOOP_OPTO.name)
      .unwrap_or(false)
    {
      ctx.activate_driver(lift_ramp::EJECT_COIL.name, ActivationMode::Tap);
    }

    // lower scoop
    if ctx
      .switches
      .is_closed(lower_scoop::OPTO.name)
      .unwrap_or(false)
    {
      ctx.activate_driver(lower_scoop::COIL.name, ActivationMode::Tap);
    }

    ctx.despawn_self();
  }
}
