use frontbox_core::prelude::*;
use frontbox_sound::*;

use crate::hardware::center_orbit::CENTER_SPINNER_CLANG;
use crate::hardware::lower_scoop::LOWER_SCOOP_EJECT;
use crate::systems::Callout;

pub mod sounds {
  use std::sync::LazyLock;

  use rand::prelude::IndexedRandom;

  pub const LANE_HIT1: &str = "lane_hit1";
  pub const LANE_HIT2: &str = "lane_hit2";
  pub const LANE_HIT3: &str = "lane_hit3";
  pub const LANE_HIT4: &str = "lane_hit4";
  pub const LANE_HIT_COMPLETE: &str = "lane_hit_complete";
  pub const ARP_HIT1: &str = "arp_hit1";
  pub const EVOLUTION_COMPLETE: &str = "evolution_complete";
  pub const COUNTING: &str = "counting";
  pub const QUEST_COMPLETE: &str = "quest_complete";

  pub const HIT_ORGANIC1: &str = "target_hit_organic1";
  pub const HIT_ORGANIC_LOW1: &str = "target_hit_organic_down1";
  pub const HIT_ORGANIC2: &str = "target_hit_organic2";
  pub const HIT_ORGANIC_LOW2: &str = "target_hit_organic_down2";
  pub const HIT_ORGANIC3: &str = "target_hit_organic3";
  pub const HIT_ORGANIC_LOW3: &str = "target_hit_organic_down3";
  pub const HIT_ORGANIC4: &str = "target_hit_organic4";
  pub const HIT_ORGANIC_LOW4: &str = "target_hit_organic_down4";

  pub const STEWARD_CHIME: &str = "steward_chime";
  pub const BRAAM_BIO1: &str = "braam_bio1";
  pub const BRAAM_BIO2: &str = "braam_bio2";
  pub const HUM: &str = "hum";
  pub const IMPACT_CINEMATIC_HEAVY: &str = "impact_cinematic_heavy";
  pub const IMPACT_DIRT: &str = "impact_dirt";
  pub const LATCH_DOUBLE: &str = "latch_double";
  pub const LATCH_HEAVY1: &str = "latch_heavy1";
  pub const LATCH_HEAVY2: &str = "latch_heavy2";
  pub const LATCH_HEAVY3: &str = "latch_heavy3";
  pub const MOTOR1: &str = "motor1";
  pub const MOTOR2: &str = "motor2";
  pub const MOTOR3: &str = "motor3";
  pub const MOTOR4: &str = "motor4";
  pub const MOTOR5: &str = "motor5";
  pub const SCAN_HEAVY_STUTTER: &str = "scan_heavy_stutter";
  pub const SCAN_LIGHT: &str = "scan_light";
  pub const SCAN_LIGHT2: &str = "scan_light2";
  pub const SCAN_MED: &str = "scan_med";
  pub const WOOSH_LIGHT: &str = "woosh_light";

  pub const LANE_HITS: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| vec![LANE_HIT1, LANE_HIT2, LANE_HIT3, LANE_HIT4]);
  pub fn rnd_lane_hit() -> &'static str {
    LANE_HITS.sample(&mut rand::rng(), 1).next().unwrap()
  }

  /// Hits go from the city taking damage to the forest winning, stand-ins until the palette exists
  const CITY_HITS: [&str; 6] = [
    LATCH_HEAVY1,
    LATCH_HEAVY2,
    HIT_ORGANIC2,
    HIT_ORGANIC3,
    HIT_ORGANIC4,
    IMPACT_DIRT,
  ];

  /// The hit for how far through a mode the player is, from 0.0 to 1.0
  pub fn city_hit(progress: f32) -> &'static str {
    let last = CITY_HITS.len() - 1;
    CITY_HITS[(progress.clamp(0.0, 1.0) * last as f32).round() as usize]
  }

  pub fn rnd_latch() -> &'static str {
    [LATCH_HEAVY1, LATCH_HEAVY2, LATCH_HEAVY3]
      .choose(&mut rand::rng())
      .unwrap()
  }
}

pub mod sounds_bytes {
  pub static LOWER_SCOOP_EJECT: &[u8] = include_bytes!("../assets/sounds/lower-scoop-exit.mp3");
  pub static LANE_HIT1: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-1.mp3");
  pub static LANE_HIT2: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-2.mp3");
  pub static LANE_HIT3: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-3.mp3");
  pub static LANE_HIT4: &[u8] = include_bytes!("../assets/sounds/lane-hit-rattle-4.mp3");
  pub static LANE_HIT_COMPLETE: &[u8] = include_bytes!("../assets/sounds/lane-hit-complete.mp3");

