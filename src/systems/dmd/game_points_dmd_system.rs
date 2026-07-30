use frontbox::prelude::*;
use frontbox_canvas::*;
use frontbox_pin2dmd::*;
use frontbox_turn_based::*;

// TODO: this is probably generalizable enough to include in the pin2dmd pafckage
pub struct GamePointsDmdSystem {
  redraw: bool,
}

impl GamePointsDmdSystem {
  pub fn new() -> Self {
    Self { redraw: true }
  }

  fn draw(&self, ctx: &Context) -> Container {
    let game = ctx.systems.get::<GameManager>().unwrap();
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
        .text(format!("PLAYER {}", current_player + 1), light_yellow, 1)
        .default_position(),
    );
    current_player_row.add(
      SIGI_REGULAR_5PX_FONT
        .text(format!("BALL {}", current_player_turn + 1), light_yellow, 1)
        .left_offset(0.5),
    );
    frame.add(current_player_row.default_position());

    // large region for score
    frame.add(
      // TODO: use a fancier/nicer pixel font
      // TODO: this should be right-aligned
      SIGI_BOLD_7PX_FONT
        .text(
          TextFormatting::number(player_scores[current_player as usize]),
          Rgba::white(),
          1,
        )
        .recolor_fade(Rgba::cyan().lighten(0.2), Rgba::blue().darken(0.20), 90.0)
        .left_offset(3)
        .top_offset(9),
    );

    // bottom row of individual player scores
    if player_count > 1 {
      // showable scores alnog the bottom
      let scores: Vec<u32> = player_scores[..current_player as usize]
        .iter()
        .chain(&player_scores[current_player as usize + 1..player_count as usize])
        .copied()
        .collect();

      let mut player_scores_row = Container::transparent();
      for (i, score) in scores.iter().enumerate() {
        player_scores_row.add(
          SIGI_REGULAR_5PX_FONT
            // TODO: fix the player number. it no longer matches
            .text(
              format!("P{} {}", i + 1, TextFormatting::abbreviate_num(*score, 4)),
              Rgba::white().darken(0.4),
              1,
            )
            .left_offset(i as f32 / scores.len() as f32),
        );
      }
      frame.add(player_scores_row.bottom_offset(0).height(5));
    }

    frame
  }
}

impl System for GamePointsDmdSystem {
  fn is_active(&self, ctx: &Context) -> bool {
    ctx.is_game_started()
  }

  fn on_tick(&mut self, _delta: Duration, ctx: &Context) {
    if self.redraw
      && let Some(mut dmd) = ctx.systems.get::<DmdSystem>()
    {
      dmd.insert_layer(0, self.draw(&ctx).default_position());
    }
  }
}
