use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::vspinner::{self, VerticalSpinnerHit};
use crate::systems::game::NimbusPromenadeMode;
use crate::systems::game::nimbus_promenade;
use crate::systems::sounds;

const REQUIRED_HITS: u8 = 20;

#[derive(Clone)]
pub struct NimbusPromenadeQualification {
  hit_effect: LedScene,
  progress_effect: LedScene,
  hits: u8,
  shutdown: bool,
}

impl NimbusPromenadeQualification {
  pub fn new() -> Self {
    Self {
      hits: 0,
      hit_effect: Self::hit_effect(),
      progress_effect: Self::progress_effect(0),
      shutdown: false,
    }
  }

  fn hit_effect() -> LedScene {
    // energy from the spinner goes outwards through the rays to the pops
    LedScene::unbound(led_scene!("nimbus_promenade/qualification_hit.ron"))
      .on("left_ray", &*vspinner::left_ray::Q)
      .unwrap()
      .on("upper_right_ray", &*vspinner::upper_right_ray::Q)
      .unwrap()
      .on("lower_right_ray", &*vspinner::lower_right_ray::Q)
      .unwrap()
      .stopped()
  }

  fn progress_effect(count: u8) -> LedScene {
    log::info!("Progress effect: {}", count);
    // the ring's 12 LEDs are 1/11 apart along a scene
    LedScene::new(
      vspinner::LEDS.q(),
      led_scene!("nimbus_promenade/qualification_progress.ron"),
    )
    .param("width", count as f32 / 11.0)
  }

  fn spinner_hit(&mut self, ctx: &SystemContext) {
    self.hits += 1;
    log::info!("Nimbus: Spinner hit: {}", self.hits);
    ctx.add_points(nimbus_promenade::points::QUAL_HIT);

    if self.hit_effect.is_complete() {
      self.hit_effect.reset();
    }
    self.hit_effect.play();

    if REQUIRED_HITS.saturating_sub(self.hits) <= 12 {
      self.progress_effect.stop(ctx);
      let progress = self.hits as f32 / REQUIRED_HITS as f32;
      self.progress_effect = Self::progress_effect((12.0 * progress) as u8);
    }

    if self.hits == REQUIRED_HITS {
      ctx.play_sfx(sounds::ARP_HIT1);
      ctx.add_points(nimbus_promenade::points::START);
      self.shutdown = true;
    }
  }
}

impl System for NimbusPromenadeQualification {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<VerticalSpinnerHit>() {
      self.spinner_hit(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.hit_effect.apply(delta, ctx);
    self.progress_effect.apply(delta, ctx);

    if self.shutdown && self.hit_effect.is_complete() {
      ctx.systems.replace_self(NimbusPromenadeMode::new());
    }
  }
}
