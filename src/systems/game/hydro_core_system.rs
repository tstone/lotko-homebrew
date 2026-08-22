use std::collections::HashSet;
use std::sync::LazyLock;

use frontbox::animation::*;
use frontbox::prelude::tags::GeneralIllumination;
use frontbox::prelude::*;
use frontbox_turn_based::{GameEnded, GameManagementExt};

use crate::hardware::arc_ramp::{self, ArcRampHit, ArcRampSubwayHit};
use crate::hardware::center_orbit;
use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::lift_ramp::{self, LiftRampHit};
use crate::hardware::lower_scoop::{self, LowerScoopBallEnter};
use crate::hardware::more_tags::ArcRamp;
use crate::hardware::right_orbit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::systems::game::{ExclusiveMode, ExclusiveModeManager};

static MODE_COLOR: LazyLock<Rgba<u8>> = LazyLock::new(|| Rgba::cyan());
static QUAL_HIT_PTS: u32 = 5_000;
static START_PTS: u32 = 10_000;
static COMBO_BASE_PTS: u32 = 5_000;

#[derive(Clone)]
pub struct HydroCoreSystem {
  state: HydroCoreState,
  led_program: LedProgram1d,
  gi_program: LedProgram1d,
  arc_program: LedProgram1d,
  combo_hits: HashSet<u8>,
}

// TODO: hit effects
// TODO: arc glow/breathe?

impl HydroCoreSystem {
  pub fn new() -> Self {
    Self {
      state: HydroCoreState::Qualification(0),
      led_program: Self::qualification_program(),
      gi_program: LedProgram1d::fixed(
        Q::tag::<GeneralIllumination>().at_z(1),
        ColorSequence::solid(Rgba::cyan().lighten(0.4)),
      ),
      arc_program: LedProgram1d::breathe(Q::tag::<ArcRamp>(), Rgba::cyan(), Cycle::Forever),
      combo_hits: HashSet::new(),
    }
  }

  fn to_qualification(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: Entering qualification state");
    // play SFX
    self.combo_hits.clear();
    self.arc_program.stop(ctx);
    self.gi_program.stop(ctx);
    self.state = HydroCoreState::Qualification(0);
    self.led_program.stop(ctx);
    self.led_program = Self::qualification_program();
  }

  fn begin_startable(&mut self, ctx: &SystemContext) {
    log::info!("HydroCore: Qualification complete");
    self.state = HydroCoreState::PendingStartable;
    self.led_program.stop(ctx);
    self.led_program = LedProgram1d::flash(
      &*lower_scoop::BOLTS_Q,
      ColorSequence::solid(*MODE_COLOR),
      Cycle::Forever,
    );
  }

  fn to_startable(&mut self, _ctx: &SystemContext) {
    log::info!("HydroCore: Entering startable state");
    // play SFX
    self.state = HydroCoreState::Startable;
  }

  fn to_combo(&mut self, ctx: &SystemContext) {
    match ctx
      .expect::<ExclusiveModeManager>()
      .take_exclusive(ExclusiveMode::HydroCore, ctx)
    {
      Ok(..) => {
        if !matches!(self.state, HydroCoreState::ComboShot(..)) {
          log::info!("HydroCore: Entering combo state");
          // play SFX
          self.arc_program.play();
          self.gi_program.play();
        }

        self.start_combo(1, ctx);
      }
      _ => {}
    }
  }

  fn start_combo(&mut self, shot: u8, ctx: &SystemContext) {
    let cue_id = ctx.cue(ComboTimeUp, Cue::Once(Duration::from_secs(20)));
    self.state = HydroCoreState::ComboShot(shot, cue_id);
    self.led_program.stop(ctx);
    // BUG: this never comes on
    self.led_program = match shot {
      1 => Self::combo_hex_program(&*lift_ramp::HEX_CENTER_LED),
      2 => Self::combo_hex_program(&*arc_ramp::HEX_CENTER_LED),
      3 => Self::combo_hex_program(&*center_orbit::HEX_CENTER_LED),
      4 => Self::combo_hex_program(&*right_orbit::HEX_CENTER_LED),
      5 => Self::combo_hex_program(&*arc_ramp::HEX_CENTER_LED),
      _ => panic!("Cannot set program for unknown shot: {}", shot),
    }
  }

