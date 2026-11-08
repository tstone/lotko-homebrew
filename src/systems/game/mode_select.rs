use frontbox_core::prelude::*;
use frontbox_dmd::*;
use frontbox_gfx::canvas::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{BallManagementExt, GameManagementExt, PlayerEntry, TurnPhase};
use frontbox_sound::SoundSystemExt;

use crate::hardware::cabinet::{self, action_button, start_button};
use crate::hardware::lower_scoop::{LowerScoopBallEnter, LowerScoopSystem};
use crate::systems::dmd::MENU_DMD_LAYER;
use crate::systems::game::{
  self, ExtraBallCollected, ExtraBallSystem, Mode, ModeManager, ScoopQualifier,
};
use crate::systems::sounds;

/// How long the menu waits before taking the highlighted row
const MENU_TIMEOUT: Duration = Duration::from_secs(10);
/// How long a menu of one option shows it for
const SINGLE_OPTION_TIME: Duration = Duration::from_millis(1500);

/// Decides what a ball in the left scoop does (DESIGN.md, Starting modes), in order:
///
/// 1. A lit extra ball collects first, on a front entry, with the ball held for its animation.
/// 2. If the scoop is lit and only one ball is in play, the mode select menu opens. With more in play the flippers are
///    busy, so the ball ejects as normal and the scoop stays lit for the next time.
/// 3. The ball goes, and the chosen mode's start phase takes it from there.
///
/// In the menu the flippers move the selection, and both flippers, action, or start choose. A timeout takes the highlighted
/// row, and a single option shows briefly and starts. Belongs in the player template.
#[derive(Clone)]
pub struct ModeSelectSystem {
  state: State,
  timeout_cue: Option<u64>,
  /// A flipper is down and moves the selection when it's let go, unless the other joins it to choose
  pending_move: Option<bool>,
}

impl ModeSelectSystem {
  pub fn new() -> Self {
    Self {
      state: State::Idle,
      timeout_cue: None,
      pending_move: None,
    }
  }

  fn on_ball_enter(&mut self, subway_entry: bool, ctx: &SystemContext) {
    let collecting = !subway_entry
      && ctx
        .systems
        .expect::<ExtraBallSystem>()
        .collect_at_scoop(ctx.into());

    if collecting {
      self.hold_ball(ctx);
      self.state = State::CollectingExtraBall;
    } else {
      self.open_menu_or_eject(ctx);
    }
  }

  fn open_menu_or_eject(&mut self, ctx: &SystemContext) {
    let rows = {
      let mode_manager = ctx.systems.expect::<ModeManager>();
      let opens = ctx
        .systems
        .expect::<ScoopQualifier>()
        .opens_menu(&mode_manager, ctx.balls_in_play());
      if opens {
        mode_manager.available_modes()
      } else {
        Vec::new()
      }
    };

    if rows.is_empty() {
      self.state = State::Idle;
      self.release_ball(ctx);
    } else {
      self.hold_ball(ctx);
      let timeout = if rows.len() == 1 {
        SINGLE_OPTION_TIME
      } else {
        MENU_TIMEOUT
      };
      self.timeout_cue = Some(ctx.cues.start(MenuTimedOut, timeout.once()));
      self.state = State::Menu { rows, selected: 0 };
      self.show_selection(ctx);
    }
  }

  fn move_selection(&mut self, ahead: bool, ctx: &SystemContext) {
    if let State::Menu { rows, selected } = &mut self.state {
      *selected = step(*selected, rows.len(), ahead);
      ctx.play_sfx(sounds::COUNTING);
      self.show_selection(ctx);
    }
  }

  fn choose(&mut self, ctx: &SystemContext) {
    let State::Menu { rows, selected } = &self.state else {
      return;
    };
    let mode = rows[*selected];
    self.close_menu(ctx);

    let started = ctx
      .systems
      .expect::<ModeManager>()
      .start_mode(mode, ctx.into());
    if started.is_ok() {
      ctx.systems.expect::<ScoopQualifier>().used(ctx.into());
      ctx.add_points(game::points::EXCL_START);
      ctx.play_sfx(sounds::IMPACT_CINEMATIC_HEAVY);
    }
    self.release_ball(ctx);
  }

