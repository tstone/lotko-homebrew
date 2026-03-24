use frontbox::prelude::*;
use frontbox::prebuilt::*;
use frontbox_turn_based::*;
use std::io::Write;

mod io_network;
use io_network::*;
mod exp_network;
use exp_network::*;

// Tween::new(
//   Duration::from_millis(1000),
//   Curve::Linear,
//   vec![Color::purple(), Color::blue(), Color::yellow()],
//   AnimationCycle::Forever,
// )

#[tokio::main]
async fn main() {
  env_logger::Builder::from_default_env()
    .format(|buf, record| writeln!(buf, "[{}] {}\r", record.level(), record.args()))
    .init();

  App::boot(BootConfig::default(), io_network(), exp_network())
    .await
    .systems(bundles::operational())
    .run(vec![
      FreePlay::new(switches::START_BUTTON), 
      StartableFlasher::new(Some(drivers::START_BUTTON), Some(switches::ACTION_BUTTON), Some(LedSetting::On(Color::red()))),
      TroughSystem::new(
        vec![switches::TROUGH_POS1, switches::TROUGH_POS2],
        drivers::TROUGH_EJECT
      ),
      AutoPlunger::new(switches::PLUNGE_LANE, drivers::AUTO_PLUNGER),
      ActionButtonEject::new(switches::ACTION_BUTTON, switches::PLUNGE_LANE, LedSetting::On(Color::aquamarine())),
      IndividualPlayerSystem::new(
        4, 
        switch_groups::BALL_IN_PLAY, 
        vec![]
      ),
      Testing::new(),
    ])
    .await;
}

#[derive(Clone)]
struct Testing;

impl Testing {
  fn new() -> Box<Self> {
    Box::new(Self)
  }

  fn activate_playfield_drivers(&self, ctx: &mut Context) {
    ctx.command(ActivateDriver::new(drivers::SLINGSHOT_LEFT, ActivationMode::Automatic(switches::SLINGSHOT_LEFT)));
    ctx.command(ActivateDriver::new(drivers::SLINGSHOT_RIGHT, ActivationMode::Automatic(switches::SLINGSHOT_RIGHT)));
    
    ctx.command(ActivateDriver::new(drivers::POP_RIGHT, ActivationMode::Automatic(switches::POP_RIGHT)));
    ctx.command(ActivateDriver::new(drivers::POP_LEFT, ActivationMode::Automatic(switches::POP_LEFT)));

    ctx.command(ActivateDriver::new(drivers::FLIPPER_MAIN_LEFT, ActivationMode::Automatic(switches::LEFT_FLIPPER1)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_MAIN_HOLD_LEFT, ActivationMode::Automatic(switches::LEFT_FLIPPER1)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_MAIN_RIGHT, ActivationMode::Automatic(switches::RIGHT_FLIPPER1)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_MAIN_HOLD_RIGHT, ActivationMode::Automatic(switches::RIGHT_FLIPPER1)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_UPPER_LEFT, ActivationMode::Automatic(switches::LEFT_FLIPPER2)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_UPPER_HOLD_LEFT, ActivationMode::Automatic(switches::LEFT_FLIPPER2)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_UPPER_RIGHT, ActivationMode::Automatic(switches::RIGHT_FLIPPER2)));
    ctx.command(ActivateDriver::new(drivers::FLIPPER_UPPER_HOLD_RIGHT, ActivationMode::Automatic(switches::RIGHT_FLIPPER2)));
  }
}

impl System for Testing {
  fn on_event(&mut self, event: &dyn Signal, ctx: &mut Context) {
    if let Some(_) = event.downcast_ref::<GameStarted>() {
      self.activate_playfield_drivers(ctx);
    } else if let Some(_) = event.downcast_ref::<PlayerTurnEnding>() {
      ctx.command(AdvanceTurn);
    }
  }
}