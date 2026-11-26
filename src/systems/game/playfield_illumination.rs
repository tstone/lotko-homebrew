use frontbox_core::prelude::*;
use frontbox_lighting::prelude::*;
use frontbox_pinball::tags::GeneralIllumination;

use crate::hardware::{arc_ramp, gi};
use crate::systems::game::{ModeManager, ModeSlotsChanged};

#[derive(Clone)]
pub struct PlayfieldIllumination {
  color: Rgba<u8>,
  /// Half the GI is red, as the Sentinels are out
  sentinels: bool,
  update: bool,
}

impl PlayfieldIllumination {
  pub fn new() -> Self {
    Self {
      color: Rgba::white(),
      sentinels: false,
      update: true,
    }
  }
}

impl System for PlayfieldIllumination {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<ModeSlotsChanged>() {
      let mode_manager = ctx.systems.expect::<ModeManager>();
      self.sentinels = mode_manager.sentinels_running();
      // under the Sentinels the rest of the GI is plain white
      self.color = if self.sentinels {
        Rgba::white()
      } else {
        mode_manager.illumination()
      }
      .lighten(0.35);
      self.update = true;
    }
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    // a declaration stands until it's replaced, so it only needs making when the color changes
    if self.update {
      self.update = false;
      ctx.declare_lights(&LightQ::tag::<GeneralIllumination>(), 0, self.color);
      ctx.declare_lights(&arc_ramp::ARC_LEDS.q(), -1, self.color);
      ctx.declare_lights(&arc_ramp::SUBWAY_LEDS.q(), -1, self.color);
      let sentinels_half = gi::sentinels_half();
      if self.sentinels {
        ctx.declare_lights(&sentinels_half, 1, Rgba::red());
      } else {
        ctx.undeclare_lights(&sentinels_half, 1);
      }
    }
  }
}
