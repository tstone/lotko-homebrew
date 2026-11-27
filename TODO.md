- Hydrocore mode hexes should flash faster as time runs out
- Nice to have: After completing a mode it can be attempted again, but harder

# Decisions to confirm

The game is built with the default for each. Numbers are kept, as code comments refer to them.

| # | Question | Default built |
|---|---|---|
| 1 | When does the wireform post get wired up? Its driver isn't in `io_network.rs` and there's no switch on the wireform. | Every lock is a virtual lock: the ball goes round to the upper flipper, and the multiball launches from the trough. |
| 2 | Which shots are lit for qualifying? | All 6 main shots start lit. A hit shot goes out, and lit shots reset for the next qualify. |
| 3 | Where does a mode's progress show? The spore bar is Charge's. | On a mode hit, the bar shows the mode's progress in its color for 1.5s. |
| 4 | Can the menu open during multiball? | No. With more than one ball in play the scoop ejects and stays lit. |
| 5 | When does a mini-wizard end? | When one ball is left or none, except Helix Vaults, which carries on. |
| 6 | Does Hydro's first shot get a timer? | Shots 2 to 5 are timed, from when shot 1 is made. |
| 8 | Does the center orbit grow roots? | Yes: the left, center and right orbits each have a ring. |
| 9 | What's the Sentinels' award at 15 drones? | 15,000,000 points. |
| 12 | What counts as high overgrowth, for the Oaks' "Soon." at game over? | 3 or more districts completed. |
| 13 | Who says "extra ball" and "shoot again"? | The Oaks, after the Steward's "Error. Invalid resident." |
| 14 | How many extra balls per game? | The framework default, 5. |
| 15 | What should the Sentinels reward? See `design/steward/sentinels.md`. | As they were as a side quest. |
| 16 | What does Skyrail's perk give, now that nothing takes up a lit lock (`LockLit`)? | Unchanged: drones still light locks, which do nothing. |

# Machine checks

- [ ] Hex positions, with `HexTest`: the tip direction and the ring's winding were worked out from the code and artwork, not measured. Spread roots fill the rings from `hex_ring()`'s first LED.
- [ ] The sling post rings fill from their first LED, assumed to be the bottom of each post.
- [ ] The pass lane save catches what it's meant to.
- [ ] Magenta and the weakened purple (120, 0, 255) are easy to tell apart on the hex centers.
- [ ] The left orbit switch's 20ms debounce: a fast ball may pass over it in less time and not count.

# Hardware

- [ ] Give every GI LED a location, or positional scenes skip them: `gi_l_sling`, `gi_r_sling`, `gi_drop1`, `gi_drop2`. `gi::LEFT_RAMP` is defined but not wired to a port.
- [ ] Wire up the wireform post (decision 1): check the wiring, connect the coil to a driver and add it to `hardware` and `io_network.rs`, check whether the machine can tell how many balls are on the post, then add `WireformSystem`. Nothing locks balls now, so this is only for Groundswell or another mode wanting the post.

# Display

- [ ] Status pages (hold a flipper) and idle rotation.
- [ ] A summary as the scoop ejects.
- [ ] The score screen: the current mode, and a row for the side quests.

# Sound

- [ ] Music: the tracks are placeholders. Apex Terraces has none. `colyn-cyclone` (Derailment's) is free.
- [ ] Music and background sounds need loop points.
- [ ] Steward integrity: dropout, wobble, stutter and bitcrush on the Steward's voice as the city falls, or pre-rendered variants.
- [ ] `callouts/combos` and `callouts/blackout` are empty, and there's no `callouts/canopy`.

## Sound effects needed

Effects a built feature wants but plays a stand-in for, or nothing. Delete a line once the effect is in.

- [ ] Skyrail: the train rushing past as it moves (stand-in `woosh-light`).
- [ ] Skyrail: brakes and a chime as it arrives at a station (stand-in `latch-double`).
- [ ] Skyrail: a rail screech and crash for each station destroyed (stand-in `impact-cinematic-heavy`).
- [ ] Skyrail: crossing bells while a station is announced, and station ambience under the mode.
- [ ] Skyrail: a Steward line for a perfect, every station destroyed before the train (stand-in `generic/skyrail-unavailable`).
- [ ] Meridian: a sound for a bloom withering.
- [ ] Cargo Quay: a metal clang for each push (stand-in `latch-heavy1`), and a crash when a stack forms.
- [ ] Cargo Quay: the crane's whine when it's ready (stand-in `motor1`), and quay ambience under the mode.
- [ ] Cargo Quay: Steward lines for the start (stand-in `generic/biological-contaminent-identified`) and the
  completion (none, only `oak/fall`).
- [ ] Sentinels: an alarm when they're sent, a drone whine under them, and a sting when they're defeated.

# Charge

- [ ] The spore bar's flickering edge, and a heartbeat on the burst insert.
- [ ] A swarm rustle and a whoosh.

# High scores

- [ ] Say `new-high-score` when a player makes a board.
- [ ] Boards for the fastest completion of each district and Caldera Generator, once something varies a district's
  score (DESIGN.md, High scores).

# Framework gaps

Written before the scene system, so check which are still open:

| Gap | Need |
|---|---|
| `noise` | A noise fill and noise masks |
| `glitch` | Stutter, glitch and flicker curves, and a bubbling effect for Helix Vaults' orbits |
| `feather` | Soft edges on shapes |
| `stitch` | One effect moving across planes, from the playfield into the backbox |
| `blend` | Blend modes between layers |
| `tempo` | A beat clock from the music to scenes |
| `voice-fx` | Runtime effects on the Steward's voice |
| `loop` | Loop points for music and background sounds |
| `stems` | Music in stems |
| `leaderboards` | Lower-is-better boards, typed values, shared initials entry |
