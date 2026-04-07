use frontbox::plugins::{AutoplungerPlugin, TroughPlugin};
use frontbox::prelude::*;
use frontbox_sound::SoundSystem;
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

  App::boot("/dev/ttyACM0", "/dev/ttyACM1", io_network(), exp_network())
    .await
    .plugin(TroughPlugin::new(
      vec![switches::TROUGH_POS1, switches::TROUGH_POS2],
      drivers::TROUGH_EJECT,
    ))
    .plugin(AutoplungerPlugin::new(
      HardwareSelection::Name(switches::PLUNGE_LANE),
      HardwareSelection::Name(drivers::AUTO_PLUNGER),
    ))
    .plugin(CompetitiveGamePlugin::new(systems![BasicPoints::new()]))
    .configure(|app| {
      app.system(LedSystem::new());
      app.system(StartableFlasher::new());
      app.system(ActivatePlayfield::new());
      app.system(AutoTurnAdvance::new());

      // TODO: this no worky
      // app.system(SoundSystem::by_name("Sound Blaster").expect("Could not initialize SoundSystem"));
      app.system(Testing);
      // app.system(DmdDisplay::default());
    })
    .run()
    .await;
}

pub struct Testing;

impl System for Testing {
  fn on_startup(&mut self, ctx: &Context, systems: &Systems) {
    ctx.cue(Anonymous, Cue::Once(Duration::from_secs(1)));
    systems.expect_mut::<LedSystem>().declare(
      ctx.current_system_id(),
      named_led(ctx, leds::ACTION_BUTTON).color(Color::alice_blue()),
    );
  }

  fn on_event(&mut self, event: &dyn Event, _ctx: &Context, systems: &Systems) {
    if event.is::<Anonymous>() {
      // systems
      //   .expect_mut::<SoundSystem>()
      //   .play_sfx("/usr/share/sounds/alsa/Front_Center.wav");
    }
  }
}
