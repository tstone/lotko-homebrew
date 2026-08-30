use frontbox::prelude::*;

use crate::systems::game::ExclusiveMode;

pub trait ExclusiveModeStarter: Send + Sync + 'static {
  const START_SND_KEY: &'static str;
  const MODE: ExclusiveMode;

  fn on_start(ctx: &SystemContext);
  fn mode_color() -> Rgba<u8>;
}
