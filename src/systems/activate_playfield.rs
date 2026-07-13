use crate::hardware::*;
use frontbox::prelude::*;
use frontbox_turn_based::GameStarted;
/// Turn on all playfield drivers once the game starts
pub struct ActivatePlayfield;

impl ActivatePlayfield {
  pub fn new() -> Self {
    Self
  }

  fn activate(&self, ctx: &Context) {
    let machine = ctx.systems.expect::<Machine>();

    machine.activate_driver(
      slingshots::LEFT_COIL.name,
      ActivationMode::Automatic(slingshots::LEFT_SWITCH.name),
      ctx,
    );
    machine.activate_driver(
      slingshots::RIGHT_COIL.name,
      ActivationMode::Automatic(slingshots::RIGHT_SWITCH.name),
      ctx,
    );
    machine.activate_driver(
      left_flipper::MAIN_COIL.name,
      ActivationMode::Automatic(cabinet::LEFT_FLIPPER_SWITCH1.name),
      ctx,
    );
    machine.activate_driver(
      left_flipper::HOLD_COIL.name,
      ActivationMode::Automatic(cabinet::LEFT_FLIPPER_SWITCH1.name),
      ctx,
    );
    machine.activate_driver(
      right_flipper::MAIN_COIL.name,
      ActivationMode::Automatic(cabinet::RIGHT_FLIPPER_SWITCH1.name),
      ctx,
    );
    machine.activate_driver(
      right_flipper::HOLD_COIL.name,
      ActivationMode::Automatic(cabinet::RIGHT_FLIPPER_SWITCH1.name),
      ctx,
    );
    machine.activate_driver(
      upper_flipper::MAIN_COIL.name,
      ActivationMode::Automatic(cabinet::RIGHT_FLIPPER_SWITCH2.name),
      ctx,
    );
    machine.activate_driver(
      upper_flipper::HOLD_COIL.name,
      ActivationMode::Automatic(cabinet::RIGHT_FLIPPER_SWITCH2.name),
      ctx,
    );

    // temporary for testing
    machine.activate_driver(
      lower_scoop::COIL.name,
      ActivationMode::Automatic(cabinet::action_button::SWITCH.name),
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
