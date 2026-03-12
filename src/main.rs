use frontbox::prelude::*;
use std::io::Write;

mod io_network;
use io_network::*;
mod exp_network;
use exp_network::*;

#[tokio::main]
async fn main() {
  env_logger::Builder::from_default_env()
    .format(|buf, record| writeln!(buf, "[{}] {}\r", record.level(), record.args()))
    .init();

  MachineBuilder::boot(BootConfig::default(), io_network().build(), exp_network())
    .await
    .build()
    .run(vec![LightStartBtn::new()])
    .await;
}

#[derive(Clone)]
struct LightStartBtn {
  start_button_on: bool,
  action_anim: Box<dyn Animation<Color>>,
  action_color: usize,
}

impl LightStartBtn {
  fn new() -> Box<Self> {
    Box::new(Self { 
      start_button_on: true, 
      action_anim: InterpolationAnimation::new(
        Duration::from_millis(1000),
        Curve::Linear,
        vec![Color::purple(), Color::blue(), Color::yellow()],
        AnimationCycle::Forever,
      ),
      action_color: 0,
    })
  }
}

impl System for LightStartBtn {
  fn on_startup(&mut self, _ctx: &Context, cmds: &mut Commands) {
    // TODO: this should happen in a better way, watchdog just needs to start up 
    cmds.start_game();

    // cmds.set_timer("flashing_start", Duration::from_millis(200), TimerMode::Repeating);
  }

  fn on_timer(&mut self, timer_name: &'static str, _ctx: &Context, cmds: &mut Commands) {
    if timer_name == "flashing_start" {
      self.start_button_on = !self.start_button_on;
      let trigger: DriverTriggerControlMode = match self.start_button_on {
        true => DriverTriggerControlMode::On,
        false => DriverTriggerControlMode::Off,
      };
      cmds.trigger_driver(drivers::START_BUTTON, trigger);
    }
  }

  fn on_event(&mut self, event: &dyn FrontboxEvent, _ctx: &Context, _cmds: &mut Commands) {
    handle_event!(event, {
      SwitchClosed => |e| {
        if e.switch.name == switches::DOOR_MENU_BLACK {
          if self.action_color == 4 {
            self.action_color = 0;
          } else {
            self.action_color += 1;
          }
        } else if e.switch.name == switches::DOOR_MENU_GREEN {
          if self.action_color == 0 {
            self.action_color = 4;
          } else {
            self.action_color -= 1;
          }
        }
      }
    });
  }

  fn leds(&mut self, delta_time: Duration, _ctx: &Context) -> std::collections::HashMap<&'static str, LedState> {
    let color = match self.action_color {
      0 => Color::purple(),
      1 => Color::blue(),
      2 => Color::yellow(),
      3 => Color::burly_wood(),
      _ => Color::black(),
    };
    LedDeclarationBuilder::new(delta_time)
      // .next_frame(leds::ACTION_BUTTON, &mut self.action_anim)
      // .on(switches::ACTION_BUTTON, color)
      .collect()
  }
}