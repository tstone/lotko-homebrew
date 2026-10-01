use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub SOLARIUM_ATRIUMS: LedDefinition = LedDefinition::single("sol_atriums")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(9.855, 27.313, 14.742));

  pub SKYRAIL_STATION: LedDefinition = LedDefinition::single("skyrail")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(7.327, 30.537, 15.109));

  pub NIMBUS_PROMENADE: LedDefinition = LedDefinition::single("nim_prom")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(8.776, 32.245, 15.304));

  pub MERIDIAN_BASINS: LedDefinition = LedDefinition::single("meridian_basin")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(6.627, 34.091, 15.514));

  pub HYDRO_CORE: LedDefinition = LedDefinition::single("hydro_core")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(12.851, 34.672, 15.58));

  pub APEX_TERRACES: LedDefinition = LedDefinition::single("apex_terraces")
    .tag(Playfield)
    .tag(Insert)
    .tag(Circle)
    .tag(CityMap)
    .location(Vec3::new(13.346, 30.964, 15.158));

  pub SPORE_COUNT_BAR: LedDefinition = LedDefinition::strip("spore_count", 16)
    .tag(Playfield)
    .tag(CityMap);
}
