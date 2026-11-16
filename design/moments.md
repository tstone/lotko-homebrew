# Moments

The moments around the places: attract, the start and end of a game and a ball, saves and tilts.

| Moment | Lighting | Sound | Display |
|---|---|---|---|
| Attract | A slow forest field. Now and then roots grow out of the spinner over the city map and pull back, and city scan lines sweep down from the top | None | The title, the skyline slowly covered by roots, and high scores |
| Game start | A pulse from the spinner, and a wave to the flippers | `steward-online`, `oak/wake` | Roots through the title |
| Ball start | A city scan, then the forest's presence by how overgrown the city is | `all-systems-within-operational-tolerance` early, `continuing-city-maintenance` later. "Overgrowth at N percent" lines to record | |
| Ball save | Green spreading up from the drain | `oak/again` | Roots catching the ball |
| Extra ball | Built | Built | Art to make |
| Drain | Light fading toward the roots, and the GI dimming | `growth-contained` | |
| Tilt | A white snap, then dark | `thank-you-for-cooperation` | A clean city font |
| Game over | The map lit green up to the places taken | "Final overgrowth" to record | A summary screen first |

## Outlane saves

| Moment | Lighting | Sound | Display |
|---|---|---|---|
| Growing | The roots climb a step up that side's post rings with each sling hit | A creak | |
| Used | A surge from the outlane to the plunger lane, and the rings empty | `oak/not-yet`, `retrieval-failed` | Roots grabbing the ball |

## Speaker rings

Each backbox speaker has a ring of 25 LEDs, the one place a player looks up at besides the DMD.

- **Overgrowth gauge.** Between modes, both rings show the city's state for the current player: city white-blue, with green spreading in from the bottom as places are taken. Not built yet.
- **Countdowns.** A ring drains for anything on a clock: the combo window, the menu timeout, Hydro's shot timer, the ball save and hurry-ups.
- **Beat.** In multiballs the rings pulse with the kick drum.
- **Left is city, right is forest.** For moments that are a fight, such as Groundswell's jackpots.
- **Warnings.** Both flash red once for a tilt warning, and go dark with the GI on a tilt.

## Unused sounds

Recorded or loaded, but not used yet:

- Callouts: `generic/alert`, `warning`, `scanning`, `city-scan-complete`, `seeking-biological-entity`. They'd suit the ball start scan, tilt warnings and attract.
- Effects: `hum`, `motor1`–`5`, `scan-light2`, `scan-med`. Candidates: the lift ramp, and glitches.
