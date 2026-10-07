use crate::hardware::more_tags::DropBank;
use crate::hardware::planes;
use frontbox::prelude::*;
use frontbox::tags::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DropBankTarget {
  /// Nearest the player/closest to the pop
  Target1,
  /// Middle
  Target2,
  /// Farthest from the player/closest to the arc ramp
  Target3,
}

impl DropBankTarget {
  pub fn next(&self) -> Self {
    match self {
      Self::Target3 => Self::Target2,
      Self::Target2 => Self::Target1,
      Self::Target1 => Self::Target3,
    }
  }
}

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("drop")
    .tag(Playfield)
    .mode(
      DriverMode::pulse()
        .trigger_mode(DriverTriggerMode::VirtualSwitchTrue)
        .initial_pwm_length(HardwareValue::config(
          "Drop Target Reset Duration",
          "Amount of time fire the coil to reset the bank",
          Duration::from_millis(25),
          Ranges::duration(5, 100)
        ))
        .initial_pwm_power(HardwareValue::config(
          "Drop Target Return Duration",
          "Power to use when resetting the bank",
          Power::FULL,
          Ranges::full_power()
        ))
        .build(),
    )
    .location(Vec3::new(5.986, 13.096, 0.0).relative_to(&planes::PLAYFIELD));

  pub TARGET1: SwitchDefinition = SwitchDefinition::new("drop_target1")
    .inverted()
    .tag(DropBank)
    .tag(Playfield)
    .location(Vec3::new(6.47, 14.584, 0.0).relative_to(&planes::PLAYFIELD));

  pub TARGET2: SwitchDefinition = SwitchDefinition::new("drop_target2")
    .inverted()
    .tag(DropBank)
    .tag(Playfield)
    .location(Vec3::new(6.663, 13.195, 0.0).relative_to(&planes::PLAYFIELD));

  pub TARGET3: SwitchDefinition = SwitchDefinition::new("drop_target3")
    .inverted()
    .tag(DropBank)
    .tag(Playfield)
    .location(Vec3::new(6.717, 11.954, 0.0).relative_to(&planes::PLAYFIELD));

  pub PADDLE_SWITCH: SwitchDefinition = SwitchDefinition::new("drop_paddle")
    .debounce_close(Duration::from_millis(20))
    .tag(Playfield)
    .location(Vec3::new(5.999, 12.225, 0.0).relative_to(&planes::PLAYFIELD));

  pub TARGET1_LEDS: LedDefinition = LedDefinition::multi("drop_target1")
    .locations(LedLayout::strip(4, Vec3::new(8.139, 14.704, 0.0), 190.0f32.to_radians(), 0.25).relative_to(&planes::PLAYFIELD))
    .tag(Insert)
    .tag(Playfield);

  pub TARGET2_LEDS: LedDefinition = LedDefinition::multi("drop_target2")
      .locations(LedLayout::strip(4, Vec3::new(8.275, 13.411, 0.0), 190.0f32.to_radians(), 0.25).relative_to(&planes::PLAYFIELD))
    .tag(Insert)
    .tag(Playfield);

  pub TARGET3_LEDS: LedDefinition = LedDefinition::multi("drop_target3")
    .locations(LedLayout::strip(4, Vec3::new(8.357, 12.124, 0.0), 190.0f32.to_radians(), 0.25).relative_to(&planes::PLAYFIELD))
    .tag(Insert)
    .tag(Playfield);

  pub PADDLE_LED: LedDefinition = LedDefinition::single("paddle")
    .tag(Playfield)
    .location(Vec3::new(6.194, 13.762, 0.0).relative_to(&planes::PLAYFIELD));
}

pub fn leds_for_target(target: &DropBankTarget) -> &'static LedDefinition {
  match target {
    DropBankTarget::Target1 => &TARGET1_LEDS,
    DropBankTarget::Target2 => &TARGET2_LEDS,
    DropBankTarget::Target3 => &TARGET3_LEDS,
  }
}

pub fn match_switch(switch: &Switch) -> Option<DropBankTarget> {
  if switch.name == TARGET1.name {
    Some(DropBankTarget::Target1)
  } else if switch.name == TARGET2.name {
    Some(DropBankTarget::Target2)
  } else if switch.name == TARGET3.name {
    Some(DropBankTarget::Target3)
  } else {
    None
  }
}

#[derive(Clone)]
pub struct DropBankSystem {
  handle: SystemHandle,
}

impl DropBankSystem {
  pub fn new() -> Self {
    Self {
      handle: SystemHandle::default(),
    }
  }

  pub fn raise_targets(&self, ctx: &ServiceContext) {
    ctx
      .for_system(self.handle)
      .activate_driver(COIL.name, ActivationMode::Tap);
  }
}

impl System for DropBankSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = *ctx.current_handle();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if event.switch.name == TARGET1.name {
        ctx.emit(DropBankTargetHit(1));
      } else if event.switch.name == TARGET2.name {
        ctx.emit(DropBankTargetHit(2));
      } else if event.switch.name == TARGET3.name {
        ctx.emit(DropBankTargetHit(3));
      } else if event.switch.name == PADDLE_SWITCH.name {
        ctx.emit(DropBankPaddleHit);
        self.raise_targets(ctx.into());
      }
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct DropBankTargetHit(pub u8);

#[derive(serde::Serialize, Event)]
pub struct DropBankPaddleHit;
