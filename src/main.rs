use frontbox::prelude::*;
use frontbox_pin2dmd::menu::{DmdMenuSystem, DmdMenuTheme, MenuSwitches};
use frontbox_pin2dmd::{DmdSystem, PanelType, Pin2Dmd};
use frontbox_sound::SoundSystem;
use frontbox_turn_based::*;
use std::io::Write;

mod systems;
use systems::*;
mod hardware;
pub mod menu;

use hardware::*;

use crate::hardware::cabinet::*;
use crate::hardware::lower_scoop::LowerScoopSystem;
use crate::hardware::trough::DRAIN_LED;
use crate::menu::MENU;
use crate::systems::non_game::game_startable;

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
    ..Default::default()
  })
  .await
  .configure(|app| {
    // core
    app.system(LedSystem::new());
    app.system(SoundSystem::by_name("Sound Blaster").expect("Could not initialize SoundSystem"));
    app.system(OperatorConfig::new());
    app.system(FreePlay::default());
    app.system(game_startable());

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

    // game
    app.system(
      ActivatePlayfieldSystem::new()
        // slings
        .driver(slingshots::LEFT_COIL.name, slingshots::LEFT_SWITCH.name)
        .driver(slingshots::RIGHT_COIL.name, slingshots::RIGHT_SWITCH.name)
        // flippers
        .driver(
          left_flipper::MAIN_COIL.name,
          cabinet::LEFT_FLIPPER_SWITCH1.name,
        )
        .driver(
          left_flipper::HOLD_COIL.name,
          cabinet::LEFT_FLIPPER_SWITCH1.name,
        )
        .driver(
          right_flipper::MAIN_COIL.name,
          cabinet::RIGHT_FLIPPER_SWITCH1.name,
        )
        .driver(
          right_flipper::HOLD_COIL.name,
          cabinet::RIGHT_FLIPPER_SWITCH1.name,
        )
        .driver(
          upper_flipper::MAIN_COIL.name,
          cabinet::RIGHT_FLIPPER_SWITCH2.name,
        )
        .driver(
          upper_flipper::HOLD_COIL.name,
          cabinet::RIGHT_FLIPPER_SWITCH2.name,
        )
        // pops
        .driver(
          pop_cluster::lower_right::COIL.name,
          pop_cluster::lower_right::SPOON_SWITCH.name,
        )
        .driver(
          pop_cluster::upper_right::COIL.name,
          pop_cluster::upper_right::SPOON_SWITCH.name,
        )
        .driver(
          pop_cluster::left::COIL.name,
          pop_cluster::left::SPOON_SWITCH.name,
        ),
    );

    app.system(GameManager::competitive(
      4,
      systems![
        BasicPoints::new(),
        LowerScoopSystem::new(),
        BallSaveSystem::new(Duration::from_secs(4)).effect(LedEffect::flash(
          DRAIN_LED.q(),
          Rgba::green(),
          Duration::from_millis(185)
        ))
      ],
      Q::tag::<tags::Playfield>(),
    ));

    // playfield
    app.system(trough::system());
    app.system(plunge_lane::system());

    // temporary stuff
    app.system(AutoTurnAdvance::new());
  })
  .run()
  .await;
}
