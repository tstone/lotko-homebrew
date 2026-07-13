use frontbox::prelude::*;
use frontbox::tags::*;
use std::sync::LazyLock;

use crate::hardware::more_tags::*;

hardware_defs! {
  pub OPTO: SwitchDefinition = SwitchDefinition::new("vspinner");

  // Circular ring under the verticals pinner
  pub LEDS: LedDefinition = LedDefinition::multi("vspinner", 12)
    .tag(Playfield);
}

pub mod left_ray {
  use super::*;
  hardware_defs! {
    pub LED1: LedDefinition = LedDefinition::single("vs_left_ray1")
      .tag(Circle)
      .tag(Playfield);

    pub LED2: LedDefinition = LedDefinition::single("vs_left_ray2")
      .tag(Circle)
      .tag(Playfield);

    pub LED3: LedDefinition = LedDefinition::single("vs_left_ray3")
      .tag(Circle)
      .tag(Playfield);

    pub LED4: LedDefinition = LedDefinition::single("vs_left_ray4")
      .tag(Circle)
      .tag(Playfield);
  }

  static Q: LazyLock<HardwareQuery> = LazyLock::new(|| {
    Q::names(vec![
      LED1.names()[0].clone(),
      LED2.names()[0].clone(),
      LED3.names()[0].clone(),
      LED4.names()[0].clone(),
    ])
  });
}

pub mod upper_right_ray {
  use super::*;
  hardware_defs! {
    pub LED1: LedDefinition = LedDefinition::single("vs_ur_ray1")
      .tag(Circle)
      .tag(Playfield);

    pub LED2: LedDefinition = LedDefinition::single("vs_ur_ray2")
      .tag(Circle)
      .tag(Playfield);

    pub LED3: LedDefinition = LedDefinition::single("vs_ur_ray3")
      .tag(Circle)
      .tag(Playfield);
  }

  static Q: LazyLock<HardwareQuery> = LazyLock::new(|| {
    Q::names(vec![
      LED1.names()[0].clone(),
      LED2.names()[0].clone(),
      LED3.names()[0].clone(),
    ])
  });
}

pub mod lower_right_ray {
  use super::*;
  hardware_defs! {
    pub LED1: LedDefinition = LedDefinition::single("vs_lr_ray1")
      .tag(Circle)
      .tag(Playfield);

    pub LED2: LedDefinition = LedDefinition::single("vs_lr_ray2")
      .tag(Circle)
      .tag(Playfield);

    pub LED3: LedDefinition = LedDefinition::single("vs_lr_ray3")
      .tag(Circle)
      .tag(Playfield);
  }

  static Q: LazyLock<HardwareQuery> = LazyLock::new(|| {
    Q::names(vec![
      LED1.names()[0].clone(),
      LED2.names()[0].clone(),
      LED3.names()[0].clone(),
    ])
  });
}
