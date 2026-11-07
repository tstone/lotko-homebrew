use frontbox_gfx::scene::file;
use frontbox_pinball::AttractSystem;

use crate::hardware::planes;

static BARS: &str = include_str!("../../assets/attract/bars.ron");
static PINWHEEL: &str = include_str!("../../assets/attract/pinwheel.ron");

pub fn attract_mode_leds() -> AttractSystem {
  let scenes = vec![file::load(BARS).unwrap(), file::load(PINWHEEL).unwrap()];
  AttractSystem::new(&planes::PLAYFIELD, scenes).unwrap()
}
