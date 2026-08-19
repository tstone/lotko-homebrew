use std::cell::RefMut;
use std::collections::HashMap;
use std::path::Path;

use crate::hardware::city_map;
use crate::systems::game::*;
use frontbox::prelude::*;
use frontbox_sound::SoundSystem;
use frontbox_turn_based::GameManagementExt;

#[derive(Clone, Default)]
pub struct CityManager {
  active_region: Option<CityRegions>,
  active_region_effect: Option<LedProgram1d>,
  refresh_map: bool,
  handle: SystemHandle,
  spore: SporeType,
}

impl CityManager {
  pub fn activate_region(&mut self, region: CityRegions, ctx: &ServiceContext) {
    log::info!("Entering city region {:?}", region);
    self.active_region = Some(region);
    self.refresh_map = true;

    if self.active_spore_system(ctx).is_none() {
      let ctx = ctx.for_system(self.handle);
      match self.spore {
        SporeType::Pink => ctx.spawn_system(PinkSpore::new()),
      }
    }
  }

  pub fn shot_amounts(&self, ctx: &ServiceContext) -> Option<HashMap<CityShot, f32>> {
    self
      .active_region_system(ctx)
      .map(|region| region.shot_amounts().clone())
  }

  pub fn apply_biospore(&mut self, shot: CityShot, amount: f32, ctx: &ServiceContext) {
    if let Some(mut system) = self.active_region_system(ctx) {
      system.apply_biospore(shot, amount);

      if system.is_complete() {
        // TODO: fancy effects on finish
        // TODO: points should vary based on which region it was, and some other factors
        let ctx = ctx.for_system(self.handle);

        ctx.add_points(100_000);
        self.active_region = None;
        self.refresh_map = true;
        ctx.spawn_system(CityCoverageQualification1::new());
      }
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

  fn active_spore_system<'a>(&self, ctx: &'a ServiceContext) -> Option<RefMut<'a, dyn Spore>> {
    match self.spore {
      SporeType::Pink => {
        let r = ctx.get::<PinkSpore>(self.handle)?;
        Some(r)
      }
      _ => None,
    }
  }

  fn render_map(&mut self, ctx: &SystemContext) {
    if self.active_region.is_none()
      && let Some(effect) = &mut self.active_region_effect
    {
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

  fn active_effect(q: HardwareQuery) -> LedProgram1d {
    LedProgram1d::breathe(q, Rgba::white(), Cycle::Forever)
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
    ctx.spawn_system(CityCoverageQualification2::new_rnd());
  }

  fn on_reactivate(&mut self, ctx: &SystemContext) {
    ctx
      .expect::<SoundSystem>()
      .play_music(Path::new("/userdata/home/armsom/music/colyn-rushing.mp3"));
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
