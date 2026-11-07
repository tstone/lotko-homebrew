use frontbox_gfx::scene::file;
use frontbox_pinball::AttractSystem;

use crate::hardware::planes;

static BARS: &str = include_str!("../../assets/attract/bars.ron");
static PINWHEEL: &str = include_str!("../../assets/attract/pinwheel.ron");
static CIRCLE_BURST: &str = include_str!("../../assets/attract/ring-burst.ron");
static HORZ: &str = include_str!("../../assets/attract/horz-wipe.ron");

pub fn attract_mode_leds() -> AttractSystem {
  let scenes = vec![
    file::load(BARS).unwrap(),
    file::load(PINWHEEL).unwrap(),
    file::load(HORZ).unwrap(),
    file::load(CIRCLE_BURST).unwrap(),
  ];
  AttractSystem::new(&planes::PLAYFIELD, scenes).unwrap()
}
