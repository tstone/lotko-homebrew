use std::sync::LazyLock;

use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::arc_ramp::State::*;
use crate::hardware::more_tags::ArcRamp;
use crate::hardware::planes;

hardware_defs! {
  pub RAMP_OPTO: SwitchDefinition = SwitchDefinition::new("arc_opto")
    .inverted()
    .debounce(Duration::from_millis(2))
    .tag(Playfield)
    .location(Vec3::new(7.607, 5.339, 0.0).relative_to(&planes::PLAYFIELD));

  /// detects when the ball has entered the arc subway
  pub SUBWAY_OPTO: SwitchDefinition = SwitchDefinition::new("arc_subway")
    .inverted()
    .tag(Playfield)
    .location(Vec3::new(1.703, 7.966, 0.0).relative_to(&planes::PLAYFIELD));

  pub SUBWAY_LEDS: LedDefinition = LedDefinition::multi("arc_subway")
    .locations(LedLayout::strip(11, Vec3::new(1.545, 15.809, -2.25), 76.0f32.to_radians(), 0.55).relative_to(&planes::PLAYFIELD))
    .tag(ArcRamp)
    .tag(Playfield);

  pub ARC_LEDS: LedDefinition = LedDefinition::multi("arc")
    .locations(LedLayout::arc(18, Vec3::new(4.072, 0.059, 0.0), 3.5, 0.0, 180.0f32.to_radians(), LedLayoutDirection::CounterClockwise).relative_to(&planes::ARC_RAMP))
    .tag(ArcRamp)
    .tag(Playfield);

  pub HEX_LEDS: LedDefinition = LedDefinition::multi("arc_ramp_lane")
    .tag(Playfield)
    .tag(Insert)
    .tag(Lane)
    .locations(LedLayout::ring(6, Vec3::new(8.336, 10.633, 0.0), 0.5, 350.0f32.to_radians(), LedLayoutDirection::Clockwise).relative_to(&planes::PLAYFIELD))
    .locations([Vec3::new(8.336, 10.633, 0.0).relative_to(&planes::PLAYFIELD)]);
}

pub static HEX_CENTER_LED: LazyLock<LedQ> = LazyLock::new(|| HEX_LEDS.child(6).unwrap().q());

pub static HEX_LINE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(6).unwrap().name(),
    HEX_LEDS.child(3).unwrap().name(),
  ])
});

pub static HEX_CIRCLE_LEDS: LazyLock<LedQ> = LazyLock::new(|| {
  LedQ::names(vec![
    HEX_LEDS.child(3).unwrap().name(),
    HEX_LEDS.child(2).unwrap().name(),
    HEX_LEDS.child(1).unwrap().name(),
    HEX_LEDS.child(0).unwrap().name(),
    HEX_LEDS.child(5).unwrap().name(),
    HEX_LEDS.child(4).unwrap().name(),
  ])
});

pub fn into_subway_program(color: Rgba<u8>) -> LedProgram1d {
  LedProgram1d::rotating(
    ARC_LEDS.q().range(0..6).reverse(),
    ColorSequence::exact(vec![color, color.darken(0.2), color.darken(0.4)]),
    Duration::from_millis(500),
    Curve::EaseIn,
    Cycle::Forever,
  )
}

#[derive(Clone)]
pub struct ArcRampSystem {
  state: State,
}

impl ArcRampSystem {
  pub fn new() -> Self {
    Self { state: Listening }
  }

  fn on_ramp_hit(&mut self, ctx: &SystemContext) {
    log::info!("Arc ramp hit");
    ctx.emit(ArcRampHit);

    // Because of the geometry and switch location on the arc ramp, the ball can easily roll up the ramp
    // then roll back down immediately, triggering two hits. Avoid this by putting the system into a
    // "cooldown" state after each hit. The ball will either go into the subway or fall into the wire form
    // and by the time it exits the cooldown is complete. This avoids double triggering the hit event.
    self.state = Cooldown;
    ctx.cue(Resume, Cue::Once(Duration::from_millis(1500)));
  }
}

impl System for ArcRampSystem {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if event.switch.name == RAMP_OPTO.name && self.state == Listening {
        self.on_ramp_hit(ctx);
      } else if event.switch.name == SUBWAY_OPTO.name {
        log::info!("Arc ramp subway entered");
        ctx.emit(ArcRampSubwayHit);
      }
    } else if event.is::<Resume>() {
      self.state = Listening;
    }
  }
}

#[derive(serde::Serialize, Event, Clone, PartialEq, Eq)]
enum State {
  Listening,
  Cooldown,
}

#[derive(serde::Serialize, Event)]
pub struct ArcRampHit;

#[derive(serde::Serialize, Event)]
pub struct ArcRampSubwayHit;

#[derive(serde::Serialize, Event)]
struct Resume;
