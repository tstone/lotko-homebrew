use std::collections::HashMap;

use crate::systems::game::{Mode, ModeState, Perk};

/// One player's way up the city (DESIGN.md, Primary progression). Each step is a tier or a gate, with one or more
/// places to choose between, and starting one marks it played and opens the next step. Choosing one closes the rest
/// of its step for the game.
#[derive(Debug, Clone)]
pub struct Progression {
  states: HashMap<Mode, ModeState>,
  /// For testing: every mode can be chosen, wherever the route has got to
  all_open: bool,
}

impl Progression {
  pub fn new() -> Self {
    let mut progression = Self {
      states: HashMap::new(),
      all_open: false,
    };
    progression.open(&[Mode::MeridianBasins, Mode::CargoQuay]);
    progression
  }

  /// For testing: lists every mode that isn't completed, and lets any of them start
  pub fn open_all(&mut self) {
    self.all_open = true;
  }

  pub fn is_all_open(&self) -> bool {
    self.all_open
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
    if self.all_open {
      return ROUTE
        .iter()
        .flat_map(|step| step.iter().copied())
        .chain(OFF_ROUTE.iter().copied())
        .filter(|mode| self.state(*mode) != ModeState::Completed && Some(*mode) != running)
        .collect();
    }
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

  /// Mark a mode as started. The first time, that closes the rest of its step and opens the next one. After that it's
  /// being resumed, which changes nothing.
  pub fn start(&mut self, mode: Mode) {
    // one started off the route is marked played, so it's resumed rather than started again
    if self.all_open && self.state(mode) == ModeState::Unplayed {
      self.states.insert(mode, ModeState::Played);
      return;
    }
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

  fn next_step(&self, step: usize) -> Vec<Mode> {
    ROUTE.get(step + 1).map(|next| next.to_vec()).unwrap_or_default()
  }
}

/// The city map (design/city/README.md): a tier, then a gate, from the riverfront up to the finale
const ROUTE: [&[Mode]; 6] = [
  // tier 1, the riverfront
  &[Mode::MeridianBasins, Mode::CargoQuay],
  // gate 1
  &[Mode::NimbusPromenade],
  // tier 2, the mid-city
  &[Mode::HydroCore, Mode::HelixVaults, Mode::SkyrailStation],
  // tier 3, the mountain
  &[Mode::ApexTerraces, Mode::HalcyonVents],
  // gate 2
  &[Mode::CalderaGenerator],
  // the finale
  &[Mode::SolariumAtrium],
];

/// Modes waiting for a place on the map, which can only be played with every mode open
const OFF_ROUTE: &[Mode] = &[Mode::Derailment];

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn the_game_opens_with_the_first_choice() {
    assert_eq!(
      Progression::new().available_modes(None),
      vec![Mode::MeridianBasins, Mode::CargoQuay]
    );
  }

  #[test]
  fn starting_a_mode_closes_the_other_road_and_opens_the_next_step() {
    let mut progression = Progression::new();
    progression.start(Mode::MeridianBasins);

    assert_eq!(progression.state(Mode::MeridianBasins), ModeState::Played);
    assert_eq!(progression.state(Mode::CargoQuay), ModeState::Unplayed);
    assert_eq!(
      progression.state(Mode::NimbusPromenade),
      ModeState::Available
    );
  }

  #[test]
  fn a_played_mode_is_listed_after_what_is_open_unless_it_is_running() {
    let mut progression = Progression::new();
    progression.start(Mode::CargoQuay);

    assert_eq!(
      progression.available_modes(Some(Mode::CargoQuay)),
      vec![Mode::NimbusPromenade]
    );
    assert_eq!(
      progression.available_modes(None),
      vec![Mode::NimbusPromenade, Mode::CargoQuay]
    );
  }

  #[test]
  fn with_all_open_every_mode_but_the_running_and_completed_is_listed() {
    let mut progression = Progression::new();
    progression.open_all();
    progression.complete(Mode::HydroCore);

    let available = progression.available_modes(Some(Mode::HalcyonVents));
    assert_eq!(available.len(), 9);
    assert!(available.contains(&Mode::SolariumAtrium));
    assert!(!available.contains(&Mode::HydroCore));
    assert!(!available.contains(&Mode::HalcyonVents));
  }

  #[test]
  fn with_all_open_a_mode_off_the_route_is_marked_played_when_started() {
    let mut progression = Progression::new();
    progression.open_all();
    progression.start(Mode::SkyrailStation);
    assert_eq!(progression.state(Mode::SkyrailStation), ModeState::Played);
  }

  #[test]
  fn a_completed_mode_is_not_listed() {
    let mut progression = Progression::new();
    progression.start(Mode::CargoQuay);
    progression.complete(Mode::CargoQuay);

    assert_eq!(
      progression.available_modes(None),
      vec![Mode::NimbusPromenade]
    );
  }

  #[test]
  fn resuming_does_not_open_anything_or_undo_a_completion() {
    let mut progression = Progression::new();
    progression.start(Mode::CargoQuay);
    progression.start(Mode::NimbusPromenade);
    progression.start(Mode::CargoQuay);
    assert_eq!(progression.state(Mode::CargoQuay), ModeState::Played);
    assert_eq!(progression.state(Mode::HelixVaults), ModeState::Available);

    progression.complete(Mode::CargoQuay);
    progression.start(Mode::CargoQuay);
    assert_eq!(progression.state(Mode::CargoQuay), ModeState::Completed);
  }

  #[test]
  fn tier_2_offers_its_three_places_whichever_tier_1_place_was_played() {
    let mut progression = Progression::new();
    progression.start(Mode::MeridianBasins);
    progression.start(Mode::NimbusPromenade);

    for mode in [Mode::HydroCore, Mode::HelixVaults, Mode::SkyrailStation] {
      assert_eq!(progression.state(mode), ModeState::Available, "{mode:?}");
    }
  }

  #[test]
  fn the_whole_route_covers_six_modes() {
    let mut progression = Progression::new();
    for mode in [
      Mode::CargoQuay,
      Mode::NimbusPromenade,
      Mode::HelixVaults,
      Mode::HalcyonVents,
      Mode::CalderaGenerator,
      Mode::SolariumAtrium,
    ] {
      assert_eq!(progression.state(mode), ModeState::Available, "{mode:?}");
      progression.start(mode);
      progression.complete(mode);
    }

    assert!(progression.available_modes(None).is_empty());
    assert_eq!(progression.districts_played(), 3);
    assert_eq!(progression.districts_completed(), 3);
    assert_eq!(progression.state(Mode::SkyrailStation), ModeState::Unplayed);
    assert_eq!(progression.state(Mode::ApexTerraces), ModeState::Unplayed);
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