  pub static HIT_ORGANIC1: &[u8] = include_bytes!("../assets/sounds/target-hit-organic-1.mp3");
  pub static HIT_ORGANIC2: &[u8] = include_bytes!("../assets/sounds/target-hit-organic-2.mp3");
  pub static HIT_ORGANIC3: &[u8] = include_bytes!("../assets/sounds/target-hit-organic-3.mp3");
  pub static HIT_ORGANIC4: &[u8] = include_bytes!("../assets/sounds/target-hit-organic-4.mp3");
  pub static HIT_ORGANIC_LOW1: &[u8] =
    include_bytes!("../assets/sounds/target-hit-organic-1-dwn-oct.mp3");
  pub static HIT_ORGANIC_LOW2: &[u8] =
    include_bytes!("../assets/sounds/target-hit-organic-2-dwn-oct.mp3");
  pub static HIT_ORGANIC_LOW3: &[u8] =
    include_bytes!("../assets/sounds/target-hit-organic-3-dwn-oct.mp3");
  pub static HIT_ORGANIC_LOW4: &[u8] =
    include_bytes!("../assets/sounds/target-hit-organic-4-dwn-oct.mp3");

  pub static ARP_HIT1: &[u8] = include_bytes!("../assets/sounds/arp-hit1.mp3");
  pub static EVOLUTION_COMPLETE: &[u8] =
    include_bytes!("../assets/sounds/freesound_gamestudio-evolution-complete-384926.mp3");
  pub static COUNTING: &[u8] = include_bytes!("../assets/sounds/counting-518133.mp3");
  pub static QUEST_COMPLETE: &[u8] =
    include_bytes!("../assets/sounds/yodguard-quest-complete-1-540192.mp3");
  pub static CENTER_SPINNER_CLANG: &[u8] =
    include_bytes!("../assets/sounds/freesound_community-metal-clang-sound-81634.mp3");

  pub static STEWARD_CHIME: &[u8] = include_bytes!("../assets/sfx/steward-chime.mp3");
  pub static BRAAM_BIO1: &[u8] = include_bytes!("../assets/sfx/braam-bio1.mp3");
  pub static BRAAM_BIO2: &[u8] = include_bytes!("../assets/sfx/braam-bio2.mp3");
  pub static HUM: &[u8] = include_bytes!("../assets/sfx/hum.mp3");
  pub static IMPACT_CINEMATIC_HEAVY: &[u8] =
    include_bytes!("../assets/sfx/impact-cinematic-heavy.mp3");
  pub static IMPACT_DIRT: &[u8] = include_bytes!("../assets/sfx/impact-dirt.mp3");
  pub static LATCH_DOUBLE: &[u8] = include_bytes!("../assets/sfx/latch-double.mp3");
  pub static LATCH_HEAVY1: &[u8] = include_bytes!("../assets/sfx/latch-heavy1.mp3");
  pub static LATCH_HEAVY2: &[u8] = include_bytes!("../assets/sfx/latch-heavy2.mp3");
  pub static LATCH_HEAVY3: &[u8] = include_bytes!("../assets/sfx/latch-heavy3.mp3");
  pub static MOTOR1: &[u8] = include_bytes!("../assets/sfx/motor1.mp3");
  pub static MOTOR2: &[u8] = include_bytes!("../assets/sfx/motor2.mp3");
  pub static MOTOR3: &[u8] = include_bytes!("../assets/sfx/motor3.mp3");
  pub static MOTOR4: &[u8] = include_bytes!("../assets/sfx/motor4.mp3");
  pub static MOTOR5: &[u8] = include_bytes!("../assets/sfx/motor5.mp3");
  pub static SCAN_HEAVY_STUTTER: &[u8] = include_bytes!("../assets/sfx/scan-heavy-stutter.mp3");
  pub static SCAN_LIGHT: &[u8] = include_bytes!("../assets/sfx/scan-light.mp3");
  pub static SCAN_LIGHT2: &[u8] = include_bytes!("../assets/sfx/scan-light2.mp3");
  pub static SCAN_MED: &[u8] = include_bytes!("../assets/sfx/scan-med.mp3");
  pub static WOOSH_LIGHT: &[u8] = include_bytes!("../assets/sfx/woosh-light.mp3");
}

impl System for SoundLoaderSystem {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    let mut snd = ctx.systems.expect::<SoundSystem>();

    snd.preload_embedded(LOWER_SCOOP_EJECT, sounds_bytes::LOWER_SCOOP_EJECT);
    snd.preload_embedded(sounds::LANE_HIT1, sounds_bytes::LANE_HIT1);
    snd.preload_embedded(sounds::LANE_HIT2, sounds_bytes::LANE_HIT2);
    snd.preload_embedded(sounds::LANE_HIT3, sounds_bytes::LANE_HIT3);
    snd.preload_embedded(sounds::LANE_HIT4, sounds_bytes::LANE_HIT4);
    snd.preload_embedded(sounds::LANE_HIT_COMPLETE, sounds_bytes::LANE_HIT_COMPLETE);

