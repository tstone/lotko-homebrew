use std::collections::HashMap;

use crate::systems::game::{Mode, ModeState, Perk};

/// One player's way up the city (DESIGN.md, Primary progression). Each step has a mode or two to choose between, and
/// starting one marks it played and opens the next step. Choosing one road closes the other for the game.
#[derive(Debug, Clone)]
pub struct Progression {
  states: HashMap<Mode, ModeState>,
}

impl Progression {
  pub fn new() -> Self {
    let mut progression = Self {
      states: HashMap::new(),
    };
    progression.open(&[Mode::MeridianBasins, Mode::HydroCore]);
    progression
  }

  pub fn state(&self, mode: Mode) -> ModeState {
    self
      .states
      .get(&mode)
      .copied()
      .unwrap_or(ModeState::Unplayed)
  }

  /// What can be chosen, as the rows of the mode select menu: what's open on the route, then what's unfinished.
  /// `running` is left out, as it can't be resumed.
  pub fn available_modes(&self, running: Option<Mode>) -> Vec<Mode> {
    let with_state = |state| {
      ROUTE
        .iter()
        .flat_map(|step| step.iter().copied())
        .filter(move |mode| self.state(*mode) == state && Some(*mode) != running)
    };
    with_state(ModeState::Available)
      .chain(with_state(ModeState::Played))
      .collect()
  }

  /// Whether a mini-wizard is waiting. They light the scoop by being reached, with no qualifying.
  pub fn mini_wizard_available(&self) -> bool {
    self
      .states
      .iter()
      .any(|(mode, state)| mode.is_mini_wizard() && *state == ModeState::Available)
  }

  /// Mark a mode as started. The first time, that closes the rest of its step and opens the next one. After that it's
  /// being resumed, which changes nothing.
  pub fn start(&mut self, mode: Mode) {
    if self.state(mode) != ModeState::Available {
      return;
    }
    self.states.insert(mode, ModeState::Played);

    let step = ROUTE.iter().position(|step| step.contains(&mode));
    if let Some(step) = step {
      for other in ROUTE[step].iter().filter(|other| **other != mode) {
        self.states.insert(*other, ModeState::Unplayed);
      }
      self.open(&self.next_step(step));
    }
  }

  pub fn complete(&mut self, mode: Mode) {
    self.states.insert(mode, ModeState::Completed);
  }

  /// Districts started this game, which is what wears the Steward down
  pub fn districts_played(&self) -> usize {
    self.count_districts(|state| matches!(state, ModeState::Played | ModeState::Completed))
  }

  pub fn districts_completed(&self) -> usize {
    self.count_districts(|state| state == ModeState::Completed)
  }

  /// A perk is the player's once its district is completed
  pub fn has_perk(&self, perk: Perk) -> bool {
    self.state(perk.district()) == ModeState::Completed
  }

  fn count_districts(&self, matches: impl Fn(ModeState) -> bool) -> usize {
    Mode::DISTRICTS
      .iter()
      .filter(|mode| matches(self.state(**mode)))
      .count()
  }

  fn open(&mut self, modes: &[Mode]) {
    for mode in modes {
      self.states.insert(*mode, ModeState::Available);
    }
  }

  fn was_played(&self, mode: Mode) -> bool {
    matches!(self.state(mode), ModeState::Played | ModeState::Completed)
  }

  /// What the step after `step` offers. A mini-wizard belongs to the road taken to it.
  fn next_step(&self, step: usize) -> Vec<Mode> {
    match ROUTE.get(step + 1) {
      Some(next) if next.iter().all(Mode::is_mini_wizard) => next
        .iter()
        .copied()
        .filter(|mode| self.was_played(mode.district()))
        .collect(),
      Some(next) => next.to_vec(),
      None => Vec::new(),
    }
  }
}

