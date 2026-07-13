use frontbox::prelude::*;
use frontbox::tags::*;

use crate::hardware::more_tags::*;

const NAME: &'static str = "l_inlane";

hardware_defs! {
  pub SOLARIUM_ATRIUMS: LedDefinition = LedDefinition::single("sol_atriums")
    .tag(Playfield)
    .tag(Circle)
    .tag(CityMap);

  pub SKYRAIL_STATION: LedDefinition = LedDefinition::single("skyrail")
    .tag(Playfield)
    .tag(Circle)
    .tag(CityMap);

  pub NIMBUS_PROMENADE: LedDefinition = LedDefinition::single("nim_prom")
    .tag(Playfield)
    .tag(Circle)
    .tag(CityMap);

  pub MERIDIAN_BASINS: LedDefinition = LedDefinition::single("meridian_basin")
    .tag(Playfield)
    .tag(Circle)
    .tag(CityMap);

  pub HYDRO_CORE: LedDefinition = LedDefinition::single("hydro_core")
    .tag(Playfield)
    .tag(Circle)
    .tag(CityMap);

  pub APEX_TERRACES: LedDefinition = LedDefinition::single("apex_terraces")
    .tag(Playfield)
    .tag(Circle)
    .tag(CityMap);
}
