use frontbox::animation::{Animation, AnimationCycle, Curve, Tween};
use frontbox::plugins::{AutoplungerPlugin, TroughPlugin};
use frontbox::prelude::*;
use frontbox_sound::SoundSystem;
use frontbox_turn_based::*;
use std::io::Write;

mod io_network;
use io_network::*;
mod exp_network;
use exp_network::*;
mod systems;
use systems::*;
mod hardware;

// Tween::new(
//   Duration::from_millis(1000),
//   Curve::Linear,
//   vec![Color::purple(), Color::blue(), Color::yellow()],
//   AnimationCycle::Forever,
// )

#[tokio::main]
async fn main() {
  env_logger::Builder::from_default_env()
    .format(|buf, record| writeln!(buf, "[{}] {}\r", record.level(), record.args()))
    .init();

  App::boot("/dev/ttyACM0", "/dev/ttyACM1", io_network(), exp_network())
    .await
    .plugin(TroughPlugin::new(
      vec![switches::TROUGH_POS1, switches::TROUGH_POS2],
      drivers::TROUGH_EJECT,
    ))
    .plugin(AutoplungerPlugin::new(
      HardwareSelection::Name(switches::PLUNGE_LANE),
      HardwareSelection::Name(drivers::AUTO_PLUNGER),
    ))
    .plugin(CompetitiveGamePlugin::new(systems![BasicPoints::new()]))
    .configure(|app| {
      app.system(LedSystem::new());
      app.system(StartableFlasher::new());
      app.system(ActivatePlayfield::new());
      app.system(AutoTurnAdvance::new());

      app.system(SoundSystem::by_name("Sound Blaster").expect("Could not initialize SoundSystem"));
      app.system(Testing::new());
      app.system(DmdDisplay::default());
    })
    .run()
    .await;
}

pub struct Testing {
  speaker_anim: Box<dyn Animation<Duration, f32>>,
  speaker_alt_anim: Box<dyn Animation<Duration, f32>>,
  mode: u8,
}

impl Testing {
  pub fn new() -> Self {
    Testing {
      mode: 0,
      speaker_anim: Tween::boxed(
        Duration::from_millis(900),
        Curve::Linear,
        vec![0.0, 360.0],
        AnimationCycle::Forever,
      ),
      speaker_alt_anim: Tween::boxed(
        Duration::from_millis(80),
        Curve::Random,
        vec![0.0, 360.0],
        AnimationCycle::Forever,
      ),
    }
  }
}

impl System for Testing {
  fn on_spawn(&mut self, ctx: &Context) {
    // ctx
    //   .systems
    //   .expect::<SoundSystem>()
    //   .preload("test", "/usr/share/sounds/alsa/Front_Center.wav");

    ctx.cue(Anonymous, Cue::Once(Duration::from_millis(600)));

    ctx.declare_leds(named_led(ctx, leds::ACTION_BUTTON).color(Rgba::alice_blue()));

    ctx.declare_leds(named_led(ctx, leds::city::SOLARIUM_ATRIUMS).color(Rgba::red()));
    ctx.declare_leds(named_led(ctx, leds::city::SKYRAIL_STATION).color(Rgba::orange()));
    ctx.declare_leds(named_led(ctx, leds::city::NIMBUS_PROMENADE).color(Rgba::yellow()));
    ctx.declare_leds(named_led(ctx, leds::city::MERIDIAN_BASINS).color(Rgba::lime()));
    ctx.declare_leds(named_led(ctx, leds::LEFT_INLANE_TARGET).color(Rgba::cyan()));

    ctx.declare_leds(named_led(ctx, leds::LOWER_SCOOP_LEFT_BOLT).color(Rgba::white()));
    ctx.declare_leds(named_led(ctx, leds::LOWER_SCOOP_RIGHT_BOLT).color(Rgba::teal()));

    ctx.declare_leds(named_led(ctx, leds::GI::BOTTOM_LEFT_TRIANGLE).color(Rgba::tan()));

    ctx.cue(Anonymous, Cue::Loop(Duration::from_secs(5)));
  }

  fn on_event(&mut self, event: &dyn Event, ctx: &Context) {
    if event.is::<Anonymous>() {
      self.mode += 1;
      if self.mode == 2 {
        self.mode = 0;
      }
    }
  }

  fn on_tick(&mut self, delta: Duration, ctx: &Context) {
    self.speaker_anim.accumulate(delta);
    self.speaker_alt_anim.accumulate(delta);

    // ctx.declare_leds(
    //   named_led_strip(ctx, leds::LEFT_SPEAKER_STRIP)
    //     .gradient(Rgba::alice_blue(), Rgba::dark_blue())
    //     .rotate_left(self.speaker_anim.sample()),
    // );

    // ctx.declare_leds(
    //   named_led_strip(ctx, leds::RIGHT_SPEAKER_STRIP)
    //     .gradient(Rgba::pink(), Rgba::dark_red())
    //     .rotate_right(self.speaker_anim.sample()),
    // );

    // if self.mode == 0 {
    //   self.speaker_anim.accumulate(delta);
    //   ctx.declare_leds(
    //     named_led_strip(ctx, leds::V_SPINNER)
    //       // .gradient(Rgba::alice_blue(), Rgba::dark_blue())
    //       .color_idx(0, Rgba::dark_blue())
    //       .color_idx(6, Rgba::dark_red())
    //       .rotate_right(self.speaker_anim.sample()),
    //   );
    // } else if self.mode == 1 {
    //   self.speaker_alt_anim.accumulate(delta);
    //   ctx.declare_leds(
    //     named_led_strip(ctx, leds::V_SPINNER)
    //       // .gradient(Rgba::alice_blue(), Rgba::dark_blue())
    //       .color_idx(3, Rgba::dark_orange())
    //       .rotate_left(self.speaker_alt_anim.sample()),
    //   );
    // }
  }
}
