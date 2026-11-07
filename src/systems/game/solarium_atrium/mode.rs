use frontbox_core::prelude::tags::*;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnStarting};
use frontbox_sound::SoundSystemExt;

use crate::game::solarium_atrium::MODE_COLOR;
use crate::hardware::arc_ramp::{self, ArcRampHit};
use crate::hardware::dome_ramp::{self, DomeRampHit};
use crate::hardware::flashers::{self, FlashersSystem};
use crate::hardware::lift_ramp::{self, LiftRampHit};
use crate::hardware::{backbox, city_map};
use crate::systems::game::{self, ExclusiveMode, LeftScoopStartable, ModeManager, SporeApplied};
use crate::systems::sounds;

static REQUIRED_HITS: u8 = 6;

pub struct SolariumAtriumMode {
  attention_effect: LedScene,
  hit_effect: LedScene,
  intensity_effect: LedScene,
  progress_effect: LedScene,
  ramp_hits: u8,
  last_ramp: Option<Ramp>,
}

impl SolariumAtriumMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(),
      hit_effect: Self::hit_effect(),
      intensity_effect: Self::intensity_effect(),
      progress_effect: Self::progress_effect(0),
      ramp_hits: 0,
      last_ramp: None,
    }
  }

  fn attention_effect() -> LedScene {
    LedScene::new(
      LedQ::any(vec![
        &*dome_ramp::HEX_CENTER_LED,
        &*arc_ramp::HEX_CENTER_LED,
        &*lift_ramp::HEX_CENTER_LED,
      ]),
      led_scene!("solarium_atrium/attention.ron"),
    )
  }

  fn hit_effect() -> LedScene {
    LedScene::new(
      LedQ::tag::<Playfield>().at_z(-1),
      led_scene!("solarium_atrium/hit.ron"),
    )
    .stopped()
  }

  fn complete_effect() -> LedScene {
    LedScene::new(
      LedQ::tag::<Playfield>().at_z(-1),
      led_scene!("solarium_atrium/complete.ron"),
    )
    .stopped()
  }

  fn intensity_effect() -> LedScene {
    LedScene::unbound(led_scene!("solarium_atrium/intensity.ron"))
      .on(
        "flashers",
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
      )
      .unwrap()
      .on("left_speaker", backbox::LEFT_SPEAKER_LEDS.q().reverse())
      .unwrap()
      .on("right_speaker", backbox::RIGHT_SPEAKER_LEDS.q().reverse())
      .unwrap()
      .stopped()
  }

  fn progress_effect(hits: u8) -> LedScene {
    city_map::spore_count_effect(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::SolariumAtrium, ctx.into());
    ctx.systems.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::SolariumAtrium,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.systems.despawn_self();
  }

  fn ramp_hit(&mut self, ramp_hit: Ramp, ctx: &SystemContext) {
    self.ramp_hits += 1;
    self.hit_effect.reset();
    ctx
      .systems
      .expect::<FlashersSystem>()
      .play(led_scene!("solarium_atrium/flashers.ron"));

    let points = match (self.last_ramp.as_ref(), &ramp_hit) {
      (Some(Ramp::DomeRamp), Ramp::LiftRamp) => game::points::EXL_MODE_HIT as f32 * 2.0,
      (Some(Ramp::LiftRamp), Ramp::DomeRamp) => game::points::EXL_MODE_HIT as f32 * 2.0,
      _ => game::points::EXL_MODE_HIT as f32,
    };
    ctx.add_points(points as u32);
    ctx.emit(SporeApplied);

    if self.ramp_hits == (REQUIRED_HITS - 2) {
      self.intensity_effect.play();
    } else if self.ramp_hits == REQUIRED_HITS {
      ctx.play_sfx(sounds::ARP_HIT1);
      self.hit_effect = Self::complete_effect();
    }
    self.hit_effect.play();

    self.last_ramp = Some(ramp_hit);
    self.progress_effect = Self::progress_effect(self.ramp_hits);
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::EXL_COMPLETION);

    // TODO: epic reaction effect
    ctx
      .systems
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::SolariumAtrium, ctx.into());
    // a city mode can only be completed once per game
    ctx.systems.despawn_self();
  }

  fn is_complete(&self) -> bool {
    self.ramp_hits == REQUIRED_HITS
  }
}

impl System for SolariumAtriumMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::SolariumAtrium)
  }

  fn on_spawn(&mut self, _ctx: &SystemContext) {
    log::info!("Solarium Atrium mode started");
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<RoutineStarted<TurnStarting>>() {
      self.revert_to_startable(ctx);
    } else if event.is::<DomeRampHit>() {
      self.ramp_hit(Ramp::DomeRamp, ctx);
    } else if event.is::<ArcRampHit>() {
      self.ramp_hit(Ramp::ArcRamp, ctx);
    } else if event.is::<LiftRampHit>() {
      self.ramp_hit(Ramp::LiftRamp, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.hit_effect.is_complete() && self.is_complete() {
      self.complete(ctx);
    }

    self.progress_effect.apply(delta, ctx);
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
  }
}

enum Ramp {
  DomeRamp,
  ArcRamp,
  LiftRamp,
}