const ROUTE: [&[Mode]; 6] = [
  &[Mode::MeridianBasins, Mode::HydroCore],
  &[Mode::NimbusPromenade],
  &[Mode::Floodgates, Mode::Blackout],
  &[Mode::SkyrailStation, Mode::ApexTerraces],
  &[Mode::SolariumAtrium],
  &[Mode::Derailment, Mode::Canopy],
];

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_game_opens_with_the_first_choice() {
    assert_eq!(
      Progression::new().available_modes(None),
      vec![Mode::MeridianBasins, Mode::HydroCore]
    );
  }

  #[test]
  fn starting_a_mode_closes_the_other_road_and_opens_the_next_step() {
    let mut progression = Progression::new();
    progression.start(Mode::MeridianBasins);

    assert_eq!(progression.state(Mode::MeridianBasins), ModeState::Played);
    assert_eq!(progression.state(Mode::HydroCore), ModeState::Unplayed);
    assert_eq!(
      progression.state(Mode::NimbusPromenade),
      ModeState::Available
    );
  }

  #[test]
  fn a_played_mode_is_listed_after_what_is_open_unless_it_is_running() {
    let mut progression = Progression::new();
    progression.start(Mode::HydroCore);

    assert_eq!(
      progression.available_modes(Some(Mode::HydroCore)),
      vec![Mode::NimbusPromenade]
    );
    assert_eq!(
      progression.available_modes(None),
      vec![Mode::NimbusPromenade, Mode::HydroCore]
    );
  }

  #[test]
  fn a_completed_mode_is_not_listed() {
    let mut progression = Progression::new();
    progression.start(Mode::HydroCore);
    progression.complete(Mode::HydroCore);

    assert_eq!(
      progression.available_modes(None),
      vec![Mode::NimbusPromenade]
    );
  }

  #[test]
  fn resuming_does_not_open_anything_or_undo_a_completion() {
    let mut progression = Progression::new();
    progression.start(Mode::HydroCore);
    progression.start(Mode::NimbusPromenade);
    progression.start(Mode::HydroCore);
    assert_eq!(progression.state(Mode::HydroCore), ModeState::Played);
    assert_eq!(progression.state(Mode::Blackout), ModeState::Available);

    progression.complete(Mode::HydroCore);
    progression.start(Mode::HydroCore);
    assert_eq!(progression.state(Mode::HydroCore), ModeState::Completed);
  }

  #[test]
  fn a_mini_wizard_follows_the_road_taken_to_it() {
    let mut progression = Progression::new();
    progression.start(Mode::MeridianBasins);
    assert!(!progression.mini_wizard_available());
    progression.start(Mode::NimbusPromenade);

    assert!(progression.mini_wizard_available());
    assert_eq!(progression.state(Mode::Floodgates), ModeState::Available);
    assert_eq!(progression.state(Mode::Blackout), ModeState::Unplayed);
  }

  #[test]
  fn the_whole_route_covers_six_modes() {
    let mut progression = Progression::new();
    for mode in [
      Mode::HydroCore,
      Mode::NimbusPromenade,
      Mode::Blackout,
      Mode::ApexTerraces,
      Mode::SolariumAtrium,
      Mode::Canopy,
    ] {
      assert_eq!(progression.state(mode), ModeState::Available, "{mode:?}");
      progression.start(mode);
      progression.complete(mode);
    }

    assert!(progression.available_modes(None).is_empty());
    assert_eq!(progression.districts_played(), 4);
    assert_eq!(progression.districts_completed(), 4);
    assert_eq!(progression.state(Mode::SkyrailStation), ModeState::Unplayed);
    assert_eq!(progression.state(Mode::Derailment), ModeState::Unplayed);
  }

  #[test]
  fn districts_played_counts_unfinished_ones() {
    let mut progression = Progression::new();
    progression.start(Mode::MeridianBasins);
    progression.start(Mode::NimbusPromenade);
    progression.complete(Mode::NimbusPromenade);

    assert_eq!(progression.districts_played(), 2);
    assert_eq!(progression.districts_completed(), 1);
  }

  #[test]
  fn a_perk_comes_with_completing_its_district() {
    let mut progression = Progression::new();
    progression.start(Mode::HydroCore);
    assert!(!progression.has_perk(Perk::LongerTimers));

    progression.complete(Mode::HydroCore);
    assert!(progression.has_perk(Perk::LongerTimers));
    assert!(!progression.has_perk(Perk::DoubleRoots));
  }
}
