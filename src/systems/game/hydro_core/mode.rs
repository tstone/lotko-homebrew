use std::collections::HashSet;

use frontbox::animation::*;
use frontbox::prelude::tags::{self, Playfield};
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::{GameManagementExt, PlayerTurnEnding};

use crate::hardware::arc_ramp::{ArcRampHit, ArcRampSubwayHit};
use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::lift_ramp::LiftRampHit;
use crate::hardware::more_tags::ArcRamp;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::hardware::{arc_ramp, center_orbit, lift_ramp, right_orbit};
use crate::systems::game::hydro_core::MODE_COLOR;
use crate::systems::game::{ExclusiveMode, HydroCoreStartable, hydro_core};
use crate::systems::game::{ExclusiveModeManager, HydroCoreQualification};
use crate::systems::sounds;

#[derive(Clone)]
pub struct HydroCoreMode {
  attention_effect: LedProgram1d,
  hit_effect: LedProgram1d,
  gi_effect: LedProgram1d,
  arc_effect: LedProgram1d,
  current_combo_shot: u8,
  /// Track which shots have been made in the past
  combo_shots_seen: HashSet<u8>,
  combo_attempts: u16,
  cue_id: u64,
}

impl HydroCoreMode {
  pub fn new() -> Self {
    Self {
      attention_effect: Self::attention_effect(&*lift_ramp::HEX_CENTER_LED),
      hit_effect: Self::hit_effect(&*lift_ramp::HEX_CENTER_LED),
      gi_effect: Self::gi_effect(),
      arc_effect: Self::arc_effect(),
      current_combo_shot: 1,
      combo_shots_seen: HashSet::new(),
      combo_attempts: 0,
      cue_id: 0,
    }
  }

  fn attention_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedProgram1d {
    LedProgram1d::rotating(
      target,
      ColorSequence::exact(vec![*MODE_COLOR, Rgba::default(), Rgba::default()]),
      Duration::from_millis(1500),
      Curve::EaseIn,
      Cycle::Forever,
    )
  }