    snd.preload_embedded(sounds::HIT_ORGANIC1, sounds_bytes::HIT_ORGANIC1);
    snd.preload_embedded(sounds::HIT_ORGANIC2, sounds_bytes::HIT_ORGANIC2);
    snd.preload_embedded(sounds::HIT_ORGANIC3, sounds_bytes::HIT_ORGANIC3);
    snd.preload_embedded(sounds::HIT_ORGANIC4, sounds_bytes::HIT_ORGANIC4);
    snd.preload_embedded(sounds::HIT_ORGANIC_LOW1, sounds_bytes::HIT_ORGANIC_LOW1);
    snd.preload_embedded(sounds::HIT_ORGANIC_LOW2, sounds_bytes::HIT_ORGANIC_LOW2);
    snd.preload_embedded(sounds::HIT_ORGANIC_LOW3, sounds_bytes::HIT_ORGANIC_LOW3);
    snd.preload_embedded(sounds::HIT_ORGANIC_LOW4, sounds_bytes::HIT_ORGANIC_LOW4);

    snd.preload_embedded(sounds::ARP_HIT1, sounds_bytes::ARP_HIT1);
    snd.preload_embedded(sounds::EVOLUTION_COMPLETE, sounds_bytes::EVOLUTION_COMPLETE);
    snd.preload_embedded(sounds::COUNTING, sounds_bytes::COUNTING);
    snd.preload_embedded(sounds::QUEST_COMPLETE, sounds_bytes::QUEST_COMPLETE);
    snd.preload_embedded(CENTER_SPINNER_CLANG, sounds_bytes::CENTER_SPINNER_CLANG);

    snd.preload_embedded(sounds::STEWARD_CHIME, sounds_bytes::STEWARD_CHIME);
    snd.preload_embedded(sounds::BRAAM_BIO1, sounds_bytes::BRAAM_BIO1);
    snd.preload_embedded(sounds::BRAAM_BIO2, sounds_bytes::BRAAM_BIO2);
    snd.preload_embedded(sounds::HUM, sounds_bytes::HUM);
    snd.preload_embedded(
      sounds::IMPACT_CINEMATIC_HEAVY,
      sounds_bytes::IMPACT_CINEMATIC_HEAVY,
    );
    snd.preload_embedded(sounds::IMPACT_DIRT, sounds_bytes::IMPACT_DIRT);
    snd.preload_embedded(sounds::LATCH_DOUBLE, sounds_bytes::LATCH_DOUBLE);
    snd.preload_embedded(sounds::LATCH_HEAVY1, sounds_bytes::LATCH_HEAVY1);
    snd.preload_embedded(sounds::LATCH_HEAVY2, sounds_bytes::LATCH_HEAVY2);
    snd.preload_embedded(sounds::LATCH_HEAVY3, sounds_bytes::LATCH_HEAVY3);
    snd.preload_embedded(sounds::MOTOR1, sounds_bytes::MOTOR1);
    snd.preload_embedded(sounds::MOTOR2, sounds_bytes::MOTOR2);
    snd.preload_embedded(sounds::MOTOR3, sounds_bytes::MOTOR3);
    snd.preload_embedded(sounds::MOTOR4, sounds_bytes::MOTOR4);
    snd.preload_embedded(sounds::MOTOR5, sounds_bytes::MOTOR5);
    snd.preload_embedded(sounds::SCAN_HEAVY_STUTTER, sounds_bytes::SCAN_HEAVY_STUTTER);
    snd.preload_embedded(sounds::SCAN_LIGHT, sounds_bytes::SCAN_LIGHT);
    snd.preload_embedded(sounds::SCAN_LIGHT2, sounds_bytes::SCAN_LIGHT2);
    snd.preload_embedded(sounds::SCAN_MED, sounds_bytes::SCAN_MED);
    snd.preload_embedded(sounds::WOOSH_LIGHT, sounds_bytes::WOOSH_LIGHT);

    for line in Callout::ALL {
      snd.preload_embedded(line.key(), line.bytes());
    }

    // We're done. Sounds will remain preloaded.
    ctx.systems.despawn_self();
  }
}

pub struct SoundLoaderSystem;

impl SoundLoaderSystem {
  pub fn new() -> Self {
    Self
  }
}

#[cfg(test)]
mod tests {
  use super::sounds::*;

  #[test]
  fn hits_go_from_the_city_to_the_forest() {
    assert_eq!(city_hit(0.0), LATCH_HEAVY1);
    assert_eq!(city_hit(0.5), HIT_ORGANIC3);
    assert_eq!(city_hit(1.0), IMPACT_DIRT);
  }

  #[test]
  fn progress_past_either_end_stays_at_that_end() {
    assert_eq!(city_hit(-1.0), LATCH_HEAVY1);
    assert_eq!(city_hit(2.0), IMPACT_DIRT);
  }
}
