use frontbox::animation::Curve;
use frontbox::prelude::color_sequence::{Anchor1d, Fill1dArea};
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::{GameManagementExt, GameManager, TurnState};

use crate::hardware::vspinner::{self, VerticalSpinnerHit};
use crate::systems::game::NimbusPromenadeMode;
use crate::systems::game::nimbus_promenade::{self, MODE_COLOR};
use crate::systems::sounds;

const REQUIRED_HITS: u8 = 20;

#[derive(Clone)]
pub struct NimbusPromenadeQualification {
  hit_effect: LedProgram1d,
  progress_effect: LedProgram1d,
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

  fn hit_effect() -> LedProgram1d {
    // energy from the spinner goes outwards through the rays to the pops
    let duration = Duration::from_millis(600);
    LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::rotating(
          &*vspinner::left_ray::Q,
          ColorSequence::exact(vec![
            *MODE_COLOR,
            Rgba::default(),
            Rgba::default(),
            Rgba::default(),
          ]),
          duration,
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::rotating(
          &*vspinner::upper_right_ray::Q,
          ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
          duration,
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::rotating(
          &*vspinner::lower_right_ray::Q,
          ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
          duration,
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .stopped()
  }

  fn progress_effect(count: u8) -> LedProgram1d {
    log::info!("Progress effect: {}", count);
    LedProgram1d::rotating(
      vspinner::LEDS.q(),
      ColorSequence::solid(*MODE_COLOR).area(Fill1dArea::anchored(Anchor1d::Start, count)),
      Duration::from_millis(1250),
      Curve::Linear,
      Cycle::Forever,
    )
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
      ctx.play_sfx(sounds::HIT_ORGANIC3);
      ctx.add_points(nimbus_promenade::points::START);
      self.shutdown = true;
    }
  }
}

impl System for NimbusPromenadeQualification {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .expect::<GameManager>()
      .game_state()
      .map(|state| *state.current_player_turn_state() == TurnState::Active)
      .unwrap_or(false)
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
      ctx.replace_self(NimbusPromenadeMode::new());
    }
  }
}
