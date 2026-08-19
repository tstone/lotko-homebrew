use dyn_clone::DynClone;
use std::cell::RefMut;
use std::collections::HashMap;

use frontbox::prelude::*;

use crate::hardware::center_orbit::CenterOrbitHit;
use crate::hardware::left_orbit::LeftOrbitHit;
use crate::hardware::right_orbit::RightOrbitHit;
use crate::systems::game::*;

pub trait Spore: DynClone {
  fn apply(
    &mut self,
    target_shot: &CityShot,
    current: &HashMap<CityShot, f32>,
    ctx: &ServiceContext,
  ) -> HashMap<CityShot, f32>;

  fn handle_shot(&mut self, shot: CityShot, ctx: &SystemContext) {
    let city_manager = ctx.expect::<CityManager>();
    if let Some(shot_amounts) = city_manager.shot_amounts(ctx.into()) {
      for (shot, amount) in self.apply(&shot, &shot_amounts, ctx.into()) {
        city_manager.apply_biospore(shot, amount, ctx.into());
      }
    }

    ctx.emit(SporeShot(shot));
  }

  fn handle_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if event.is::<LeftOrbitHit>() {
      self.handle_shot(CityShot::LeftOrbit, ctx);
    } else if event.is::<CenterOrbitHit>() {
      self.handle_shot(CityShot::CenterOrbit, ctx);
    } else if event.is::<RightOrbitHit>() {
      self.handle_shot(CityShot::RightOrbit, ctx);
    }
  }
}

dyn_clone::clone_trait_object!(Spore);

#[derive(serde::Serialize, Event)]
pub struct SporeShot(pub CityShot);
