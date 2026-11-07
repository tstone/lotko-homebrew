use frontbox_core::prelude::*;
use frontbox_dmd::*;
use frontbox_gfx::canvas::*;
use frontbox_led::prelude::*;
use frontbox_pinball::{GameEnding, PlayerTilted, TiltWarned, TurnStarting};

use crate::systems::dmd::ANNOUNCEMENT_DMD_LAYER;
use crate::systems::{Callout, SayExt};

/// How long a tilt warning is shown
const WARNING_DURATION: Duration = Duration::from_millis(1500);

/// Shows tilt warnings and tilts (DESIGN.md, Presentation). A warning flashes the speaker rings red and shows WARNING on
/// the DMD. A tilt puts every playfield light out and shows TILT until the next turn starts or the game ends.
///
/// At the root, as it outlives the tilted turn: the playfield stays dark while the balls drain.
pub struct TiltPresenter {
  warning_effect: LedPlayback,
  warning_cue: Option<u64>,
  tilted: bool,
}

impl TiltPresenter {
  pub fn new() -> Self {
    Self {
      warning_effect: led_playback!("tilt_warning.scene").stopped(),
      warning_cue: None,
      tilted: false,
    }
  }

  fn warned(&mut self, remaining: u8, ctx: &SystemContext) {
    self.warning_effect.reset();
    self.warning_effect.play();
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
    ctx.declare_leds(
      &LedQ::tag::<tags::Playfield>().at_z(50),
      ColorSequence::solid(Rgba::black()),
    );
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

  fn show(&self, label: String, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      let mut frame = Container::new(Fill2d::Solid(Rgba::black()));
      frame.add(
        BOLD_PIXELS_10PX_REGULAR_FONT
          .center_aligned(label, Rgba::red())
          .top_offset(11),
      );
      dmd.insert_layer(ANNOUNCEMENT_DMD_LAYER, frame.default_position());
    }
  }

  fn remove_screen(&self, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.systems.get::<DmdSystem>() {
      dmd.remove_layer(ANNOUNCEMENT_DMD_LAYER);
    }
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

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    self.warning_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct WarningShown;
