use std::cell::RefMut;
use std::collections::HashMap;

use crate::hardware::city_map;
use crate::systems::game::*;
use frontbox::prelude::*;

#[derive(Clone, Default)]
pub struct CityManager {
  active_region: Option<CityRegions>,
  active_region_effect: Option<LedEffect>,
  refresh_map: bool,
  handle: SystemHandle,
}

impl CityManager {
  // pub fn active_region(&self) -> &Option<CityRegions> {
  //   &self.active_region
  // }

  pub fn shot_amounts(&self, ctx: &ServiceContext) -> Option<HashMap<CityShot, f32>> {
    self
      .active_region_system(ctx)
      .map(|region| region.shot_amounts().clone())
  }

  pub fn apply_biospore(&self, shot: CityShot, amount: f32, ctx: &ServiceContext) {
    if let Some(mut system) = self.active_region_system(ctx) {
      system.apply_biospore(shot, amount);
    }
  }

  fn active_region_system<'a>(
    &self,
    ctx: &'a ServiceContext,
  ) -> Option<RefMut<'a, dyn CityRegion>> {
    match self.active_region {
      Some(CityRegions::MeridianBasins) => Some(ctx.expect::<MeridianBasins>(self.handle)),
      Some(CityRegions::HydroCore) => Some(ctx.expect::<HydroCore>(self.handle)),
      _ => None,
    }
  }

  fn render_map(&mut self, ctx: &SystemContext) {
    if let Some(effect) = &mut self.active_region_effect {
      effect.stop(ctx);
    }

    // meridian basins
    let meridian_basins = ctx.expect::<MeridianBasins>();
    if self.active_region == Some(CityRegions::MeridianBasins) {
      self.active_region_effect = Some(Self::active_effect(city_map::MERIDIAN_BASINS.q()));
    } else if meridian_basins.is_complete() {
      ctx.declare_leds(&city_map::MERIDIAN_BASINS.q(), Self::complete_color());
    } else if meridian_basins.is_started() {
      ctx.declare_leds(&city_map::MERIDIAN_BASINS.q(), Self::started_color());
    }

    // hydro core
    let hydro_core = ctx.expect::<HydroCore>();
    if self.active_region == Some(CityRegions::HydroCore) {
      self.active_region_effect = Some(Self::active_effect(city_map::HYDRO_CORE.q()));
    } else if hydro_core.is_complete() {
      ctx.declare_leds(&city_map::HYDRO_CORE.q(), Self::complete_color());
    } else if hydro_core.is_started() {
      ctx.declare_leds(&city_map::HYDRO_CORE.q(), Self::started_color());
    }
  }

  fn active_effect(q: HardwareQuery) -> LedEffect {
    LedEffect::flash_on_off(q, Rgba::white(), Duration::from_millis(186), Cycle::Forever)
  }

  fn complete_color() -> ColorSequence {
    ColorSequence::solid(Rgba::green())
  }

  fn started_color() -> ColorSequence {
    ColorSequence::solid(Rgba::yellow())
  }
}

impl System for CityManager {
  fn on_spawn(&mut self, ctx: &SystemContext) {
    self.handle = *ctx.current_handle();

    // spawn all the region systems
    ctx.spawn_system(MeridianBasins::new());
    ctx.spawn_system(HydroCore::new());
    // and the first qualification mode
    ctx.spawn_system(CityCoverageQualification1::new());
  }

  fn on_tick(&mut self, delta: Duration, ctx: &SystemContext) {
    if self.refresh_map {
      self.render_map(ctx);
      self.refresh_map = false;
    }

    if let Some(effect) = &mut self.active_region_effect {
      effect.apply(delta, ctx);
    }
  }

  fn on_event(&mut self, event: &dyn Event, _ctx: &SystemContext) {
    if event.is::<SporeShot>() {
      self.refresh_map = true;
    }
  }
}
