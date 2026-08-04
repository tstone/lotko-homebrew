use frontbox::prelude::*;
use frontbox_sound::*;

use crate::hardware::lower_scoop::LOWER_SCOOP_EJECT_SND;

pub mod sounds {
  use std::sync::LazyLock;

  use rand::prelude::IndexedRandom;

  pub const LANE_HIT1: &str = "lane_hit1";
  pub const LANE_HIT2: &str = "lane_hit2";
  pub const LANE_HIT3: &str = "lane_hit3";
  pub const LANE_HIT4: &str = "lane_hit4";
  pub const LANE_HIT_COMPLETE: &str = "lane_hit_complete";

  pub const LANE_HITS: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| vec![LANE_HIT1, LANE_HIT2, LANE_HIT3, LANE_HIT4]);
  pub fn rnd_lane_hit() -> &'static str {
    LANE_HITS.sample(&mut rand::rng(), 1).next().unwrap()
  }
}

pub mod sounds_bytes {
  pub static LOWER_SCOOP_EJECT: &[u8] = include_bytes!("../assets/sounds/lower-scoop-exit.mp3");
  pub static LANE_HIT1: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-1.mp3");
  pub static LANE_HIT2: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-2.mp3");
  pub static LANE_HIT3: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-3.mp3");
  pub static LANE_HIT4: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-4.mp3");
  pub static LANE_HIT_COMPLETE: &[u8] = include_bytes!("../assets/sounds/lane-hit-complete.mp3");
}

impl System for SoundLoaderSystem {
  fn on_spawn(&mut self, ctx: &Context) {
    let mut snd = ctx.systems.expect::<SoundSystem>();

    snd.preload_embedded(LOWER_SCOOP_EJECT_SND, sounds_bytes::LOWER_SCOOP_EJECT);
    snd.preload_embedded(sounds::LANE_HIT1, sounds_bytes::LANE_HIT1);
    snd.preload_embedded(sounds::LANE_HIT2, sounds_bytes::LANE_HIT2);
    snd.preload_embedded(sounds::LANE_HIT3, sounds_bytes::LANE_HIT3);
    snd.preload_embedded(sounds::LANE_HIT4, sounds_bytes::LANE_HIT4);
    snd.preload_embedded(sounds::LANE_HIT_COMPLETE, sounds_bytes::LANE_HIT_COMPLETE);

    // We're done. Sounds will remain preloaded.
    ctx.despawn_self();
  }
}

pub struct SoundLoaderSystem;

impl SoundLoaderSystem {
  pub fn new() -> Self {
    Self
  }
}
