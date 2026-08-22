use frontbox::prelude::*;

#[derive(Clone)]
pub struct ExclusiveModeManager {
  exclusive_mode: Option<ExclusiveMode>,
}

impl ExclusiveModeManager {
  pub fn new() -> Self {
    Self {
      exclusive_mode: None,
    }
  }

  pub fn current_mode(&self) -> &Option<ExclusiveMode> {
    &self.exclusive_mode
  }

  pub fn take_exclusive(&mut self, mode: ExclusiveMode) -> Result<(), String> {
    if self.exclusive_mode.is_some() {
      let msg = format!(
        "Cannot start {:?} because {:?} is already exclusive.",
        mode,
        self.exclusive_mode.as_ref().unwrap()
      );
      log::warn!("{}", msg);
      Err(msg)
    } else {
      self.exclusive_mode = Some(mode);
      Ok(())
    }
  }
}

impl System for ExclusiveModeManager {}

#[derive(Debug, Clone)]
pub enum ExclusiveMode {
  SolariumAtrium,
  HydroCore,
  SkyrailStation,
  MeridianBasins,
  Wizard,
}
