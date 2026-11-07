# Rule Revamp Plan

This is the work needed to get from today's code to `RULES.md`. `RULES.md` stays the record of what the game should be,
and this file is how we get there. Where they disagree on what the game *is*, `RULES.md` wins, unless "RULES.md items to
revisit" below says otherwise.

How it's laid out:

- **Part 1** is the infra and core work everything else sits on.
- **Part 2** has one section per district. **Part 3** has one per mini-wizard.
- **Parts 4 and 5** cover what isn't a region but still has to be built: multiballs, side quests, combos, saves and
  global moments.
- **Part 6** is the order to do it in.

Each mode section says what exists, the rules work, what to build now, and a **To fully finish** note for the gap
between the approximation and `RULES.md`. Those notes use the gap IDs from [1.13](#113-framework-gaps).

In the presentation tables:

- **=** means this matches `RULES.md` with what the framework can do today.
- **≈** means it's an approximation. Ship it, and the mode's To fully finish note says what replaces it.

Callout text comes from the file names. Nobody has checked the audio against these lists. Timings and counts that
`RULES.md` leaves open are starting points to tune on the machine.

## Decisions needed

Work can start on all of these with the default. Each one changes something real, so they're worth confirming.

| # | Question | Why it came up | Default used in this plan |
|---|---|---|---|
| 1 | When does the wireform post get wired up? | The post is physically on the machine, but its driver probably isn't wired. `io_network.rs` has no post driver and no switch on the wireform, and the rear scoop eject and upper flipper are all that's defined up there. | Build all locks as **virtual locks** until it's wired: the ball goes round to the upper flipper and the multiball launches from the trough. Real locks then replace them (see 1.7). |
| 2 | Which shots are lit for qualifying? | `RULES.md` says "lit shots (white hex centers)" and "2, 3, 4…" but not which ones. | All 6 main shots start lit. A hit shot goes out. N hits lights the scoop. Lit shots reset for the next qualify. |
| 3 | Where does district progress show? | Modes use the spore bar for progress today, and Charge now owns it. | The bar is Charge's. On a mode hit it shows mode progress in the mode's color for about 1.5s, then goes back to Charge. The DMD shows progress all the time. |
| 4 | Can the menu open during multiball? | The flippers drive the menu, and a hold would stop play for the other balls. | No. With more than one ball in play the scoop ejects as normal and stays lit. The menu opens the next time a single ball reaches it. |
| 5 | When does a mini-wizard end? | `RULES.md` gives scoring but no end condition. | It ends when one ball is left or none. It counts as played, and if it wasn't completed it stays in the menu with its progress, like any unfinished mode. |
| 6 | Does Hydro's shot 1 get a timer? | `RULES.md` says each shot has 25s, but a timer on the first shot has nothing to restart. The code leaves shots 1 and 5 untimed. | Shots 2 to 5 are timed. The timer starts when shot 1 is made. |
| 7 | What about the Solarium "test lab" lines? | `test-lab-shutdown-initiated`, `atrium-containment-enacted` and `the-science-will-not-be-lost` aren't in `RULES.md`, while "There is nothing on the glass" hasn't been recorded. | Use them at the intensity stages and completion (see Solarium). |

### RULES.md items to revisit

`RULES.md` hasn't been edited. These parts of it no longer match the framework or the repo:

- **Drain order, item 3 (extra ball), and the open question in Code impact about skipping bonus.** The framework settled
  this: an extra ball is another turn for the same player after a normal `TurnEnding`, so bonus runs. An extra ball is
  not a `BallDraining` participant, and the routine is `BallSaveSystem`, then the outlane save.
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

## 1.1 Main shots

`ModeShots` in Meridian is the start of this, and its own TODO says so. Every district, combos, Root tunnels and
qualifying need one definition of a shot.

- [x] Add `MainShot` with `LeftOrbit, DomeRamp, ArcRamp, CenterOrbit, LiftRamp, RightOrbit`, in playfield order.
- [x] Give it `hex_center()`, `hex_ring()`, `origin() -> Vec2` (for scene params), `neighbors()` and `farthest_from()`.
- [x] Add a `ShotSystem` at the root that turns the hit events into one `MainShotHit(MainShot)`. Modes stop matching on
      six event types.
- [x] A lift ramp hit with the ramp up doesn't count as a `MainShotHit` for combos. Report it with the event so each
      consumer can decide (`ramp_up: bool`).
- [x] Fix the hex LED positions to match `HEX_LINE_LEDS`. The ring LEDs sit on the hex's flat edges, and each line's last
      LED points up its shot.
- [ ] Check the hex positions on the machine with `HexTest`. The tip direction and the ring's winding were worked out
      from the code and artwork, not measured.

## 1.2 Progression and `ModeManager`

Today there's `ExclusiveMode`, `NonExclusiveMode`, two completion sets and a map with 2 states. `ModeManager` is already
in the player template, so "each player has their own map state" needs no extra work.

- [ ] Replace both enums with one `Mode`: the 6 districts and 4 mini-wizards, each with a color, map insert, tier and
      music. Multiballs leave this enum (see 1.3).
- [ ] Track `ModeState` per mode: `Unplayed`, `Available`, `Played`, `Completed`.
- [ ] Add `available_modes()`, which returns the menu rows from those states:
  1. Meridian or Hydro. Choosing one closes the other for the game.
  2. Nimbus.
  3. Floodgates or Blackout, whichever matches step 1.
  4. Skyrail or Apex.
  5. Solarium.
  6. Derailment or Canopy, whichever matches step 4.
  - Starting a mode marks it played and opens the next step. Mini-wizards are required, per the draft in `RULES.md`.
  - Played but uncompleted modes stay listed to resume.
- [ ] Render all four states on the map: off, a pulse in the mode color, solid mode color, green. Green for completed
      keeps green the forest's color. It's what `RULES.md` asks for in Meridian's completion.
- [ ] Add `districts_played()` for Steward integrity (1.9) and `districts_completed()` for bonus, perks and high scores.
- [ ] Move Apex from green to **gold**. There's no `Rgba::gold()`, so define it in lotko, and check it reads apart from
      Meridian's orange on the actual inserts.
- [ ] Update `metrics.rs` for the new enums, adding the mini-wizards.

## 1.3 The mode slots

`RULES.md` needs a district and a multiball running together, so a single exclusive slot isn't enough.

- [ ] Hold two slots in `ModeManager`: `mode: Option<Mode>` and `multiball: Option<Multiball>`.
- [ ] Enforce the stacking rules. Groundswell stacks with districts but not mini-wizards. Heartwood stacks with any mode
      but not Groundswell.
- [ ] Music priority is mini-wizard, then multiball, then district, then default.
- [ ] `PlayfieldIllumination` follows the slots. It should tint GI, not only the arc ramp and subway.

## 1.4 Mode lifecycle and resuming

All the district modes repeat the same pattern: effects, hits, wait for the completion effect, despawn. Add a start
phase and resume, and it's worth sharing.

- [ ] Add a small helper each mode owns, say `ModeRun`, with the phases from `RULES.md`: Start, Attention, Hit, Intensity
      and Completion. It handles:
  - The start phase: scoop held, intro scene and callout, then release (1.6).
  - Intensity stages at 25 / 50 / 75%, calling back into the mode when a stage is crossed.
  - An 8s ball save at the start, requested when the ball leaves the scoop (`RULES.md`, Timed ball saves).
  - Completion: points, callout, takeover scene, then `ModeManager::complete`.
- [ ] Clean up on `RoutineStarted<TurnEnding>` everywhere. Today Hydro does it there, but Meridian, Skyrail and Solarium
      wait for `TurnStarting`, so they hold the slot through the end of the ball.
- [ ] **Resume by keeping the mode system alive.** On a drain, the mode gives up its slot and goes inactive, and resuming
      makes it active again. `is_active` already follows the current mode, and `on_deactivate` / `on_reactivate` are how
      `SkyrailStationMode` handles its ramp. This keeps progress with no snapshot type. Cues sit in the system's container
      and appear to tick with the system, so confirm a paused timer really pauses.
- [ ] Completion stops depending on the mode staying alive for its effect. Generalize `FlashersSystem` into a one-shot
      effects system (`play(effect)` and forget). It's a stand-in for gap `oneshot`.

## 1.5 Scoop qualifier

- [ ] Add a single `ScoopQualifier`, replacing `ExclusiveModeQualification<T>`, the per-mode qualifiers,
      `NimbusPromenadeQualification` and `SporeMultiballQualification`.
- [ ] It needs 2, then 3, then 4… lit shots (decision 2), and is inactive while a mode runs.
- [ ] Add `add_lit_shot(ctx)` so a Surveillance set counts as one.
- [ ] A mini-wizard lights the scoop as soon as its step is reached, with no qualifying.
- [ ] Remove `LiftRampStartable` from mode starting. Every mode now starts at the left scoop. Its ramp work moves to
      Groundswell's lock (Part 4).

## 1.6 Scoop hold and mode select

`LowerScoopSystem` ejects 750ms after a ball lands, every time, and nothing can stop it. This blocks the menu, mode
intros and the extra ball animation.

- [ ] Add `hold(requester, ctx)` and `release(requester, ctx)` to `LowerScoopSystem`, keyed by `ctx.id()` as ball save
      does. It ejects once nothing is holding.
- [ ] Emit `LowerScoopEjected`. Combos and the mode ball save both need it.
- [ ] A tilt or the end of a turn clears all holds and ejects.
- [ ] Rework `LeftScoopStartable` into `ModeSelectSystem`, in this order on a ball entering:
  1. A lit extra ball collects first (front entry only), with its animation.
  2. If a mode is lit and only one ball is in play, open the menu.
  3. Release, and the chosen mode's start phase takes over.
- [ ] The menu follows `RULES.md`: flippers move, both flippers or start choose, a timeout takes the highlighted row,
      and a single option shows briefly then starts.
- [ ] Mode select LEDs: scoop bolts pulse, available inserts pulse, the highlighted one goes solid.

## 1.7 Hardware layer

- [ ] Add `LeftOutlaneSystem` to `main.rs`. It isn't registered, so `LeftOutlaneRollover` never fires and
      `LeftOutlaneSkillShot` can't award.
- [ ] Add a matching right outlane system and event, then sling hit events (`LeftSlingHit`, `RightSlingHit`).
- [ ] Add `DropBankCleared`, read from the target switches, and raise the bank on it. It only raises on the paddle or
      when asked today, and Sentinels and Skyrail both need this.
- [ ] Add pass lane events for the hidden right outlane save.
- [ ] Share the lift ramp. Both Skyrail and Groundswell's lock want it up. Use `request_up(requester, ctx)` and
      `release(requester, ctx)` in place of `lift_up` / `lift_down`, and the ramp stays up while anyone asks.
- [ ] Give every GI LED a location, or positional scenes will skip them: `gi_l_sling`, `gi_r_sling`, `gi_drop1`,
      `gi_drop2`. `gi::LEFT_RAMP` is also defined but isn't wired to a port.
- [ ] Measure the speaker rings' start and direction.
- [ ] Wire up the wireform post (decision 1). The post is physically there, but its driver probably isn't connected:
  - Check the wiring, then connect the coil to a driver and add it to `hardware` and `io_network.rs`.
  - Check whether the machine can tell how many balls are on the post. No switch for this is defined, so it may need one.
  - Add `WireformSystem`, then switch Groundswell and Derailment from virtual to real locks.

## 1.8 DMD

Everything here must run on what exists today: the `Canvas`, pixel fonts and numbered layers. `DmdScene` loads and the
console edits it, but its own docs say "there's nothing to play one with yet", so no scene art plays in a game yet.

- [ ] **Virtual DMD.** `main.rs` skips the DMD when `VIRTUAL` is set, so none of this can be tested off the machine.
      Listed as a prereq in `RULES.md` and in the frontbox `TODO.md`.
- [ ] **`DmdCard`:** a shared card with a title line, a body line and a duration. `ExtraBallSystem`, `TiltPresenter` and
      `BonusSystem` each hand-build this today. It's the stand-in for all DMD art in Parts 2 to 5.
- [ ] **Layer plan.** Tilt and extra ball share layer 2, so one clears the other's screen, and bonus and turn end share
      layer 1. Give each purpose its own layer: score, side quest row, mode, card, menu, tilt.
- [ ] **Score screen:** add the mode line and the side quest row with pips.
- [ ] **Mode select menu** in lotko. The operator menu is built around its own config sections, so take the input
      handling from `InitialsEntrySystem` instead.
- [ ] **Status pages** (hold a flipper) and **idle rotation**. Flipper switches are ordinary `SwitchClosed` events, so the
      "input routing" need in `RULES.md` is mostly met already.
- [ ] **Scoop eject summary.**
- [ ] Each mode's DMD now line is a `DmdCard` plus the mode line, unless it says otherwise.

## 1.9 Sound and callouts

40 callouts are on disk and **none are loaded or played**. `sound_loader.rs` only has SFX, and nothing calls
`play_callout` yet.

- [ ] Add the callouts to `sound_loader.rs` the way SFX are done (`include_bytes!` and `preload_embedded`).
- [ ] Add a `Callout` enum and a single `ctx.say(Callout::…)` so call sites don't hold file keys. Steward chimes and
      integrity variants can then be added in one place.
- [ ] Fix the frontbox callout queue. `callout_queue` is a `Vec` used with `push` and `pop`, so the newest queued line
      plays first, and it never drops stale ones. Make it first in, first out, and give lines a priority so a mode start
      can replace a queued hit line.
- [ ] Add a `CalloutCompleted` event so an intro can hold the scoop for exactly as long as its line.
- [ ] Set up escalating hits as a shared helper: `city_hit(progress)`. It extends `rising_organic_hit` from city to
      forest. Today's organic sounds are the stand-in until the palette exists.
- [ ] Retire the old voice in `sounds/hydro-core/` (a different voice from the Steward).
- [ ] Add the Steward's two-note chime. Bake it into the files, or sequence it in `say` once the queue is fixed.

Music: `ModeManager` has placeholder commercial tracks for Hydro, Skyrail, Solarium, Meridian, Nimbus, spore multiball
and the default. Apex, the mini-wizards and Groundswell have nothing. `play_music` plays a track once, so anything longer than the track goes
silent (gap `loop`). All original music is still to come.

### Lines still to record

- **Oaks: all 18 are missing**, none found by file name. Wake, Again, Hold, Not yet, Soon, Ours, Drink, The way is open,
  Nothing is yours, Open sky, Open the gates, Darkness, Fall, Higher, Ready, Stay, Rise, Heartwood.
- **Steward:** each section lists its own. `callouts/combos`, `callouts/side`, `callouts/multiball` and
  `callouts/blackout` are empty.
- **Numbers:** "N of six" only has `four-of-six`, and "Walkway N" only has `walkway-4`.

## 1.10 LED conventions and shared scenes

All 67 game scenes use `Surface: Sequence`. Only attract uses `Area`, so positional effects are new to the game code.
The system tick is 67ms (about 15 updates a second), so anything strobing past about 7Hz will alias. Design around
that rather than trying to fix it.

**Z layers.** Agree on these now, or stacked side quests, modes and multiballs will fight:

| z | Use |
|---|---|
| -1 | Ambient: GI tint, arc and subway base |
| 0 | Side quests: spore bar, hex rings, drop strips, map |
| 10 | Mode attention, hex centers |
| 20 | Hits and short reactions |
| 30 | Full-field moments: starts, completions, multiball starts |
| 40 | Blackout's dark layer |
| 41 | What Blackout leaves lit: roots and lit shots |
| 50 | Tilt (existing) |

**Shared scenes.** `RULES.md` reuses a few effects across every mode. Build these once and drive them with params.
`Prop::Param` works for `Translate` (`Vec2`), colors and floats, so one scene can serve every shot.

| Scene | Params | Used for |
|---|---|---|
| `shared/shot_wave.scene` | `origin`, `color`, `green` | Radial hit on any shot |
| `shared/surge.scene` | `from`, `to`, `color` | Packet between two points, a `Sweep` along a `Route` |
| `shared/takeover.scene` | `origin` | Every completion (≈ growth) |
| `shared/city_scan.scene` | `color` | Steward alerts, ball start, top-down sweeps |
| `shared/ring_fill.scene` | `count`, `color` | Hex ring segments (Tunnels, Apex, locks) |
| `shared/arc_timer.scene` | `color`, via `.over(duration)` | Arc ramp timers, adapted from `hydro_core/progress.scene`, which is on the spore bar |
| `shared/speaker_timer.scene` | `color`, via `.over(duration)` | Countdowns on the speaker rings |
| `shared/heartbeat.scene` | `color`, via `.over(beat)` | Heartwood, ready states |
| `shared/forest_ambient.scene` | `amount` | Ball start ambient (≈ noise) |

### Primitive status

| Primitive | Status | Approximation for now | Gap |
|---|---|---|---|
| Radial wave | Works | `Ring` with animated radius on `Area("Playfield")`, as `ring-burst.scene` does. A `Radial` gradient softens it. | `feather` |
| Plane sweep | Partly | `Rect` with animated `Translate`, one layer per plane with times offset by hand | `stitch` |
| Growth | Missing | A green ring expanding from the seed, with `Prop::Random` flicker. It lights by distance, not along branches. | `growth` |
| Path flow | Works | `Sweep` along a `Route` or `Bezier`, or a moving shape on a `Sequence` of ordered LEDs | — |
| Noise field | Missing | A few large soft blobs with `Prop::Random` positions and opacity. For hero moments, a pre-rendered animated image in an `Image` fill. | `noise` |
| Glitch | Partly | `Curve::Random` / `Steps` and `Prop::Random` on `Opacity` and `HueShift` | `glitch` |
| Heartbeat | Works | A keyframed double pulse, with tempo through `.over()` | — |
| Fill | Works | Animated `Rect` height, or a `width` param as `spore_bar.scene` does | — |
| Layers and masks | Partly | Per-layer masks, and z across systems. `LedQ` has no "all but", so "all but the roots" is a dark layer with the roots one z above it. Alpha compositing only. | `blend` |
| Tempo sync | Missing | Set `.over()` by hand to match a track's BPM. It drifts, so keep these short. | `tempo` |

## 1.11 Spores

- [ ] `SporesSystem` gains a `mode_shot(ctx) -> u8` that every mode calls on a hit. It applies 1 spore, or 2 in a burst,
      and returns the progress to add. This replaces `ctx.emit(SporeApplied)` in each mode.
- [ ] The charge and burst logic is in Part 5.

## 1.12 Scoring and perks

- [ ] Put all values in `points`. Nimbus and the spore multiball still have literals.
- [ ] Add a `PerksSystem` (or a query on `ModeManager`): `has(Perk)` from completed districts. Part 5 lists where each is
      consumed.
- [ ] Add a base value for the center spinner. Apex's perk doubles it, but once Apex is rewritten nothing scores it.

## 1.13 Framework gaps

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
| `oneshot` | led | Fire-and-forget effect that outlives the system that started it | Yes |
| `dmd-player` | dmd | Play a `DmdScene` in a game, not just the console | Partly |
| `virtual-dmd` | dmd, console | DMD output in `VIRTUAL` mode | Yes |
| `dmd-stack` | dmd | Screen stack with priority, replacing numbered layers | Not listed |
| `callout-queue` | sound | Ordered queue, priority, and a finished event (needed in 1.9, not optional) | Not listed |
| `voice-fx` | sound | Runtime dropout, wobble, stutter, bitcrush for Steward integrity. Pre-rendered variants are the alternative. | Yes |
| `loop` | sound | Loop points for music and ambient beds | Yes |
| `stems` | sound | Multi-stem music | Yes |
| `leaderboards` | pinball | Lower-is-better, typed values, shared initials entry | Yes |

## 1.14 Fixes to make along the way

- [ ] `main.rs`: `HighScoresSystem` isn't registered, so no high score is ever saved or entered.
- [ ] `main.rs`: register `LeftOutlaneSystem` (1.7).
- [ ] `PlayfieldIllumination::update` is never set back to false, so it redeclares GI on every tick. Fix it, and check
      whether it helps the open console slowdown. That link is a guess, so test it rather than assuming.
- [ ] `apex_terraces.rs` should become a folder like the other districts when it's rewritten.
- [ ] `ExclusiveMode::start` has `todo!()` arms. They go away with 1.2.

---

# Part 2: Districts

Every district plays on the 6 main shots plus the drops, uses `ModeRun`, and starts from the menu. All of them need the
same note on sound and music, so it's stated here once. Palette SFX, a real music track, and Steward integrity
processing (`voice-fx`) are outstanding for each. Each section lists only what's specific to it.

## Meridian Basins — orange

Sweeping basins. Tier 1, alternative to Hydro.

**Today:** works as an exclusive mode with 6 hits to complete, using a window of 3 over 10 "shots" that include the pops
and captive ball. It starts from the lift ramp. There's no start phase and no callouts.

**Rules work**

- [ ] Move to `MainShot`, so the window is 3 of 6, moving right every 2.25s and wrapping.
- [ ] Fix the first window. It's `LeftOrbit, DomeRamp, LeftOrbit` today, and should be left orbit, dome ramp, arc ramp.
- [ ] Each hit adds progress through `mode_shot` (a burst counts double). Complete at 6.
- [ ] Resume keeps hits.

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

Coolant combo. Tier 1, alternative to Meridian.

**Today:** the closest to `RULES.md`. The 5-shot combo works, with scenes for attention, ring, hit, intensity, progress,
flashers and completion.

**Rules work**

- [ ] Time shots 2 to 5 at 25s (decision 6). The code leaves shot 5 untimed.
- [ ] Fix the off-by-one. `on_spawn` calls `restart_combo`, which counts as a failed attempt, so the first try already
      has +5s and plays a hit sound.
- [ ] Use the Hydro perk's +5s through a shared `mode_timer(base)` helper once perks exist.
- [ ] Decide on resume: keep attempts and the shots seen, and restart the combo at the lift ramp.
- [ ] Remove the stale TODO about changing the shot order. `RULES.md` has the current order.

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

Choke points. Tier 1 gateway, the easiest district.

**Today:** a non-exclusive pop-chase mode that qualifies from the vertical spinner. Replaced entirely.

**Rules work**

- [ ] Rewrite as a `Mode` on the 6 main shots, with each shot `Open`, `Sealed` or `Choked`.
- [ ] Every 6s (start value), telegraph a seal on a random open shot, then seal it about 1s later. Never seal the last
      open shot, and shorten the interval as chokes go up.
- [ ] A hit on an open shot chokes it for the rest of the mode. A hit on a sealed shot reopens it.
- [ ] Complete at 4 chokes.
- [ ] Resume keeps which shots are choked. All others come back open.
- [ ] Delete `NimbusPromenadeQualification` and the pop logic. Keep `qualification_hit.scene` and
      `qualification_progress.scene`: they suit Charge (Part 5).

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

**Today:** close to `RULES.md`, with attention, hit, intensity, flasher and completion scenes.

**Rules work**

- [ ] Fix completion. `RULES.md` wants 5 drop hits during the finale, but `target_hits` also counts the 2 before it, so
      the code needs only 3.
- [ ] Five hits on a 3-target bank means raising it mid-finale. Use `DropBankCleared`.
- [ ] Use the shared lift ramp requests. Lock lights wait while Skyrail owns the ramp.
- [ ] Keep the ball save running until the mode ends.
- [ ] Fix the progress total to match the new count (5 + 3 ramp shots + 2 drops = 10).
- [ ] Resume: restart at the current stage. The finale restarts with its 2nd ball.

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

**Still to record:** no Steward lines missing. `RULES.md` has no Oaks line for it.

**To fully finish**

- `noise` or a dissolve mask so blue "scatters", `growth` for the takeover.
- `dmd-player` plus art: train arriving, board glitching, roots tearing up the track.
- SFX: station ambience (`loop`), train rushing past, signal clank, crossing bells, rail screech and crash.

## Apex Terraces — gold

Overload. Tier 2, alternative to Skyrail.

**Today:** a non-exclusive mode on the pop targets and spinner, always running from game start. Replaced entirely.

**Rules work**

- [ ] Rewrite as a `Mode` in its own folder, on the 6 main shots, in gold.
- [ ] One lit shot. Each hit fills a ring segment. A full ring bursts for a big award, and the lit shot moves to
      `farthest_from`.
- [ ] Terraces need 3, then 4, then 5 hits. Complete at 3 bursts.
- [ ] The ring loses a segment after 6s without a hit (start value).
- [ ] The lit shot's ring shows overload over Root tunnels (z 10 over z 0).
- [ ] Resume keeps bursts and restarts the current ring empty.
- [ ] Delete the target and spinner scenes.

**Presentation**

| Phase | LEDs now | Sound now |
|---|---|---|
| Start | = Gold bands stepping up the playfield (`Rect` with a `Steps` curve), ending on the first shot | `residents-evacuate-now` |
| Attention | = Center glows gold, filled segments green (`ring_fill`). = The next segment to drain flickers | `this-is-an-exclusive-area` once, early. Chimes missing |
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

**Today:** a ramps mode that `RULES.md` calls too thin, with scenes for attention, hit, intensity, flashers and
completion. The rules are replaced, and the flashers and intensity scenes may carry over.

**Rules work**

- [ ] Rewrite on the 6 main shots. One lit shot moves to a neighbor every 3.5s (start value), never straight back.
- [ ] Choose the next shot ahead of time, so the pre-light is true.
- [ ] Each hit shortens the interval.
- [ ] Complete at 6 hits. Resume keeps hits.
- [ ] Remove the dome-to-lift double scoring.

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

Tier 1 is 2-ball, tier 2 is 3 to 4 ball. All four are new. All need the mini-wizard flow first:

- [ ] Lit as soon as the step is reached, started from the menu like a district.
- [ ] Eject the balls, with `MULTIBALL_BALL_SAVE` timed from `EjectCompleted`. `SporeMultiballMode` has the pattern.
- [ ] End per decision 5, watching `RoutineCompleted<BallDraining>` for `AllSteps`, as multiballs do.
- [ ] Groundswell can't run alongside one. Lock lights wait.
- [ ] A longer intro and a big finish, per `RULES.md`. They use their own tracks, and tempo sync is `tempo` for all four.

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

- [ ] Rework `LiftRampStartable` into `GroundswellLock`. Sentinels light locks, the lit lock requests the ramp up, and
      the shot feeds the rear scoop.
- [ ] Virtual locks (decision 1): count the lock, and let the ball continue to the upper flipper.
- [ ] Lock 3 to start a 3-ball multiball. Jackpots are on the main shots, and the super jackpot is the lift ramp, upper
      flipper, then a drop shot.
- [ ] Relighting gets harder and jackpots reset for each run. The first run's super jackpot lights
      `ExtraBallSource::Groundswell`.
- [ ] Stacks with districts, not mini-wizards. With real locks, the multiplayer post logic in `RULES.md` applies, using
      `ball_temporarily_removed_from_play` and `ball_returned_to_play`. With virtual locks there's nothing shared.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Lock lit | = Lift ramp hex outer ring green | `generic/access-to-city-restricted`, standing in for "Restricted area" |
| Ball locked | = Pulse up the lift ramp. ≈ Ambient throb: `heartbeat` that strengthens per lock | Rumble missing |
| Start | ≈ `Random` strobing, then a green sweep up the playfield from the roots | "Seismic event detected" missing. O: "Rise." |
| Jackpot | = `shot_wave` in green. Speakers: left stays city, right grows | Missing |
| Super jackpot | ≈ Sweep from the drops upward, with the backbox timed by hand | Missing |

**To fully finish:** `noise` and `glitch` for the earthquake, `stitch` for the sweep into the backbox, `tempo` for the
speaker beat, `dmd-player` plus art for the splitting ground and toppling tower, and real locks once the post is wired.

## Heartwood multiball

**Rules work**

- [ ] Rename and rework `SporeMultiballMode`. 2-ball, started by a full Heartwood charge rather than a captive ball shot
      when "startable".
- [ ] Move from the exclusive slot to the multiball slot, so it stacks with any mode but not Groundswell.
- [ ] Finish the super jackpot TODO. The current one is on the captive ball every 5th jackpot. Keep that unless
      `RULES.md` says otherwise.
- [ ] Replace `spore_multiball/*.scene` colors (yellow today) and rename the folder.

**Presentation**

| Moment | LEDs now | Sound now |
|---|---|---|
| Start | ≈ Whole playfield on a `heartbeat`, timed by hand to the track | Heartbeat stinger missing. O: "Heartwood." |
| Jackpot | = Heartbeat `shot_wave` from the shot | Double-thump missing |

**To fully finish:** `tempo` (the whole point of this mode's look), and `dmd-player` plus art for the glowing heart.

---

# Part 5: Side quests, combos, saves and global moments

## Side quests

| Quest | Rules work | LEDs now | Sound and DMD now | To fully finish |
|---|---|---|---|---|
| **Charge** | Spinner spins and pop hits fill the bar. Full lights the burst. Each charge needs more. Burst doubles mode shots for a few seconds, or scores big with no mode. | = Packets along the rays with `surge`; reuse `qualification_hit.scene`. ≈ Bar with a flickering edge using `Random`. Burst insert on `heartbeat`. Burst: `shot_wave` from the left inlane. | `biological-contaminent-identified` on a burst. Row icon and pips. | `feather`, swarm SFX, O: "Ready.", "Airborne contaminant detected" |
| **Surveillance** | 3 pop target cameras. All blinded is a set, and each set needs one more hit per camera. A set adds a lit shot. Every 3rd set lights an outlane. | = Steady red with slow scan. ≈ Hit: `Random` flicker, then dark. = Set: white flash, `surge` down each ray. | Row pips. | `glitch`, "Camera network… offline", camera SFX, static on DMD feeds |
| **Sentinels** | Each bank clear downs a drone. Every 2 light a lock (1 with the Skyrail perk). Awards at 5, 10, 15; 10 lights `ExtraBallSource::Sentinels`. | = Strips scan red, spark and go dark. | Row pips. The 15 award is TBD in `RULES.md`. | "Drone N offline", drone SFX |
| **Heartwood** | Captive hits charge the heart. Full starts the multiball. Each run needs more. | = Bolts on `heartbeat`, faster with charge. | Row pips. | Sub heartbeat and wood knock SFX |
| **Root tunnels** | Orbit hits fill that orbit's hex ring. Full collects value and adds a hit to the current district. | = `ring_fill` per orbit. ≈ Collect: `surge` to the map. | Row pips. | `growth`, dirt SFX. Confirm the center orbit counts as an orbit. |

Modes add district progress through `ModeRun::add_progress` so Tunnels can do it without knowing the mode. Each quest is
a player-template system, so its state is per player with no extra work.

### Perks

| District | Perk | Where it's used |
|---|---|---|
| Meridian | Tunnel hits double | Root tunnels |
| Hydro | +5s on timers | `mode_timer` helper |
| Nimbus | Pops charge twice as fast | Charge |
| Skyrail | One clear lights a lock | Sentinels |
| Apex | Center spinner ×2 | The new base spinner value |
| Solarium | Captive hits charge twice as fast | Heartwood |

## Combos

- [ ] Add `ComboSystem` in the player template, listening to `MainShotHit`, with the 8 routes and a paid set.
- [ ] Each shot within 4s of the last. `LowerScoopBallEnter` pauses the window and `LowerScoopEjected` restarts it.
- [ ] Lift ramp shots with the ramp up break the combo.
- [ ] A 3-shot route also pays 2-shot routes inside it. All 8 make Gridlock.
- [ ] Applies during modes and multiball.

| Moment | LEDs now | Sound now |
|---|---|---|
| In progress | = Path flow toward the flipper. = Continuing shots blink green. Speaker ring countdown | Engine note missing |
| Route made | = `surge` shot to shot along the route, in green | Horn and screech missing |
| Gridlock | = All routes light, shots stall red and pulse, then green | City-wide horn jam missing. O: "Stay." |

**Still to record:** all of it. `callouts/combos` is empty. One line per route plus Gridlock.

**To fully finish:** `dmd-player` plus the route card template (8 routes). A pixel font card works until then.

## Saves and extra balls

**Extra balls** are mostly done. `ExtraBallSystem` handles lit, collected, front-entry only, the drain LED and points past
the limit.

- [ ] Wire `ExtraBallSource::Sentinels` and `::Groundswell`.
- [ ] Hold the scoop for the collect animation, and collect before the menu (1.6).
- [ ] Add callouts: `unregistered-resident` when lit, `error-invalid-resident` when collected.

**Timed saves:** the mode start save is in 1.4. The ball start and multiball saves exist.

**Outlane saves**

- [ ] Add `OutlaneSaveSystem` in lotko, as sling growth and LEDs are this game's. Add it after `BallSaveSystem` in
      `routine![…]`.
- [ ] Sling hits grow both post rings on that side (8 LEDs each, already tagged as GI, so draw above GI).
- [ ] Both rings full lights that outlane. One per side, kept across balls. Each use raises the hits: 12, 18, 24.
- [ ] An outlane drain is one where that outlane's switch fired just before. A hit re-serves with `Launch::Auto` and
      halts the routine.
- [ ] Skip during multiball, leaving it lit.
- [ ] Add the hidden right pass lane save inside this system as a short window, with no effect shown.

| Moment | LEDs now | Sound now |
|---|---|---|
| Growing | = Rings fill a step per hit (`ring_fill`) | Creak missing |
| Lit | = Outlane LED green with a slow `heartbeat` | O: "Hold." missing |
| Used | ≈ `surge` from outlane to plunger lane, rings empty | `retrieval-failed`. O: "Not yet." missing |

**To fully finish:** `growth` for the rings and surge, and `dmd-player` plus art for the roots grabbing the ball.

## Global moments

| Moment | LEDs now | Sound now | To fully finish |
|---|---|---|---|
| Attract | ≈ Existing 4 scenes, plus a city scan and a slow forest scene | none | `noise`, `growth` for roots from the spinner; themed DMD; breathing speaker rings |
| Game start | ≈ `takeover`-style pulse from the spinner, wave to the flippers | `game-start` | O: "Wake." Roots through the title |
| Ball start | = `city_scan`. ≈ `forest_ambient` by overgrowth level | `all-systems-within-operational-tolerance` early, `continuing-city-maintenance` later | "Overgrowth at N percent" lines, `noise` |
| Ball save | ≈ Green radial from the drain; `ball_save.scene` today | O: "Again." missing | Art of roots catching the ball |
| Extra ball | = `extra_ball_lit.scene` | As above | DMD art |
| Drain | ≈ Fade effects toward the roots, GI dims | `growth-contained` | `growth`, integrity on this line |
| Tilt | = Exists (`TiltPresenter`). Add a white snap before dark | `thank-you-for-cooperation` | Clean city font |
| Game over | ≈ Green map up to the districts taken | "Final overgrowth" missing. O: "Soon." missing | Summary screen first, as in 1.8 |

Speaker rings as an overgrowth gauge go on z 0 as their own small system, and everything else overlays them.

**High scores**

- [ ] Register `HighScoresSystem` (1.14) and set the custom categories at game end.
- [ ] Compare by hand for lower-is-better Fastest Blackout, and format it as a time. Proper boards are `leaderboards`.

**Final wizard:** out of scope in `RULES.md`. `final/steward-core-breached` is ready for it. For now, finishing everything
leaves the scoop unlit.

---

# Part 6: Order of work

Each milestone leaves a playable game. This follows the build order in `RULES.md`, with the core pulled forward.

1. **Foundations.** The 1.14 fixes, `MainShot`, hardware events, `CalloutCompleted` and the queue fix, callout loading,
   z layers, `DmdCard`. Virtual DMD if it can be done cheaply, since everything after it needs testing.
2. **Progression.** `Mode`, `ModeManager` with slots and map states, `ModeRun`, resume, `ScoopQualifier`, scoop hold and
   the mode select menu. Exit test: a full route of placeholder modes works for 2 players.
3. **Shared scenes.** Build `shot_wave`, `surge`, `takeover`, `city_scan` and `ring_fill`, then prove them on one mode.
4. **Districts.** Meridian first, because it's the smallest change and proves the pipeline. Then Hydro, Skyrail, Nimbus,
   Apex, Solarium. `RULES.md` asks for Nimbus and Apex early because they block the exclusive slot, and milestone 2
   already removes that.
5. **Saves and side quests.** Outlane saves, extra ball wiring, Charge, Sentinels, Surveillance, Heartwood, Root tunnels.
6. **Multiballs.** Groundswell, then Heartwood.
7. **Combos, perks, status and idle screens.**
8. **Mini-wizards.** Floodgates and Blackout, then Derailment and Canopy.
9. **High scores and global moments.**
10. **To fully finish.** Take the gaps in the order they unlock the most: `dmd-player`, `noise`, `growth`, `tempo`,
    `glitch`, then the rest. Audio goes alongside: record the Oaks, fill in the Steward lines, build the palettes, then
    the music.
