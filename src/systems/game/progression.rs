use std::collections::HashMap;

use crate::systems::game::{Mode, ModeState, Perk};

/// One player's way up the city (DESIGN.md, Primary progression). Each step is a tier or a gate, with one or more
/// places to choose between, and starting one marks it played and opens the next step. Choosing one closes the rest
/// of its step for the game.
#[derive(Debug, Clone)]
pub struct Progression {
  states: HashMap<Mode, ModeState>,
  /// Times each mode on the route has been started or resumed from the menu
  plays: HashMap<Mode, u8>,
  /// For testing: every mode can be chosen, wherever the route has got to
  all_open: bool,
}

impl Progression {
  pub fn new() -> Self {
    let mut progression = Self {
      states: HashMap::new(),
      plays: HashMap::new(),
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

  /// What can be chosen, as the rows of the mode select menu: what's lit or open, an earned mode first, then what's
  /// unfinished. `running` is left out, as it can't be resumed.
  pub fn available_modes(&self, running: Option<Mode>) -> Vec<Mode> {
    let modes = || {
      EARNED
        .iter()
        .copied()
        .chain(ROUTE.iter().flat_map(|step| step.iter().copied()))
        .filter(move |mode| Some(*mode) != running)
    };
    if self.all_open {
      return modes()
        .filter(|mode| self.state(*mode) != ModeState::Completed)
        .collect();
    }
    let with_state = |state| modes().filter(move |mode| self.state(*mode) == state);
    with_state(ModeState::Available)
      .chain(with_state(ModeState::Played).filter(|mode| !self.is_out_of_tries(*mode)))
      .collect()
  }

  /// Light an earned mode ([`Mode::is_earned`]), unless it's already lit or waiting to be resumed. Returns whether it
  /// lit.
  pub fn light(&mut self, mode: Mode) -> bool {
    let unlit = self.state(mode) == ModeState::Unplayed;
    if unlit {
      self.states.insert(mode, ModeState::Available);
    }
    unlit
  }

  /// Earned modes that are lit and not started, which open the menu without qualifying
  pub fn lit_earned_modes(&self) -> Vec<Mode> {
    EARNED
      .iter()
      .copied()
      .filter(|mode| self.state(*mode) == ModeState::Available)
      .collect()
  }

  /// Mark a mode as started. The first time, that closes the rest of its step and opens the next one. After that it's
  /// being resumed, which changes nothing.
  pub fn start(&mut self, mode: Mode) {
    if !mode.is_earned() && self.state(mode) != ModeState::Completed {
      *self.plays.entry(mode).or_default() += 1;
    }
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

  /// A mode gave up its slot unfinished. An earned one can't be resumed: it goes back to unlit, to be earned again.
  /// Anything else waits to be resumed.
  pub fn end(&mut self, mode: Mode) {
    if mode.is_earned() {
      self.states.remove(&mode);
    }
  }

  /// Starts and resumes a mode on the route gets: [`TRIES`], and one more with [`Perk::ExtraTry`]
  pub fn tries(&self) -> u8 {
    TRIES + self.has_perk(Perk::ExtraTry) as u8
  }

  /// Played unfinished, with every try used, so it's lost for the game. The perk can give one back.
  pub fn is_out_of_tries(&self, mode: Mode) -> bool {
    self.state(mode) == ModeState::Played
      && self.plays.get(&mode).copied().unwrap_or(0) >= self.tries()
  }

  /// An earned mode goes back to unlit, to be earned again
  pub fn complete(&mut self, mode: Mode) {
    if mode.is_earned() {
      self.states.remove(&mode);
    } else {
      self.states.insert(mode, ModeState::Completed);
    }
  }

  /// Each of the first `steps` on the route has a completed place
  pub fn steps_completed(&self, steps: usize) -> bool {
    ROUTE.iter().take(steps).all(|step| {
      step
        .iter()
        .any(|mode| self.state(*mode) == ModeState::Completed)
    })
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
    ROUTE
      .get(step + 1)
      .map(|next| next.to_vec())
      .unwrap_or_default()
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

/// Times a mode on the route can be started or resumed: the first try and one resume
const TRIES: u8 = 2;

/// Modes lit by play rather than by the route ([`Mode::is_earned`])
const EARNED: &[Mode] = &[Mode::Groundswell];

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
    assert_eq!(progression.state(Mode::Groundswell), ModeState::Unplayed);
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

  #[test]
  fn a_lit_earned_mode_comes_first_and_closes_nothing() {
    let mut progression = Progression::new();
    assert!(progression.light(Mode::Groundswell));
    assert_eq!(progression.lit_earned_modes(), vec![Mode::Groundswell]);
    assert_eq!(
      progression.available_modes(None),
      vec![Mode::Groundswell, Mode::MeridianBasins, Mode::CargoQuay]
    );

    progression.start(Mode::Groundswell);
    assert!(progression.lit_earned_modes().is_empty());
    assert_eq!(
      progression.state(Mode::MeridianBasins),
      ModeState::Available
    );
    assert_eq!(progression.state(Mode::CargoQuay), ModeState::Available);
  }

  #[test]
  fn an_earned_mode_can_be_earned_again_once_completed() {
    let mut progression = Progression::new();
    assert!(progression.light(Mode::Groundswell));
    progression.start(Mode::Groundswell);
    assert!(!progression.light(Mode::Groundswell));
    assert_eq!(progression.state(Mode::Groundswell), ModeState::Played);

    progression.complete(Mode::Groundswell);
    assert_eq!(progression.state(Mode::Groundswell), ModeState::Unplayed);
    assert!(progression.light(Mode::Groundswell));
    assert_eq!(progression.state(Mode::Groundswell), ModeState::Available);
  }

  #[test]
  fn a_mode_can_be_resumed_once_and_nimbus_gives_another_try() {
    let mut progression = Progression::new();
    progression.start(Mode::CargoQuay);
    assert!(progression.available_modes(None).contains(&Mode::CargoQuay));
    progression.start(Mode::CargoQuay);
    assert!(progression.is_out_of_tries(Mode::CargoQuay));
    assert!(!progression.available_modes(None).contains(&Mode::CargoQuay));

    progression.start(Mode::NimbusPromenade);
    progression.complete(Mode::NimbusPromenade);
    assert!(!progression.is_out_of_tries(Mode::CargoQuay));
    progression.start(Mode::CargoQuay);
    assert!(progression.is_out_of_tries(Mode::CargoQuay));
  }

  #[test]
  fn an_earned_mode_ended_unfinished_is_not_resumed() {
    let mut progression = Progression::new();
    progression.light(Mode::Groundswell);
    progression.start(Mode::Groundswell);
    progression.end(Mode::Groundswell);
    assert_eq!(progression.state(Mode::Groundswell), ModeState::Unplayed);
    assert!(
      !progression
        .available_modes(None)
        .contains(&Mode::Groundswell)
    );

    progression.start(Mode::CargoQuay);
    progression.end(Mode::CargoQuay);
    assert_eq!(progression.state(Mode::CargoQuay), ModeState::Played);
  }

  #[test]
  fn tier_one_and_gate_one_count_once_both_are_completed() {
    let mut progression = Progression::new();
    progression.start(Mode::CargoQuay);
    progression.start(Mode::NimbusPromenade);
    progression.complete(Mode::NimbusPromenade);
    assert!(!progression.steps_completed(2));

    progression.complete(Mode::CargoQuay);
    assert!(progression.steps_completed(2));
    assert!(!progression.steps_completed(3));
  }
}