  fn advance_combo(&mut self, current_combo: u8, cue_id: u64, ctx: &SystemContext) {
    log::info!("HydroCore: Advaning combo to {}", current_combo + 1);

    // play SFX
    ctx.cancel_cue(cue_id);

    // Points for combo only score the first time, not repeated times
    if !self.combo_hits.contains(&current_combo) {
      ctx.add_points(COMBO_BASE_PTS * current_combo as u32);
      self.combo_hits.insert(current_combo);
    }

    let next_shot = current_combo + 1;
    self.start_combo(next_shot, ctx);
  }

  fn combo_time_up(&mut self, ctx: &SystemContext) {
    // play SFX
    self.start_combo(1, ctx);
  }

  fn complete(&mut self, cue_id: u64, ctx: &SystemContext) {
    log::info!("HydroCore: Complete. Resetting.");

    // play SFX
    ctx.cancel_cue(cue_id);
    self.to_qualification(ctx);
  }

  fn qualification_program() -> LedProgram1d {
    LedProgram1d::fixed(
      (&*arc_ramp::HEX_CENTER_LED).at_z(1),
      ColorSequence::solid(Rgba::white()),
    )
  }

  fn combo_hex_program<T: Contextual<LedIdentifications> + Send + Sync + 'static>(
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
}

impl System for HydroCoreSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx
      .expect::<ExclusiveModeManager>()
      .current_mode()
      .is_none()
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let HydroCoreState::Qualification(mut hits) = self.state
      && event.is::<ArcRampSubwayHit>()
    {
      hits += 1;
      log::info!("HydroCore: qualifying hit ({})", hits);
      ctx.add_points(QUAL_HIT_PTS);

      if hits >= 2 {
        self.begin_startable(ctx);
      } else {
        self.state = HydroCoreState::Qualification(hits);
      }
    } else if let Some(event) = event.downcast_ref::<LowerScoopBallEnter>() {
      if self.state == HydroCoreState::PendingStartable {
        self.to_startable(ctx);
      } else if self.state == HydroCoreState::Startable {
        // Additional points for making it in the scoop vs subway
        let pts = if event.subway_entry {
          START_PTS
        } else {
          START_PTS * 2
        };
        ctx.add_points(pts);
        self.to_combo(ctx);
      }
    } else if let HydroCoreState::ComboShot(1, cue_id) = self.state
      && event.is::<LiftRampHit>()
    {
      log::info!("Lift ramp combo hit?");
      self.advance_combo(1, cue_id, ctx);
    } else if let HydroCoreState::ComboShot(2, cue_id) = self.state
      && event.is::<ArcRampHit>()
    {
      self.advance_combo(2, cue_id, ctx);
    } else if let HydroCoreState::ComboShot(3, cue_id) = self.state
      && event.is::<CenterOrbitHit>()
    {
      self.advance_combo(3, cue_id, ctx);
    } else if let HydroCoreState::ComboShot(4, cue_id) = self.state
      && event.is::<RightOrbitHit>()
    {
      self.advance_combo(4, cue_id, ctx);
    } else if let HydroCoreState::ComboShot(5, cue_id) = self.state
      && event.is::<ArcRampHit>()
    {
      self.complete(cue_id, ctx);
    } else if event.is::<ComboTimeUp>() {
      self.combo_time_up(ctx);
    } else if event.is::<GameEnded>() {
      self.led_program.stop(ctx);
      self.gi_program.stop(ctx);
      self.arc_program.stop(ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.led_program.apply(delta, ctx);

    if matches!(self.state, HydroCoreState::ComboShot(..)) {
      self.gi_program.apply(delta, ctx);
      self.arc_program.apply(delta, ctx);
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HydroCoreState {
  Qualification(u8),
  PendingStartable,
  Startable,
  ComboShot(u8, u64),
}

#[derive(serde::Serialize, Event)]
struct ComboTimeUp;
