use frontbox::prelude::*;
use frontbox_canvas::*;
use frontbox_pin2dmd::*;
use frontbox_turn_based::*;

// TODO: this is probably generalizable enough to include in the pin2dmd package
pub struct GamePointsDmdSystem {
  redraw: bool,
}

impl GamePointsDmdSystem {
  pub fn new() -> Self {
    Self { redraw: true }
  }

  fn draw(&self, ctx: &SystemContext) -> Container {
    let game = ctx.get::<GameManager>().unwrap();
    let game_state = game.game_state();

    match game_state {
      Some(GameState::Competitive {
        player_count,
        current_player,
        player_turns,
        player_scores,
        ..
      }) => self.draw_competitive(
        *player_count,
        player_scores,
        *current_player,
        player_turns[*current_player as usize],
      ),
      _ => todo!(),
    }
  }

  fn draw_competitive(
    &self,
    player_count: u8,
    player_scores: &Vec<u32>,
    current_player: u8,
    current_player_turn: u8,
  ) -> Container {
    let mut frame = Container::transparent().with_padding_all(1);
    let light_yellow = Rgba::yellow().lighten(0.3);

    // top row labels
    let mut current_player_row = Container::transparent();
    current_player_row.add(
      SIGI_REGULAR_5PX_FONT
        .left_aligned(format!("PLAYER {}", current_player + 1), light_yellow)
        .default_position(),
    );
    current_player_row.add(
      SIGI_REGULAR_5PX_FONT
        .left_aligned(format!("BALL {}", current_player_turn + 1), light_yellow)
        .left_offset(0.5),
    );
    frame.add(current_player_row.default_position());

    // large region for score
    frame.add(
      // TODO: use a fancier/nicer pixel font
      SIGI_BOLD_7PX_FONT
        .right_aligned(
          TextFormatting::number(player_scores[current_player as usize]),
          Rgba::white(),
        )
        .recolor_fade(Rgba::cyan().lighten(0.3), Rgba::blue().darken(0.20), 90.0)
        .right_offset(3)
        .top_offset(10),
    );

    // bottom row of individual player scores
    let mut player_scores_row = Container::transparent();
    for (i, score) in player_scores.iter().enumerate() {
      player_scores_row.add(
        SIGISH_REGULAR_4PX_FONT
          .left_aligned(
            format!("{}", TextFormatting::abbreviate_num(*score, 4)),
            Rgba::white().darken(0.4),
          )
          .left_offset(i as f32 / player_count as f32),
      );
    }
    frame.add(player_scores_row.bottom_offset(0).height(5));

    frame
  }
}

impl System for GamePointsDmdSystem {
  fn is_active(&self, ctx: &SystemContext) -> bool {
    ctx.is_game_started()
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &SystemContext) {
    if self.redraw
      && let Some(mut dmd) = ctx.get::<DmdSystem>()
    {
      dmd.insert_layer(0, self.draw(&ctx).default_position());
    }
  }
}
