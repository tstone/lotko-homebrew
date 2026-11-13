use frontbox_core::prelude::*;
use frontbox_led::prelude::*;
use frontbox_pinball::ScoopEjected;
use frontbox_sound::SoundSystemExt;
use rand::prelude::IndexedRandom;

use crate::game::solarium_atrium::MODE_COLOR;
use crate::hardware::city_map;
use crate::hardware::lower_scoop::Lower;
use crate::hardware::main_shot::{MainShot, MainShotHit};
use crate::systems::game::{
  EffectsSystem, Mode, ModeManager, ModeRun, RootLashed, SporesSystem, flash_effect, glow_effect,
};
use crate::systems::{Callout, SayExt, sounds};

static REQUIRED_HITS: u8 = 6;

/// How long the vine's head stays on a shot before it starts speeding up (start value)
const START_INTERVAL: Duration = Duration::from_millis(3500);

/// How much sooner each hit makes the vine move
const SPEEDUP: Duration = Duration::from_millis(350);

/// The fastest the vine gets
const MIN_INTERVAL: Duration = Duration::from_millis(1500);

/// Crawling vine (DESIGN.md). One main shot is lit as the vine's head, and every few seconds it moves to a neighbor,
/// never straight back unless it's at the end of the row. Where it goes next is chosen ahead of time and pre-lit dimly.
/// Hitting the head is a hit, which moves the vine on and speeds it up. A resume keeps the hits.
pub struct SolariumAtriumMode {
  run: ModeRun,
  head: MainShot,
  next: MainShot,
  head_effect: LedPlayback,
  hit_effect: LedPlayback,
  intensity_effect: LedPlayback,
  hits: u8,
  move_cue: Option<u64>,
}

impl SolariumAtriumMode {
  pub fn new() -> Self {
    let head = *MainShot::ALL
      .choose(&mut rand::rng())
      .unwrap_or(&MainShot::ArcRamp);
    Self {
      run: ModeRun::new(Mode::SolariumAtrium, REQUIRED_HITS),
      head,
      next: next_shot(head, None),
      head_effect: Self::head_effect(head),
      hit_effect: Self::hit_effect(),
      intensity_effect: Self::intensity_effect(),
      hits: 0,
      move_cue: None,
    }
  }

  fn head_effect(head: MainShot) -> LedPlayback {
    glow_effect(head.hex_center(), *MODE_COLOR)
  }

  fn hit_effect() -> LedPlayback {
    led_playback!("solarium_atrium/hit.scene").stopped()
  }

  fn complete_effect() -> LedPlayback {
    led_playback!("solarium_atrium/complete.scene").stopped()
  }

  fn intensity_effect() -> LedPlayback {
    led_playback!("solarium_atrium/intensity.scene").stopped()
  }

  fn progress_effect(hits: u8) -> LedPlayback {
    city_map::mode_progress_effect(*MODE_COLOR, hits as f32 / REQUIRED_HITS as f32)
  }

  fn interval(&self) -> Duration {
    START_INTERVAL
      .saturating_sub(SPEEDUP * self.hits as u32)
      .max(MIN_INTERVAL)
  }

  fn on_shot_hit(&mut self, shot: MainShot, ctx: &SystemContext) {
    if shot == self.head {
      let hits = ctx.systems.expect::<SporesSystem>().mode_shot(ctx.into());
      self.add_hits(hits, ctx);
    }
  }

  /// Hits on the head, or from a Root Lash, each scoring
  fn add_hits(&mut self, hits: u8, ctx: &SystemContext) {
    self.hits = (self.hits + hits).min(REQUIRED_HITS);
    log::info!("SolariumAtrium: hits = {}", self.hits);
    self.run.score_hit(hits, ctx);
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(led_playback!("solarium_atrium/flashers.scene"));
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(self.head.hex_ring(), Rgba::white()));
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(Self::progress_effect(self.hits));

    if self.hits == REQUIRED_HITS {
      self.cancel_move(ctx);
      self.head_effect.stop(ctx);
      ctx.undeclare_leds(self.next.hex_center());
      self.run.complete(Self::complete_effect(), ctx);
      return;
    }

