use frontbox::animation::Curve;
use frontbox::prelude::Cycle::Forever;
use frontbox::prelude::*;
use frontbox_pin_console::{WebTracer, console_plane};
use frontbox_pin2dmd::menu::{DmdMenuSystem, DmdMenuTheme, MenuSwitches};
use frontbox_pin2dmd::{DmdSystem, PanelType, Pin2Dmd};
use frontbox_pinball::*;
use frontbox_sound::SoundSystem;
use std::io::Write;

mod systems;
use systems::*;
mod hardware;
pub mod menu;

use hardware::*;

use crate::hardware::cabinet::*;
use crate::hardware::flashers::FlashersSystem;
use crate::hardware::trough::DRAIN_LED;
use crate::menu::MENU;
use crate::systems::dmd::*;
use crate::systems::game::*;
use crate::systems::non_game::*;

#[tokio::main]
async fn main() {
  let virtual_mode = std::env::var("VIRTUAL").is_ok();

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

  App::new(BootConfig {
    platform: if virtual_mode {
      Platform::Virtual
    } else {
      Platform::Neuron {
        io_net_port_path: "/dev/ttyACM0",
        exp_port_path: "/dev/ttyACM1",
        watchdog_interval: Duration::from_millis(1500),
      }
    },
    io_network: io_network(),
    exp_network: exp_network(),
    system_interval: Duration::from_millis(67),
    ..Default::default()
  })
  .configure(|app| {
    let ball_save_effect =
      LedProgram1d::flash(DRAIN_LED.q(), ColorSequence::solid(Rgba::green()), Forever);

    app.tracer(
      WebTracer::new()
        .plane(console_plane!(planes::PLAYFIELD))
        .plane(console_plane!(planes::ARC_RAMP))
        .plane(console_plane!(planes::BACKBOX_PANEL))
        .plane(console_plane!(planes::BACKBOARD))
        .plane(console_plane!(planes::CABINET_FRONT)),
    );

    // core
    app.system(LedSystem::new());
    app.system(SoundSystem::by_name_or_default("Sound Blaster"));
    app.system(OperatorConfig::default());
    app.system(FreePlay::new(start_button::SWITCH.q()));
    app.system(SoundLoaderSystem::new());
    app.system(game_startable());
    app.system(AttractModeLedsSystem::new());
    app.system(StartupEject::new());
    app.system(quit_game_combo(vec![
      &cabinet::LEFT_FLIPPER_SWITCH1,
      &start_button::SWITCH,
    ]));

    // dmd
    if !virtual_mode {
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
    }

    // hardware ops
    app.system(drop_bank::DropBankSystem::new());
    app.system(lower_scoop::LowerScoopSystem::new());
    app.system(dome_ramp::DomeRampSystem::new());
    app.system(lift_ramp::LiftRampSystem::new());
    app.system(left_orbit::LeftOrbitSystem::new());
    app.system(center_orbit::CenterOrbitSystem::new());
    app.system(right_orbit::RightOrbitSystem::new());
    app.system(arc_ramp::ArcRampSystem::new());
    app.system(vspinner::VerticalSpinner::new());
    app.system(captive_ball::CaptiveBallSystem::new());
    app.system(
      BallManager::standard(
        [
          trough::SWITCH1.name,
          trough::SWITCH2.name,
          trough::SWITCH3.name,
          trough::SWITCH4.name,
          trough::SWITCH5.name,
          // trough::SWITCH6.name, -- off by one ala opto
        ],
        trough::COIL.name,
      )
      .plunge_lane(plunge_lane::SWITCH.name)
      .re_enter_timeout(Duration::from_millis(1200))
      .ball_present_effect(LedProgram1d::rotating(
        plunge_lane::LED_STRIP.q(),
        ColorSequence::exact(vec![
          Rgba::purple(),
          Rgba::purple().lighten(0.35),
          Rgba::default(),
          Rgba::default(),
        ]),
        Duration::from_millis(900),
        Curve::Linear,
        Cycle::Forever,
      ))
      .autoplunger(plunge_lane::COIL.name)
      .launch_button(action_button::SWITCH.q())
      .launch_ready_effect(LedProgram1d::pulse(
        action_button::LED.q(),
        Rgba::magenta(),
        Duration::from_millis(1250),
        Cycle::Forever,
      ))
      .build(),
    );

    // game
    app.system(activate_playfield());
    app.system(EndOfBallSystem::new());
    app.system(GameManager::competitive(
      4,
      systems![
        BasicPoints::new(),
        // operation
        BallSaveSystem::new()
          .on_turn_start(Duration::from_secs(8))
          .led_effect(ball_save_effect),
        PlayfieldIllumination::new(),
        SkillshotManager::new(),
        FlashersSystem::new(),
        // modes
        ModeManager::new(),
        LeftScoopStartable::new(),
        LiftRampStartable::new(),
        // exclusive
        HydroCoreQualification::new(),
        SkyrailStationQualification::new(),
        SolariumAtriumQualification::new(),
        MeridianBasinsQualification::new(),
        SporeMultiballQualification::new(),
        // non-exclusive
        NimbusPromenadeQualification::new(),
        ApexTerracesMode::new(),
      ],
      SwitchQ::tag::<tags::Playfield>(),
    ));
  })
  .run()
  .await;
}
