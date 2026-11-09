use frontbox_core::{prelude::*, tags::GeneralIllumination};
use frontbox_led::prelude::*;

use crate::hardware::arc_ramp;
use crate::systems::game::{ModeManager, ModeSlotsChanged};

#[derive(Clone)]
pub struct PlayfieldIllumination {
  color: Rgba<u8>,
  update: bool,
}

impl PlayfieldIllumination {
  pub fn new() -> Self {
    Self {
      color: Rgba::white(),
      update: true,
    }
  }

  fn set_color(&mut self, color: Rgba<u8>) {
    self.color = color;
    self.update = true;
  }
}

impl System for PlayfieldIllumination {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<ModeSlotsChanged>() {
      let color = ctx.systems.expect::<ModeManager>().illumination();
      self.set_color(color.lighten(0.35));
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    // a declaration stands until it's replaced, so it only needs making when the color changes
    if self.update {
      self.update = false;
      ctx.declare_leds(&LedQ::tag::<GeneralIllumination>(), self.color);
      ctx.declare_leds(&arc_ramp::ARC_LEDS.q().at_z(-1), self.color);
      ctx.declare_leds(&arc_ramp::SUBWAY_LEDS.q().at_z(-1), self.color);
    }
  }
}
