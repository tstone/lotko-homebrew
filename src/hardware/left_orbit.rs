use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::more_tags::*;

const NAME: &'static str = "l_orbit";

hardware_defs! {

  pub SWITCH: SwitchDefinition = SwitchDefinition::new(NAME);
  pub UPPER_SWITCH: SwitchDefinition = SwitchDefinition::new("l_orbit_upper");

  pub HEX_LEDS: LedDefinition = LedDefinition::multi(NAME, 7)
    .tag(Playfield)
    .tag(Insert)
    .tag(Hex)
    .tag(Lane);
}

pub fn hex_center_led_q() -> HardwareQuery {
  HEX_LEDS.child(6).unwrap().q()
}

pub fn hex_line_leds_q() -> HardwareQuery {
  Q::names(vec![
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
}

pub fn hex_circle_leds_q() -> HardwareQuery {
  // TODO: verify order
  Q::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
  ])
}

#[derive(Clone)]
pub struct LeftOrbitSystem {
  skip_next: bool,
  clear_cue_id: Option<u64>,
}

impl LeftOrbitSystem {
  pub fn new() -> Self {
    Self {
      skip_next: false,
      clear_cue_id: None,
    }
  }

  fn reset(&mut self) {
    self.skip_next = false;
    self.clear_cue_id = None;
  }
}

impl System for LeftOrbitSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<CenterOrbitHit>() {
      self.skip_next = true;
      // TODO: tune duration
      self.clear_cue_id = Some(ctx.cue(ClearSkipNext, Cue::Once(Duration::from_millis(800))));
    } else if event.is::<ClearSkipNext>() {
      self.reset();
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == SWITCH.name
    {
      if self.skip_next {
        self.skip_next = false;
      } else {
        ctx.emit(LeftOrbitHit);
      }
    }
  }

  fn on_reactivate(&mut self, _ctx: &SystemContext) {
    self.reset();
  }
}

#[derive(serde::Serialize, Event)]
pub struct LeftOrbitHit;
#[derive(serde::Serialize, Event)]
struct ClearSkipNext;
