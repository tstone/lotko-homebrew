use frontbox_pinball::AttractSystem;
use frontbox_scene::Scene;

static BARS: &str = include_str!("../../assets/attract/bars.scene");
static PINWHEEL: &str = include_str!("../../assets/attract/pinwheel.scene");
static CIRCLE_BURST: &str = include_str!("../../assets/attract/ring-burst.scene");
static HORZ: &str = include_str!("../../assets/attract/horz-wipe.scene");
static GRAPH_BURST: &str = include_str!("../../assets/attract/graph-burst.scene");
static SHADOW: &str = include_str!("../../assets/attract/path.scene");

pub fn attract_mode_leds() -> AttractSystem {
  let scenes = [BARS, GRAPH_BURST, PINWHEEL, HORZ, SHADOW, CIRCLE_BURST]
    .map(|scene| Scene::load(scene).unwrap())
    .into();
  AttractSystem::new(scenes)
}
