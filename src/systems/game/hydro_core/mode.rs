use std::collections::HashSet;
use std::sync::LazyLock;

use frontbox_core::prelude::tags::Playfield;
use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameManagementExt, TurnEnding};
use frontbox_sound::SoundSystemExt;

use crate::hardware::arc_ramp::ArcRampHit;
use crate::hardware::backbox::{LEFT_SPEAKER_LEDS, RIGHT_SPEAKER_LEDS};
use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::dome_ramp::DomeRampHit;
use crate::hardware::flashers::{self, FlashersSystem};
use crate::hardware::left_orbit::LeftOrbitHit;
use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::more_tags::ArcRamp;
use crate::hardware::{arc_ramp, center_orbit, city_map, dome_ramp, left_orbit, lift_ramp};
use crate::systems::game::ModeManager;
use crate::systems::game::{self, ExclusiveMode, LeftScoopStartable, SporeApplied};
use crate::systems::sounds;

static BASE_SHOT_TIME: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs(25));

#[derive(Clone)]
pub struct HydroCoreMode {
  attention_effect: LedScene,
  intensity_effect: LedScene,
  hit_effect: LedScene,
  arc_effect: LedScene,
  progress_effect: Option<LedScene>,
  current_combo_shot: u8,
  /// Track which shots have been made in the past
  combo_shots_seen: HashSet<u8>,
  combo_attempts: u16,
  cue_id: Option<u64>,
  mode_complete: bool,
}