    ctx.play_sfx(sounds::city_hit(self.hits as f32 / REQUIRED_HITS as f32));
    match self.run.progress(self.hits) {
      Some(25) => ctx.say(Callout::AtriumIntegrityFailing),
      Some(50) => {
        ctx.say(Callout::TestLabShutdownInitiated);
        self.intensity_effect.play();
      }
      Some(75) => ctx.say(Callout::AtriumContainmentEnacted),
      _ => {}
    }
    self.hit_effect.reset();
    self.hit_effect.play();
    self.move_vine(ctx);
  }

  /// The head moves to the shot that was pre-lit, leaving a fading green trail, and the next one is chosen
  fn move_vine(&mut self, ctx: &SystemContext) {
    let from = self.head;
    ctx
      .systems
      .expect::<EffectsSystem>()
      .play(flash_effect(from.hex_center(), Rgba::green()));
    self.head = self.next;
    self.next = next_shot(self.head, Some(from));
    self.render(ctx);
    self.schedule_move(ctx);
  }

  /// The head glows, and the shot it's moving to next is dimly lit beneath it
  fn render(&mut self, ctx: &SystemContext) {
    self.head_effect.stop(ctx);
    self.head_effect = Self::head_effect(self.head);
    for shot in MainShot::ALL {
      ctx.undeclare_leds(shot.hex_center());
    }
    ctx.declare_leds(
      self.next.hex_center(),
      MODE_COLOR.mix_with(Rgba::black(), 0.8),
    );
  }

  fn schedule_move(&mut self, ctx: &SystemContext) {
    self.cancel_move(ctx);
    self.move_cue = Some(ctx.cues.start(VineMoveDue, self.interval().once()));
  }

  fn cancel_move(&mut self, ctx: &SystemContext) {
    if let Some(cue) = self.move_cue.take() {
      ctx.cues.cancel(cue);
    }
  }
}

impl System for SolariumAtriumMode {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.systems.expect::<ModeManager>().current_mode() == Some(Mode::SolariumAtrium)
  }

  fn on_spawn(&mut self, ctx: &SystemContext) {
    log::info!("Solarium Atrium mode started");
    self.run.begin(ctx);
    self.render(ctx);
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx.activate_led_declarations();
    self.run.begin(ctx);
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.cancel_move(ctx);
    self.run.interrupt(ctx);
    ctx.deactivate_led_declarations();
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    self.run.on_event(event, ctx);

    if event.is::<VineMoveDue>() {
      self.move_cue = None;
      self.move_vine(ctx);
    } else if event.is::<ScoopEjected<Lower>>() && self.move_cue.is_none() {
      // the vine starts crawling once the ball is in play
      self.schedule_move(ctx);
    } else if let Some(hit) = event.downcast_ref::<MainShotHit>() {
      self.on_shot_hit(hit.shot, ctx);
    } else if let Some(lash) = event.downcast_ref::<RootLashed>() {
      self.add_hits(lash.hits, ctx);
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.run.apply(delta, ctx) {
      return;
    }
    self.head_effect.apply(delta, ctx);
    self.hit_effect.apply(delta, ctx);
    self.intensity_effect.apply(delta, ctx);
  }
}

#[derive(serde::Serialize, Event)]
struct VineMoveDue;

/// A neighbor of `head` to move to, other than the shot it came `from`, unless that's the only way to go
fn next_shot(head: MainShot, from: Option<MainShot>) -> MainShot {
  let neighbors = head.neighbors();
  let onward: Vec<MainShot> = neighbors
    .iter()
    .copied()
    .filter(|shot| Some(*shot) != from)
    .collect();
  let choices = if onward.is_empty() {
    &neighbors
  } else {
    &onward
  };
  *choices.choose(&mut rand::rng()).unwrap_or(&head)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_vine_never_turns_straight_back_mid_row() {
    for _ in 0..20 {
      assert_eq!(
        next_shot(MainShot::ArcRamp, Some(MainShot::DomeRamp)),
        MainShot::CenterOrbit
      );
    }
  }

  #[test]
  fn the_vine_turns_back_at_the_end_of_the_row() {
    assert_eq!(
      next_shot(MainShot::RightOrbit, Some(MainShot::LiftRamp)),
      MainShot::LiftRamp
    );
  }

  #[test]
  fn hits_speed_the_vine_up_to_a_limit() {
    let mut mode = SolariumAtriumMode::new();
    assert_eq!(mode.interval(), START_INTERVAL);
    mode.hits = 2;
    assert_eq!(mode.interval(), Duration::from_millis(2800));
    mode.hits = 6;
    assert_eq!(mode.interval(), MIN_INTERVAL);
  }
}
