# Skyrail Station

**Status:** built as beating the train to its stations, except the sounds, the perfect callout and the DMD. The
announce and travel times are first guesses to tune by playing.

## The place

The main hub of the city's monorail, raised above the lower city next to Nimbus Promenade. The line stops in every district, but this is where every route meets. Driverless trains are always moving and always rerouting, and the departures board ticks through its services to the ring of crossing bells.

## Signature

Trains on a timetable. The board announces the next stop, and the train is there before you know it.

## Gameplay

The stations are the 6 main shots. The forest destroys them before the city's train can reach them.

- The game picks a station, any main shot but the one the train is at. It flashes blue for 3 seconds as the train is
  announced: the train is headed here.
- Then the train leaves for it, a comet running along the hexes, and gets there in 3 seconds wherever it's coming from.
  It fades out once it's there, and the station stays lit.
- Hit the station to destroy it: double a mode hit before the train gets there, three quarters after. Then the next
  station is announced, and the train leaves from the one just destroyed.
- Destroy 4 stations to complete the mode.
- **Perfect:** destroy all 4 before the train gets there, for a bonus of half the completion on top of it, and the
  Steward's line.
- Each hit of a Root Lash destroys a station at the rate it's at, so a lash's second hit beats the train to the next
  one.
- Single ball. There's no add-a-ball.

## Lighting

All in `assets/leds/skyrail_station/`.

- **Station:** its hex center flashes blue until the train gets there, then is solid blue. A shot status, not a scene.
- **Announced** (`crossing.scene`, stretched to the 3 seconds): the flashers swap back and forth in amber like crossing
  lights, faster as departure nears.
- **The train** (`train.scene`): a bright head and four tail segments, each dimmer and a little smaller, swept along a
  line through the hex centers. They all leave together and ease in and out, each segment taking 120ms longer than the
  one ahead, so the tail stretches out as it speeds up and closes up as it arrives. It fades out as the tail catches up.
  If the station is destroyed first, it fades out where it is.
- **Station destroyed** (`station_burst.scene`): a white-hot flash, a shockwave ring, debris thrown down toward the
  flippers, and the wreck smoldering orange on the hex. The flashers spin up white to orange and slow to a stop, like
  locked wheels throwing sparks (`flashers.scene`).
- **Completion** (`complete.scene`), from the last station: it flashes white and its blue stutters as the power fails,
  then it derails as a bright comet down the playfield to the flippers, throwing sparks. It crashes there, with a
  shockwave climbing the whole playfield. The six hexes light blue and close one by one, left to right, then green
  spreads out from the roots (the vertical spinner) over everything.

## Sound

Stand-ins are in place: `woosh-light` as the train leaves, `latch-double` as it arrives, `impact-cinematic-heavy` for a station destroyed. Still needed (see TODO.md, Sound effects needed):

- Train rushing past, brakes and a chime when it arrives.
- Crossing bells while it's announced, and the station ambience under the mode.
- Rail screech and a crash for each station destroyed.

Recorded and used: `skyrail-service-suspended` and `clear-the-platform` at the start, `the-515-is-delayed` with the
first station destroyed, `all-lines-are-now-closed` at completion. A perfect says `generic/skyrail-unavailable` as a
stand-in for a line of its own.

## Display

- The departures board: the next station and a countdown, and each line flips to DELAYED as stations are destroyed.
- Complete: every line DELAYED, then roots tearing up the rail.

## Open questions

- Whether 4 stations is too easy, and how long the announce and the trip should be once it's been played.
- Whether the train should get faster with each station destroyed.