impl HydroCoreMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(&*lift_ramp::HEX_CENTER_LED),
      hit_effect: Self::hit_effect(&*lift_ramp::HEX_CENTER_LED),
      arc_effect: Self::arc_effect(),
      intensity_effect: Self::intensity_effect(),
      progress_effect: None,
      current_combo_shot: 1,
      combo_shots_seen: HashSet::new(),
      combo_attempts: 0,
      cue_id: None,
      mode_complete: false,
    }
  }

  fn attention_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedScene {
    LedScene::new(target, led_scene!("hydro_core/attention.ron"))
  }

  fn attention_ring<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedScene {
    LedScene::new(target, led_scene!("hydro_core/attention_ring.ron"))
  }

  fn intensity_effect() -> LedScene {
    LedScene::unbound(led_scene!("hydro_core/intensity.ron"))
      .on(
        "flashers",
        LedQ::any(vec![
          &flashers::LEFT_FLASHER.q(),
          &flashers::CENTER_FLASHER.q(),
        ]),
      )
      .unwrap()
      .on("left_speaker", LEFT_SPEAKER_LEDS.q())
      .unwrap()
      .on("right_speaker", RIGHT_SPEAKER_LEDS.q())
      .unwrap()
      .stopped()
  }

  fn hit_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(target: T) -> LedScene {
    LedScene::unbound(led_scene!("hydro_core/hit.ron"))
      .on("shot", target)
      .unwrap()
      .on("playfield", LedQ::tag::<Playfield>().at_z(-1))
      .unwrap()
  }

  fn arc_effect() -> LedScene {
    LedScene::new(LedQ::tag::<ArcRamp>(), led_scene!("hydro_core/arc.ron"))
  }

  fn progress_effect(duration: Duration) -> LedScene {
    LedScene::new(
      city_map::SPORE_COUNT_BAR.q().reverse(),
      led_scene!("hydro_core/progress.ron"),
    )
    .over(duration)
    .end_behavior(EndBehavior::Hold)
  }

  fn restart_combo(&mut self, ctx: &SystemContext) {
    self.combo_attempts += 1;
    self.advance_combo(1, ctx);
    self.intensity_effect.stop(ctx);
  }

  fn clear_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.cue_id.as_ref() {
      ctx.cues.cancel(*cue_id);
      self.cue_id = None;
    }
  }

  fn advance_combo(&mut self, shot: u8, ctx: &SystemContext) {
    self.clear_cue(ctx);

    if let Some(effect) = self.progress_effect.as_mut() {
      effect.stop(ctx);
    }
    self.attention_effect.stop(ctx);
    self.hit_effect.stop(ctx);

    if shot == 6 {
      ctx.play_sfx(sounds::HYDRO_CORE_PURGED);
      self.hit_effect.stop(ctx);
      self.hit_effect = LedScene::new(LedQ::Every, led_scene!("hydro_core/complete.ron"));
      self.mode_complete = true;
      return;
    }

    ctx.play_sfx(sounds::rnd_lane_hit());

    // first and last shot aren't timed
    if shot > 1 && shot < 5 {
      // Player only has a limited amount of time to make the next shot BUT
      // to avoid frustrating the player, keep making the combo duration longer as they fail attempts
      // (this results in less points but is still completable)
      let handicap = Duration::from_secs(5 * self.combo_attempts as u64);
      let final_time = *BASE_SHOT_TIME + handicap;
      log::info!(
        "combo_attempts={} handicap={:?} final_time={:?}",
        self.combo_attempts,
        handicap,
        final_time
      );
      self.progress_effect = Some(Self::progress_effect(final_time));
      self.cue_id = Some(ctx.cues.start(ComboTimeUp, final_time.once()));
    }

    self.current_combo_shot = shot;

    match shot {
      1 => {
        self.attention_effect = Self::attention_effect(&*lift_ramp::HEX_CENTER_LED);
        self.hit_effect = Self::attention_ring(&*lift_ramp::HEX_CIRCLE_LEDS);
      }
      2 => {
        self.attention_effect = Self::attention_effect(&*arc_ramp::HEX_CENTER_LED);
        self.hit_effect = Self::attention_ring(&*arc_ramp::HEX_CIRCLE_LEDS);
      }
      3 => {
        self.attention_effect = Self::attention_effect(&*left_orbit::HEX_CENTER_LED);
        self.hit_effect = Self::attention_ring(&*left_orbit::HEX_CIRCLE_LEDS);
      }
      4 => {
        self.attention_effect = Self::attention_effect(&*center_orbit::HEX_CENTER_LED);
        self.hit_effect = Self::attention_ring(&*center_orbit::HEX_CIRCLE_LEDS);
        self.intensity_effect.play();
      }
      5 => {
        self.attention_effect = Self::attention_effect(&*dome_ramp::HEX_CENTER_LED);
        self.hit_effect = Self::attention_ring(&*dome_ramp::HEX_CIRCLE_LEDS);
      }
      _ => panic!("Cannot set program for unknown shot: {}", shot),
    };
  }

  fn combo_hit(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: Combo shot hit");

    // Play SFX
    self.hit_effect.reset();
    self.hit_effect.play();

    self.advance_combo(self.current_combo_shot + 1, ctx);
    ctx.emit(SporeApplied);
    ctx
      .systems
      .expect::<FlashersSystem>()
      .play(led_scene!("hydro_core/flashers.ron"));

    // Points for combo only score the first time, not repeated times
    if !self.combo_shots_seen.contains(&self.current_combo_shot) {
      ctx.add_points(game::points::EXL_MODE_HIT * self.current_combo_shot as u32);
      self.combo_shots_seen.insert(self.current_combo_shot - 1);
    }
  }

  fn combo_time_up(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: Combo time up!");
    ctx.play_sfx(sounds::HIT_ORGANIC_LOW1);
    self.restart_combo(ctx);
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<ModeManager>()
      .release_exclusive(&ExclusiveMode::HydroCore, ctx.into());
    ctx.systems.expect::<LeftScoopStartable>().make_startable(
      ExclusiveMode::HydroCore,
      Duration::ZERO,
      ctx.into(),
    );
    ctx.systems.despawn_self();
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx.add_points(game::points::EXL_COMPLETION);
    ctx
      .systems
      .expect::<ModeManager>()
      .complete_exclusive(ExclusiveMode::HydroCore, ctx.into());
    // a city mode can only be completed once per game
    ctx.systems.despawn_self();
  }
}

impl System for HydroCoreMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == &Some(ExclusiveMode::HydroCore)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.restart_combo(ctx);
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.arc_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);

    if let Some(effect) = self.progress_effect.as_mut() {
      effect.apply(delta, ctx);
    }

    if self.mode_complete && self.hit_effect.is_complete() {
      self.complete(ctx);
    }
  }

  // TODO: change order to lift ramp => arc ramp => left ramp => right orbit => center orbit to get to N spins (untimed)

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if self.current_combo_shot == 1 && event.is::<LiftRampHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 2 && event.is::<ArcRampHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 3 && event.is::<LeftOrbitHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 4 && event.is::<CenterOrbitHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 5 && event.is::<DomeRampHit>() {
      self.combo_hit(ctx);
    } else if event.is::<ComboTimeUp>() {
      self.combo_time_up(ctx);
    } else if event.is::<RoutineStarted<TurnEnding>>() {
      self.revert_to_startable(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ComboTimeUp;
