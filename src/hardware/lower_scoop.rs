use frontbox::animation::Curve;
use frontbox::prelude::DriverTriggerMode::*;
use frontbox::prelude::*;
use frontbox::tags::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::GameManagementExt;

use crate::hardware::arc_ramp::SUBWAY_SWITCH;
use crate::hardware::more_tags::*;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("lower_scoop")
    .tag(Playfield)
    .mode(PulseKickMode {
      trigger_mode: VirtualSwitchTrue,
      initial_pwm_length: HardwareValue::config(
        "Lower Scoop Touch Time",
        "Duration by which the eject plunger is brought into contact with the ball, before full eject",
        Duration::from_millis(7),
        Ranges::duration(0, 100),
      ),
      initial_pwm_power: HardwareValue::fixed(
        Power::percent(75),
      ),
      secondary_pwm_power: HardwareValue::Fixed(Power::ZERO),
      secondary_pwm_length: HardwareValue::Fixed(Duration::ZERO),
      kick_length: HardwareValue::config(
        "Lower Scoop Eject Time",
        "Duration that the plunger exert full power onto the ball (kick)",
        Duration::from_millis(40),
        Ranges::duration(10, 300),
      ),
      ..Default::default()
    });

  pub OPTO: SwitchDefinition = SwitchDefinition::new("lower_scoop").inverted().tag(Playfield);

  pub LEFT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt1")
    .tag(Bolt)
    .tag(Playfield);

  pub RIGHT_BOLT: LedDefinition = LedDefinition::single("scoop_bolt2")
    .tag(Bolt)
    .tag(Playfield);
}

// -- System --

pub const LOWER_SCOOP_EJECT_SND: &'static str = "dmd_menu_select";

#[derive(Clone)]
pub struct LowerScoopSystem {
  eject_effect: LedEffect,
  subway_entry: bool,
}

impl LowerScoopSystem {
  pub fn new() -> Self {
    let mut eject_effect = LedEffect::cycle(
      LEFT_BOLT.q().or(RIGHT_BOLT.q()),
      Duration::from_millis(750 / 4),
      Curve::Steps(2),
      Cycle::Times(4),
      vec![
        ColorSequence::exact(vec![Rgba::white(), Rgba::default()]),
        ColorSequence::exact(vec![Rgba::default(), Rgba::white()]),
      ],
    );
    eject_effect.stop();

    Self {
      eject_effect,
      subway_entry: false,
    }
  }

  pub fn eject(&mut self, ctx: &Context) {
    // If the player gets the ball into the scoop without using the subway, it gives points
    if !self.subway_entry {
      ctx.add_points(250);
      self.subway_entry = false;
    }

    ctx.play_sfx(LOWER_SCOOP_EJECT_SND);
    ctx.cue(EjectLowerScoop, Cue::Once(Duration::from_millis(750)));
    self.eject_effect.resume();
  }

  fn complete_eject(&mut self, ctx: &Context) {
    ctx.activate_driver(COIL.name, ActivationMode::Tap);
    self.eject_effect.stop_and_clear(ctx);
  }
}

impl System for LowerScoopSystem {
  fn on_spawn(&mut self, ctx: &Context) {
    // check if there is a ball at startup and eject if so
    if ctx.switches.is_closed(OPTO.name).unwrap_or(false) {
      log::debug!("Lower scoop is occupied. Ejecting.");
      self.complete_eject(ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if event.switch.name == OPTO.name {
        self.eject(ctx);
      } else if event.switch.name == SUBWAY_SWITCH.name {
        self.subway_entry = true;
      }
    } else if event.is::<EjectLowerScoop>() {
      self.complete_eject(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    self.eject_effect.apply(delta, ctx);
  }
}

struct EjectLowerScoop;
