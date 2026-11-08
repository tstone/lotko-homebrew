# Rule Revamp Plan

What's left to get from today's code to `DESIGN.md`. Everything here is still to do: decisions to confirm, checks and
recordings, and the presentation still to build. How the game works is in `DESIGN.md`, which wins where the two
disagree.

- **Part 1** is the infra and core work everything else sits on.
- **Parts 2 to 5** are the presentation still to build for each district, mini-wizard, multiball, side quest and
  global moment, plus Groundswell and high scores.
- **Part 6** is the order to do it in.

In the presentation tables, **=** is what `DESIGN.md` asks for, buildable today, and **≈** is an approximation to ship
until the gap in the **To fully finish** note is closed. Gap IDs are in [1.5](#15-framework-gaps). Every district and
mini-wizard also needs palette SFX, a real music track and Steward integrity processing (`voice-fx`), so their sections
don't repeat it. Each mode's DMD now line is a `DmdCard` plus the mode line, unless it says otherwise.

## Decisions to confirm

The game is built with the default for each of these. Once confirmed, the answer moves into `DESIGN.md`.

| # | Question | Why it came up | Default built |
|---|---|---|---|
| 1 | When does the wireform post get wired up? | The post is physically on the machine, but its driver probably isn't wired. `io_network.rs` has no post driver and no switch on the wireform. | Build all locks as **virtual locks** until it's wired: the ball goes round to the upper flipper and the multiball launches from the trough. |
| 2 | Which shots are lit for qualifying? | `DESIGN.md` says "lit shots (white hex centers)" and "2, 3, 4…" but not which ones. | All 6 main shots start lit. A hit shot goes out. N hits lights the scoop. Lit shots reset for the next qualify. |
| 3 | Where does district progress show? | The spore bar is Charge's now. | On a mode hit the bar shows mode progress in the mode's color for 1.5s. The DMD should show progress all the time once it can. |
| 4 | Can the menu open during multiball? | The flippers drive the menu, and a hold would stop play for the other balls. | No. With more than one ball in play the scoop ejects as normal and stays lit. The menu opens the next time a single ball reaches it. |
| 5 | When does a mini-wizard end? | `DESIGN.md` gives scoring but no end condition. | When one ball is left or none. It counts as played, and if it wasn't completed it stays in the menu with its progress. |
| 6 | Does Hydro's shot 1 get a timer? | Each shot has 25s, but a timer on the first shot has nothing to restart. | Shots 2 to 5 are timed. The timer starts when shot 1 is made. |
| 7 | What about the Solarium "test lab" lines? | `test-lab-shutdown-initiated`, `atrium-containment-enacted` and `the-science-will-not-be-lost` aren't in `DESIGN.md`, while "There is nothing on the glass" hasn't been recorded. | Use them at the intensity stages and completion (see Solarium). |
| 8 | Does the center orbit grow roots? | `DESIGN.md` says "orbit hits", and the center orbit is the spinner shot. | Yes: left, center and right orbits each have a ring of roots. |
| 9 | What's the third Sentinels award, at 15 drones? | `DESIGN.md` leaves it TBD. | 15,000,000 points. |
| 12 | What counts as high overgrowth, for the Oaks' "Soon." at game over? | There's no overgrowth level yet. It's read from the player who was up last. | 3 or more districts completed. |
| 13 | Who says "extra ball" and "shoot again"? | Both voices have them recorded. | The Oaks, after the Steward's "Error. Invalid resident." The Steward's two are kept loaded as spares. |
| 14 | How many extra balls per game? | `DESIGN.md` said 2, and lotko doesn't set `MAX_EXTRA_BALLS`. | The framework default, 5. |

---

# Part 1: Infra and core

## 1.1 Checks on the machine

- [ ] Hex positions, with `HexTest`. The tip direction and the ring's winding were worked out from the code and
      artwork, not measured. Spread roots fill the rings from `hex_ring()`'s first LED, so check that too.
- [ ] Whether the illumination fix helps the open console slowdown. That link is a guess, so test it rather than
      assuming.
- [ ] The sling post rings fill from their first LED, assumed to be the bottom of each post.
- [ ] The pass lane save catches what it's meant to.
- [ ] The hand-written stand-in scenes: `shared/flash`, `shared/fill`, `shared/heartbeat`, `shared/glow`,
      `mode_progress`, `side/charge_ray`, `side/burst_ready`, `side/burst`, `apex_terraces/ring`, `blackout/lit`, and
      Nimbus's `seal_warning` (a 5Hz magenta flicker) with its green choke and white reopen flashes.
- [ ] Tune by ear: chime gap, `city_hit` steps, the 3s wait, and Meridian's gate clunk every 2.25s.

## 1.2 Hardware layer

- [ ] Give every GI LED a location, or positional scenes will skip them: `gi_l_sling`, `gi_r_sling`, `gi_drop1`,
      `gi_drop2`. `gi::LEFT_RAMP` is also defined but isn't wired to a port.
- [ ] Wire up the wireform post (decision 1):
  - Check the wiring, then connect the coil to a driver and add it to `hardware` and `io_network.rs`.
  - Check whether the machine can tell how many balls are on the post. No switch for this is defined, so it may need one.
  - Add `WireformSystem`, then switch Groundswell and Derailment from virtual to real locks.

## 1.3 DMD

Everything here must run on what exists today: the `Canvas`, pixel fonts and numbered layers. No `DmdScene` can play in
a game yet.

- [ ] **Virtual DMD.** `main.rs` skips the DMD when `VIRTUAL` is set, so none of this can be tested off the machine.
- [ ] **`DmdCard`:** a shared card with a title line, a body line and a duration. `ExtraBallSystem`, `TiltPresenter` and
      `BonusSystem` each hand-build this today. It's the stand-in for all DMD art in Parts 2 to 5.
- [ ] **Layer plan.** Tilt and extra ball share layer 2, and bonus and turn end share layer 1. Give each purpose its own
      layer: score, side quest row, mode, card, menu, tilt.
- [ ] **Score screen:** add the mode line and the side quest row with pips.
- [ ] **Mode select menu** in lotko. Take the input handling from `InitialsEntrySystem`, not the operator menu.
- [ ] **Status pages** (hold a flipper) and **idle rotation**. Flipper switches are ordinary `SwitchClosed` events.
- [ ] **Scoop eject summary.**

## 1.4 Sound, callouts and music

**Recording (Steward):** each section lists its own lines still to record. Also:

- "N of six" only has `four-of-six`, "Walkway N" only has `walkway-4`, and "Decommissioning drone N" only has 1 and 3.
- `callouts/combos` and `callouts/blackout` are empty, and there's no `callouts/canopy`.

**Music:** `ModeManager` has placeholder commercial tracks for Hydro, Skyrail, Solarium, Meridian, Nimbus, Heartwood,
Blackout, Derailment, Groundswell and the default. Apex, Floodgates and Canopy have nothing. `play_music` plays a track once, so anything longer
goes silent (gap `loop`). All original music is still to come.

- [ ] Find moments for `generic/alert`, `warning`, `scanning`, `city-scan-complete` and `seeking-biological-entity`. They
      suit the ball start scan, tilt warnings and attract.
- [ ] Find a use for `hum`, `motor1`–`5`, `scan-light2` and `scan-med`. `hum` needs `loop` to be a bed, and the motors
      aren't a stepped set, so not Hydro's rising hum. Candidates: lift ramp, glitches.

## 1.5 Framework gaps

Work in frontbox-rs that the To fully finish notes point to. All are in frontbox's `TODO.md` unless marked.

| Gap | Crate | Need |
|---|---|---|
| `growth` | gfx | Line and branch fills: spreads from a seed along an LED neighbor graph or a generated tree |
| `noise` | gfx | Simplex noise fill and noise masks |
| `glitch` | gfx | Stutter, glitch and flicker curves |
| `feather` | gfx | Soft edges on layer shapes |
| `stitch` | gfx, led | One effect moving across planes (playfield into backbox) |
| `blend` | gfx | Blend modes between layers |
| `tempo` | gfx, sound | BPM clock from music to scenes |
| `oneshot` | led | Fire-and-forget effect that outlives the system that started it. `EffectsSystem` stands in. |
| `dmd-player` | dmd | Play a `DmdScene` in a game, not just the console (partly listed) |
| `virtual-dmd` | dmd, console | DMD output in `VIRTUAL` mode |
| `dmd-stack` | dmd | Screen stack with priority, replacing numbered layers (not listed) |
| `voice-fx` | sound | Runtime dropout, wobble, stutter, bitcrush for Steward integrity. Pre-rendered variants are the alternative. |
| `loop` | sound | Loop points for music and ambient beds |
| `stems` | sound | Multi-stem music |
| `leaderboards` | pinball | Lower-is-better, typed values, shared initials entry |

## 1.6 Shared scenes

Build these once and drive them with params. `Prop::Param` works for `Translate` (`Vec2`), colors and floats, so one
scene can serve every shot. Each gets a helper in `effects.rs` that takes the LEDs, as `flash_effect`, `fill_effect`,
`heartbeat_effect` and `glow_effect` do.

| Scene | Params | Used for |
|---|---|---|
| `shared/shot_wave.scene` | `origin`, `color`, `green` | Radial hit on any shot |
| `shared/surge.scene` | `from`, `to`, `color` | Packet between two points, a `Sweep` along a `Route` |
| `shared/takeover.scene` | `origin` | Every completion (≈ growth) |
| `shared/city_scan.scene` | `color` | Steward alerts, ball start, top-down sweeps |
| `shared/arc_timer.scene` | `color`, via `.over(duration)` | Arc ramp timers, adapted from `hydro_core/progress.scene` |
| `shared/speaker_timer.scene` | `color`, via `.over(duration)` | Countdowns on the speaker rings |
| `shared/forest_ambient.scene` | `amount` | Ball start ambient (≈ noise) |

How to approximate each primitive until its gap is closed:

| Primitive | Approximation | Gap |
|---|---|---|
| Radial wave | `Ring` with animated radius on `Area("Playfield")`, as `side/burst.scene` does. A `Radial` gradient softens it. | `feather` |
| Plane sweep | `Rect` with animated `Translate`, one layer per plane with times offset by hand | `stitch` |
| Growth | A green ring expanding from the seed, with `Prop::Random` flicker. It lights by distance, not along branches. | `growth` |
| Noise field | A few large soft blobs with `Prop::Random` positions and opacity. For hero moments, a pre-rendered image in an `Image` fill. | `noise` |
| Glitch | `Curve::Random` / `Steps` and `Prop::Random` on `Opacity` and `HueShift` | `glitch` |
| Layers and masks | `LedQ` has no "all but", so "all but the roots" is a dark layer with the roots one z above it. Alpha only. | `blend` |
| Tempo sync | Set `.over()` by hand to match a track's BPM. It drifts, so keep these short. | `tempo` |

---

# Part 2: Districts

## Meridian Basins — orange

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Orange chase along an ordered `Sequence` of the 6 hex centers, settling on the first window | `root-intrusion` |
| Attention | = Rippling lit hexes. ≈ On a move, short fade-out and fade-in scenes on the leaving and arriving shots | Hum and gate clunk missing |
| Hit | = `shot_wave` in orange, with `green` rising per hit | `channel-breach`, then `four-of-six` on hit 4 |
| Intensity | ≈ 25%: `Random` opacity shimmer on GI, alternating orange and blue. ≈ 50%: same, faster. = 75%: flashers pulse | Sirens missing |
| Completion | ≈ `takeover` from the roots, then the insert goes green | `basins-reclassified`, `oak/ours` |

**DMD now:** card "MERIDIAN BASINS", mode line "BREACH 2/6".

**Still to record:** "Notice… Channel gates are closing", "N of six" for 1, 2, 3, 5, 6.

**To fully finish:** `noise` for the GI shimmer, `growth` for roots pouring through the channels, `feather` for the
water wave, and `dmd-player` plus art (basins, flowing channels, a crack, rising water, green overflow). SFX: water hum
(`loop`), lock gate clunk, water rush with concrete cracking, distant sirens.

## Hydro Core — cyan

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Coolant chase in combo order along a `Sequence` of the 5 hexes | `coolant-core-dropping` |
| Attention | = Timer moves to the arc ramp with `arc_timer`, also on the speaker rings (`progress.scene` runs it on the spore bar today). Hex flashes faster: rebuild its playback at 50% and 25% of the time left | Reactor hum missing |
| Hit | = `surge` white-hot from this shot to the next. The made shot stays green at z 0 | `city_hit` rising per shot |
| Intensity | ≈ 50%: `Random` opacity flicker on GI. = 75%: flashers as a warning strobe | `core-temperature-rising`. Klaxon missing |
| Completion | = White flash on everything. ≈ `takeover` from the subway, then GI tints green until the next mode | `core-offline`, `oak/drink` |

**DMD now:** card "HYDRO CORE", mode line with shot number, and a coolant gauge as a filled `Rectangle` driven by time
left.

**Still to record:** "Engineers to the pipes", unless `coolant-core-dropping` includes it.

**To fully finish:** `glitch` for the power instability, `growth` for the green flood from the subway, and `dmd-player`
plus art (core cutaway with 5 pipes, roots wrapping pipes, warning lights, the core going dark). SFX: a reactor hum
rising in pitch per shot (`loop`, plus pitch control or 5 stepped files), power thump, klaxon, power-down whine.

## Nimbus Promenade — magenta

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Magenta from mid-playfield out along the shots, using `Sweep`s to each hex | `promenade-closed`, `walkways-will-be-sealed` |
| Hit | = Choke: burst from center to hex, stays green | `city_hit`. Tearing SFX missing |
| Intensity | ≈ Haze: a few `Random` magenta blobs, masked to open shots. ≈ 75%: speakers on a `heartbeat` at the track's rough tempo | Detuned chimes missing |
| Completion | ≈ `takeover` in every direction, Nimbus map insert lights, then `surge`s to the Skyrail and Apex inserts | `access-to-upper-city-restricted`, `oak/the-way-is-open` |

**DMD now:** card "NIMBUS PROMENADE", and a row of 6 pips (open / sealed / choked) drawn on the canvas.

**Still to record:** "Walkway N… sealed" for 1, 2, 3, 5, 6.

**To fully finish:** `noise` for the haze, `tempo` for the speaker pulse, `growth` for the spread, and `dmd-player` plus
art (walkway map, shutters, roots knotting, clouds, the pan up). SFX: wind chimes clean and detuned, shutter slam, wooden
thump and splintering, shutter tearing.

## Skyrail Station — blue

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Blue chase up the lift ramp hex and bolt | `skyrail-service-suspended`, `clear-the-platform` |
| Completion | ≈ The chase stops dead, and blue fades while `takeover` runs from the ramp down | `all-lines-are-now-closed` |

**DMD now:** a departures board from the pixel fonts: rows of lines, flipping to DELAYED per hit.

**To fully finish:** `noise` or a dissolve mask so blue "scatters", `growth` for the takeover, and `dmd-player` plus art
(train arriving, board glitching, roots tearing up the track). SFX: station ambience (`loop`), train rushing past,
signal clank, crossing bells, rail screech and crash.

## Apex Terraces — gold

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Gold bands stepping up the playfield (`Rect` with a `Steps` curve), ending on the first shot | `residents-evacuate-now` |
| Attention | = The next segment to drain flickers | Chimes missing |
| Hit | = Burst: `shot_wave` in green, then `surge` in gold to the next shot | Creaks and stone cracking missing |
| Intensity | = Bands rise faster. = 75%: speakers go gold | Detuned chimes missing |
| Completion | ≈ Gold breaks into green: `takeover` plus `forest_ambient` over the playfield | `apex-terraces-offline`, `oak/nothing-is-yours` |

**DMD now:** card "APEX TERRACES", mode line "TERRACE 2/3" and a gauge for the current ring.

**Still to record:** "Please keep off the terraces", "Terrace three: structural concern", "This is a… premium… area",
if those are wanted over what's recorded.

**To fully finish:** `noise` for the wild green field, which is this mode's defining finish, and `dmd-player` plus art
(terraced towers, swelling hedges, a terrace bursting, towers swaying). SFX: clean expensive chimes tuned and detuned,
wet creaks deepening per segment, stone cracking and soil spilling.

## Solarium Atrium — purple

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = `city_scan` in purple, top down, then `surge` from the roots to the first shot | `atrium-integrity-verified` |
| Hit | = `shot_wave` in white as the shatter flash | `city_hit`. Glass SFX missing |
| Intensity | = White points flicker: small circles with `Random` positions, more at each stage. = 75%: GI warm purple-white | 25%: `atrium-integrity-failing`. 50%: `test-lab-shutdown-initiated`. 75%: `atrium-containment-enacted` (decision 7) |
| Completion | = White burst, then green down from the top with `city_scan` | `the-science-will-not-be-lost`, `oak/open-sky` |

**DMD now:** card "SOLARIUM ATRIUM", mode line "CRACKS 2/6". A crack can be a growing line of `Rectangle`s.

**Still to record:** "There is nothing on the… glass" with its stutter (decision 7).

**To fully finish:** `growth` for the vine trail along the LEDs between shots, `stitch` for a sweep that continues into
the backbox, and `dmd-player` plus art (dome from below, moving vine head, spreading cracks, falling glass). SFX: glass
stress groans and creaks, cracking louder per hit, a huge shatter with falling glass.

---

# Part 3: Mini-wizards

- [ ] A longer intro and a big finish for each, per `DESIGN.md`. Today they have the districts' intro, and the finish
      is `startable/hit.scene` in a color. Each needs its own track, and tempo sync is `tempo` for all four.

## Floodgates (Meridian)

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | = Fill rising from the flippers | `oak/open-the-gates`, `flood-barriers-engaged` |
| Play | ≈ Moving water surface: an edge band with `Random` height | Rushing water missing |
| Super jackpot | = `city_scan` as a wave from the top | Crash missing |

**DMD now:** a water gauge as a `Rectangle`, and the total.

**Still to record:** a variant of "Flood barriers… engaged. Engaged. Engaged", if the file doesn't already stutter.

**To fully finish:** `noise` for the surface, `tempo`, and `dmd-player` plus art for the gates opening. SFX: gates
grinding, rushing water.

## Blackout (Hydro)

| Moment | LEDs now | Sound now |
|---|---|---|
| Hit | = Shot and its section go dark in hard steps | Clunk missing |
| Super jackpot | = 1s of total dark, then a `shot_wave` from the spinner | Missing |

**DMD now:** a city grid drawn on the canvas, with sectors going black.

**Still to record:** more fragmenting Steward lines (only "System 67 invalid" so far).

**To fully finish:** `glitch` for the breakdown, `feather` for the explosion of light, `dmd-player` for the finish. SFX:
descending power-down whine, section clunks.

## Derailment (Skyrail)

- [ ] Keep the player's locked balls as virtual locks, once Groundswell has locks to keep.

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ Falling railcars: a chase down the lift ramp hex and drop strips per ball, as there are no LEDs on the wireform | `train-derailment-subverting`, `oak/fall` |
| Jackpot | ≈ Blue on the drop strips shattered by `shot_wave` in green | Rail screech missing |

**DMD now:** a card per jackpot, and train cars as sprites if time allows.

**Still to record:** "Train… train… train" (it could be cut from existing audio).

**To fully finish:** the real wireform release needs the post (decision 1), and a real falling chase needs LEDs that
don't exist. `noise` or a dissolve for the shatter, `tempo`, and `dmd-player` plus art of cars tumbling. SFX: rail
screech, metal tearing, crashes.

## Canopy (Apex)

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ `forest_ambient` climbing the playfield, then on the speakers | `oak/higher` |
| Collect | ≈ `surge` from the ramp, then a matching sweep on the speakers, timed by hand | Trunks and wind missing |

**DMD now:** the current value and a rising bar.

**Still to record:** any Steward lines.

**To fully finish:** `noise` and `stitch` (a noise field that climbs into the backbox is the whole mode's look), `tempo`,
and `dmd-player` plus art of the canopy spreading over the skyline. SFX: giant trunks creaking, a deep roar of wind.

---

# Part 4: Multiballs

## Groundswell

- [ ] Build `GroundswellLock`, from `LiftRampStartable` in git history (`lift_ramp_startable.rs`). Sentinels already
      emit `LockLit`, which nothing takes up yet. The lit lock requests the ramp up, and the shot feeds the rear scoop.
      While Skyrail or Derailment holds the ramp up, the lock light waits, so their shots aren't taken as locks.
- [ ] Virtual locks (decision 1): count the lock, and let the ball continue to the upper flipper.
- [ ] The multiball itself, per `DESIGN.md`. The first run's super jackpot lights `ExtraBallSource::Groundswell`.
- [ ] With real locks, the multiplayer post logic in `DESIGN.md`, using `ball_temporarily_removed_from_play` and
      `ball_returned_to_play`. With virtual locks there's nothing shared.

| Moment | LEDs now | Sound now |
|---|---|---|
| Lock lit | = Lift ramp hex outer ring green | `generic/restricted-area`, `generic/skyrail-unavailable` |
| Ball locked | = Pulse up the lift ramp. ≈ Ambient throb: `heartbeat` that strengthens per lock | `multiball/seismic-activity-alert`. Rumble missing |
| Start | ≈ `Random` strobing, then a green sweep up the playfield from the roots | `multiball/seismic-event-detected`, `oak/rise` |
| Jackpot | = `shot_wave` in green. Speakers: left stays city, right grows | `multiball/seismic-damage-identified` |
| Super jackpot | ≈ Sweep from the drops upward, with the backbox timed by hand | Missing |

`generic/access-to-city-restricted` is also loaded for it.

**To fully finish:** `noise` and `glitch` for the earthquake, `stitch` for the sweep into the backbox, `tempo` for the
speaker beat, `dmd-player` plus art for the splitting ground and toppling tower, and real locks once the post is wired.

## Heartwood multiball

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ Whole playfield on a `heartbeat`, timed by hand to the track | `oak/heartwood`. Heartbeat stinger missing |
| Jackpot | = Heartbeat `shot_wave` from the shot | Double-thump missing |

**To fully finish:** `tempo` (the whole point of this mode's look), and `dmd-player` plus art for the glowing heart.

---

# Part 5: Side quests, combos, saves and global moments

## Side quests

| Quest | Still to do |
|---|---|
| **Charge** | Bar's flickering edge (`Random`). The burst insert is a hand-made green and violet beat, not `heartbeat`. Swarm rustle and whoosh SFX. |
| **Surveillance** | Steady red today: add the slow scan, and a static glitch on a hit. Camera whir and lens crack SFX. |
| **Sentinels** | Strips scan rather than glow, and spark on a drone down. Drone whine SFX. |
| **Heartwood** | Sub heartbeat and wood knock SFX. |
| **Spread roots** | Collect: `surge` from the orbit to the district's map insert, and `growth` on the ring. Dirt SFX. |

## Combos

| Moment | LEDs now | Sound now |
|---|---|---|
| In progress | = Path flow toward the flipper. = Continuing shots blink green. Speaker ring countdown | Engine note missing |
| Route made | = `surge` shot to shot along the route, in green | Horn and screech missing |
| Gridlock | = All routes light, shots stall red and pulse, then green | `oak/stay`. City-wide horn jam missing |

**Still to record:** one line per route plus Gridlock.

**To fully finish:** `dmd-player` plus the route card template (8 routes). A pixel font card works until then.

## Saves

| Moment | LEDs now | Sound now |
|---|---|---|
| Outlane save growing | = Rings fill a step per hit | Creak missing |
| Outlane save used | ≈ `surge` from outlane to plunger lane, rings empty | `oak/not-yet`, `retrieval-failed` |

**To fully finish:** `growth` for the rings and surge, and `dmd-player` plus art for the roots grabbing the ball.

## Global moments

| Moment | LEDs now | Sound now | To fully finish |
|---|---|---|---|
| Attract | ≈ Existing 4 scenes, plus a city scan and a slow forest scene | none | `noise`, `growth` for roots from the spinner; themed DMD; breathing speaker rings |
| Game start | ≈ `takeover`-style pulse from the spinner, wave to the flippers | `steward-online`, `oak/wake` | Roots through the title |
| Ball start | = `city_scan`. ≈ `forest_ambient` by overgrowth level | `all-systems-within-operational-tolerance` early, `continuing-city-maintenance` later | "Overgrowth at N percent" lines, `noise` |
| Ball save | ≈ Green radial from the drain; `ball_save.scene` today | `oak/again` | Art of roots catching the ball |
| Extra ball | Done | Done | DMD art |
| Drain | ≈ Fade effects toward the roots, GI dims | `growth-contained` | `growth`, integrity on this line |
| Tilt | = Add a white snap before dark | `thank-you-for-cooperation` | Clean city font |
| Game over | ≈ Green map up to the districts taken | "Final overgrowth" missing | Summary screen first, as in 1.3 |

- [ ] The speaker rings as an overgrowth gauge, on z 0 as their own small system.

## High scores

At the end, once the modes are in.

- [ ] Register `HighScoresSystem` in `main.rs`, so scores are saved and entered, and set the custom categories at game
      end. Say `new-high-score`.
- [ ] Compare by hand for lower-is-better Fastest Blackout, and format it as a time. Proper boards are `leaderboards`.
- [ ] Record the mini-wizard totals (`MiniWizardRun::total`) and Blackout's time, which are kept but only logged.

---

# Part 6: Order of work

Each milestone leaves a playable game.

1. **DMD foundations.** `DmdCard`, the layer plan, and a virtual DMD if it can be done cheaply, since everything after
   it needs testing.
2. **Shared scenes.** Build `shot_wave`, `surge`, `takeover` and `city_scan`, then prove them on one mode.
3. **District presentation** for all six.
4. **Groundswell.**
5. **Status and idle screens**, and the side quest row.
6. **Mini-wizard presentation**, with their intros and finishes.
7. **High scores and global moments.**
8. **To fully finish.** Take the gaps in the order they unlock the most: `dmd-player`, `noise`, `growth`, `tempo`,
   `glitch`, then the rest. Audio goes alongside: fill in the Steward lines, build the palettes, then the music.
