# Rule Revamp Plan

What's left to get from today's code to `DESIGN.md`. Finished work is taken out of this file as it's done, so
everything here is still to do. `DESIGN.md` stays the record of what the game should be. Where the two disagree on what
the game *is*, `DESIGN.md` wins, unless "DESIGN.md items to revisit" below says otherwise.

How it's laid out:

- **Part 1** is the infra and core work everything else sits on.
- **Part 2** has one section per district. **Part 3** has one per mini-wizard.
- **Parts 4 and 5** cover what isn't a region but still has to be built: multiballs, side quests, combos, saves and
  global moments.
- **Part 6** is the order to do it in.

Mode sections give what's left of the rules, the presentation to build, and a **To fully finish** note for the gap
between the approximation and `DESIGN.md`. Those notes use the gap IDs from [1.6](#16-framework-gaps).

In the presentation tables:

- **=** means this matches `DESIGN.md` with what the framework can do today.
- **≈** means it's an approximation. Ship it, and the mode's To fully finish note says what replaces it.

Callout text comes from the file names. Nobody has checked the audio against these lists. Timings and counts that
`DESIGN.md` leaves open are starting points to tune on the machine.

## Decisions to confirm

The game is built with the default for each of these. Each one changes something real, so they're worth confirming.

| # | Question | Why it came up | Default built |
|---|---|---|---|
| 1 | When does the wireform post get wired up? | The post is physically on the machine, but its driver probably isn't wired. `io_network.rs` has no post driver and no switch on the wireform, and the rear scoop eject and upper flipper are all that's defined up there. | Build all locks as **virtual locks** until it's wired: the ball goes round to the upper flipper and the multiball launches from the trough. Real locks then replace them (see 1.2). |
| 2 | Which shots are lit for qualifying? | `DESIGN.md` says "lit shots (white hex centers)" and "2, 3, 4…" but not which ones. | All 6 main shots start lit. A hit shot goes out. N hits lights the scoop. Lit shots reset for the next qualify. |
| 3 | Where does district progress show? | Modes used the spore bar for progress, and Charge owns it now. | The bar is Charge's. On a mode hit it shows mode progress in the mode's color for 1.5s, then goes back to Charge (`city_map::mode_progress_effect`). The DMD should show progress all the time once it can. |
| 4 | Can the menu open during multiball? | The flippers drive the menu, and a hold would stop play for the other balls. | No. With more than one ball in play the scoop ejects as normal and stays lit. The menu opens the next time a single ball reaches it. |
| 5 | When does a mini-wizard end? | `DESIGN.md` gives scoring but no end condition. | It ends when one ball is left or none. It counts as played, and if it wasn't completed it stays in the menu with its progress, like any unfinished mode. |
| 6 | Does Hydro's shot 1 get a timer? | `DESIGN.md` says each shot has 25s, but a timer on the first shot has nothing to restart. | Shots 2 to 5 are timed. The timer starts when shot 1 is made. |
| 7 | What about the Solarium "test lab" lines? | `test-lab-shutdown-initiated`, `atrium-containment-enacted` and `the-science-will-not-be-lost` aren't in `DESIGN.md`, while "There is nothing on the glass" hasn't been recorded. | Use them at the intensity stages and completion (see Solarium). |
| 8 | Does the center orbit dig a root tunnel? | `DESIGN.md` says "orbit hits", and the center orbit is the spinner shot. | Yes: left, center and right orbits each have a tunnel. |
| 9 | What's the third Sentinels award, at 15 drones? | `DESIGN.md` leaves it TBD. | 15,000,000 points. |

### DESIGN.md items to revisit

`DESIGN.md` hasn't been edited. These parts of it no longer match the framework or the repo:

- **Drain order, item 3 (extra ball), and the open question in Code impact about skipping bonus.** The framework settled
  this: an extra ball is another turn for the same player after a normal `TurnEnding`, so bonus runs. An extra ball is
  not a `BallDraining` participant, and the routine is `BallSaveSystem`, then `OutlaneSaveSystem`.
- **`frontbox-pin2dmd`** is now `frontbox-dmd`.
- **Most extra balls, default 2.** `MAX_EXTRA_BALLS` defaults to 5 in frontbox-pinball. Setting it to 0 already gives
  "points instead", because `ExtraBallSystem` awards points when `grant_extra_ball` refuses.
- **Backbox rings.** "Assume a ring" is confirmed: `backbox.rs` has 2 rings of 25. Where each ring starts and which way
  it runs still needs measuring.
- **Speaker LED voice meters** appear in the build order, but no section describes them. This plan leaves them out.
- **`TODO.md`:** "Nimbus: hit each pop twice" is replaced by choke points. The Hydro flash item is in the Hydro section.
  "Attempt a completed mode again, harder" stays a post-launch idea.

---

# Part 1: Infra and core

Follow the frontbox `AGENTS.md` conventions in new systems. Public methods take `&ServiceContext`, callers identify
themselves with `ctx.id()`, events are in the past tense, and anything that can stop an outcome is a routine, not an
event.

## 1.1 Checks on the machine

- [ ] Hex positions, with `HexTest`. The tip direction and the ring's winding were worked out from the code and
      artwork, not measured. Root tunnels fill the rings from `hex_ring()`'s first LED, so check that too.
- [ ] Gold (255, 155, 0) reads apart from Meridian's orange on the inserts.
- [ ] Whether the illumination fix helps the open console slowdown. That link is a guess, so test it rather than
      assuming.
- [ ] The sling post rings fill from their first LED, assumed to be the bottom of each post.

## 1.2 Hardware layer

- [ ] Give every GI LED a location, or positional scenes will skip them: `gi_l_sling`, `gi_r_sling`, `gi_drop1`,
      `gi_drop2`. `gi::LEFT_RAMP` is also defined but isn't wired to a port.
- [ ] Measure the speaker rings' start and direction.
- [ ] Wire up the wireform post (decision 1). The post is physically there, but its driver probably isn't connected:
  - Check the wiring, then connect the coil to a driver and add it to `hardware` and `io_network.rs`.
  - Check whether the machine can tell how many balls are on the post. No switch for this is defined, so it may need one.
  - Add `WireformSystem`, then switch Groundswell and Derailment from virtual to real locks.

## 1.3 DMD

Everything here must run on what exists today: the `Canvas`, pixel fonts and numbered layers. `DmdScene` loads and the
console edits it, but its own docs say "there's nothing to play one with yet", so no scene art plays in a game yet.

- [ ] **Virtual DMD.** `main.rs` skips the DMD when `VIRTUAL` is set, so none of this can be tested off the machine.
      Listed as a prereq in `DESIGN.md` and in the frontbox `TODO.md`.
- [ ] **`DmdCard`:** a shared card with a title line, a body line and a duration. `ExtraBallSystem`, `TiltPresenter` and
      `BonusSystem` each hand-build this today. It's the stand-in for all DMD art in Parts 2 to 5.
- [ ] **Layer plan.** Tilt and extra ball share layer 2, so one clears the other's screen, and bonus and turn end share
      layer 1. Give each purpose its own layer: score, side quest row, mode, card, menu, tilt.
- [ ] **Score screen:** add the mode line and the side quest row with pips.
- [ ] **Mode select menu** in lotko. The operator menu is built around its own config sections, so take the input
      handling from `InitialsEntrySystem` instead.
- [ ] **Status pages** (hold a flipper) and **idle rotation**. Flipper switches are ordinary `SwitchClosed` events, so the
      "input routing" need in `DESIGN.md` is mostly met already.
- [ ] **Scoop eject summary.**
- [ ] Each mode's DMD now line is a `DmdCard` plus the mode line, unless it says otherwise.

## 1.4 Sound and callouts

- [ ] Tune by ear on the machine: chime gap, `city_hit` steps, the 3s wait, and Meridian's gate clunk every 2.25s.

What the SFX stand in for today, until the palettes exist:

| SFX | Used for |
|---|---|
| `steward-chime` | Before every Steward line |
| `latch-heavy1`–`3` | First two `city_hit` steps; Meridian's gate clunk; Nimbus seals; a camera blinded |
| `impact-dirt` | Last `city_hit` step; spore burst; drone down; tunnel collected |
| `impact-cinematic-heavy` | Choosing a mode in the menu |
| `braam-bio1` | District completion |
| `braam-bio2` | Mini-wizard completion; Heartwood start; Gridlock |
| `scan-light` | Ball start, with the status line |
| `scan-heavy-stutter` | Surveillance set |
| `woosh-light`, `latch-double` | Skyrail ramp (train past) and drop (signal clank); Nimbus reopen |
| `arp_hit1` (old) | Burst ready, outlane lit, combo route made |
| `hum`, `motor1`–`5`, `scan-light2`, `scan-med` | Loaded, not used. `hum` needs `loop` to be a bed, and the motors aren't a stepped set (their lengths differ), so not Hydro's rising hum. Candidates: lift ramp, glitches. |

Lines loaded with no moment in the game yet:

- **Waiting on Groundswell:** `generic/restricted-area` and `generic/skyrail-unavailable` (lock lit, together "Restricted
  area. Do not use the Skyrail"), `generic/access-to-city-restricted`, and `multiball/seismic-event-detected`,
  `seismic-activity-alert` and `seismic-damage-identified`.
- **Waiting on high scores:** `new-high-score`.
- **Global moments:** `generic/alert`, `warning`, `scanning`, `city-scan-complete` and `seeking-biological-entity`. They
  suit the ball start scan, tilt warnings and attract.
- **Final wizard:** `final/steward-core-breached`.
- **Spares:** `meridian-basins-uninhabitable`, `hydro-core-offline` and `nimbus-promenade-offline`.

Music: `ModeManager` has placeholder commercial tracks for Hydro, Skyrail, Solarium, Meridian, Nimbus, Heartwood and the
default. Apex, the mini-wizards and Groundswell have nothing. `play_music` plays a track once, so anything longer than
the track goes silent (gap `loop`). All original music is still to come.

### Lines still to record

- **Oaks: all 18 are missing**, none found by file name. Wake, Again, Hold, Not yet, Soon, Ours, Drink, The way is open,
  Nothing is yours, Open sky, Open the gates, Darkness, Fall, Higher, Ready, Stay, Rise, Heartwood.
- **Steward:** each section lists its own. `callouts/combos` and `callouts/blackout` are empty.
- **Numbers:** "N of six" only has `four-of-six`, "Walkway N" only has `walkway-4`, and "Decommissioning drone N" only
  has 1 and 3 (other drones say `drone-disengaged`).

## 1.5 LED conventions and shared scenes

All game scenes use `Surface: Sequence` except the spore burst, and attract uses `Area`, so positional effects are
mostly new to the game code. The system tick is 67ms (about 15 updates a second), so anything strobing past about 7Hz
will alias. Design around that rather than trying to fix it.

**Z layers.** Agreed. Keep to these, or stacked side quests, modes and multiballs will fight:

| z | Use |
|---|---|
| -1 | Ambient: GI tint, arc and subway base |
| 0 | Side quests: spore bar, hex rings, drop strips, sling rings, pop targets |
| 10 | Mode attention, hex centers |
| 20 | Hits and short reactions, mode progress on the spore bar |
| 30 | Full-field moments: starts, completions, multiball starts, spore burst |
| 40 | Blackout's dark layer |
| 41 | What Blackout leaves lit: roots and lit shots |
| 50 | Tilt (existing) |

**Shared scenes.** `DESIGN.md` reuses a few effects across every mode. Build these once and drive them with params.
`Prop::Param` works for `Translate` (`Vec2`), colors and floats, so one scene can serve every shot.

Built, with a helper in `effects.rs` that takes the LEDs: `shared/flash.scene` (`flash_effect`), `shared/fill.scene`
(`fill_effect`, standing in for `ring_fill`) and `shared/heartbeat.scene` (`heartbeat_effect`). Still to build:

| Scene | Params | Used for |
|---|---|---|
| `shared/shot_wave.scene` | `origin`, `color`, `green` | Radial hit on any shot |
| `shared/surge.scene` | `from`, `to`, `color` | Packet between two points, a `Sweep` along a `Route` |
| `shared/takeover.scene` | `origin` | Every completion (≈ growth) |
| `shared/city_scan.scene` | `color` | Steward alerts, ball start, top-down sweeps |
| `shared/arc_timer.scene` | `color`, via `.over(duration)` | Arc ramp timers, adapted from `hydro_core/progress.scene`, which is on the spore bar |
| `shared/speaker_timer.scene` | `color`, via `.over(duration)` | Countdowns on the speaker rings |
| `shared/forest_ambient.scene` | `amount` | Ball start ambient (≈ noise) |

- [ ] Check the hand-written stand-in scenes on the machine: `shared/flash`, `shared/fill`, `shared/heartbeat`,
      `mode_progress`, `side/charge_ray`, `side/burst_ready`, `side/burst`, and Nimbus's `seal_warning`.

### Primitive status

| Primitive | Status | Approximation for now | Gap |
|---|---|---|---|
| Radial wave | Works | `Ring` with animated radius on `Area("Playfield")`, as `ring-burst.scene` and `side/burst.scene` do. A `Radial` gradient softens it. | `feather` |
| Plane sweep | Partly | `Rect` with animated `Translate`, one layer per plane with times offset by hand | `stitch` |
| Growth | Missing | A green ring expanding from the seed, with `Prop::Random` flicker. It lights by distance, not along branches. | `growth` |
| Path flow | Works | `Sweep` along a `Route` or `Bezier`, or a moving shape on a `Sequence` of ordered LEDs | — |
| Noise field | Missing | A few large soft blobs with `Prop::Random` positions and opacity. For hero moments, a pre-rendered animated image in an `Image` fill. | `noise` |
| Glitch | Partly | `Curve::Random` / `Steps` and `Prop::Random` on `Opacity` and `HueShift` | `glitch` |
| Heartbeat | Works | `shared/heartbeat.scene`, with tempo through `.over()` | — |
| Fill | Works | `shared/fill.scene`, or animated `Rect` height | — |
| Layers and masks | Partly | Per-layer masks, and z across systems. `LedQ` has no "all but", so "all but the roots" is a dark layer with the roots one z above it. Alpha compositing only. | `blend` |
| Tempo sync | Missing | Set `.over()` by hand to match a track's BPM. It drifts, so keep these short. | `tempo` |

## 1.6 Framework gaps

Work in frontbox-rs that the To fully finish notes point to. Nothing here blocks shipping an approximation. Ones in
frontbox's `TODO.md` are marked.

| Gap | Crate | Need | In `TODO.md` |
|---|---|---|---|
| `growth` | gfx | Line and branch fills: spreads from a seed along an LED neighbor graph or a generated tree | Yes |
| `noise` | gfx | Simplex noise fill and noise masks | Yes |
| `glitch` | gfx | Stutter, glitch and flicker curves | Yes |
| `feather` | gfx | Soft edges on layer shapes | Yes |
| `stitch` | gfx, led | One effect moving across planes (playfield into backbox) | Yes |
| `blend` | gfx | Blend modes between layers | Yes |
| `tempo` | gfx, sound | BPM clock from music to scenes | Yes |
| `oneshot` | led | Fire-and-forget effect that outlives the system that started it. `EffectsSystem` stands in. | Yes |
| `dmd-player` | dmd | Play a `DmdScene` in a game, not just the console | Partly |
| `virtual-dmd` | dmd, console | DMD output in `VIRTUAL` mode | Yes |
| `dmd-stack` | dmd | Screen stack with priority, replacing numbered layers | Not listed |
| `voice-fx` | sound | Runtime dropout, wobble, stutter, bitcrush for Steward integrity. Pre-rendered variants are the alternative. | Yes |
| `loop` | sound | Loop points for music and ambient beds | Yes |
| `stems` | sound | Multi-stem music | Yes |
| `leaderboards` | pinball | Lower-is-better, typed values, shared initials entry | Yes |

---

# Part 2: Districts

Every district plays on the 6 main shots plus the drops, uses `ModeRun`, and starts from the menu. All of them need the
same note on sound and music, so it's stated here once. Palette SFX, a real music track, and Steward integrity
processing (`voice-fx`) are outstanding for each. Each section lists only what's specific to it.

A collected root tunnel emits `TunnelCollected`, and the running district counts it as one hit. Meridian, Hydro, Nimbus
and the stand-in mini-wizard do. The three still to rewrite need to as well.

## Meridian Basins — orange

Sweeping basins. Tier 1, alternative to Hydro. The rules are done.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Orange chase along an ordered `Sequence` of the 6 hex centers, settling on the first window | `root-intrusion` |
| Attention | = Rippling lit hexes. ≈ On a move, short fade-out and fade-in scenes on the leaving and arriving shots, since playbacks are swapped whole | Hum and gate clunk missing |
| Hit | = `shot_wave` in orange, with `green` rising per hit for the "more green" read | `channel-breach`, then `four-of-six` on hit 4 |
| Intensity | ≈ 25%: `Random` opacity shimmer on GI, alternating orange and blue. ≈ 50%: same, faster. = 75%: flashers pulse | Sirens missing |
| Completion | ≈ `takeover` from the roots, then the insert goes green | `basins-reclassified` |

**DMD now:** card "MERIDIAN BASINS", mode line "BREACH 2/6".

**Spare line:** `meridian-basins-uninhabitable`, usable at 75% or on a resume.

**Still to record:** "Notice… Channel gates are closing", "N of six" for 1, 2, 3, 5, 6. O: "Ours."

**To fully finish**

- `noise` for the GI shimmer, `growth` for roots pouring through the channels, and `feather` for the water wave.
- `dmd-player` plus art: basins overhead view, flowing channels, a crack along a channel, rising water, green overflow.
- SFX: water hum (`loop`), lock gate clunk, water rush with concrete cracking, distant sirens.

## Hydro Core — cyan

Coolant combo. Tier 1, alternative to Meridian. The rules are done. A collected tunnel makes the shot the combo is on.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Coolant chase in combo order along a `Sequence` of the 5 hexes | `coolant-core-dropping` |
| Attention | = Spinning cyan hex (existing). = Timer moves to the arc ramp with `arc_timer`, also on the speaker rings. `progress.scene` runs it on the spore bar today, which is Charge's now. Hex flashes faster: rebuild its playback at 50% and 25% of the time left | Reactor hum missing |
| Hit | = `surge` white-hot from this shot to the next. The made shot stays green at z 0 | `city_hit` rising per shot |
| Intensity | ≈ 50%: `Random` opacity flicker on GI. = 75%: existing flashers as a warning strobe | `core-temperature-rising`. Klaxon missing |
| Completion | = White flash on everything. ≈ `takeover` from the subway, then GI tints green until the next mode | `core-offline` |

**DMD now:** card "HYDRO CORE", mode line with shot number. Coolant gauge as a filled `Rectangle` driven by time left.

**Spare line:** `hydro-core-offline`.

**Still to record:** "Engineers to the pipes", unless `coolant-core-dropping` includes it. O: "Drink."

**To fully finish**

- `glitch` for the power instability, `growth` for the green flood from the subway.
- `dmd-player` plus art: core cutaway with 5 pipes, roots wrapping pipes, warning lights, the core going dark.
- SFX: reactor hum that rises in pitch per shot (`loop`, plus pitch control or 5 stepped files), power thump, klaxon,
  power-down whine.

## Nimbus Promenade — magenta

Choke points. Tier 1 gateway, the easiest district. The rules are done. A collected tunnel chokes a random open
walkway.

- [ ] Check on the machine: `seal_warning.scene` (a 5Hz magenta flicker) and the flashes on the hex ring (green for a
      choke, white for a reopen). The seal and reopen SFX are a latch and a woosh.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Magenta from mid-playfield out along the shots, using `Sweep`s to each hex | `promenade-closed`, then `walkways-will-be-sealed` |
| Attention | = Open shots shimmer. = Flicker, then a hard cut on seal (`Steps` curve) | `walkway-4-sealed` when walkway 4 seals; the shutter SFX alone for the others |
| Hit | = Choke: burst from center to hex, stays green. = Reopen: white flash, back to magenta | `city_hit`. Tearing SFX missing |
| Intensity | ≈ Haze: a few `Random` magenta blobs, masked to open shots. ≈ 75%: speakers on a `heartbeat` at the track's rough tempo | Detuned chimes missing |
| Completion | = All walkways flood green. ≈ `takeover` in every direction, Nimbus map insert lights, then `surge`s to Skyrail and Apex inserts | `access-to-upper-city-restricted` |

**DMD now:** card "NIMBUS PROMENADE", and a row of 6 pips (open / sealed / choked) drawn on the canvas.

**Spare lines:** `nimbus-promenade-offline`, `generic/access-to-city-restricted`.

**Still to record:** "Walkway N… sealed" for 1, 2, 3, 5, 6. O: "The way is open."

**To fully finish**

- `noise` for the haze, `tempo` for the speaker pulse, `growth` for the spread.
- `dmd-player` plus art: walkway map, shutters, roots knotting, clouds, the pan up to the upper districts.
- SFX: wind chimes in clean and detuned versions, shutter slam, wooden thump and splintering, shutter tearing.

## Skyrail Station — blue

Lift ramp and drops. Tier 2, alternative to Apex.

**Today:** close to `DESIGN.md`, with attention, hit, intensity, flasher and completion scenes.

**Rules work**

- [ ] Fix completion. `DESIGN.md` wants 5 drop hits during the finale, but `target_hits` also counts the 2 before it, so
      the code needs only 3.
- [ ] Five hits on a 3-target bank means raising it mid-finale. Use `DropBankCleared`.
- [ ] Use the shared lift ramp requests. Lock lights wait while Skyrail owns the ramp.
- [ ] Keep the ball save running until the mode ends.
- [ ] Fix the progress total to match the new count (5 + 3 ramp shots + 2 drops = 10).
- [ ] Resume: restart at the current stage. The finale restarts with its 2nd ball.
- [ ] Count `TunnelCollected` as a hit.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Blue chase up the lift ramp hex and bolt | `skyrail-service-suspended`, then `clear-the-platform` |
| Attention | = Ramp hex chases, drop strips glow blue (existing) | Platform hum missing |
| Hit | = Fast chase on a ramp shot. = Signal goes red on a drop strip | `the-515-is-delayed` on the first ramp shot, `city_hit` after |
| Intensity | = Drop strips flash like crossing signals. = 75%: flashers with the bells | Crossing bells missing |
| Completion | = Chase stops dead. ≈ Blue fades while `takeover` runs from the ramp down | `all-lines-are-now-closed` |

**DMD now:** a real departures board from the pixel fonts: rows of lines, flipping to DELAYED per hit. No scene player
needed.

**Still to record:** no Steward lines missing. `DESIGN.md` has no Oaks line for it.

**To fully finish**

- `noise` or a dissolve mask so blue "scatters", `growth` for the takeover.
- `dmd-player` plus art: train arriving, board glitching, roots tearing up the track.
- SFX: station ambience (`loop`), train rushing past, signal clank, crossing bells, rail screech and crash.

## Apex Terraces — gold

Overload. Tier 2, alternative to Skyrail.

**Today:** the old mode on the pop targets, the left inlane target and the spinner. Replaced entirely. While it runs it
shares the pop targets with Surveillance and the inlane target with Charge's burst, which the rewrite frees up.

**Rules work**

- [ ] Rewrite as a `Mode` on the 6 main shots, in gold.
- [ ] One lit shot. Each hit fills a ring segment. A full ring bursts for a big award, and the lit shot moves to
      `farthest_from`.
- [ ] Terraces need 3, then 4, then 5 hits. Complete at 3 bursts.
- [ ] The ring loses a segment after 6s without a hit (start value).
- [ ] The lit shot's ring shows overload over Root tunnels (z 10 over z 0).
- [ ] Resume keeps bursts and restarts the current ring empty.
- [ ] Count `TunnelCollected` as a hit.
- [ ] Delete the target and spinner scenes, and move its values into `points`.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Gold bands stepping up the playfield (`Rect` with a `Steps` curve), ending on the first shot | `residents-evacuate-now` |
| Attention | = Center glows gold, filled segments green (`fill_effect`). = The next segment to drain flickers | `this-is-an-exclusive-area` once, early. Chimes missing |
| Hit | = One more segment, center flash. = Burst: `shot_wave` in green, then `surge` in gold to the next shot | `warning-structural-concern` on each burst, `city_hit` per hit |
| Intensity | = Bands rise faster. = 75%: speakers go gold | Detuned chimes missing |
| Completion | ≈ Gold breaks into green: `takeover` plus `forest_ambient` over the playfield | `apex-terraces-offline` |

**DMD now:** card "APEX TERRACES", mode line "TERRACE 2/3" and a gauge for the current ring.

**Still to record:** "Please keep off the terraces", "Terrace three: structural concern", "This is a… premium… area",
if those are wanted over what's recorded. O: "Nothing is yours."

**To fully finish**

- `noise` for the wild green field, which is this mode's defining finish.
- `dmd-player` plus art: terraced towers, swelling hedges, a terrace bursting, towers swaying and overgrown.
- SFX: clean expensive chimes in tuned and detuned versions, wet creaks deepening per segment, stone cracking and soil
  spilling.

## Solarium Atrium — purple

Crawling vine. Tier 2 finale, the hardest district.

**Today:** a ramps mode that `DESIGN.md` calls too thin, with scenes for attention, hit, intensity, flashers and
completion. The rules are replaced, and the flashers and intensity scenes may carry over.

**Rules work**

- [ ] Rewrite on the 6 main shots. One lit shot moves to a neighbor every 3.5s (start value), never straight back.
- [ ] Choose the next shot ahead of time, so the pre-light is true.
- [ ] Each hit shortens the interval.
- [ ] Complete at 6 hits. Resume keeps hits.
- [ ] Remove the dome-to-lift double scoring.
- [ ] Count `TunnelCollected` as a hit.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = `city_scan` in purple, top down, then `surge` from the roots to the first shot | `atrium-integrity-verified` |
| Attention | = Head glows purple. = Trail of past shots in dim green, fading. = Next shot pre-lit dimly | Glass groans missing |
| Hit | = `shot_wave` in white as the shatter flash | `city_hit`. Glass SFX missing |
| Intensity | = White points flicker: small circles with `Random` positions, adding some at each stage. = 75%: GI warm purple-white | 25%: `atrium-integrity-failing`. 50%: `test-lab-shutdown-initiated`. 75%: `atrium-containment-enacted` |
| Completion | = White burst, then green down from the top with `city_scan` | `the-science-will-not-be-lost` |

**DMD now:** card "SOLARIUM ATRIUM", mode line "CRACKS 2/6". A crack can be a growing line of `Rectangle`s.

**Still to record:** "There is nothing on the… glass" with its stutter (decision 7). O: "Open sky."

**To fully finish**

- `growth` for the vine trail along the LEDs between shots, `stitch` for a sweep that continues into the backbox.
- `dmd-player` plus art: dome from below, moving vine head, spreading cracks, falling glass.
- SFX: glass stress groans and creaks, cracking that gets louder per hit, huge shatter with falling glass.

---

# Part 3: Mini-wizards

`MiniWizardMode` stands in for all four for now, so the route can be played to its end: 2 balls in tier 1, 3 in tier 2, any
main shot or collected tunnel is a hit, 10 hits complete it, and it ends per decision 5.

Tier 1 is 2-ball, tier 2 is 3 to 4 ball. All four are new. All need the mini-wizard flow first:

- [ ] Lit as soon as the step is reached, started from the menu like a district.
- [ ] Eject the balls, with `MULTIBALL_BALL_SAVE` timed from `EjectCompleted`. `SporeMultiballMode` has the pattern.
- [ ] End per decision 5, watching `RoutineCompleted<BallDraining>` for `AllSteps`, as multiballs do.
- [ ] Groundswell can't run alongside one. Lock lights wait.
- [ ] A longer intro and a big finish, per `DESIGN.md`. They use their own tracks, and tempo sync is `tempo` for all four.

## Floodgates (Meridian, tier 1)

**Rules work**

- [ ] 2-ball. Orbit hits raise a water level, shown on the arc ramp. 8 hits to high water (start value).
- [ ] At high water, the ramps are super jackpots.
- [ ] Track the total for the Biggest Flood high score.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | = Fill rising from the flippers | `flood-barriers-engaged` |
| Play | = Fill on the playfield and the arc ramp as a gauge. ≈ Moving surface: an edge band with `Random` height | Rushing water missing |
| Super jackpot | = `city_scan` as a wave from the top | Crash missing |

**DMD now:** a water gauge as a `Rectangle`, and the total.

**Still to record:** a variant of "Flood barriers… engaged. Engaged. Engaged", if the file doesn't already stutter.
O: "Open the gates."

**To fully finish**

- `noise` for the surface, `tempo` for the music sync, and `dmd-player` plus art for the gates opening.
- SFX: gates grinding, rushing water.

## Blackout (Hydro, tier 1)

**Rules work**

- [ ] 2-ball. Each main shot hit darkens that shot. When all are dark, the left scoop is the super jackpot.
- [ ] Time it from start to super jackpot for Fastest Blackout, which is lower-is-better.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | = GI and playfield to black at z 40, with roots and lit shots at z 41 | No lines yet |
| Hit | = Shot and its section go dark in hard steps | Clunk missing |
| Super jackpot | = 1s of total dark, then a `shot_wave` from the spinner | Missing |

**DMD now:** a city grid drawn on the canvas, with sectors going black. It needs no art.

**Still to record:** everything, as `callouts/blackout` is empty. The fragmenting Steward lines and O: "Darkness."

**To fully finish**

- `glitch` for the breakdown, `feather` for the explosion of light, `dmd-player` for the finish.
- SFX: descending power-down whine, section clunks.

## Derailment (Skyrail, tier 2)

**Rules work**

- [ ] 3 to 4 ball. Jackpots at the drops, fed by the upper flipper. Raise the bank on `DropBankCleared`.
- [ ] Locked balls are kept as virtual locks. If decision 1 stays virtual, every lock already is one.
- [ ] Track the total for the high score.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ Falling railcars: a chase down the lift ramp hex and drop strips per ball, as there are no LEDs on the wireform | `train-derailment-subverting` |
| Jackpot | ≈ Blue on the drop strips shattered by `shot_wave` in green | Rail screech missing |

**DMD now:** card per jackpot, and train cars as sprites if time allows.

**Still to record:** "Train… train… train" (it could be cut from existing audio). O: "Fall."

**To fully finish**

- The real wireform release needs the post (decision 1), and a real falling chase needs LEDs that don't exist.
- `noise` or a dissolve for the shatter, `tempo`, and `dmd-player` plus art of cars tumbling.
- SFX: rail screech, metal tearing, crashes.

## Canopy (Apex, tier 2)

**Rules work**

- [ ] 3 to 4 ball. The vertical and center spinners and the pops build a value, and the ramps collect it.
- [ ] Track the total for the high score.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ `forest_ambient` climbing the playfield, then on the speakers | No lines yet |
| Collect | ≈ `surge` from the ramp, then a matching sweep on the speakers, timed by hand | Trunks and wind missing |

**DMD now:** the current value and a rising bar.

**Still to record:** any Steward lines, and O: "Higher." There's no `callouts/canopy` folder yet.

**To fully finish**

- `noise` and `stitch`: a noise field that climbs into the backbox is the whole mode's look.
- `tempo`, and `dmd-player` plus art of the canopy spreading over the skyline.
- SFX: giant trunks creaking, a deep roar of wind.

---

# Part 4: Multiballs

## Groundswell

**Rules work**

- [ ] Build `GroundswellLock`, from `LiftRampStartable` in git history (`lift_ramp_startable.rs`). Sentinels light locks
      by emitting `LockLit` (every 2 drones, or 1 with the Skyrail perk), which nothing takes up yet. The lit lock
      requests the ramp up, and the shot feeds the rear scoop.
- [ ] Virtual locks (decision 1): count the lock, and let the ball continue to the upper flipper.
- [ ] Lock 3 to start a 3-ball multiball. Jackpots are on the main shots, and the super jackpot is the lift ramp, upper
      flipper, then a drop shot.
- [ ] Relighting gets harder and jackpots reset for each run. The first run's super jackpot lights
      `ExtraBallSource::Groundswell`.
- [ ] Stacks with districts, not mini-wizards. With real locks, the multiplayer post logic in `DESIGN.md` applies, using
      `ball_temporarily_removed_from_play` and `ball_returned_to_play`. With virtual locks there's nothing shared.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Lock lit | = Lift ramp hex outer ring green | `generic/restricted-area`, then `generic/skyrail-unavailable` |
| Ball locked | = Pulse up the lift ramp. ≈ Ambient throb: `heartbeat` that strengthens per lock | `multiball/seismic-activity-alert`. Rumble missing |
| Start | ≈ `Random` strobing, then a green sweep up the playfield from the roots | `multiball/seismic-event-detected`. O: "Rise." |
| Jackpot | = `shot_wave` in green. Speakers: left stays city, right grows | `multiball/seismic-damage-identified` |
| Super jackpot | ≈ Sweep from the drops upward, with the backbox timed by hand | Missing |

**To fully finish:** `noise` and `glitch` for the earthquake, `stitch` for the sweep into the backbox, `tempo` for the
speaker beat, `dmd-player` plus art for the splitting ground and toppling tower, and real locks once the post is wired.

## Heartwood multiball

`HeartwoodCharge` starts it from a full charge (Part 5). What's left of the multiball itself:

- [ ] Rename `SporeMultiballMode` to Heartwood.
- [ ] Finish the super jackpot TODO. The current one is on the captive ball every 5th jackpot. Keep that unless
      `DESIGN.md` says otherwise.
- [ ] Replace `spore_multiball/*.scene` colors (yellow today) and rename the folder. `qualification.scene` and
      `startable_attention.scene` are no longer used, and can go.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ Whole playfield on a `heartbeat`, timed by hand to the track | Heartbeat stinger missing. O: "Heartwood." |
| Jackpot | = Heartbeat `shot_wave` from the shot | Double-thump missing |

**To fully finish:** `tempo` (the whole point of this mode's look), and `dmd-player` plus art for the glowing heart.

---

# Part 5: Side quests, combos, saves and global moments

## Side quests

The rules for all five are built, each a player-template system with every perk applied. Counts and values are start
values to tune. What's left is presentation, the DMD side quest row (1.3) and sound:

| Quest | Built | Still to do |
|---|---|---|
| **Charge** (`SporesSystem`) | A spin charges 1, a pop 3 (6 with the Nimbus perk). 60 for the first burst, 30 more each after. The burst lasts 8s, or scores 2,500,000 with no mode. Pops send a packet down their ray. | Bar's flickering edge (`Random`). Burst insert is a hand-made green and violet beat, not `heartbeat`. Swarm rustle and whoosh SFX. O: "Ready." |
| **Surveillance** | One hit per camera, one more each set. A set counts as a lit shot, but only between modes, as the scoop doesn't qualify during one. Every 3rd set lights an outlane. | Steady red today: add the slow scan, and a static glitch on a hit. Camera whir and lens crack SFX. |
| **Sentinels** | Strips are red while their target is up. Locks are only an event until Groundswell. | Strips scan rather than glow, and spark on a drone down. Drone whine SFX. |
| **Heartwood** (`HeartwoodCharge`) | 4 hits, then 2 more each time (×2 with the Solarium perk). The bolts beat from 1.4s down to 0.5s as it charges. | Sub heartbeat and wood knock SFX. |
| **Root tunnels** | 6 segments, 2 a hit with the Meridian perk. A collect scores and emits `TunnelCollected`. | Collect: `surge` from the orbit to the district's map insert, and `growth` on the ring. Dirt SFX. Decision 8. |

## Combos

`ComboSystem` is built: the 8 routes, the 4s window that waits in the scoop, the lift ramp breaking a route, 3-shot
routes paying the 2-shot routes inside them, and Gridlock. A route lights its hex centers green, and Gridlock lights all
six.

| Moment | LEDs now | Sound now |
|---|---|---|
| In progress | = Path flow toward the flipper. = Continuing shots blink green. Speaker ring countdown | Engine note missing |
| Route made | = `surge` shot to shot along the route, in green | Horn and screech missing |
| Gridlock | = All routes light, shots stall red and pulse, then green | City-wide horn jam missing. O: "Stay." |

**Still to record:** all of it. `callouts/combos` is empty. One line per route plus Gridlock.

**To fully finish:** `dmd-player` plus the route card template (8 routes). A pixel font card works until then.

## Saves

**Extra balls:** only `ExtraBallSource::Groundswell` is left to wire, with Groundswell (Part 4).

**Outlane saves:** `OutlaneSaveSystem` is built: sling hits fill the side's two post rings (8, then 12, 18, 24 hits), a
lit side heartbeats its outlane LED, a drain within 3s of that outlane's switch is saved, multiball leaves it lit, and
the right pass lane gives a 3s hidden save.

- [ ] Check on the machine that the pass lane save catches what it's meant to. It starts on either pass lane switch.

| Moment | LEDs now | Sound now |
|---|---|---|
| Growing | = Rings fill a step per hit | Creak missing |
| Lit | = Outlane LED green on a slow heartbeat | O: "Hold." missing |
| Used | ≈ `surge` from outlane to plunger lane, rings empty | `retrieval-failed`. O: "Not yet." missing |

**To fully finish:** `growth` for the rings and surge, and `dmd-player` plus art for the roots grabbing the ball.

## Global moments

| Moment | LEDs now | Sound now | To fully finish |
|---|---|---|---|
| Attract | ≈ Existing 4 scenes, plus a city scan and a slow forest scene | none | `noise`, `growth` for roots from the spinner; themed DMD; breathing speaker rings |
| Game start | ≈ `takeover`-style pulse from the spinner, wave to the flippers | `game-start` | O: "Wake." Roots through the title |
| Ball start | = `city_scan`. ≈ `forest_ambient` by overgrowth level | `all-systems-within-operational-tolerance` early, `continuing-city-maintenance` later | "Overgrowth at N percent" lines, `noise` |
| Ball save | ≈ Green radial from the drain; `ball_save.scene` today | O: "Again." missing | Art of roots catching the ball |
| Extra ball | = `extra_ball_lit.scene` | `unregistered-resident` lit, `error-invalid-resident` collected, `extra-ball` on shoot again | DMD art |
| Drain | ≈ Fade effects toward the roots, GI dims | `growth-contained` | `growth`, integrity on this line |
| Tilt | = Exists (`TiltPresenter`). Add a white snap before dark | `thank-you-for-cooperation` | Clean city font |
| Game over | ≈ Green map up to the districts taken | "Final overgrowth" missing. O: "Soon." missing | Summary screen first, as in 1.3 |

Speaker rings as an overgrowth gauge go on z 0 as their own small system, and everything else overlays them.

**High scores** (at the end, once the modes are in)

- [ ] Register `HighScoresSystem` in `main.rs`, so scores are saved and entered, and set the custom categories at game
      end. Say `new-high-score`.
- [ ] Compare by hand for lower-is-better Fastest Blackout, and format it as a time. Proper boards are `leaderboards`.

**Final wizard:** out of scope in `DESIGN.md`. `final/steward-core-breached` is ready for it. For now, finishing everything
leaves the scoop unlit.

---

# Part 6: Order of work

Each milestone leaves a playable game. The progression, the first three districts, the side quests, combos and saves
are in.

1. **DMD foundations.** `DmdCard`, the layer plan, and a virtual DMD if it can be done cheaply, since everything after
   it needs testing.
2. **Shared scenes.** Build `shot_wave`, `surge`, `takeover` and `city_scan`, then prove them on one mode.
3. **Districts.** Skyrail, Apex, Solarium, then presentation for all six.
4. **Multiballs.** Groundswell, then finish Heartwood.
5. **Status and idle screens**, and the side quest row.
6. **Mini-wizards.** Floodgates and Blackout, then Derailment and Canopy.
7. **High scores and global moments.**
8. **To fully finish.** Take the gaps in the order they unlock the most: `dmd-player`, `noise`, `growth`, `tempo`,
   `glitch`, then the rest. Audio goes alongside: record the Oaks, fill in the Steward lines, build the palettes, then
   the music.
