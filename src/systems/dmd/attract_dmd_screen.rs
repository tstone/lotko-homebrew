use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttractDmdScreen {
  Spore,
  LastScores(usize),
  PressStart,
  NeonBluePinball,
  FastFrontboxLogos,
}

impl AttractDmdScreen {
  pub fn ordered() -> &'static Vec<AttractDmdScreen> {
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

static ATTRACT_DMD_STATES: LazyLock<Vec<AttractDmdScreen>> = LazyLock::new(|| {
  vec![
    AttractDmdScreen::NeonBluePinball,
    AttractDmdScreen::Spore,
    AttractDmdScreen::LastScores(0),
    AttractDmdScreen::LastScores(1),
    AttractDmdScreen::PressStart,
    AttractDmdScreen::LastScores(2),
    AttractDmdScreen::LastScores(3),
    AttractDmdScreen::FastFrontboxLogos,
  ]
});
