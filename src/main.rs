use frontbox_console::WebTracer;
use frontbox_core::prelude::*;
use frontbox_dmd::menu::{DmdMenuSystem, DmdMenuTheme, MenuSwitches};
use frontbox_dmd::{DmdSystem, PanelType, Pin2Dmd};
use frontbox_led::prelude::*;
use frontbox_pinball::*;
use frontbox_sound::SoundSystem;
use std::io::Write;

#[macro_use]
mod led_scenes;
mod systems;
use systems::*;
mod hardware;
pub mod menu;
mod metrics;

use hardware::*;

use crate::hardware::cabinet::*;
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
    // fixed on the machine so release and debug builds share settings, wherever each was built
    config_path: Some(if virtual_mode {
      concat!(env!("CARGO_MANIFEST_DIR"), "/operator_config.toml").into()
    } else {
      "/userdata/home/armsom/operator_config.toml".into()
    }),
    metrics_path: Some(if virtual_mode {
      concat!(env!("CARGO_MANIFEST_DIR"), "/metrics.db").into()
    } else {
      "/userdata/home/armsom/metrics.db".into()
    }),
    content_path: Some(if virtual_mode {
      concat!(env!("CARGO_MANIFEST_DIR"), "/src/assets").into()
    } else {
      "/userdata/home/armsom/assets".into()
    }),
    ..Default::default()
  })
  .configure(|app| {
    let ball_save_effect = led_playback!("ball_save.scene");

    app.plane(app_plane!(planes::PLAYFIELD));
    app.plane(app_plane!(planes::ARC_RAMP));
    app.plane(app_plane!(planes::BACKBOX_PANEL));
    app.plane(app_plane!(planes::BACKBOARD));
    app.plane(app_plane!(planes::CABINET_FRONT));

    app.tracer(WebTracer::new());

    // core
    app.system(LedSystem::new());
    let mut sound = SoundSystem::by_name_or_default("Sound Blaster");
    sound.set_callout_volume(9.0);

    app.system(sound);
    app.system(FreePlay::new(start_button::SWITCH.q()));
    app.system(SoundLoaderSystem::new());
    app.system(game_startable());
    app.system(attract_mode_leds());
    app.system(StartupEject::new());
    app.system(quit_game_combo(vec![
      &cabinet::LEFT_FLIPPER_SWITCH1,
      &start_button::SWITCH,
    ]));
    app.system(cabinet::double_flip_combo());

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
    app.system(main_shot::ShotSystem::new());
    app.system(left_outlane::LeftOutlaneSystem::new());
    app.system(right_outlane::RightOutlaneSystem::new());
    app.system(right_pass_lane::RightPassLaneSystem::new());
    app.system(slingshots::SlingshotSystem::new());
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
      .ball_present_effect(led_playback!("plunge_lane.scene"))
      .autoplunger(plunge_lane::COIL.name)
      .launch_button(action_button::SWITCH.q())
      .launch_ready_effect(led_playback!("launch_ready.scene"))
      .build(),
    );

    // game
    app.routine::<TurnEnding>(routine![EndOfBallSystem, BonusSystem, TurnEndScoreSystem]);
    app.routine::<BallDraining>(routine![BallSaveSystem, OutlaneSaveSystem]);
    app.routine::<GameEnding>(routine![EndOfBallSystem]);

    app.system(EndOfBallSystem::new());
    app.system(GameGreeting::new());
    app.system(TiltPresenter::new());
    app.system(
      GameManager::competitive(
        4,
        systems![
          BasicPoints::new(),
          // operation
          BallSaveSystem::new().led_effect(ball_save_effect),
          TiltSystem::new(&cabinet::TILT_BOB_SWITCH),
          ExtraBallSystem::new(),
          PlayfieldIllumination::new(),
          SkillshotManager::new(),
          EffectsSystem::new(),
          // modes
          ModeManager::new(),
          SporesSystem::new(),
          CenterSpinnerScoring::new(),
          BonusSystem::new(),
          TurnEndScoreSystem::new(),
          ScoopQualifier::new(),
          ModeSelectSystem::new(),
          // side quests
          HeartwoodCharge::new(),
          SurveillanceSystem::new(),
          SentinelsSystem::new(),
          SpreadRootsSystem::new(),
          ComboSystem::new(),
          ArcStreakSystem::new(),
          OutlaneSaveSystem::new(),
        ],
        SwitchQ::tag::<tags::Playfield>(),
      )
      .playfield_drivers(DriverQ::tag::<tags::Playfield>()),
    );
  })
  .run()
  .await;
}
