use std::sync::LazyLock;

use crate::hardware::*;
use frontbox_pin2dmd::menu::*;
use frontbox_pinball::configs;

pub static MENU: LazyLock<MenuSection> = LazyLock::new(|| {
  MenuSection::root()
    .section(
      MenuSection::new("Hardware")
        .section(
          MenuSection::new("Flippers")
            .section(MenuSection::new("Left Flipper").configs(left_flipper::MAIN_COIL.configs()))
            .section(MenuSection::new("Right Flipper").configs(right_flipper::MAIN_COIL.configs()))
            .section(MenuSection::new("Upper Flipper").configs(upper_flipper::MAIN_COIL.configs())),
        )
        .section(
          MenuSection::new("Slingshots")
            .configs(slingshots::LEFT_COIL.configs())
            .configs(slingshots::RIGHT_COIL.configs()),
        )
        .section(
          MenuSection::new("Pop Bumpers")
            .configs(pop_cluster::left::COIL.configs())
            .configs(pop_cluster::upper_right::COIL.configs())
            .configs(pop_cluster::lower_right::COIL.configs()),
        )
        .section(
          MenuSection::new("Trough")
            .configs(trough::COIL.configs())
            .configs(vec![configs::TROUGH_EJECT_RETRIES.display()]),
        )
        .section(MenuSection::new("Autoplunger").configs(plunge_lane::COIL.configs()))
        .section(MenuSection::new("Lower Scoop").configs(lower_scoop::COIL.configs()))
        .section(MenuSection::new("Lift Ramp").configs(lift_ramp::RAMP_COIL.configs()))
        .section(MenuSection::new("Rear Scoop").configs(lift_ramp::EJECT_COIL.configs()))
        .section(MenuSection::new("Drop Targets").configs(drop_bank::COIL.configs())),
    )
    .section(MenuSection::new("Game").configs(vec![
      configs::TURN_COUNT.display(),
      configs::BALL_SAVE_TIME.display(),
    ]))
    .section(MenuSection::new("Rules"))
    .section(MenuSection::new("Stats"))
    .terminate()
    .shutdown()
});
