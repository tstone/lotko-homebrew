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
  GameStart => "game-start",
  AllSystemsWithinOperationalTolerance => "all-systems-within-operational-tolerance",
  ContinuingCityMaintenance => "generic/continuing-city-maintenance",
  GrowthContained => "growth-contained",
  ThankYouForCooperation => "thank-you-for-cooperation",
  UnregisteredResident => "unregistered-resident",
  ErrorInvalidResident => "error-invalid-resident",
  RetrievalFailed => "retrieval-failed",
  AccessToCityRestricted => "generic/access-to-city-restricted",
  BiologicalContaminantIdentified => "generic/biological-contaminent-identified",
  FourOfSix => "generic/four-of-six",
  StewardCoreBreached => "final/steward-core-breached",

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
  /// How the line is said. Every line comes with the chime. A passing line is dropped if it has to wait too long, and
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
        | Self::FourOfSix
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
    )
  }
}

pub trait SayExt {
  /// Have the Steward say a line. Lines are said one at a time, and `CalloutCompleted` reports the line's `key`.
  fn say(&self, line: Callout);
}

impl<'a> SayExt for SystemContext<'a> {
  fn say(&self, line: Callout) {
    self.play_callout(line.request());
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
}
