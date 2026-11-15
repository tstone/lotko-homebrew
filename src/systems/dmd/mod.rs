mod dmd_screen;
mod turn_end_score_system;

pub use dmd_screen::*;
pub use turn_end_score_system::*;

/// The DMD layer for the attract mode and game score screens
pub const BASE_DMD_LAYER: i8 = 0;

/// The DMD layer for the end-of-turn screens, above the game score
pub const TURN_END_DMD_LAYER: i8 = 1;

/// The DMD layer for announcements during a game (tilt, extra ball), above the end-of-turn screens
pub const ANNOUNCEMENT_DMD_LAYER: i8 = 2;

/// The DMD layer for the mode select menu, above everything else
pub const MENU_DMD_LAYER: i8 = 3;
