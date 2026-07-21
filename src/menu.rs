use std::sync::LazyLock;

use crate::hardware::*;
use frontbox_pin2dmd::menu::*;

pub static MENU: LazyLock<MenuSection> = LazyLock::new(|| {
  MenuSection::root()
    .section(
      MenuSection::new("HARDWARE")
        .section(MenuSection::new("Flippers"))
        .configs(left_flipper::MAIN_COIL.generalized_config_values())
        .configs(left_flipper::HOLD_COIL.generalized_config_values())
        .configs(right_flipper::MAIN_COIL.generalized_config_values())
        .configs(right_flipper::HOLD_COIL.generalized_config_values())
        .section(MenuSection::new("Trough"))
        .configs(trough::COIL.generalized_config_values()),
    )
    .section(MenuSection::new("Game"))
    .section(MenuSection::new("Rules"))
});
