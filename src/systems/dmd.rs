use std::time::Duration;

use frontbox::{animation::*, prelude::*};
use frontbox_pin2dmd::*;
use frontbox_turn_based::GameManager;

pub struct DmdDisplay {
  dmd: Pin2Dmd,
  bold_10px: PixelFont,
  start_flasher: Box<dyn Animation<Duration, Rgba<u8>>>,
}

impl Default for DmdDisplay {
  fn default() -> Self {
    Self::new(128, 32, PanelType::Rgb)
  }
}

impl DmdDisplay {
  pub fn new(width: usize, height: usize, panel_type: PanelType) -> Self {
    let bold_10px = PixelFontBuilder::new()
      .path(local_asset("bold_pixels.png"))
      .sheet_layout(4, 16)
      .default_char_width(9)
      .custom_char_width(',', 3)
      .build();

    Self {
      dmd: Pin2Dmd::connect(width, height, panel_type).unwrap(),
      bold_10px,
      start_flasher: Tween::boxed(
        Duration::from_millis(350),
        Curve::EaseInOut,
        vec![Rgba::yellow(), Rgba::black(), Rgba::aqua(), Rgba::black()],
        AnimationCycle::Forever,
      ),
    }
  }

  fn render_in_game(&mut self, ctx: &Context) {
    let game_manager = ctx.systems.expect::<GameManager>();
    let game_state = game_manager.game_state().unwrap();
    let score = game_state.current_player_score().unwrap_or(0);

    let mut frame = Frame::for_dmd(&self.dmd);
    frame.add(
      self
        .bold_10px
        .text(format!("P{}:", game_state.current_player() + 1)),
    );
    frame.add(
      self
        .bold_10px
        .text(format!("B{}", game_state.current_player_turn() + 1))
        .top(15),
    );

    frame.add(
      self
        .bold_10px
        .text(TextFormatting::number(score))
        .recolor_vgradient(Rgba::medium_turquoise(), Rgba::dark_blue())
        .left(30),
    );

    self.dmd.render(&frame).ok();
  }

  fn render_attract(&mut self, _ctx: &Context) {
    let mut frame = Frame::for_dmd(&self.dmd);
    frame.add(
      self
        .bold_10px
        .text("PRESS START")
        .recolor(self.start_flasher.sample())
        .offset(14, 10),
    );
    self.dmd.render(&frame).ok();
  }
}

impl System for DmdDisplay {
  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    if ctx
      .systems
      .get::<GameManager>()
      .map(|gm| gm.is_game_started())
      .unwrap_or(false)
    {
      self.render_in_game(ctx);
    } else {
      self.start_flasher.accumulate(delta);
      self.render_attract(ctx);
    }
  }

  // TODO: clear DMD on_despawn
}

// TODO: how should this handle assets after compilation/not using cargo
fn local_asset(path: &str) -> String {
  format!("{}/src/assets/{}", env!("CARGO_MANIFEST_DIR"), path)
}
