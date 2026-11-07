use std::collections::HashSet;
use std::sync::LazyLock;

use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::GameManagementExt;
use frontbox_sound::SoundSystemExt;

use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{self, EffectsSystem, Mode, ModeManager, ModeRun, SporeApplied};
use crate::systems::{Callout, SayExt, sounds};

static BASE_SHOT_TIME: LazyLock<Duration> = LazyLock::new(|| Duration::from_secs(25));

const COMBO: [MainShot; 5] = [
  MainShot::LiftRamp,
  MainShot::ArcRamp,
  MainShot::LeftOrbit,
  MainShot::CenterOrbit,
  MainShot::DomeRamp,
];

#[derive(Clone)]
pub struct HydroCoreMode {
  run: ModeRun,
  attention_effect: LedPlayback,
  intensity_effect: LedPlayback,
  hit_effect: LedPlayback,
  arc_effect: LedPlayback,
  progress_effect: Option<LedPlayback>,
  current_combo_shot: u8,
  /// Track which shots have been made in the past
  combo_shots_seen: HashSet<u8>,
  combo_attempts: u16,
  cue_id: Option<u64>,
}

impl HydroCoreMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::HydroCore, COMBO.len() as u8),
      attention_effect: Self::attention_effect(COMBO[0].hex_center()),
      hit_effect: Self::hit_effect(),
      arc_effect: Self::arc_effect(),
      intensity_effect: Self::intensity_effect(),
      progress_effect: None,
      current_combo_shot: 1,
      combo_shots_seen: HashSet::new(),
      combo_attempts: 0,
      cue_id: None,
    }
  }

  fn attention_effect(target: &LedQ) -> LedPlayback {
    led_playback!("hydro_core/attention.scene", target.clone())
  }

  fn attention_ring(target: &LedQ) -> LedPlayback {
    led_playback!("hydro_core/attention_ring.scene", target.clone())
  }

  fn intensity_effect() -> LedPlayback {
    led_playback!("hydro_core/intensity.scene").stopped()
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("hydro_core/hit.scene")
  }

  fn arc_effect() -> LedPlayback {
    led_playback!("hydro_core/arc.scene")
  }

  fn progress_effect(duration: Duration) -> LedPlayback {
    led_playback!("hydro_core/progress.scene")
      .over(duration)
      .end_behavior(EndBehavior::Hold)
  }

  fn restart_combo(&mut self, ctx: &SystemContext) {
    self.combo_attempts += 1;
    self.run.reset_progress();
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
      self.run.complete(
        game::points::EXL_COMPLETION,
        led_playback!("hydro_core/complete.scene"),
        ctx,
      );
      return;
    }

    if shot > 1 {
      ctx.play_sfx(sounds::city_hit((shot - 1) as f32 / COMBO.len() as f32));
    }

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

    let target = COMBO[shot as usize - 1];
    self.attention_effect = Self::attention_effect(target.hex_center());
    self.hit_effect = Self::attention_ring(target.hex_ring());

    match self.run.progress(shot - 1) {
      Some(50) => self.intensity_effect.play(),
      Some(75) => ctx.say(Callout::CoreTemperatureRising),
      _ => {}
    }
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
      .expect::<EffectsSystem>()
      .play(led_playback!("hydro_core/flashers.scene"));

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

}

impl System for HydroCoreMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::HydroCore)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
    self.restart_combo(ctx);
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }

    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.arc_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);

    if let Some(effect) = self.progress_effect.as_mut() {
      effect.apply(delta, ctx);
    }
  }

  // TODO: change order to lift ramp => arc ramp => left ramp => right orbit => center orbit to get to N spins (untimed)

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      if hit.shot == COMBO[self.current_combo_shot as usize - 1] {
        self.combo_hit(ctx);
      }
    } else if event.is::<ComboTimeUp>() {
      self.combo_time_up(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ComboTimeUp;
