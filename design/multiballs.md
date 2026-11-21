# Multiballs

The multiballs that aren't places: Groundswell and Heartwood. Their rules are in `DESIGN.md`.

## Groundswell

**Status:** not built.

To build:

- `GroundswellLock`, from `LiftRampStartable` in git history (`lift_ramp_startable.rs`). The Sentinels already emit `LockLit`, which nothing takes up yet. A lit lock raises the lift ramp, and the shot feeds the rear scoop. While Derailment or the Sentinels hold the ramp up, the lock waits, so their shots aren't taken as locks.
- Virtual locks until the wireform post is wired: count the lock, and let the ball carry on to the upper flipper.
- The multiball itself. The first run's super jackpot lights an extra ball.
- With real locks, the multiplayer post logic in `DESIGN.md`, using `ball_temporarily_removed_from_play` and `ball_returned_to_play`.

Where its locks come from is open, since they come from the Sentinels today (see [steward/sentinels.md](steward/sentinels.md)).

| Moment | Lighting | Sound |
|---|---|---|
| Lock lit | Lift ramp hex outer ring green | `generic/restricted-area`, `generic/skyrail-unavailable` |
| Ball locked | A pulse up the lift ramp, and a heartbeat that strengthens with each lock | `multiball/seismic-activity-alert`, a rumble |
| Start | Strobing, then a green sweep up the playfield from the roots | `multiball/seismic-event-detected`, `oak/rise` |
| Jackpot | A green wave from the shot. Speakers: the left stays city and the right grows | `multiball/seismic-damage-identified` |
| Super jackpot | A sweep up from the drops into the backbox | To record |

Display: the ground splitting and a tower toppling.

## Heartwood multiball

**Status:** built.

| Moment | Lighting | Sound |
|---|---|---|
| Start | The whole playfield on a heartbeat, in time with the track | `oak/heartwood`, a heartbeat stinger |
| Jackpot | A heartbeat wave from the shot | A double thump |

The heartbeat needs the lights to follow the music's tempo. Display: the glowing heart.

Charging it at the captive ball: a sub heartbeat and a wood knock.
