use frontbox_canvas::{Container, Gif, Horizontal, Layer, StaticImage, Vertical};
use frontbox_pin2dmd::menu::DmdMenuSystem;
use frontbox_pin2dmd::{
  BOLD_PIXELS_10PX_REGULAR_FONT, DmdSystem, SIGI_BOLD_7PX_FONT, SIGI_REGULAR_5PX_FONT,
  TextFormatting,
};
use image::RgbaImage;
use std::path::PathBuf;

use frontbox::animation::{Accumulator, Animation, Curve, Sequence, Tween};
use frontbox::prelude::*;
use frontbox_canvas::animation::Frame;
use frontbox_turn_based::*;

use crate::hardware::cabinet::{LEFT_FLIPPER_SWITCH1, RIGHT_FLIPPER_SWITCH1};
use crate::systems::dmd::attract_dmd_state::AttractDmdState;

pub struct AttractModeDmdSystem {
  bio_spore: Vec<Frame>,
  spore_anim: Sequence<Duration, RgbaImage>,
  press_start_anim: Tween<Duration, Rgba<u8>>,
  last_scores: Option<Vec<(&'static str, u32)>>,
  state: AttractDmdState,
  cue_id: Option<u64>,
}

impl AttractModeDmdSystem {
  pub fn new() -> Self {
    let frames = Gif::decode_from_path(local_asset("gif/bio-spore-pink.gif"));
    Self {
      spore_anim: Self::rnd_animation(&frames),
      press_start_anim: Tween::new(
        Duration::from_millis(250),
        Curve::ElasticInOut,
        vec![Rgba::yellow(), Rgba::orange()],
        Cycle::Forever,
      )
      .playing(),
      bio_spore: frames,
      last_scores: None,
      state: AttractDmdState::ordered()[0],
      cue_id: None,
    }
  }

  fn draw(&mut self, delta: Duration, ctx: &SystemContext) -> Container {
    match self.state {
      AttractDmdState::Spore => self.draw_spore(delta),
      AttractDmdState::LastScores(idx) => {
        if let Some(scores) = &self.last_scores
          && let Some((text, score)) = scores.get(idx)
        {
          self.draw_last_score(text, *score)
        } else {
          self.state = self.state.next();
          self.draw(delta, ctx)
        }
      }
      AttractDmdState::NeonBluePinball => self.draw_neon_blue_pinball(),
      AttractDmdState::PressStart => {
        if let Some(game_manager) = ctx.get::<GameManager>()
          && game_manager.is_player_addable()
        {
          self.draw_press_start(delta)
        } else {
          self.state = self.state.next();
          self.draw(delta, ctx)
        }
      }
    }
  }

  fn draw_spore(&mut self, delta: Duration) -> Container {
    if self.spore_anim.is_complete() {
      self.spore_anim = Self::rnd_animation(&self.bio_spore);
    } else {
      self.spore_anim.accumulate(delta);
    }

    let img = self.spore_anim.sample();
    let w = img.width();
    let h = img.height();

    let mut window = Container::transparent();

    window.add(
      StaticImage::from_image(img)
        .width(w)
        .height(h)
        .horizontal(Horizontal::Centered)
        .vertical(Vertical::Centered),
    );

    window
  }

  fn rnd_animation(frames: &Vec<Frame>) -> Sequence<Duration, RgbaImage> {
    let end_frame = rand::random_range(1..frames.len());

    Tween::ping_pong(
      Duration::from_millis(rand::random_range(2000..=3000)),
      Curve::EaseInOut,
      vec![
        frames[0].buffer().clone(),
        frames[end_frame].buffer().clone(),
      ],
      Cycle::Times(rand::random_range(1..=2)),
    )
    .playing()
  }

  fn draw_last_score(&mut self, text: &'static str, score: u32) -> Container {
    let mut window = Container::transparent().with_padding(5, 5, 5, 5);

    window.add(
      SIGI_REGULAR_5PX_FONT
        .left_aligned(text, Rgba::white())
        .default_position(),
    );

    window.add(
      SIGI_BOLD_7PX_FONT
        .left_aligned(TextFormatting::number(score), Rgba::white())
        .top_offset(10),
    );

    window
  }

  fn draw_neon_blue_pinball(&mut self) -> Container {
    let mut window = Container::transparent();

    // TODO: replace this with a graphic
    window.add(
      BOLD_PIXELS_10PX_REGULAR_FONT
        .center_aligned("neon blue pinball", Rgba::cyan())
        .recolor_fade(Rgba::cyan(), Rgba::blue(), 45.0)
        .horizontal(Horizontal::Centered)
        .top_offset(10),
    );

    window
  }

  fn draw_press_start(&mut self, delta: Duration) -> Container {
    self.press_start_anim.accumulate(delta);

    let mut window = Container::transparent().with_padding_all(9);

    window.add(
      BOLD_PIXELS_10PX_REGULAR_FONT
        .center_aligned("Press Start", self.press_start_anim.sample())
        .default_position(),
    );

    window
  }

  fn cue_next(&mut self, ctx: &SystemContext) {
    self.cancel_cue(ctx);
    self.cue_id = Some(ctx.cue(NextScreen, Cue::Loop(Duration::from_secs(7))));
  }

  fn cancel_cue(&mut self, ctx: &SystemContext) {
    if let Some(cue_id) = self.cue_id {
      ctx.cancel_cue(cue_id);
    }
  }
}

impl System for AttractModeDmdSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    // Either the game isn't running or the menu isn't active
    !ctx.is_game_started()
      && !ctx
        .get::<DmdMenuSystem>()
        .map(|menu| menu.is_active(ctx))
        .unwrap_or(false)
  }

  fn on_deactivate(&mut self, ctx: &SystemContext) {
    self.cancel_cue(ctx);
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    self.cue_next(ctx);
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<NextScreen>() {
      self.state = self.state.next();
    } else if let Some(GameEnded { scores }) = event.downcast_ref::<GameEnded>() {
      self.last_scores = Some(scores.clone());
      // jump to last scores so players can see what they finished with
      self.state = AttractDmdState::LastScores(0);
    } else if let Some(event) = event.downcast_ref::<SwitchClosed>() {
      if event.switch.name == LEFT_FLIPPER_SWITCH1.name {
        self.state = self.state.prev();
        self.cue_next(ctx);
      } else if event.switch.name == RIGHT_FLIPPER_SWITCH1.name {
        self.state = self.state.next();
        self.cue_next(ctx);
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(mut dmd) = ctx.get::<DmdSystem>() {
      let screen = self.draw(delta, ctx);
      dmd.insert_layer(0, screen.default_position());
    }
  }
}

fn local_asset(path: &str) -> PathBuf {
  PathBuf::from(format!(
    "{}/src/assets/{}",
    env!("CARGO_MANIFEST_DIR"),
    path
  ))
}

#[derive(serde::Serialize, Event)]
struct NextScreen;
