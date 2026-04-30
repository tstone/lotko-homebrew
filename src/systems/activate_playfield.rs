use frontbox::prelude::*;
use frontbox_turn_based::GameStarted;

use crate::io_network::{drivers, switches};

/// Turn on all playfield drivers once the game starts
pub struct ActivatePlayfield;

impl ActivatePlayfield {
  pub fn new() -> Self {
    Self
  }

  fn activate(&self, ctx: &Context) {
    let machine = ctx.systems.expect::<Machine>();

    machine.activate_driver(
      drivers::SLINGSHOT_LEFT,
      ActivationMode::Automatic(switches::SLINGSHOT_LEFT),
      ctx,
    );
    machine.activate_driver(
      drivers::SLINGSHOT_RIGHT,
      ActivationMode::Automatic(switches::SLINGSHOT_RIGHT),
      ctx,
    );
    machine.activate_driver(
      drivers::POP_RIGHT,
      ActivationMode::Automatic(switches::POP_RIGHT),
      ctx,
    );
    machine.activate_driver(
      drivers::POP_LEFT,
      ActivationMode::Automatic(switches::POP_LEFT),
      ctx,
    );

    machine.activate_driver(
      drivers::FLIPPER_MAIN_LEFT,
      ActivationMode::Automatic(switches::LEFT_FLIPPER1),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_MAIN_HOLD_LEFT,
      ActivationMode::Automatic(switches::LEFT_FLIPPER1),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_MAIN_RIGHT,
      ActivationMode::Automatic(switches::RIGHT_FLIPPER1),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_MAIN_HOLD_RIGHT,
      ActivationMode::Automatic(switches::RIGHT_FLIPPER1),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_UPPER_LEFT,
      ActivationMode::Automatic(switches::LEFT_FLIPPER2),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_UPPER_HOLD_LEFT,
      ActivationMode::Automatic(switches::LEFT_FLIPPER2),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_UPPER_RIGHT,
      ActivationMode::Automatic(switches::RIGHT_FLIPPER2),
      ctx,
    );
    machine.activate_driver(
      drivers::FLIPPER_UPPER_HOLD_RIGHT,
      ActivationMode::Automatic(switches::RIGHT_FLIPPER2),
      ctx,
    );
  }
}

impl System for ActivatePlayfield {
  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if let Some(_) = event.downcast_ref::<GameStarted>() {
      log::info!("Activating playfield drivers due to game start");
      self.activate(ctx);
    }
  }
}
