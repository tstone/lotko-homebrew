use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameEnding, PlayerTilted, TiltWarned, TurnStarting};

use crate::systems::dmd::{ANNOUNCEMENT_DMD_LAYER, DmdScreen};
use crate::systems::{Callout, SayExt};

/// How long a tilt warning is shown
const WARNING_DURATION: Duration = Duration::from_millis(1500);

/// Shows tilt warnings and tilts (DESIGN.md, Presentation). A warning flashes the speaker rings red and shows WARNING on
/// the DMD. A tilt puts every playfield light out and shows TILT until the next turn starts or the game ends.
///
/// At the root, as it outlives the tilted turn: the playfield stays dark while the balls drain.
pub struct TiltPresenter {
  warning_scene: ScenePlayer,
  warning_cue: Option<u64>,
  tilted: bool,
  screen: DmdScreen,
}

impl TiltPresenter {
  pub fn new() -> Self {
    Self {
      warning_scene: ScenePlayer::new(scene!("tilt_warning.scene")),
      warning_cue: None,
      tilted: false,
      screen: DmdScreen::new("dmd/announcement.scene", ANNOUNCEMENT_DMD_LAYER),
    }
  }

  fn warned(&mut self, remaining: u8, ctx: &SystemContext) {
    self.warning_scene.restart(ctx);
    let label = match remaining {
      0 => "LAST WARNING".to_string(),
      _ => "WARNING".to_string(),
    };
    self.show(label, ctx);
    if let Some(cue) = self.warning_cue.take() {
      ctx.cues.cancel(cue);
    }
    self.warning_cue = Some(ctx.cues.start(WarningShown, WARNING_DURATION.once()));
  }

  fn tilted(&mut self, ctx: &SystemContext) {
    self.tilted = true;
    if let Some(cue) = self.warning_cue.take() {
      ctx.cues.cancel(cue);
    }
    ctx.declare_leds(&LedQ::tag::<tags::Playfield>().at_z(50), Rgba::black());
    self.show("TILT".to_string(), ctx);
    ctx.say(Callout::ThankYouForCooperation);
  }

  /// The tilt is over: the next turn starts, or the game ends
  fn clear(&mut self, ctx: &SystemContext) {
    if self.tilted {
      self.tilted = false;
      ctx.undeclare_leds(&LedQ::tag::<tags::Playfield>().at_z(50));
      self.remove_screen(ctx);
    }
  }

  fn show(&mut self, label: String, ctx: &SystemContext) {
    self.screen.show([("text", label.into()), ("color", Rgba::red().into())], ctx);
  }

  fn remove_screen(&mut self, ctx: &SystemContext) {
    self.screen.clear(ctx);
  }
}

impl System for TiltPresenter {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(TiltWarned { remaining }) = event.downcast_ref::<TiltWarned>() {
      self.warned(*remaining, ctx);
    } else if event.is::<PlayerTilted>() {
      self.tilted(ctx);
    } else if event.is::<WarningShown>() {
      self.warning_cue = None;
      if !self.tilted {
        self.remove_screen(ctx);
      }
    } else if event.is::<RoutineStarted<TurnStarting>>() || event.is::<RoutineStarted<GameEnding>>()
    {
      self.clear(ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
struct WarningShown;
