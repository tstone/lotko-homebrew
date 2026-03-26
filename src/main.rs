use frontbox::plugins::TroughPlugin;
use frontbox::prelude::*;
use frontbox_turn_based::*;
use std::io::Write;

mod io_network;
use io_network::*;
mod exp_network;
use exp_network::*;
mod systems;
use systems::*;

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
    .plugin(TroughPlugin::new(
      vec![switches::TROUGH_POS1, switches::TROUGH_POS2],
      drivers::TROUGH_EJECT,
    ))
    .plugin(CompetitiveGamePlugin::new(
      4,
      vec![],
      switches::START_BUTTON,
      vec![],
    ))
    .configure(|app| {
      app.system(StartableFlasher::new(
        Some(drivers::START_BUTTON),
        Some(switches::ACTION_BUTTON),
        Some(LedSetting::On(Color::red())),
      ));
      app.system(ActivatePlayfield::new());
      app.system(AutoTurnAdvance::new());
    })
    .run()
    .await;
}
