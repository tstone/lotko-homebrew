use frontbox_console::WebTracer;
use frontbox_core::prelude::*;
use frontbox_display::menu::{DmdMenuSystem, MenuSwitches};
use frontbox_display::{DisplayDefinition, DisplaySystem};
use frontbox_dmd::{PanelType, Pin2Dmd};
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
    let ball_save_scene = scene!("ball_save.scene");

    app.plane(app_plane!(planes::PLAYFIELD));
    app.plane(app_plane!(planes::ARC_RAMP));
    app.plane(app_plane!(planes::BACKBOX_PANEL));
    app.plane(app_plane!(planes::BACKBOARD));
    app.plane(app_plane!(planes::CABINET_FRONT));
    app.plane(app_plane!(planes::CABINET_LEFT));
    app.plane(app_plane!(planes::CABINET_RIGHT));

    let console = WebTracer::new();

    // core
    app.system(SceneSystem::new());
    app.system(LedSystem::new());
    let mut sound = SoundSystem::new("Sound Blaster");
    sound.set_callout_volume(9.0);

    app.system(sound);
    app.system(FreePlay::new(start_button::SWITCH.q()));
    app.system(CoinDoorSystem::new(coin_door::OPEN_SWITCH.name));
    app.system(SoundLoaderSystem::new());
    app.system(game_startable());
    app.system(attract_mode_leds());
    app.system(quit_game_combo(vec![
      &cabinet::LEFT_FLIPPER_SWITCH1,
      &start_button::SWITCH,
    ]));
    app.system(cabinet::double_flip_combo());

    // dmd
    app.system(DisplaySystem::new().display(
      DisplayDefinition::new(DMD, Pin2Dmd::new(DMD_WIDTH, DMD_HEIGHT, PanelType::Rgb)).mirror(
        // 320 x 80mm, centered between the speaker rings
        console.display_view().on(&planes::BACKBOX_PANEL, Vec2::new(7.325, 2.425), Vec2::new(12.6, 3.15)),
      ),
    ));
    app.tracer(console);
    app.system(DmdMenuSystem::new(
      MenuSwitches {
        back_btn: coin_door::MENU_GREEN_SWITCH.name,
        select_btn: coin_door::MENU_BLACK_SWITCH.name,
        inc_btn: coin_door::MENU_RED_R_SWITCH.name,
        dec_btn: coin_door::MENU_RED_L_SWITCH.name,
        coin_door: coin_door::OPEN_SWITCH.name,
      },
      &MENU,
    ));
    app.system(
      DmdAttractSystem::new(vec![
        AttractScreen::Scene("dmd/neon_blue.scene"),
        AttractScreen::Scene("dmd/spore.scene"),
        AttractScreen::LastScore(0),
        AttractScreen::LastScore(1),
        AttractScreen::PressStart,
        AttractScreen::LastScore(2),
        AttractScreen::LastScore(3),
        AttractScreen::Scene("dmd/fast_frontbox_logos.scene"),
      ])
      .flippers(LEFT_FLIPPER_SWITCH1.name, RIGHT_FLIPPER_SWITCH1.name)
      .z(BASE_DMD_LAYER),
    );
    app.system(DmdPointsSystem::new().z(BASE_DMD_LAYER));

    // hardware ops
    app.system(drop_bank::DropBankSystem::new());
    app.system(lower_scoop::system());
    app.system(lower_scoop::LowerScoopFeedback);
    app.system(dome_ramp::DomeRampSystem::new());
    app.system(lift_ramp::LiftRampSystem::new());
    app.system(rear_scoop::system());
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
      .ball_present_scene(scene!("plunge_lane.scene"))
      .autoplunger(plunge_lane::COIL.name)
      .launch_button(action_button::SWITCH.q())
      .launch_ready_scene(scene!("launch_ready.scene"))
      .build(),
    );

    // game
    app.routine::<TurnEnding>(routine![EndOfBallSystem, BonusSystem, TurnEndScoreSystem]);
    app.routine::<BallDraining>(routine![BallSaveSystem, OutlaneSaveSystem]);
    app.routine::<GameEnding>(routine![EndOfBallSystem]);

    // ball search
    app.system(
      BallSearchSystem::new(SwitchQ::tag::<tags::Playfield>())
        .hold_switches(SwitchQ::names([
          cabinet::LEFT_FLIPPER_SWITCH1.name,
          cabinet::RIGHT_FLIPPER_SWITCH1.name,
        ]))
        .coils(DriverQ::names([
          pop_cluster::left::COIL.name,
          pop_cluster::upper_right::COIL.name,
          pop_cluster::lower_right::COIL.name,
          slingshots::LEFT_COIL.name,
          slingshots::RIGHT_COIL.name,
        ])),
    );
    app.routine::<BallSearching>(routine![
      ScoopSystem<lower_scoop::Lower>,
      ScoopSystem<rear_scoop::Rear>,
      lift_ramp::LiftRampSystem,
      drop_bank::DropBankSystem,
    ]);

    app.system(EndOfBallSystem::new());
    app.system(GameGreeting::new());
    app.system(TiltPresenter::new());
    app.system(
      GameManager::competitive(
        4,
        systems![
          BasicPoints::new(),
          // operation
          BallSaveSystem::new().scene(ball_save_scene),
          TiltSystem::new(&cabinet::TILT_BOB_SWITCH),
          ExtraBallSystem::new(),
          PlayfieldIllumination::new(),
          SkillshotManager::new(),
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
