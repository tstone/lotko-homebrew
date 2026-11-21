# Skyrail Station

**Status:** built as the train, except the sounds and the DMD. Speeds and stop times are first guesses to tune by playing.

## The place

The main hub of the city's monorail, raised above the lower city next to Nimbus Promenade. The line stops in every district, but this is where every route meets. Driverless trains are always moving and always rerouting, and the departures board ticks through its services to the ring of crossing bells.

## Signature

Trains on a timetable. Fast between stations, stopped at stations, never still for long.

## Gameplay

The train is three lit shots side by side.

- It runs back and forth along the 6 main shots, stepping through them along their arc, too fast to hit. Then it stops at a station for 5 seconds.
- Each trip it picks a station at random at the end it's heading to: the left orbit or dome ramp on the left, the lift ramp or right orbit on the right. The front car stops on the station.
- While it's stopped, each car takes two hits: the first turns it cyan, and the second knocks it off. The train closes up behind its front car, so it goes from three lit shots to two to one. Shots made while it's moving don't count.
- The last car is the derailment: knocking it off completes the mode. That's 6 hits.
- The Steward fights with speed: each car lost makes the train move faster (250ms a step, then 200ms, then 150ms).
- If the player goes 20 seconds without a hit, the stops get longer, 6 seconds, until the next hit.
- Each hit of a Root Lash goes to the front car, even while the train is moving.
- Single ball. There's no add-a-ball.

## Lighting

All in `assets/leds/skyrail_station/`.

- **The train** (`train.scene`): each car is a blue glow the size of a hex, swept along a line through the hex centers, so it's seen moving between shots and lights the LEDs along the way. It eases out of one station and into the next. The front car is brighter so its direction is clear, and a car that's been hit once is cyan. While it's moving, the station it's heading to has its hex ring lit.
- **Stopped**: the cars pulse. The flashers swap back and forth in amber like crossing lights, faster as departure nears (`crossing.scene`, stretched to the stop's length).
- **Car knocked off** (`car_burst.scene`): a white-hot flash, a shockwave ring, debris thrown down toward the flippers, and the wreck smoldering orange on the hex. The flashers spin up white to orange and slow to a stop, like locked wheels throwing sparks (`flashers.scene`). The other cars close up.
- **Completion** (`complete.scene`), from the last car's shot: it flashes white and its blue stutters as the power fails, then it derails as a bright comet down the playfield to the flippers, throwing sparks. It crashes there, with a shockwave climbing the whole playfield. The six hexes light blue and close one by one, left to right, then green spreads out from the roots (the vertical spinner) over everything.

## Sound

Stand-ins are in place: `woosh-light` as the train leaves, `latch-double` as it stops, `impact-cinematic-heavy` for a car knocked off. Still needed (see TODO.md, Sound effects needed):

- Train rushing past between stations, brakes and a chime when it stops.
- Crossing bells and the station ambience under the mode.
- Rail screech and a crash for each car knocked off.

Recorded and used: `skyrail-service-suspended` and `clear-the-platform` at the start, `the-515-is-delayed` with the first car knocked off, `all-lines-are-now-closed` at completion.

## Display

- The departures board: the next station and a countdown, and each line flips to DELAYED as cars come off.
- Complete: every line DELAYED, then roots tearing up the rail.

## Open questions

- How long it stops, and how fast it gets, once it's been played.
