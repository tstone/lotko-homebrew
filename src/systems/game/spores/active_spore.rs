use std::cell::RefMut;

use frontbox::prelude::*;

use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::left_orbit::LeftOrbitHit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::systems::game::*;

pub struct ActiveSpore {
  spore: Box<dyn Spore>,
}

impl Default for ActiveSpore {
  fn default() -> Self {
    Self {
      spore: Box::new(PinkSpore::new()),
    }
  }
}

impl ActiveSpore {
  fn handle_shot(&mut self, shot: CityShot, ctx: &SystemContext) {
    if let Some(mut system) = self.active_region_system(ctx) {
      let shot_amounts = system.shot_amounts();
      for (shot, amount) in self.spore.apply(&shot, &shot_amounts) {
        system.apply_biospore(shot, amount);
      }
    }

    ctx.emit(SporeShot(shot));
  }

  fn active_region_system<'a>(&self, ctx: &'a SystemContext) -> Option<RefMut<'a, dyn CityRegion>> {
    let city_manager = ctx.expect::<CityManager>();
    match city_manager.active_region() {
      Some(CityRegions::MeridianBasins) => Some(ctx.expect::<MeridianBasins>()),
      _ => None,
    }
  }
}

impl System for ActiveSpore {
  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if let Some(mut system) = self.active_region_system(ctx) {
      if let Some(shot_amounts) = self.spore.tick(delta, &system.shot_amounts()) {
        for (shot, amount) in shot_amounts {
          system.apply_biospore(shot, amount);
        }
      }
    }
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftOrbitHit>() {
      self.handle_shot(CityShot::LeftOrbit, ctx);
    } else if event.is::<CenterOrbitHit>() {
      self.handle_shot(CityShot::CenterOrbit, ctx);
    } else if event.is::<RightOrbitHit>() {
      self.handle_shot(CityShot::RightOrbit, ctx);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct SporeShot(pub CityShot);