  fn hit_effect<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
    target: T,
  ) -> LedProgram1d {
    LedProgram1d::timeline()
      .at(
        Duration::ZERO,
        LedProgram1d::rotating(
          target,
          ColorSequence::exact(vec![
            *MODE_COLOR,
            MODE_COLOR.darken(0.4),
            MODE_COLOR.darken(0.8),
          ]),
          Duration::from_millis(600),
          Curve::Linear,
          Cycle::Once,
        ),
      )
      .at(
        Duration::ZERO,
        LedProgram1d::tween(
          LedQ::tag::<Playfield>().at_z(-1),
          Duration::from_millis(500),
          Curve::ExponentialOut,
          Cycle::Once,
          vec![
            ColorSequence::fade(*MODE_COLOR, Rgba::default()).shuffle(rand::random()),
            ColorSequence::solid(Rgba::default()),
          ],
        ),
      )
  }

  fn gi_effect() -> LedProgram1d {
    LedProgram1d::fixed(
      LedQ::tag::<tags::GeneralIllumination>().at_z(1),
      ColorSequence::solid(MODE_COLOR.lighten(0.4)),
    )
  }

  fn arc_effect() -> LedProgram1d {
    LedProgram1d::breathe(
      LedQ::tag::<ArcRamp>(),
      *MODE_COLOR,
      Duration::bpm(150), // sync with mode music
      Cycle::Forever,
    )
  }

  fn restart_combo(&mut self, ctx: &SystemContext) {
    self.combo_attempts += 1;
    self.advance_combo(1, ctx);
  }

  fn advance_combo(&mut self, shot: u8, ctx: &SystemContext) {
    ctx.cancel_cue(self.cue_id);

    // sfx
    match shot {
      1 => ctx.play_sfx(sounds::HYDRO_CORE_FLUID_ROUTING_ACTIVE),
      2 => ctx.play_sfx(sounds::rnd_lane_hit()),
      3 => ctx.play_sfx(sounds::HYDRO_CORE_PRESSURE_RISING),
      4 => ctx.play_sfx(sounds::rnd_lane_hit()),
      5 => ctx.play_sfx(sounds::rnd_lane_hit()),
      6 => {
        ctx.play_sfx(sounds::HYDRO_CORE_PURGED);
        self.complete(ctx);
        return;
      }
      _ => {}
    }

    // Player only has a limited amount of time to make the next shot BUT
    // to avoid frustrating the player, keep making the combo duration longer as they fail attempts
    // (this results in less points but is still completable)
    let handicap = Duration::from_secs(3 * self.combo_attempts as u64);
    self.cue_id = ctx.cue(ComboTimeUp, Cue::Once(Duration::from_secs(20) + handicap));
    self.current_combo_shot = shot;

    self.attention_effect.stop(ctx);
    self.hit_effect.stop(ctx);

    match shot {
      1 => {
        self.attention_effect = Self::attention_effect(&*lift_ramp::HEX_CENTER_LED);
        self.hit_effect = Self::attention_effect(&*lift_ramp::HEX_CIRCLE_LEDS);
      }
      2 => {
        self.attention_effect = Self::attention_effect(&*arc_ramp::HEX_CENTER_LED);
        self.hit_effect = Self::attention_effect(&*arc_ramp::HEX_CIRCLE_LEDS);
      }
      3 => {
        self.attention_effect = Self::attention_effect(&*center_orbit::HEX_CENTER_LED);
        self.hit_effect = Self::attention_effect(&*center_orbit::HEX_CIRCLE_LEDS);
      }
      4 => {
        self.attention_effect = Self::attention_effect(&*right_orbit::HEX_CENTER_LED);
        self.hit_effect = Self::attention_effect(&*right_orbit::HEX_CIRCLE_LEDS);
      }
      5 => {
        self.attention_effect = arc_ramp::into_subway_program(*MODE_COLOR);
        self.hit_effect = Self::attention_effect(&*arc_ramp::HEX_CIRCLE_LEDS);
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

    // Points for combo only score the first time, not repeated times
    if !self.combo_shots_seen.contains(&self.current_combo_shot) {
      ctx.add_points(hydro_core::points::COMBO_BASE * self.current_combo_shot as u32);
      self.combo_shots_seen.insert(self.current_combo_shot - 1);
    }
  }

  fn combo_time_up(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: Combo time up!");

    // play SFX
    self.restart_combo(ctx);
  }

  fn revert_to_startable(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<ExclusiveModeManager>()
      .release_exclusive(ExclusiveMode::HydroCore, ctx);
    ctx.replace_self(HydroCoreStartable::new(Duration::ZERO));
  }

  fn complete(&mut self, ctx: &SystemContext) {
    ctx.add_points(hydro_core::points::COMPLETION / self.combo_attempts.min(10) as u32);

    // TODO: epic reaction effect
    ctx
      .expect::<ExclusiveModeManager>()
      .release_exclusive(ExclusiveMode::HydroCore, ctx);
    ctx.replace_self(HydroCoreQualification::new());
  }
}

impl System for HydroCoreMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.expect::<ExclusiveModeManager>().current_mode() == &Some(ExclusiveMode::HydroCore)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.restart_combo(ctx);
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.attention_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.gi_effect.apply(delta, ctx);
    self.arc_effect.apply(delta, ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if self.current_combo_shot == 1 && event.is::<LiftRampHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 2 && event.is::<ArcRampHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 3 && event.is::<CenterOrbitHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 4 && event.is::<RightOrbitHit>() {
      self.combo_hit(ctx);
    } else if self.current_combo_shot == 5 && event.is::<ArcRampSubwayHit>() {
      self.combo_hit(ctx);
    } else if event.is::<ComboTimeUp>() {
      self.combo_time_up(ctx);
    } else if event.is::<PlayerTurnEnding>() {
      self.revert_to_startable(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct ComboTimeUp;
