use frontbox::{prelude::*, tags::GeneralIllumination};

use crate::hardware::arc_ramp;
use crate::systems::game::ExclusiveModeStarted;

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

  pub fn set_color(&mut self, color: Rgba<u8>) {
    self.color = color;
    self.update = true;
  }

  pub fn clear_color(&mut self) {
    self.color = Rgba::white();
    self.update = true;
  }
}

impl System for PlayfieldIllumination {
  fn on_event(&mut self, event: &dyn Event, _ctx: &SystemContext) {
    if let Some(ExclusiveModeStarted(mode)) = event.downcast_ref::<ExclusiveModeStarted>() {
      self.set_color(mode.color().lighten(0.4));
    }
  }

  fn on_render(&mut self, ctx: &SystemContext) {
    if self.update {
      ctx.declare_leds(
        &LedQ::tag::<GeneralIllumination>(),
        ColorSequence::solid(Rgba::white()),
      );
      ctx.declare_leds(
        &arc_ramp::ARC_LEDS.q().at_z(-1),
        ColorSequence::solid(self.color),
      );
      ctx.declare_leds(
        &arc_ramp::SUBWAY_LEDS.q().at_z(-1),
        ColorSequence::solid(self.color),
      );
    }
  }
}
