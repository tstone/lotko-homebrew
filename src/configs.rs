//! Lotko's own operator settings, added to the menu in [`crate::menu`]

use std::sync::LazyLock;

use frontbox_core::prelude::*;

/// For testing a mode without playing up to it. It only takes effect in debug builds, as settings on the machine are
/// shared with release builds.
pub static ALL_MODES_OPEN: LazyLock<Config<bool>> = LazyLock::new(|| {
  Config::new(
    "All Modes Open",
    "For testing, in debug builds only: from the next game, the scoop is lit from the start and its menu has every mode",
    false,
    Toggle,
  )
  .key("all_modes_open")
});

/// Whether every mode is open for this game, which is never so in a release build
pub fn all_modes_open(ctx: &SystemContext) -> bool {
  cfg!(debug_assertions) && ALL_MODES_OPEN.get(ctx)
}
