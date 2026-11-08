use frontbox_core::prelude::*;
use frontbox_sound::{CalloutRequest, SoundSystemExt};

use crate::systems::sounds;

/// How long a line about what just happened can wait for its turn before it's old news
const PASSING_LINE_WAIT: Duration = Duration::from_secs(3);

macro_rules! callouts {
  ($($name:ident => $file:literal,)*) => {
    /// Everything the Steward can say. Each line is a file in `assets/callouts`, which is also its sound key.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Callout {
      $($name,)*
    }

    impl Callout {
      pub const ALL: &[Callout] = &[$(Self::$name,)*];

      pub fn key(self) -> &'static str {
        match self {
          $(Self::$name => $file,)*
        }
      }

      pub fn bytes(self) -> &'static [u8] {
        match self {
          $(Self::$name => include_bytes!(concat!("../assets/callouts/", $file, ".mp3")),)*
        }
      }
    }
  };
}

callouts! {
  // global moments
  GameStart => "steward-online",
  AllSystemsWithinOperationalTolerance => "all-systems-within-operational-tolerance",
  ContinuingCityMaintenance => "generic/continuing-city-maintenance",
  GrowthContained => "growth-contained",
  ThankYouForCooperation => "thank-you-for-cooperation",
  UnregisteredResident => "unregistered-resident",
  ErrorInvalidResident => "error-invalid-resident",
  RetrievalFailed => "retrieval-failed",
  ExtraBall => "extra-ball",
  SamePlayerShootAgain => "same-player-shoot-again",
  NewHighScore => "new-high-score",
  AccessToCityRestricted => "generic/access-to-city-restricted",
  BiologicalContaminantIdentified => "generic/biological-contaminent-identified",
  StewardCoreBreached => "final/steward-core-breached",
  Alert => "generic/alert",
  Warning => "generic/warning",
  Scanning => "generic/scanning",
  CityScanComplete => "generic/city-scan-complete",
  SeekingBiologicalEntity => "generic/seeking-biological-entity",
  RestrictedArea => "generic/restricted-area",
  SkyrailUnavailable => "generic/skyrail-unavailable",

  // words for building lines with `say_all`
  One => "generic/one",
  Two => "generic/two",
  Three => "generic/three",
  Four => "generic/four",
  Five => "generic/five",
  Six => "generic/six",
  Seven => "generic/seven",
  Eight => "generic/eight",
  Of => "generic/of",

  // side quests
  AirborneContaminantDetected => "side/airborne-contantaminant-detected",
  CameraNetworkOffline => "side/camera-network-offline",
  DecommissioningDrones => "side/decomissioning-drone",
  DecommissioningDroneOne => "side/decomissioning-drone-1",
  DecommissioningDroneTwo => "side/decomissioning-drone-2",
  DecommissioningDroneThree => "side/decomissioning-drone-3",
  DecommissioningDroneFour => "side/decomissioning-drone-4",
  DroneDisengaged => "side/drone-disengaged",
  SurveillanceEnabled => "side/surveillance-enabled",

  // multiballs
  SeismicEventDetected => "multiball/seismic-event-detected",
  SeismicActivityAlert => "multiball/seismic-activity-alert",
  SeismicDamageIdentified => "multiball/seismic-damage-identified",

  // meridian basins
  RootIntrusion => "meridian/root-intrusion",
  ChannelBreach => "meridian/channel-breach",
  BasinsReclassified => "meridian/basins-reclassified",
  MeridianBasinsUninhabitable => "meridian/meridian-basins-uninhabitable",

  // hydro core
  CoolantCoreDropping => "hydro-core/coolant-core-dropping",
  CoreTemperatureRising => "hydro-core/core-temperature-rising",
  CoreOffline => "hydro-core/core-offline",
  HydroCoreOffline => "hydro-core/hydro-core-offline",

  // nimbus promenade
  PromenadeClosed => "nimbus/promenade-closed",
  WalkwaysWillBeSealed => "nimbus/walkways-will-be-sealed",
  WalkwayFourSealed => "nimbus/walkway-4-sealed",
  AccessToUpperCityRestricted => "nimbus/access-to-upper-city-restricted",
  NimbusPromenadeOffline => "nimbus/nimbus-promenade-offline",

  // skyrail station
  SkyrailServiceSuspended => "skyrail/skyrail-service-suspended",
  ClearThePlatform => "skyrail/clear-the-platform",
  The515IsDelayed => "skyrail/the-515-is-delayed",
  AllLinesAreNowClosed => "skyrail/all-lines-are-now-closed",

  // apex terraces
  ResidentsEvacuateNow => "apex/residents-evacuate-now",
  ThisIsAnExclusiveArea => "generic/this-is-an-exclusive-area",
  WarningStructuralConcern => "apex/warning-structural-concern",
  ApexTerracesOffline => "apex/apex-terraces-offline",

  // solarium atrium
  AtriumIntegrityVerified => "solarium/atrium-integrity-verified",
  AtriumIntegrityFailing => "solarium/atrium-integrity-failing",
  TestLabShutdownInitiated => "solarium/test-lab-shutdown-initiated",
  AtriumContainmentEnacted => "solarium/atrium-containment-enacted",
  TheScienceWillNotBeLost => "solarium/the-science-will-not-be-lost",

  // mini-wizards
  FloodBarriersEngaged => "floodgates/flood-barriers-engaged",
  TrainDerailmentSubverting => "derailment/train-derailment-subverting",
}

