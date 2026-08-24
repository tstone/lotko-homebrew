use frontbox::prelude::*;
use frontbox::tags::*;
use frontbox_turn_based::ActivatedPlayfieldDrivers;
use frontbox_turn_based::GameManager;

use crate::hardware::more_tags::DropBank;

hardware_defs! {
  pub COIL: DriverDefinition = DriverDefinition::new("drop")
    .tag(Playfield)
    .mode(PulseMode {
      trigger_mode: DriverTriggerMode::VirtualSwitchTrue,
      initial_pwm_length: HardwareValue::config(
        "Drop Target Reset Duration",
        "Amount of time fire the coil to reset the bank",
        Duration::from_millis(25),
        Ranges::duration(5, 100)
      ),
      initial_pwm_power: HardwareValue::config(
        "Drop Target Return Duration",
        "Power to use when resetting the bank",
        Power::FULL,
        Ranges::full_power()
      ),
      ..Default::default()
    });

  pub TARGET1: SwitchDefinition = SwitchDefinition::new("drop_target1")
    .inverted()
    .tag(DropBank)
    .tag(Playfield);

  pub TARGET2: SwitchDefinition = SwitchDefinition::new("drop_target2")
    .inverted()
    .tag(DropBank)
    .tag(Playfield);

  pub TARGET3: SwitchDefinition = SwitchDefinition::new("drop_target3")
    .inverted()
    .tag(DropBank)
    .tag(Playfield);

  pub PADDLE_SWITCH: SwitchDefinition = SwitchDefinition::new("drop_paddle")
    .tag(Playfield);

  pub TARGET1_LEDS: LedDefinition = LedDefinition::strip("target1", 4)
    .tag(Insert)
    .tag(Playfield);

  pub TARGET2_LEDS: LedDefinition = LedDefinition::strip("target2", 4)
    .tag(Insert)
    .tag(Playfield);

  pub TARGET3_LEDS: LedDefinition = LedDefinition::strip("target3", 4)
    .tag(Insert)
    .tag(Playfield);

  pub PADDLE_LED: LedDefinition = LedDefinition::single("paddle")
    .tag(Playfield);
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
    if event.is::<ActivatedPlayfieldDrivers>()
      && let Some(game_state) = ctx.expect::<GameManager>().game_state()
      && game_state.current_player_turn() == 0
    {
      log::info!("DropBank: Raising targets for player (game start)");
      self.raise_targets(ctx.into());
    }
  }
}
