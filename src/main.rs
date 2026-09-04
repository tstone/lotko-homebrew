use frontbox::prelude::Cycle::Forever;
use frontbox::prelude::*;
use frontbox::provided::{AutoPlungerSystem, DoubleFlipSystem, PlungeLaneSystem};
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

    // hardware ops
    app.system(drop_bank::DropBankSystem::new());
    app.system(lower_scoop::LowerScoopSystem::new());
    app.system(lift_ramp::LiftRampSystem::new());
    app.system(left_orbit::LeftOrbitSystem::new());
    app.system(center_orbit::CenterOrbitSystem::new());
    app.system(right_orbit::RightOrbitSystem::new());
    app.system(arc_ramp::ArcRampSystem::new());
    app.system(vspinner::VerticalSpinner::new());
    app.system(DoubleFlipSystem::new(
      cabinet::LEFT_FLIPPER_SWITCH1.q(),
      cabinet::RIGHT_FLIPPER_SWITCH1.q(),
    ));

    // game
    app.system(activate_playfield());
    app.system(EndOfBallSystem::new());
    app.system(GameManager::competitive(
      4,
      systems![
        BasicPoints::new(),
        // operation
        BallSaveSystem::new(Duration::from_secs(8)).effect(LedProgram1d::flash(
          DRAIN_LED.q(),
          ColorSequence::solid(Rgba::green()),
          Forever
        )),
        PlayfieldIllumination::new(),
        SkillshotManager::new(),
        // modes
        ModeManager::new(),
        LeftScoopStartable::new(),
        LiftRampStartable::new(),
        HydroCoreQualification::new(),
        SkyrailStationQualification::new(),
        // NimbusPromenadeQualification::new(),
        SolariumAtriumQualification::new(),
      ],
      SwitchQ::tag::<tags::Playfield>(),
    ));

    // playfield
    app.system(trough::system());
    app.system(PlungeLaneSystem::new(
      plunge_lane::SWITCH.name,
      Duration::from_millis(1200),
    ));
    app.system(AutoPlungerSystem::new(plunge_lane::COIL.name));
    // TODO: action button plunge
  })
  .run()
  .await;
}