impl Callout {
  /// The word for a number from 1 to 8, the ones recorded
  pub fn number(n: u8) -> Option<Callout> {
    const NUMBERS: [Callout; 8] = [
      Callout::One,
      Callout::Two,
      Callout::Three,
      Callout::Four,
      Callout::Five,
      Callout::Six,
      Callout::Seven,
      Callout::Eight,
    ];
    NUMBERS.get((n as usize).checked_sub(1)?).copied()
  }

  /// How the line is said. A run of lines starts with the chime. A passing line is dropped if it has to wait too long, and
  /// any other line takes the place of the passing lines waiting ahead of it.
  fn request(self) -> CalloutRequest {
    let line = CalloutRequest::new(self.key()).lead_in(sounds::STEWARD_CHIME);
    if self.is_passing() {
      line.max_wait(PASSING_LINE_WAIT)
    } else {
      line.priority(1)
    }
  }

  /// Commentary on play, as opposed to a moment that the player should always hear
  fn is_passing(self) -> bool {
    matches!(
      self,
      Self::AllSystemsWithinOperationalTolerance
        | Self::ContinuingCityMaintenance
        | Self::ChannelBreach
        | Self::MeridianBasinsUninhabitable
        | Self::CoreTemperatureRising
        | Self::WalkwayFourSealed
        | Self::The515IsDelayed
        | Self::ThisIsAnExclusiveArea
        | Self::WarningStructuralConcern
        | Self::AtriumIntegrityFailing
        | Self::TestLabShutdownInitiated
        | Self::AtriumContainmentEnacted
        | Self::DroneDisengaged
        | Self::SeismicDamageIdentified
        | Self::One
        | Self::Two
        | Self::Three
        | Self::Four
        | Self::Five
        | Self::Six
        | Self::Seven
        | Self::Eight
    )
  }
}

pub trait SayExt {
  /// Have the Steward say a line. Lines are said one at a time, and `CalloutCompleted` reports the line's `key`.
  fn say(&self, line: Callout);

  /// Have the Steward say several lines back to back as one, such as "four", "of", "six". It's said the way the first
  /// is, and `CalloutCompleted` reports the first's `key`.
  fn say_all(&self, lines: &[Callout]);

  /// "Four of six", said as passing commentary. Says nothing for a number that wasn't recorded.
  fn say_count(&self, count: u8, of: u8) {
    if let (Some(count), Some(of)) = (Callout::number(count), Callout::number(of)) {
      self.say_all(&[count, Callout::Of, of]);
    }
  }
}

impl<'a> SayExt for SystemContext<'a> {
  fn say(&self, line: Callout) {
    self.play_callout(line.request());
  }

  fn say_all(&self, lines: &[Callout]) {
    let Some((first, rest)) = lines.split_first() else {
      return;
    };
    let request = rest
      .iter()
      .fold(first.request(), |request, line| request.then(line.key()));
    self.play_callout(request);
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;

  #[test]
  fn each_line_is_its_own_sound() {
    let keys: HashSet<_> = Callout::ALL.iter().map(|line| line.key()).collect();
    assert_eq!(keys.len(), Callout::ALL.len());
  }

  #[test]
  fn only_the_recorded_numbers_have_words() {
    assert_eq!(Callout::number(0), None);
    assert_eq!(Callout::number(1), Some(Callout::One));
    assert_eq!(Callout::number(8), Some(Callout::Eight));
    assert_eq!(Callout::number(9), None);
  }
}
