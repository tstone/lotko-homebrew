use frontbox_led::LedScene;
use frontbox_pinball::AttractSystem;

static BARS: &str = include_str!("../../assets/attract/bars.scene");
static PINWHEEL: &str = include_str!("../../assets/attract/pinwheel.scene");
static CIRCLE_BURST: &str = include_str!("../../assets/attract/ring-burst.scene");
static HORZ: &str = include_str!("../../assets/attract/horz-wipe.scene");

pub fn attract_mode_leds() -> AttractSystem {
  let scenes = vec![
    LedScene::load(BARS).unwrap(),
    LedScene::load(PINWHEEL).unwrap(),
    LedScene::load(HORZ).unwrap(),
    LedScene::load(CIRCLE_BURST).unwrap(),
  ];
  AttractSystem::new(scenes).unwrap()
}
