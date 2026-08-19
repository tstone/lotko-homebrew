use frontbox::animation::Curve;
use frontbox::prelude::*;
use frontbox_sound::SoundSystemExt;
use frontbox_turn_based::*;

use crate::hardware::lower_scoop::LowerScoopSystem;
use crate::hardware::{ScoopBallEntered, lift_ramp, lower_scoop};
use crate::systems::game::{CityManager, CityRegions};
use crate::systems::sounds;

#[derive(Clone)]
pub struct CityCoverageQualification3 {
  lift_ramp_program: Option<LedProgram1d>,
  lower_scoop_program: LedProgram1d,
  handle: SystemHandle,
}

impl CityCoverageQualification3 {
  pub fn new() -> Self {
    let colors = vec![
      ColorSequence::solid(Rgba::white()),
      ColorSequence::solid(Rgba::magenta()),
      ColorSequence::solid(Rgba::cyan()),
    ];
    Self {
      lift_ramp_program: Some(LedProgram1d::tween(
        lift_ramp::BOLT_LED.q(),
        Duration::from_millis(83 * 3),
        Curve::SmoothRandom,
        Cycle::Forever,
        colors.clone(),
      )),
      lower_scoop_program: LedProgram1d::tween(
        lower_scoop::bolts_q(),
        Duration::from_millis(83 * 3),
        Curve::SmoothRandom,
        Cycle::Forever,
        colors,
      ),
      handle: SystemHandle::default(),
    }
  }

  pub fn complete(&mut self, svc_ctx: &ServiceContext) {
    let ctx = &svc_ctx.for_system(self.handle);
    log::info!("City coverage qualification 3 complete");
    ctx.play_sfx(sounds::LANE_HIT_COMPLETE);
    ctx.add_points(50000);

    let mut lift_ramp = ctx.expect::<lift_ramp::LiftRampSystem>();
    lift_ramp.lift_down(svc_ctx);

    // clear effects
    if let Some(effect) = &mut self.lift_ramp_program {
      effect.stop(ctx);
    }
    self.lift_ramp_program = None;
    self.lower_scoop_program.stop(ctx);

    // TODO: launch menu
    // TODO: move this to menu system once thats implemented
    ctx
      .expect::<lower_scoop::LowerScoopSystem>()
      .set_mode(lower_scoop::LowerScoopMode::AutoEject, svc_ctx);

    // TEMP: Pick a random uncompleted tier 1 city (this should probably somehow factor in completion state)
    // let region = if rand::random_bool(0.5) {
    //   CityRegions::MeridianBasins
    // } else {
    //   CityRegions::HydroCore
    // };
    ctx
      .expect::<CityManager>()
      .activate_region(CityRegions::MeridianBasins, svc_ctx);
    ctx.despawn_self();
  }
}

impl System for CityCoverageQualification3 {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .get::<GameManager>()
      .map(|game| game.turn_state() == Some(&TurnState::Active))
      .unwrap_or(false)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = *ctx.current_handle();

    ctx
      .expect::<lower_scoop::LowerScoopSystem>()
      .set_mode(lower_scoop::LowerScoopMode::ModeStart, ctx.into());
    ctx
      .expect::<lift_ramp::LiftRampSystem>()
      .lift_up(ctx.into());

    // TODO: should this hurry up time be flexed up or down depending on other achievements? (yes)
    ctx.cue(LiftRampHurryUpDone, Cue::Once(Duration::from_secs(25)));
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(effect) = self.lift_ramp_program.as_mut() {
      effect.apply(delta, ctx);
    }
    self.lower_scoop_program.apply(delta, ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LiftRampHurryUpDone>() {
      ctx
        .expect::<lift_ramp::LiftRampSystem>()
        .lift_down(ctx.into());
      if let Some(effect) = &mut self.lift_ramp_program {
        effect.stop(ctx);
      }
      self.lift_ramp_program = None;
    } else if let Some(ScoopBallEntered(name)) = event.downcast_ref::<ScoopBallEntered>() {
      let svc_ctx: &ServiceContext = ctx.into();
      if (*name).eq(lower_scoop::SCOOP_NAME) {
        self.complete(ctx.into());
        ctx.expect::<LowerScoopSystem>().eject(svc_ctx);
      } else if (*name).eq(lift_ramp::SCOOP_NAME) {
        self.complete(svc_ctx);
        ctx.expect::<lift_ramp::LiftRampSystem>().eject(svc_ctx);
      }
    }
  }
}

#[derive(serde::Serialize, Event)]
struct LiftRampHurryUpDone;
