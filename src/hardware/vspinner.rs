use frontbox::prelude::*;
use frontbox::tags::*;
use std::sync::LazyLock;

use crate::hardware::more_tags::*;
use crate::hardware::planes;

hardware_defs! {
  pub OPTO: SwitchDefinition = SwitchDefinition::new("vspinner")
    .inverted()
    .debounce(Duration::from_millis(25))
    .tag(Playfield)
    .tag(Spinner)
    .location(Vec3::new(13.869, 18.161, 0.0).relative_to(&planes::PLAYFIELD));

  // Circular ring under the verticals pinner
  pub LEDS: LedDefinition = LedDefinition::multi("vspinner", 12)
    .tag(Spinner)
    .tag(Playfield);
}

pub mod left_ray {
  use super::*;
  hardware_defs! {
    pub LED1: LedDefinition = LedDefinition::single("vs_left_ray1")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(8.741, 18.65, 0.0).relative_to(&planes::PLAYFIELD));

    pub LED2: LedDefinition = LedDefinition::single("vs_left_ray2")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(9.882, 18.56, 0.0).relative_to(&planes::PLAYFIELD));


    pub LED3: LedDefinition = LedDefinition::single("vs_left_ray3")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(11.137, 18.458, 0.0).relative_to(&planes::PLAYFIELD));

    pub LED4: LedDefinition = LedDefinition::single("vs_left_ray4")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(12.274, 18.279, 0.0).relative_to(&planes::PLAYFIELD));
  }

  pub static Q: LazyLock<LedQ> = LazyLock::new(|| {
    LedQ::names(vec![
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
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(15.716, 14.458, 0.0).relative_to(&planes::PLAYFIELD));

    pub LED2: LedDefinition = LedDefinition::single("vs_ur_ray2")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(15.182, 15.54, 0.0).relative_to(&planes::PLAYFIELD));

    pub LED3: LedDefinition = LedDefinition::single("vs_ur_ray3")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(14.653, 16.629, 0.0).relative_to(&planes::PLAYFIELD));
  }

  pub static Q: LazyLock<LedQ> = LazyLock::new(|| {
    LedQ::names(vec![
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
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(15.936, 21.845, 0.0).relative_to(&planes::PLAYFIELD));

    pub LED2: LedDefinition = LedDefinition::single("vs_lr_ray2")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(15.411, 20.793, 0.0).relative_to(&planes::PLAYFIELD));

    pub LED3: LedDefinition = LedDefinition::single("vs_lr_ray3")
      .tag(Circle)
      .tag(Insert)
      .tag(Playfield)
      .location(Vec3::new(14.754, 19.71, 0.0).relative_to(&planes::PLAYFIELD));
  }

  pub static Q: LazyLock<LedQ> = LazyLock::new(|| {
    LedQ::names(vec![
      LED1.names()[0].clone(),
      LED2.names()[0].clone(),
      LED3.names()[0].clone(),
    ])
  });
}

#[derive(Clone)]
pub struct VerticalSpinner;

impl VerticalSpinner {
  pub fn new() -> Self {
    Self
  }
}

impl System for VerticalSpinner {
  fn on_event(&mut self, event: &dyn Event, ctx: &SystemContext) {
    if let Some(event) = event.downcast_ref::<SwitchClosed>()
      && event.switch.name == OPTO.name
    {
      ctx.emit(VerticalSpinnerHit);
    }
  }
}

#[derive(serde::Serialize, Event)]
pub struct VerticalSpinnerHit;