  fn close_menu(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.timeout_cue.take() {
      ctx.cues.cancel(cue);
    }
    if matches!(self.state, State::Menu { .. }) {
      ctx.systems.expect::<ModeManager>().hide_menu(ctx.into());
      if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
        dmd.remove_layer(MENU_DMD_LAYER);
      }
    }
    self.pending_move = None;
    self.state = State::Idle;
  }

  fn show_selection(&self, ctx: &SystemContext) {
    let State::Menu { rows, selected } = &self.state else {
      return;
    };
    let mode = rows[*selected];
    log::info!("ModeSelect: {} of {}: {:?}", selected + 1, rows.len(), mode);
    ctx
      .systems
      .expect::<ModeManager>()
      .show_menu(rows.clone(), mode, ctx.into());

    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      let mut frame = Container::new(Fill2d::Solid(Rgba::black()));
      frame.add(
        BOLD_PIXELS_10PX_REGULAR_FONT
          .center_aligned(mode.name().to_string(), mode.color())
          .top_offset(11),
      );
      dmd.insert_layer(MENU_DMD_LAYER, frame.default_position());
    }
  }

  fn hold_ball(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LowerScoopSystem>()
      .hold(ctx.id(), ctx.into());
  }

  fn release_ball(&self, ctx: &SystemContext) {
    ctx
      .systems
      .expect::<LowerScoopSystem>()
      .release(ctx.id(), ctx.into());
  }

  fn on_switch_closed(&mut self, switch: &Switch, ctx: &SystemContext) {
    let (left, right) = (
      cabinet::LEFT_FLIPPER_SWITCH1.name,
      cabinet::RIGHT_FLIPPER_SWITCH1.name,
    );
    let held = |name: &str| ctx.switches.is_closed(name).unwrap_or(false);

    if switch.name == action_button::SWITCH.name {
      self.choose(ctx);
    } else if switch.name == start_button::SWITCH.name {
      // while players can still join, start is for that
      if ctx.states.is(PlayerEntry::Closed) {
        self.choose(ctx);
      }
    } else if switch.name == left || switch.name == right {
      let other = if switch.name == left { right } else { left };
      if held(other) {
        self.pending_move = None;
        self.choose(ctx);
      } else {
        self.pending_move = Some(switch.name == right);
      }
    }
  }

  fn on_switch_opened(&mut self, switch: &Switch, ctx: &SystemContext) {
    if switch.name == cabinet::LEFT_FLIPPER_SWITCH1.name
      || switch.name == cabinet::RIGHT_FLIPPER_SWITCH1.name
    {
      if let Some(ahead) = self.pending_move.take() {
        self.move_selection(ahead, ctx);
      }
    }
  }
}

impl System for ModeSelectSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.states.is(TurnPhase::Active)
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(enter) = event.downcast_ref::<LowerScoopBallEnter>() {
      self.on_ball_enter(enter.subway_entry, ctx);
    } else if event.is::<ExtraBallCollected>() && self.state == State::CollectingExtraBall {
      self.open_menu_or_eject(ctx);
    } else if event.is::<MenuTimedOut>() {
      self.timeout_cue = None;
      self.choose(ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && matches!(self.state, State::Menu { .. })
    {
      self.on_switch_closed(&event.switch, ctx);
    } else if let Some(event) = event.downcast_ref::<SwitchOpened>()
      && matches!(self.state, State::Menu { .. })
    {
      self.on_switch_opened(&event.switch, ctx);
    }
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    // the scoop clears its own holds at the end of the turn
    self.close_menu(ctx);
  }
}

/// The next row along, wrapping around the menu
fn step(selected: usize, rows: usize, ahead: bool) -> usize {
  if ahead {
    (selected + 1) % rows
  } else {
    (selected + rows - 1) % rows
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Idle,
  /// Holding the ball for the extra ball animation
  CollectingExtraBall,
  Menu {
    rows: Vec<Mode>,
    selected: usize,
  },
}

#[derive(serde::Serialize, Event)]
struct MenuTimedOut;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_selection_wraps_around_both_ways() {
    assert_eq!(step(0, 3, true), 1);
    assert_eq!(step(2, 3, true), 0);
    assert_eq!(step(0, 3, false), 2);
    assert_eq!(step(1, 3, false), 0);
  }
}
