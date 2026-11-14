use std::collections::HashSet;

use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::GameManagementExt;
use frontbox_sound::SoundSystemExt;

use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{self, Mode, ModeManager, ModeRun, RootLashed, SporesSystem};
use crate::systems::{Callout, SayExt, sounds};

/// The time for each shot after the first
const BASE_SHOT_TIME: Duration = Duration::from_secs(25);

/// What each failed attempt adds to the shot time
const ATTEMPT_HANDICAP: Duration = Duration::from_secs(5);

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
  attention_scene: ScenePlayer,
  intensity_scene: ScenePlayer,
  hit_scene: ScenePlayer,
  arc_scene: ScenePlayer,
  progress_scene: Option<ScenePlayer>,
  current_combo_shot: u8,
  /// The combo shots, by number, that have been made and scored
  combo_shots_seen: HashSet<u8>,
  /// Failed attempts, each adding to the shot time
  combo_attempts: u16,
  cue_id: Option<u64>,
}

impl HydroCoreMode {
  pub fn new() -> Self {
    Self {
      run: ModeRun::new(Mode::HydroCore, COMBO.len() as u8),
      attention_scene: ScenePlayer::new(Self::attention_scene(COMBO[0].hex_center())),
      hit_scene: ScenePlayer::new(Self::hit_scene()),
      arc_scene: ScenePlayer::new(Self::arc_scene()),
      intensity_scene: ScenePlayer::new(Self::intensity_scene()),
      progress_scene: None,
      current_combo_shot: 1,
      combo_shots_seen: HashSet::new(),
      combo_attempts: 0,
      cue_id: None,
    }
  }

  fn attention_scene(target: &LedQ) -> PlayableScene {
    scene!("hydro_core/attention.scene", target.clone())
  }

  fn attention_ring(target: &LedQ) -> PlayableScene {
    scene!("hydro_core/attention_ring.scene", target.clone())
  }

  fn intensity_scene() -> PlayableScene {
    scene!("hydro_core/intensity.scene")
  }

  fn hit_scene() -> PlayableScene {
    scene!("hydro_core/hit.scene")
  }

  fn arc_scene() -> PlayableScene {
    scene!("hydro_core/arc.scene")
  }

  fn progress_scene(duration: Duration) -> PlayableScene {
    scene!("hydro_core/progress.scene").over(duration)
  }

  /// Start the combo from the lift ramp, as the mode starts or resumes, or after a failed attempt
  fn start_combo(&mut self, ctx: &SystemContext) {
    self.run.reset_progress();
    self.advance_combo(1, ctx);
    self.intensity_scene.stop(ctx);
  }

  fn clear_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.cue_id.as_ref() {
      ctx.cues.cancel(*cue_id);
      self.cue_id = None;
    }
  }

  fn advance_combo(&mut self, shot: u8, ctx: &SystemContext) {
    self.clear_cue(ctx);

    if let Some(scene) = self.progress_scene.as_mut() {
      scene.stop(ctx);
    }
    self.attention_scene.stop(ctx);
    self.hit_scene.stop(ctx);

    if shot == 6 {
      self
        .run
        .complete(completing_scene!("hydro_core/complete.scene"), ctx);
      return;
    }

    if shot > 1 {
      ctx.play_sfx(sounds::city_hit((shot - 1) as f32 / COMBO.len() as f32));
    }

    // the first shot isn't timed, as there's nothing to restart (decision 6)
    if shot > 1 {
      // Player only has a limited amount of time to make the next shot BUT
      // to avoid frustrating the player, keep making the combo duration longer as they fail attempts
      // (this results in less points but is still completable)
      let handicap = ATTEMPT_HANDICAP * self.combo_attempts as u32;
      let final_time = ctx
        .systems
        .expect::<ModeManager>()
        .mode_timer(BASE_SHOT_TIME + handicap);
      log::info!(
        "combo_attempts={} handicap={:?} final_time={:?}",
        self.combo_attempts,
        handicap,
        final_time
      );
      self.progress_scene = Some(ScenePlayer::started(Self::progress_scene(final_time), ctx));
      self.cue_id = Some(ctx.cues.start(ComboTimeUp, final_time.once()));
    }

    self.current_combo_shot = shot;

    let target = COMBO[shot as usize - 1];
    self
      .attention_scene
      .replace_scene(Self::attention_scene(target.hex_center()), ctx);
    self
      .hit_scene
      .replace_scene(Self::attention_ring(target.hex_ring()), ctx);

    match self.run.progress(shot - 1) {
      Some(50) => self.intensity_scene.start(ctx),
      Some(75) => ctx.say(Callout::CoreTemperatureRising),
      _ => {}
    }
  }

  /// The shot the combo is on was made, scoring `times` over for a burst
  fn combo_hit(&mut self, times: u8, ctx: &SystemContext) {
    log::info!("HydroCore: Combo shot hit");

    // Play SFX
    self.hit_scene.restart(ctx);

    let made = self.current_combo_shot;
    ctx.play_scene(&completing_scene!("hydro_core/flashers.scene"));

    // a shot only scores the first time it's made, not again on a later attempt. Each is worth half a shot more than
    // the last, so the 5 come to as much as 7 and a half.
    if self.combo_shots_seen.insert(made) {
      let value = game::points::mode_hit(Mode::HydroCore) / 2 * made as u64;
      ctx.add_points(value * times as u64);
    }

    self.advance_combo(made + 1, ctx);
  }

  fn combo_time_up(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: Combo time up!");
    ctx.play_sfx(sounds::HIT_ORGANIC_LOW1);
    self.combo_attempts += 1;
    self.start_combo(ctx);
  }
}

impl System for HydroCoreMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::HydroCore)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.run.begin(ctx);
    self.start_combo(ctx);
  }

  /// A resume keeps the failed attempts and the shots that have scored, and starts the combo over
  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.run.begin(ctx);
    self.start_combo(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.run.apply(ctx) {
      return;
    }

    self.attention_scene.start(ctx);
    self.hit_scene.start(ctx);
    self.arc_scene.start(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      if hit.shot == COMBO[self.current_combo_shot as usize - 1] {
        // the combo goes a shot at a time, so a burst doubles the points rather than skipping one
        let times = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
        self.combo_hit(times, ctx);
      }
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      // each hit of a lash makes the shot the combo is on
      for _ in 0..lash.hits {
        self.combo_hit(1, ctx);
      }
    } else if event.is::<ComboTimeUp>() {
      self.combo_time_up(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ComboTimeUp;
