//! The game's leaderboards, kept by `LeaderboardsSystem`. See `frontbox_pinball`'s leaderboards for how they work.

use std::sync::LazyLock;

use frontbox_core::prelude::*;
use frontbox_pinball::{GameManager, Leaderboard, LeaderboardValue, LeaderboardsSystem};

use crate::systems::game::Mode;

/// Each player's score at the end of a game, filled in by `LeaderboardsSystem`
pub static HIGH_SCORES: LazyLock<Leaderboard<u64>> = LazyLock::new(|| {
  Leaderboard::highest("High Scores", 5)
    .key("high_scores")
    .rank_titles(["Grand Champion"])
    .defaults([
      ("AAA", 400_000_000),
      ("BBB", 300_000_000),
      ("CCC", 200_000_000),
      ("DDD", 150_000_000),
      ("EEE", 100_000_000),
    ])
});

/// The most Helix Vaults has scored in a game
pub static BIGGEST_FLOOD: LazyLock<Leaderboard<u64>> =
  LazyLock::new(|| Leaderboard::highest("Biggest Flood", 1).key("biggest_flood"));

/// The most Halcyon Vents has scored in a game
pub static HALCYON_VENTS_CHAMPION: LazyLock<Leaderboard<u64>> =
  LazyLock::new(|| Leaderboard::highest("Halcyon Vents Champion", 1).key("halcyon_vents_champion"));

/// The most one Groundswell has scored
pub static GROUNDSWELL_CHAMPION: LazyLock<Leaderboard<u64>> =
  LazyLock::new(|| Leaderboard::highest("Groundswell Champion", 1).key("groundswell_champion"));

/// The most one Heartwood multiball has scored
pub static HEARTWOOD_CHAMPION: LazyLock<Leaderboard<u64>> =
  LazyLock::new(|| Leaderboard::highest("Heartwood Champion", 1).key("heartwood_champion"));

/// The most times the roots were spread in a game
pub static DEEP_ROOTS: LazyLock<Leaderboard<u32>> =
  LazyLock::new(|| Leaderboard::highest("Deep Roots", 1).key("deep_roots"));

/// The most spores applied in a game
pub static SPORE_CHAMPION: LazyLock<Leaderboard<u32>> =
  LazyLock::new(|| Leaderboard::highest("Spore Champion", 1).key("spore_champion"));

/// The longest run of reflex shots
pub static ARC_STREAK_CHAMPION: LazyLock<Leaderboard<u32>> =
  LazyLock::new(|| Leaderboard::highest("ArcStreak Champion", 1).key("arc_streak_champion"));

/// The most sets of cameras blinded in a game
pub static BLIND_SPOT: LazyLock<Leaderboard<u32>> =
  LazyLock::new(|| Leaderboard::highest("Blind Spot", 1).key("blind_spot"));

/// Every board, in the order they're shown
pub fn system(path: &str) -> LeaderboardsSystem {
  LeaderboardsSystem::new(path)
    .final_scores(&HIGH_SCORES)
    .board(&BIGGEST_FLOOD)
    .board(&HALCYON_VENTS_CHAMPION)
    .board(&GROUNDSWELL_CHAMPION)
    .board(&HEARTWOOD_CHAMPION)
    .board(&DEEP_ROOTS)
    .board(&SPORE_CHAMPION)
    .board(&ARC_STREAK_CHAMPION)
    .board(&BLIND_SPOT)
}

/// The board for what a mini-wizard scores. `None` for one without a board: Caldera Generator scores much the same
/// every time, and is waiting on a board for the fastest.
pub fn mini_wizard_total(mode: Mode) -> Option<&'static Leaderboard<u64>> {
  match mode {
    Mode::HelixVaults => Some(&BIGGEST_FLOOD),
    Mode::HalcyonVents => Some(&HALCYON_VENTS_CHAMPION),
    Mode::Groundswell => Some(&GROUNDSWELL_CHAMPION),
    _ => None,
  }
}

/// Offer `value` for `board` from the player whose turn it is. Only their best is kept, so a running count or total
/// can be submitted each time it goes up.
pub fn submit<T: LeaderboardValue>(board: &Leaderboard<T>, value: T, ctx: &SystemContext) {
  let player = ctx
    .systems
    .get_dyn::<dyn GameManager>()
    .and_then(|game| Some(game.session()?.current_player()));
  let Some(player) = player else {
    return;
  };
  ctx
    .systems
    .with::<LeaderboardsSystem, _>(|leaderboards| leaderboards.submit(board, player, value));
}
