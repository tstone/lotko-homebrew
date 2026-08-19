use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttractDmdState {
  Spore,
  LastScores(usize),
  PressStart,
  NeonBluePinball,
}

impl AttractDmdState {
  pub fn ordered() -> &'static Vec<AttractDmdState> {
    &ATTRACT_DMD_STATES
  }

  pub fn next(&self) -> Self {
    let mut idx = Self::ordered().iter().position(|s| s == self).unwrap();
    idx += 1;
    if idx >= Self::ordered().len() {
      idx = 0;
    }
    Self::ordered()[idx]
  }

  pub fn prev(&self) -> Self {
    let mut idx = Self::ordered().iter().position(|s| s == self).unwrap();
    if idx == 0 {
      idx = Self::ordered().len() - 1;
    } else {
      idx = idx - 1;
    }
    Self::ordered()[idx]
  }
}

static ATTRACT_DMD_STATES: LazyLock<Vec<AttractDmdState>> = LazyLock::new(|| {
  vec![
    AttractDmdState::NeonBluePinball,
    AttractDmdState::Spore,
    AttractDmdState::LastScores(0),
    AttractDmdState::LastScores(1),
    AttractDmdState::PressStart,
    AttractDmdState::LastScores(2),
    AttractDmdState::LastScores(3),
  ]
});
