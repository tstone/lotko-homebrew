use frontbox::{prelude::*, tags::GeneralIllumination};

use crate::hardware::arc_ramp;

#[derive(Clone)]
pub struct PlayfieldIllumination;

impl PlayfieldIllumination {
  pub fn new() -> Self {
    Self
  }
}

impl System for PlayfieldIllumination {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    ctx.declare_leds(
      &Q::tag::<GeneralIllumination>(),
      ColorSequence::solid(Rgba::white()),
    );
    ctx.declare_leds(
      &arc_ramp::ARC_LEDS.q().at_z(-1),
      ColorSequence::tile(vec![Rgba::default(), Rgba::white(), Rgba::default()]),
    )
  }
}
