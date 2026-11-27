//! The game's leaderboards, kept by `LeaderboardsSystem`. See `frontbox_pinball`'s leaderboards for how they work.

use std::sync::LazyLock;

use frontbox_pinball::Leaderboard;

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
