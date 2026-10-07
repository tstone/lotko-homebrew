use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub SOLARIUM_ATRIUMS: LedDefinition = LedDefinition::single("sol_atriums")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(8.851, 24.224, 0.0).relative_to(&planes::PLAYFIELD));

  pub SKYRAIL_STATION: LedDefinition = LedDefinition::single("skyrail")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(6.327, 27.505, 0.0).relative_to(&planes::PLAYFIELD));

  pub NIMBUS_PROMENADE: LedDefinition = LedDefinition::single("nim_prom")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(7.773, 29.201, 0.0).relative_to(&planes::PLAYFIELD));

  pub MERIDIAN_BASINS: LedDefinition = LedDefinition::single("meridian_basin")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(5.623, 31.04, 0.0).relative_to(&planes::PLAYFIELD));

  pub HYDRO_CORE: LedDefinition = LedDefinition::single("hydro_core")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(11.829, 31.639, 0.0).relative_to(&planes::PLAYFIELD));

  pub APEX_TERRACES: LedDefinition = LedDefinition::single("apex_terraces")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(12.311, 27.882, 0.0).relative_to(&planes::PLAYFIELD));

  pub SPORE_COUNT_BAR: LedDefinition = LedDefinition::multi("spore_count")
    .locations(LedLayout::strip(16, Vec3::new(7.141, 33.849, 0.0), 0.0, 0.25).relative_to(&planes::PLAYFIELD))
    .tag(Playfield)
    .tag(CityMap);
}
