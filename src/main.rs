use frontbox::prelude::Cycle::Forever;
use frontbox::prelude::*;
use frontbox::provided::{AutoPlungerSystem, PlungeLaneSystem};
use frontbox_pin2dmd::menu::{DmdMenuSystem, DmdMenuTheme, MenuSwitches};
use frontbox_pin2dmd::{DmdSystem, PanelType, Pin2Dmd};
use frontbox_sound::SoundSystem;
use frontbox_turn_based::*;
use frontbox_web_console::WebTracer;
use std::io::Write;

mod systems;
use systems::*;
mod hardware;
pub mod menu;

use hardware::*;

use crate::hardware::cabinet::*;
use crate::hardware::lift_ramp::LiftRampSystem;
use crate::hardware::lower_scoop::LowerScoopSystem;
use crate::hardware::trough::DRAIN_LED;
use crate::menu::MENU;
use crate::systems::dmd::*;
use crate::systems::game::*;
use crate::systems::non_game::*;

#[tokio::main]
async fn main() {
  env_logger::Builder::from_default_env()
    .format(|buf, record| {
      writeln!(
        buf,
        "[{} {}] {}\r",
        buf.timestamp_millis(),
        record.level(),
        record.args()
      )
    })
    .init();

  App::boot(BootConfig {
    io_network: io_network(),
    exp_network: exp_network(),
    system_interval: Duration::from_millis(67),
    ..Default::default()
  })
  .await
  .configure(|app| {
    app.tracer(WebTracer::new());

    // core
    app.system(LedSystem::new());
    app.system(SoundSystem::by_name("Sound Blaster").expect("Could not initialize SoundSystem"));
    app.system(OperatorConfig::new());
    app.system(FreePlay::new(start_button::SWITCH.q()));
    app.system(SoundLoaderSystem::new());
    app.system(game_startable());
    app.system(AttractModeLedsSystem::new());
    app.system(StartupEject::new());

    // dmd
    let dmd = Pin2Dmd::connect(128, 32, PanelType::Rgb).unwrap();
    app.system(DmdSystem::new(dmd));
    app.system(DmdMenuSystem::new(
      MenuSwitches {
        back_btn: coin_door::MENU_GREEN_SWITCH.name,
        select_btn: coin_door::MENU_BLACK_SWITCH.name,
        inc_btn: coin_door::MENU_RED_R_SWITCH.name,
        dec_btn: coin_door::MENU_RED_L_SWITCH.name,
        coin_door: coin_door::OPEN_SWITCH.name,
      },
      &MENU,
      DmdMenuTheme::default(),
    ));
    app.system(AttractModeDmdSystem::new());
    app.system(GamePointsDmdSystem::new());

    // game
    app.system(activate_playfield());
    app.system(GameManager::competitive(
      4,
      systems![
        BasicPoints::new(),
        // TODO: move hardware management to root level, but make only active when in game
        LowerScoopSystem::new(),
        LiftRampSystem::new(),
        left_orbit::LeftOrbitSystem::new(),
        center_orbit::CenterOrbitSystem::new(),
        right_orbit::RightOrbitSystem::new(),
        BallSaveSystem::new(Duration::from_secs(5)).effect(LedEffect::flash_on_off(
          DRAIN_LED.q(),
          Rgba::green(),
          Duration::from_millis(185),
          Forever
        )),
        CityCoverageQualification2::new(false, false, true),
      ],
      Q::tag::<tags::Playfield>(),
    ));

    // playfield
    app.system(trough::system());
    app.system(PlungeLaneSystem::new(
      plunge_lane::SWITCH.name,
      Duration::from_millis(1200),
    ));
    app.system(AutoPlungerSystem::new(plunge_lane::COIL.name));
    // TODO: action button plunge

    // temporary stuff
    app.system(AutoTurnAdvance::new());
  })
  .run()
  .await;
}
