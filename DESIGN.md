# Last of the Kilmore Oaks — Rules Plan

## Theme

The last Kilmore Oaks are retaking a futuristic city that paved over them. The forest is an ancient, hostile force,
not a garden. Its roots reach the city's conduits and take over its systems from below.

- **Spores** are the forest's weapon and the resource you build. The roots charge them ("charge for spores" by the
  vertical spinner), and the forest fires them into the city.
- **Overgrowth** is territory: districts the forest has taken.
- **The city fights back** with drones, defoliant crews and its civic AI, **the Steward**. This gives side quests
  something to fight.

Words to use: breach, overrun, infiltrate, choke, crack, uproot, reclaim, short-circuit.
Words to avoid: plant, water, grow, seed, sprout, bloom.

## Voices

There are two voices, one for each side.

### The Steward (the city)

The city's civic AI. It was built to manage the city's green spaces: to trim them, pave them and keep nature contained. It's the antagonist, and it does most of the talking.

- **Its alerts are the instructions.** Every mode start is a public announcement that tells the player what to shoot, without breaking the fiction: *"Attention residents: unauthorized growth detected on the Solarium ramps. Please stand clear."*
- **Direction:** calm, polite, corporate, unhurried, like a transit announcement. A soft two-note chime comes before each line. It never shouts until late in the game.

- **It breaks down as the city falls.** A global *Steward integrity* level, set by the number of districts played
  (0–6), controls the processing:
  - 0–1: clean, with light PA reverb.
  - 2–3: occasional dropouts and pitch wobble.
  - 4–5: stutters, repeated syllables, wrong words.
  - 6: barely holding together, with bitcrush and glitches.
- **Casting:** text-to-speech works here, since a synthetic voice suits an AI. Record it clean and add the
  breakdown as processing, so one set of lines covers every integrity level.

### The Oaks (the forest)

The forest speaks rarely, and when it does it should land: game start, mini-wizards, multiball starts, ball save,
game end. Lines are one to three words.

- **Direction:** ancient and cold, not a wise old storyteller. Several voices layered (3–5 takes, some pitched down an
  octave), slow, between a whisper and a low murmur. Wood creaks and sub-bass sit under each line. It should sound
  like a whole forest speaking at once.
- **Casting:** record the lines several times yourself (or with a few friends), layer and pitch them, and add
  convolution reverb with a large, dark impulse response.

**Rule:** the Steward explains and the Oaks threaten. A player never needs an Oaks line to understand a rule.

## Sound

- **City palette:** clean synth pads, PA chimes, servo whine, electric hum, alarms, glass, metal.
- **Forest palette:** wood creaks and cracks, concrete splitting, roots tearing through metal, deep sub drones, hollow
  wood percussion, a dense rustling swarm (spores).
- **Hits escalate from city to forest.** Early hits in a mode sound like the city taking damage (a metal clank with a
  creak under it). Late hits sound like the forest winning (concrete shattering, roots ripping).
- **Music:** cyberpunk / synthwave for the city, with forest percussion and drones layering in.
  - Each district mode, mini-wizard and multiball has its own track.
  - Stretch goal: the base track is built from stems, and organic layers come in as more districts are played.
- **Braams are for the biggest moments only:** the start of the finale (`braam-bio1`, not on a resume) and
  the start of the final wizard (`braam-bio2`). Completions don't get one.
- **The Oaks' lines have no chime** and are never dropped for waiting. The Steward's passing commentary is dropped if it
  waits more than 3s.

### Stand-in SFX

Until the palettes exist, the recorded SFX stand in like this:

| SFX | Used for |
|---|---|
| `steward-chime` | Before every Steward line |
| `latch-heavy1`–`3` | First two `city_hit` steps; Meridian's gate clunk; Nimbus blocks; a camera blinded |
| `impact-dirt` | Last `city_hit` step; spore burst; drone down; roots spread |
| `impact-cinematic-heavy` | Choosing a mode in the menu; Gridlock; Helix Vaults and Groundswell super jackpots |
| `scan-light` | Ball start, with the status line |
| `scan-heavy-stutter` | Surveillance set |
| `woosh-light`, `latch-double` | Skyrail train leaving and stopping |
| `error1` | Hitting a shot the Steward has blocked (Nimbus) |
| `arp_hit1` | Burst ready, outlane lit, combo route made, ArcStreak reflex shot |
| `cymbal-mid` | Each vertical spinner spin |

`hum`, `motor1`–`5`, `scan-light2` and `scan-med` are loaded but unused.

## Lighting language

- **City:** cold white, cyan, pale blue. Clean and geometric: scanlines, grids, hard on/off, even pulses. City effects
  sweep in straight lines from the top of the playfield, where the skyline is, and from the backbox.
- **Forest:** bioluminescent yellow-green, amber, deep moss, with violet for spores. Organic: noise, spreading,
  uneven, flickering. Forest effects start at **the roots** (the vertical spinner) or at the point a ball hit.
- **The central motif is takeover:** a forest effect spreads over a city effect and replaces it. Every completion
  uses it.
- **Mode colors.** Plain green is the forest's alone: Charge, completions, a completed district on the city map, and
  takeovers. Shades of one color family can share a tier, but not cross tiers.

  | Tier | Colors |
  |---|---|
  | 1 | Meridian yellow (255, 220, 0), Cargo Quay orange |
  | Gate 1 | Nimbus magenta |
  | 2 | Hydro cyan, Helix Vaults neon yellow-green (the only green place), Skyrail blue |
  | 3 | Apex gold, Halcyon Vents pale sky blue (120, 200, 255) |
  | Gate 2 | Caldera Generator ember (255, 75, 0) |
  | Finale | Solarium purple |
  | Off the map | Groundswell bronze, Heartwood lime |
- **The Steward is red.** Anything the Steward does to the playfield (a blocked walkway, a watching camera) is lit red,
  and red means the player can't make that shot count right now. No district or forest effect uses red.
- **Lit means "hit this".** A shot stays lit only while hitting it does something. Once it's done, it goes dark.
- **The system tick is 67ms** (about 15 updates a second), so anything strobing faster than about 7Hz aliases. Design
  around it.

**Z layers.** Side quests, modes and multiballs stack, so each keeps to its layer:

| z | Use |
|---|---|
| -1 | Ambient: GI tint, arc and subway base |
| 0 | Side quests: spore bar, hex rings, drop strips, sling rings, pop targets; the speaker overgrowth gauge |
| 10 | Mode attention, hex centers |
| 20 | Hits and short reactions, mode progress on the spore bar |
| 30 | Full-field moments: starts, completions, multiball starts, spore burst |
| 40 | Caldera Generator's dark layer |
| 41 | What Caldera Generator leaves lit: roots and lit shots |
| 50 | Tilt |

### Effect primitives (input to the 2D/3D LED roadmap)

The effects below are built from these. Every LED, GI lamp and flasher has a position in 3D space (playfield, backbox
speakers, cabinet), and all of them take part.

| Primitive | What it does | Used by |
|---|---|---|
| Radial wave | A ring expanding from a point, with speed, width and falloff | Hits, spore burst, jackpots |
| Plane sweep | A plane moving through space, e.g. from the playfield up into the backbox | Steward alerts, multiball starts, super jackpots |
| Growth | Spreads from a seed along an LED nearest-neighbor graph, branching randomly; the trail fades or stays | Roots, completions, Charge, Spread roots |
| Path flow | Packets travel along an ordered path (pop rays, ramps, subway, orbits) | Charge, locks, Hydro, Skyrail |
| Noise field | An animated simplex noise field for color and brightness | Ambient forest, intensity, Halcyon Vents |
| Glitch | Random dropouts, stutter, wrong colors, hard cuts | Steward breakdown, Caldera Generator, Hydro intensity |
| Heartbeat | A double pulse at a tempo, local or global | Heartwood, timers, spore burst ready |
| Fill | Fills space up to a height, with a moving surface | Helix Vaults, mode progress |
| Layers and masks | Effects composited by priority with blend modes, limited to regions (e.g. "all but the roots") | Side quests over modes, Caldera Generator |
| Tempo sync | Effect timing locks to the music's BPM | Multiballs, mini-wizards |

### Backbox speaker LEDs

Each speaker has a ring of 25 LEDs. They're in the 3D space, so sweeps, waves and noise fields carry through them like
any other LED. They also have jobs of their own, because they're the one place a player looks up at, next to the DMD.

- **Overgrowth gauge.** During no modes, both rings show the city's state for the current player: city white-blue,
  with green spreading in from the bottom as districts are played. It's the background layer and always comes back.
- **Countdowns.** A ring drains for anything on a clock, so players can read time without looking down: the combo
  window, the mode select menu timeout, Hydro's shot timer, the ball save and hurry-ups.
- **Beat.** In modes and multiballs the rings pulse with the kick drum (tempo sync).
- **Left is city, right is forest.** For moments that are a fight, e.g. Groundswell's jackpots, the left ring stays
  city and the right ring grows, and each jackpot pushes green across into the left ring.
- **Warnings.** Tilt warning: both rings flash red once. Tilt: they go dark with the GI.
- **Attract.** Slow breathing in the city colors, with roots creeping up from the bottom now and then. While a high
  score page shows, the rings take that category's color.

## Playfield resources

| Feature | Use |
|---|---|
| City map (6 inserts) | District state: unplayed / available / played / completed |
| Spore bar (16 LEDs, "spores") | Spore charge |
| Biospore insert, left inlane target | Spore burst ready |
| Roots + pop rays around the vertical spinner | Charge animation flowing into the roots |
| Main shot hexes (7 LEDs) | Center: district mode / qualifier. Outer 6: side quest progress or lock lit |
| Arc ramp LEDs | Mode timers and hurry-ups |
| Drop target strips (4 LEDs × 3) | Each orbit's spreads toward Groundswell; Sentinel drones; Groundswell's foundations |
| Captive ball bolts | Heartwood charge |
| Left scoop bolts | Scoop lit (district/mini-wizard select) |
| Pop LEDs, pop target LEDs | Charge (pops) and Surveillance (pop targets) |

Ball paths that matter:
- Left (dome) ramp → left flipper.
- Arc ramp → subway → back of the left scoop. The front of the scoop is a harder direct shot.
- Lift ramp down → right flipper. Lift ramp up → rear scoop → VUK → wireform with up-post (holds 3) → upper flipper,
  which aims at the drops.

## Primary progression: overgrow the city

The city map (design/city/README.md): three tiers by height, with a gate between each, and the finale at the top.
Multiballs (*) are places like any other.

```
FINALE   Solarium Atrium
GATE 2   Caldera Generator*
TIER 3   Apex Terraces · Halcyon Vents*
TIER 2   Hydro Core · Helix Vaults* · Skyrail Station
GATE 1   Nimbus Promenade
TIER 1   Meridian Basins · Cargo Quay
         [ spores bar — the roots ]
```

A player plays one place on each step, so a game covers 6 of the 10 places on the map:

1. Tier 1: Meridian **or** Cargo Quay
2. Gate 1: Nimbus
3. Tier 2: Hydro **or** Helix Vaults **or** Skyrail
4. Tier 3: Apex **or** Halcyon Vents
5. Gate 2: Caldera Generator
6. Finale: Solarium, for now an ordinary district. Spire Core, its second phase, isn't built.

Groundswell, the forest's own multiball, isn't on the map: spreading roots lights it (see Multiballs). Cargo Quay has
no insert on the city map, which has six.

### Rules

- **Playing a mode moves you forward.** Starting a mode marks it *played*, and its next step becomes available.
  A mode only ends by completing it or by draining.
- **No going back or sideways.** Choosing Meridian closes Cargo Quay for the rest of the game, and so on.
- **Exception: unfinished modes can be resumed.** A mode that was played but not completed stays in the menu, and
  resuming it keeps its progress. Groundswell isn't resumed: see Multiballs.
- **Two tries at each place.** Every start or resume from the menu is a try, and a place gets two: the first and one
  resume. One given back by the Sentinels doesn't use one. Out of tries, it's lost for the game: it leaves the menu,
  its map insert goes dark, and its perk can't be had. The route doesn't wait on it, as the next step opened when it
  was first started. Completing Nimbus Promenade, which everyone plays, gives every place a third try, including
  ones already lost.
- **Completing a district earns a persistent perk** for the rest of the game (see Perks).
- **Starting modes:** lit shots (white hex centers) light the left scoop. A shot into the scoop, from the arc ramp or
  the front, opens the mode select menu. Multiball places qualify the same way as districts.
- Qualifying gets harder each time (e.g. 2, 3, 4… lit shots).
- Each player has their own map state.

Open:
- The finale as a wizard mode in two phases, Solarium Atrium then Spire Core ("Last of the Kilmore Oaks"). Out of
  scope for now, and finishing everything leaves the scoop unlit. `final/steward-core-breached` and `braam-bio2` are
  recorded for its start.
- Hydro's timer currently restarts when it runs out. Leave it, and revisit once modes can be resumed.

### District modes

Keep the strong modes. Rework the thin ones to the level of Hydro (combo timing). Every district now plays on the 6 main shots (left orbit, dome ramp, arc ramp, center orbit, lift ramp, right
orbit) plus the drops. The pops, pop targets and vertical spinner are left to the side quests.

| District | Current | Plan |
|---|---|---|
| Meridian Basins | Sweeping window of 3 lit shots | Rework as **spreading growth** (built) |
| Cargo Quay | (new) | **Pushing containers to the crane** (built) |
| Hydro Core | Timed 5-shot combo | Keep; hexes flash faster as time runs out (TODO) |
| Nimbus Promenade | Pops (non-exclusive) | Rework as **choke points** |
| Skyrail Station | Lift ramp and drops in turn, then 2-ball | Rework as **the train** (built) |
| Apex Terraces | Pop targets + charged center spinner (non-exclusive) | Rework as **overload** |
| Solarium Atrium | Ramps — too thin | Rework as **crawling vine** |

**Meridian Basins: spreading growth.** The forest takes root in the farm's basins and spreads from one to the next
(design/city/meridian-basins.md).
- 2 main shots are seeded at random, flashing yellow: the farm's grow lights.
- Hitting a flashing shot blooms it (it goes dark, as it's done), and the growth spreads to the shots on either side, which start
  flashing. The outer orbits have only one neighbor.
- Every 15 seconds without a bloom, there's an even chance the last one withers back to flashing, along with the
  spread it gave.
- Each hit of a Root Lash blooms a flashing shot at random.
- Complete: all 6 shots bloomed.

**Cargo Quay: the crane.** The forest loads the city's freight onto the crane (design/city/cargo-quay.md).
- The lift ramp is the crane. It stays down until both containers are in, as its coil buzzes while holding it up.
- 2 containers start on main shots, lit orange, in one of three layouts that each take 5 pushes: left orbit and center
  orbit, left orbit and right orbit, or dome ramp and arc ramp.
- Hitting a container's shot pushes it one shot toward the crane. Containers pushed onto the same shot stack (the
  whole hex lights) and move together.
- Once a mode, a hit has a 1 in 3 chance to push two shots instead. It's only rolled 2 or more shots from the crane.
- Once both are in, the ramp goes up and the crane (the lift ramp's hex and the bolt under it) flashes. Shoot it to
  complete.
- Each hit of a Root Lash pushes a container at random, or makes the crane shot once both are in.
- Single ball, no timer.

**Hydro Core: coolant combo.** The forest drinks the core dry, one pipe at a time, before the Steward can reroute.
- A timed combo: lift ramp → arc ramp → left orbit → center orbit → dome ramp. Each shot has 25s.
- When time runs out the combo restarts from the lift ramp, and each failed attempt adds 5s to the timer.
- Complete: make the full combo.
- Each hit of a Root Lash makes the shot the combo is on.

**Skyrail Station: the train.** The forest destroys the stations before the city's train can reach them
(design/city/skyrail-station.md).
- A station, any main shot but the last one, flashes blue for 3 seconds as the train is announced. Then the train, a
  comet with a fading tail, runs along the hexes to it in 3 seconds and fades out, leaving the station lit.
- Destroying the station before the train gets there is worth double a mode hit, and after, three quarters. Then the
  next station is announced, and the train leaves from the one just destroyed.
- Each hit of a Root Lash destroys a station, at the rate it's at.
- Complete: 4 stations. Destroying all 4 before the train gets there is a perfect, worth half the completion again.
  Single ball, no add-a-ball.

**Nimbus Promenade: choke points.** The Steward is sealing the walkways between the pavilions to keep the forest out
of the upper city.
- All 6 main shots start lit magenta. These are the open walkways.
- It takes 2 hits to close a walkway: the first weakens it (purple), and the second closes it (dark for the rest of
  the mode). A closed shot isn't lit, so nothing lit is ever done.
- Every few seconds the Steward blocks a walkway that isn't closed: it flashes red, then stays red for 8 seconds. It
  never blocks the last one that can be hit. A blocked walkway keeps its progress for when the block lifts.
- Hitting a blocked walkway does nothing but an error buzz, and the Steward says *"Restricted area."*
- Complete: close 3 walkways. Nimbus is the gateway, so it's the easiest district.
- Each hit of a Root Lash moves a random walkway on a step.

**Apex Terraces: overload.** The forest feeds one terrace until it bursts.
- One shot is lit gold, starting on a random main shot. Each hit on it fills one segment of its hex's outer ring.
- A full ring bursts the terrace for a big award, and the lit shot moves to the shot farthest away.
- Each terrace needs more hits than the last: 3, then 4, then 5.
- Pressure: the ring loses one segment after a few seconds without a hit.
- Complete: burst 3 terraces.
- While the mode runs, the lit shot's outer ring shows overload instead of Spread roots progress.

**Solarium Atrium: crawling vine.** A vine crawls across the glass dome, and you chase its head.
- One shot is lit, starting on a random main shot. Every few seconds the lit shot moves to a neighboring shot. Only the
  head is lit. The shot it leaves flashes green as the trail.
- A hit on the head moves the vine on at once, and speeds it up: it moves every 3.5s, 0.35s sooner per hit, down to
  1.5s (start values).
- Complete: 6 hits. Solarium is the finale, so it's the hardest.

### Perks (from completing a district)

Each perk boosts a side quest, so the progression and the side quests feed each other.

| District | Perk |
|---|---|
| Meridian | Spread roots grow twice as fast |
| Cargo Quay | Pops charge spores twice as fast |
| Hydro | +5s on all mode timers and hurry-ups |
| Nimbus | One more try at every place (3 plays, normally 2) |
| Skyrail | One Sentinel clear lights a lock (normally two) |
| Apex | Center spinner values ×2 |
| Solarium | Captive ball hits charge Heartwood twice as fast |

The multiball places have no perk.

### Multiball places (mini-wizards)

The places that play as a multiball, still called mini-wizards in the code. They're chosen and qualified like
districts. Every one is 2-ball, and Heartwood can still stack on top. The second ball is added as the first leaves
the scoop, with the multiball ball save (`MULTIBALL_BALL_SAVE`) timed from the eject. Completing one ends it while
the balls play on, and marks it completed.

- **Helix Vaults** (tier 2, 2-ball, design/city/helix-vaults.md). The forest drains the biomatter vats. Orbit hits
  drain the vat, and after enough of them a ramp lights for a rupture. Complete: 3 ruptures.
- **Halcyon Vents** (tier 3, 2-ball, design/city/halcyon-vents.md). The spinners and pops are the plant's turbines and
  build value; the ramps vent it, and the value starts over from its base. Complete: 3 vents.
- **Caldera Generator** (gate 2, 2-ball, design/city/caldera-generator.md). The forest cuts the city's power. GI goes
  off and the playfield goes dark except for the roots (the vertical spinner's ring and the pop rays) and lit shots.
  Each hit darkens another shot. When they're all dark, the scoop is the super jackpot. Complete: the super jackpot.
- **Groundswell** isn't a place, but runs the same way: see Multiballs.

### Scoring

A mode is worth more the higher up the city it is. Each step sets a district's shot value and every completion
(`points::mode_hit` and `points::mode_completion`), with shots carrying about a third of a district. A full run is
worth about 1B from the modes alone, before side quests and bonus.

| Step | Shot | Completion | About |
|---|---|---|---|
| 1: Meridian, Cargo Quay | 1.5M | 35M | 45M |
| 2: Nimbus | 2.25M | 50M | 60M |
| 3: Hydro, Skyrail | 3.5M | 70M | 105M |
| 3: Helix Vaults | (own scoring) | 110M | 235M |
| 4: Apex | 4.25M | 80M | 105M |
| 4: Halcyon Vents | (own scoring) | 225M | 345–375M |
| 5: Caldera Generator | (own scoring) | 225M | 375M |
| 6: Solarium | 5M | 90M | 120M |

Districts and multiballs on the same step don't match yet: the multiballs kept the values they had as mini-wizards,
and Groundswell is valued as tier 3. To rebalance.

- **Hydro** escalates: each combo shot is worth half a step shot more than the last, so the 5 come to 7.5 shots.
- **Apex** takes 12 hits, so its hits are 2M, plus 2M times the bursts so far on each burst.
- **Mini-wizards**, so the choices on a step match:
  - **Helix Vaults:** 1M an orbit, super jackpots of 20M times how many.
  - **Caldera Generator:** 4M a shot, and a 120M super jackpot.
  - **Groundswell:** 3M a target times the clears so far plus one, and 16M times the clears on each clear.
  - **Halcyon Vents:** builds from 25M, at 250K a spin, 1M a pop and 5M a Root Lash hit.
- A burst or a Root Lash doubles a shot's points along with its progress. Where a shot can't count twice (Hydro,
  Nimbus, Skyrail), it still scores double.

## Side quests

Side quests run all the time, on top of district modes. The DMD shows their status (see DMD).

### Charge (spores)

- Vertical spinner spins and pop hits fill the spore bar: a spin charges 1, a pop 3 (6 with the Nimbus perk). The pop
  rays animate toward the spinner.
- A full bar lights the biospore insert: **spore burst ready**.
- Hitting the left inlane target fires the burst. For 8s every mode shot counts double: for spores applied, its points,
  and its mode progress where the mode allows it, with S: *"Double score."* With no mode running, it scores 2,500,000
  instead.
- The bar empties, and each charge after that needs more: 60 for the first burst, 30 more each after.
- While a mode is running, a mode hit shows the mode's progress on the bar in its color for 1.5s, then the bar goes
  back to the charge.
- **Spores applied:** every mode shot applies one spore, which a burst doubles. This replaces `SporeApplied` as it is
  today and feeds the high score.

### Surveillance (pop targets)

The Steward watches the pops through three cameras, one on each pop target.
- The cameras come online after 40 playfield switch hits in a player's game, and stay on for the rest of it.
- The pop target LEDs are lit red while their camera is watching. Hitting a target blinds that camera, and its LED
  goes dark.
- Blinding all 3 is a set. The cameras come back on, and each set needs one more hit per camera than the last.
- Each set counts as one lit shot toward lighting the left scoop, so the pops help you reach the next district. It only
  counts between modes, as the scoop doesn't qualify during one.
- Every third set lights an outlane save on a side that isn't already lit (see Outlane saves).
- **The Steward is watching.** Once the cameras are online, one has to be blinded at least once a minute. Each blinded
  camera starts the minute over, and so does each new ball. If a minute passes without one, the Steward dispatches the
  Sentinels. The minute is wall-clock time, so cradling the ball counts against the player. Watch whether that feels
  fair.

### Sentinels (city drones)

A takeover mode: the city's drones are the drop targets.
- Dispatched by Surveillance (above), with S: *"Sentinels deployed."* Each dispatch sends half a drone more than the
  last, rounded down: 1, 1, 2, 2, 3, and so on. The count goes by dispatches, not defeats, so draining doesn't make the
  next one easier.
- **It takes over above everything.** The priority is the Sentinels, then the district, then everything else. A
  running district is suspended, and picks up again in play where it left off once the Sentinels are defeated. No
  district or multiball can start while they're out.
- **It waits for the drops and the balls.** If the minute runs out during a mode that uses the drops or adds balls
  (every mini-wizard) or a multiball, the Sentinels come once it's over: when it completes, or at the start of
  the next ball if it ended with a drain. They also wait for the ball to leave a scoop.
- Half the GI goes the Steward's red and the other half stays white. The drop strips are red on the targets still up,
  and the bank is raised at the start.
- The lift ramp is up while they're out, so the ramp feeds the upper flipper, which aims at the drops. This is a
  gentler feed than the left orbit. The bolt under the ramp flashes red and the flashers cycle red.
- Each drop bank clear downs a drone. Down all the drones dispatched to defeat the Sentinels.
- A drain ends them. A district they suspended is left to resume from the menu.
- Outside the Sentinels the drops score consolation points and feed the roots (see Spread roots).
- **Rewards, for now:** every 2 drones downed lights a lock (1 with the Skyrail perk), and 5 and 15 drones give
  points. These came from when the drones were a side quest, and need
  revisiting now that they're a punishment (see design/steward/sentinels.md).

### Heartwood (captive ball)

- Captive ball hits charge the heart; the bolts show the charge, beating from 1.4s down to 0.5s as it charges.
- Fully charged starts **Heartwood multiball** (replaces Spore Multiball). It needs more charge each
  time: 4 hits, then 8, then 2 more each time (10, 12, ...).

### Spread roots (orbits)

- Orbit hits grow roots. The orbit hexes' outer rings fill one segment per hit, 6 to a ring. The rings start over
  each ball.
- **The drops feed the orbits**, one each: target 1 (nearest the player) the left orbit, target 2 the center, target 3
  (nearest the arc ramp) the right. Knocking one down grows its orbit's ring a segment, like an orbit hit. The orbits
  can be hard to make, and confusing, so this is a second way in.
- **Clearing the bank ends a Root Lash's cooldown**, so the roots grow again straight away.
- **The strips show the way to Groundswell.** Each target's strip fills bronze with its orbit's spreads toward
  lighting Groundswell, and is lit only while the target is standing.
- The Sentinels and Groundswell have the drops to themselves while they run, so none of this happens then.
- A full ring spreads the roots, scoring their value, and arms **Root Lash** on the action button. It's bronze, and
  pulses lime while a district mode or mini-wizard is running.
- Pressing it during a mode counts as 2 shots in it. With no mode running, it stays armed. It doesn't fire while the ball is in the shooter lane (the button launches it) or the mode select menu is open.
- After a lash, the roots stop growing until 50 more playfield switch hits. A spread while a lash is armed only
  scores.
- Spreading the roots on all three orbits in one ball lights an extra ball, once a game (see Extra balls).

## Combos: traffic routes

The forest is learning the city's roads. A combo is a 2- or 3-shot route made in a row, and each route pays once per
game.

Shots that return the ball to a flipper start or continue a route:
- **To the left flipper:** the dome ramp, the right orbit (fast, but catchable), and the left scoop eject (it bounces
  off the right flipper).
  - Left flipper shots: center orbit (spinner), lift ramp, right orbit, the right pops and their targets, the vertical
    spinner, the captive ball.
- **To the right flipper:** the lift ramp (down).
  - Right flipper shots: the left scoop, left orbit, dome ramp, arc ramp, the captive ball, the left pop and its
    target.
- The left orbit also returns, but too fast to catch reliably, so it only ends routes.

| Route | Shots | Note |
|---|---|---|
| Interchange | Dome ramp → lift ramp | The basic cross-flipper combo |
| Return trip | Lift ramp → dome ramp | The other direction |
| Express line | Lift ramp → arc ramp | Ends in the subway and the back of the scoop |
| Roundabout | Right orbit → right orbit | Repeat the orbit off the left flipper |
| Ring road | Right orbit → lift ramp | |
| Underpass | Arc ramp → lift ramp | Through the subway and scoop eject, to the left flipper |
| Cloverleaf | Dome ramp → lift ramp → dome ramp | 3 shots |
| Grand tour | Dome ramp → right orbit → lift ramp | 3 shots |
| **Gridlock** | Make all 8 routes | Big award |

- Each shot after the first must come within about 4s of the one before. Time in the scoop doesn't count, so the
  window starts again when the scoop ejects.
- A 3-shot route also pays any 2-shot route inside it that the player hasn't made yet.
- Combos through the lift ramp only work with the ramp down. When it's up for a lock, the shot goes to the wireform,
  and the combo breaks.
- Combos run alongside modes and score in multiball too.

## ArcStreak: reflex shots

Hit the ball straight off the lower scoop's eject and up the arc ramp, within about 2s of the eject, for a reflex shot.
The arc ramp feeds the subway back to the scoop, so a good player can loop it.

| Reflex shot | Award | Callout |
|---|---|---|
| First | 2,000,000 | S: *"Two million."* |
| Second | 3,000,000 | S: *"Three million."* |
| Each after | 5,000,000 | S: *"Five million."* |

- The streak breaks when an eject goes by without a reflex shot, when the ball comes into the scoop any way but the
  subway, or when the ball ends.
- ArcStreak runs alongside modes and combos.

## Multiballs

### Groundswell (from spreading roots)

The roots heave up the ground under the city. It's the forest's own multiball, earned rather than reached on the map.
It replaces both Derailment and the old plan for a Groundswell lock, which had lost its locks with the Sentinels
becoming a punishment.

- **Lit by spreading roots** over the game: once on each of the three orbits. The count is kept across balls, as the
  rings start over each ball. Each time after, it needs one more spread on every orbit (2 each, then 3). The Steward
  says *"Seismic activity alert."*
- **Lit at the scoop without qualifying.** It's the first row of the menu, and choosing it neither uses up a qualify
  nor closes anything on the map. Each lighting plays it once: whether it's completed or ends unfinished, it goes
  out, and lighting it again takes the roots spread again. It starts fresh each time.
- **2-ball.** It takes the mode slot, so it runs between districts. Heartwood can stack on it.
- **Play:** the lift ramp is up for the whole mode, so the balls feed the upper flipper into the drops: the city's
  foundations. Each drop target is a jackpot, and each bank clear breaks a foundation for a super jackpot. Complete:
  3 foundations. Only the targets still standing are lit, so a knocked down target goes dark until the bank
  comes back up.
- Valued as tier 3: 3M a target times the clears so far plus one, 16M times the clears on each clear, and 225M to
  complete.

### Heartwood multiball

- 2-ball, from the captive ball.
- Every main shot is a jackpot. A hit dims that shot, and hitting it again scores half a jackpot and puts it out.
- Every 5th jackpot lights the captive ball as a super jackpot for 20s. Hitting it lights every jackpot again. With
  every shot out, the super jackpot stays lit until it's hit.
- Making the arc ramp all the way to the subway twice adds a ball, once a multiball, with S: *"Additional
  deployment."*
- Can stack with any mode, multiballs included
- Its track plays only when no mode is running. Over a mode, the mode's music keeps playing.

## Extra balls and saves

### Extra balls

An extra ball is first **lit** and then **collected** with a shot, so earning one always takes a second shot.

Ways to light one, each once per game:

| Source | When |
|---|---|
| Tier 1 and gate 1 | Completing both the place played in tier 1 and Nimbus Promenade (gate 1). Unfinished places can be resumed, so it's a reason to go back and finish them |
| Root network | Spreading the roots on all three orbits in one ball |

- **Collect at the front of the left scoop.** It's the hardest shot on the playfield, so it suits the biggest award.
  A ball coming in from the arc ramp doesn't collect, since the subway opto shows it came in the back. The scoop then
  holds the ball for the animation.
- If the extra ball and a mode are lit together, the extra ball is collected first, then the mode select menu opens.
- **Operator settings:** most extra balls per game (`MAX_EXTRA_BALLS`, framework default 5). Past the limit, each
  source awards points, so setting it to 0 gives points instead (for competition).
- The drain LED (shoot again) stays lit amber while an extra ball is waiting.

### Timed ball saves

These already exist through `BallSaveSystem`:

- At the start of each ball.
- At the start of each multiball, mini-wizard included (`MULTIBALL_BALL_SAVE`, 12s).
- New: a short save (about 8s) when a district mode starts, since the scoop eject is a common drain.

### Outlane saves: roots catch the ball

No physical kickback, so these are software saves: a drain through a lit outlane is saved, and the ball is
re-served from the plunger and auto-launched.

- **Earn them at the slings.** Each sling hit grows roots up that sling's two post LED rings (8 LEDs each). When both
  rings on a side are full, that side's outlane is lit. Every third Surveillance set also lights one.
- One save per side at most. A save lasts until used, and carries over between balls for that player.
- Each use raises the hits needed to relight that side: 12, then 18, then 24, and it stays at 24.
- **Only outlane drains count.** A drain is an outlane drain if that outlane's switch fired in the 3s before it. A
  center drain isn't saved.
- During multiball the outlane save isn't used and stays lit; multiball has its own timed save.

The right pass lane's lower switch grants a hidden 3s right outlane save, for a ball that doesn't make it all the way
through.

### Drain order

Following the framework's `BallDraining` routine, the systems get the chance to save a drain in this order:

1. `BallSaveSystem`: timed saves.
2. `OutlaneSaveSystem`, so a lit outlane isn't spent while a timed save would have covered the drain.

An extra ball isn't a drain save. It's another turn for the same player after a normal `TurnEnding`, so bonus still
runs.

## Presentation

Every mode uses the same phases as the code does today, with a **start** phase added in front:

- **Start:** the mode's intro.
- **Attention:** shows the player what to hit.
- **Hit:** the reaction to a shot. It can change as the mode goes on.
- **Intensity:** ambient effects that build tension. Stages are at 25 / 50 / 75% progress.
- **Completion:** the machine's reaction when the mode is completed.

Lines in quotes are callouts: **S** is the Steward, **O** is the Oaks. Lines are drafts.

### Global moments

| Moment | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Attract | Slow forest noise field. Every so often roots grow out of the spinner over the city map and pull back. City scan lines sweep down from the top | none | Title; city skyline slowly covered by roots; high scores |
| Game start | Roots ignite at the spinner, then a wave runs down to the flippers | S: *"Good morning, residents. The city is clear."* O: *"Wake."* | Roots crack through the title |
| Ball start | City-white scan down the playfield, then the forest ambient for the current overgrowth level | S: a status line, e.g. *"Overgrowth at twelve percent."* | Player / ball / overgrowth % |
| Mode select | The scoop bolts pulse. Available districts on the city map pulse in their colors; the highlighted one is solid | Wooden clicks to move; a deep impact to choose | The currently selected city region. Name and perk |
| Ball save | A green radial pulse from the drain | O: *"Again."* | Roots catch a falling ball |
| Extra ball lit | The left scoop bolts flash white | S: *"Unregistered resident detected near scoop."* | EXTRA BALL LIT |
| Extra ball collected | A growth from the roots to the drain LED, which stays lit amber | S: *"Error. Invalid resident."* | A second root breaks the surface |
| Outlane save growing | Each sling hit grows the roots a step up that side's post rings | A short creak | — |
| Outlane save lit | Both post rings on that side are full; the outlane LED glows green with a slow heartbeat | O, whispered: *"Hold."* | — |
| Outlane save used | Roots surge from the outlane up to the plunger lane; the post rings empty | O: *"Not yet."* S: *"Retrieval… failed."* | Roots grab the ball as it falls and throw it back |
| Drain | Forest effects pull back into the roots; GI dims | S: *"Growth contained."* As integrity drops, it sounds less sure | Roots withdraw underground |
| Tilt | Everything snaps to city white, then goes off | S, smug: *"Thank you for your cooperation."* | TILT in a clean city font |
| Game over | Growth spreads over the city map up to the districts the player took, then holds | S: *"Final overgrowth: thirty-four percent."* O (at high overgrowth): *"Soon."* | Per-player summary: score, districts, spores applied |

### District modes

#### Meridian Basins — spreading growth, yellow

The hydroponic farms. The forest's seeds take root in the sterile basins and spread from one to the next.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | The mode's start scene in yellow, then 2 seeded hexes flash yellow | Air pressure release; S: *"Crop contamination detected."* | Rows of plants fade in |
| Attention | Flashing hexes are the shots to hit; bloomed hexes are dark | — | — |
| Bloom | Green energy bursts from the shot, and the hexes on either side start flashing | A swell of growth | — |
| Wither | The last bloomed hex goes back to flashing, and the hexes it spread to go dark | — (TODO) | — |
| Completion | 7 to 10 bursts all over the playfield | The swell builds; S: *"Meridian Basins uninhabitable."* O: *"Ours."* | Roots wrap the rows of plants |

#### Cargo Quay — the crane, orange

The river freight terminal. The forest pushes the city's containers onto the crane.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | The mode's start scene in orange, then the 2 containers light | S: stand-in *"Biological contaminant identified."* (TODO) | The gantry crane over the stacked containers (TODO) |
| Attention | A container lights its hex center, a stack its whole hex | — | — |
| Push | The container slides to the next hex, with a short tail; the map shows the pushes | A heavy latch (stand-in for a metal clang) | — |
| Crane ready | The lift ramp goes up, and its hex and the bolt under it flash | A motor (stand-in for the crane's whine) | — |
| Completion | A white flash and an orange shockwave from the crane, then green spreads over the playfield | O: *"Fall."* (the Steward's line is TODO) | A stack falling (TODO) |

#### Hydro Core — coolant combo, timed, cyan

The water-cooled power plant. The forest drinks the coolant, and each combo shot is another pipe.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | Coolant chases through the combo in order: lift ramp, arc ramp and subway, left orbit, center orbit, dome ramp | S: *"Hydro Core coolant pressure… dropping. Engineers to the pipes."* | Cutaway of the core, 5 pipes full of cyan coolant |
| Attention | The next combo shot's hex spins cyan; the arc ramp LEDs are the timer | A reactor hum | Coolant gauge, with the next pipe highlighted |
| Hit | A white-hot surge from the shot to the next one in the combo, and the shot just made stays green | The hum rises in pitch with each combo shot; a power thump | Roots wrap another pipe |
| Intensity | The timer drains on the arc ramp and flashes faster near the end (TODO). 50%: GI flickers from power instability. 75%: flashers strobe like a warning light | Klaxon at 75%; S: *"Core temperature… rising."* | Coolant level falls; warning lights |
| Completion | A power overload: everything flashes white, then green floods in from the subway. GI returns tinted green | A huge power-down whine. S: *"Core offline."* O: *"Drink."* | The core goes dark and roots fill it |

#### Nimbus Promenade — choke points, magenta (the gateway)

A boardwalk of open-air pavilions high among the clouds. The main shots are the walkways between them.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | Magenta runs out from the center of the playfield along every main shot, lighting the walkways | S: *"The Promenade is closed for maintenance. Walkways will be sealed shortly."* | A map of walkways between pavilion lanterns |
| Attention | Open walkways glow magenta and weakened ones purple, each with a slow shimmer. Before a block, the target shot flickers red for a moment, then stays red like a shutter coming down | Faint wind chimes; a metal latch on each block. S: *"Walkway four… sealed."* | Shutters drop over walkways |
| Hit | Weaken: a purple flash and the shot goes purple. Close: growth bursts from the shot's center into its hex, then it goes dark. Blocked: a red flash | A thick wooden thump + the boardwalk splintering; a blocked shot is an error buzz and S: *"Restricted area."* | Roots knot across the walkway |
| Intensity | A magenta haze (noise field) drifts over the shots that are still open. Seals come faster. 75%: the backbox speaker LEDs pulse with the music | The chimes detune; seals sound more frantic | Clouds roll in |
| Completion | Every walkway floods green at once. A growth spreads from the roots in every direction, Nimbus lights on the map, and growth climbs to Skyrail and Apex | S: *"Access to the upper city is… restricted."* O: *"The way is open."* | Camera pans up to the upper districts |

#### Skyrail Station — the train, blue

The elevated train hub.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | The mode's start scene in blue | S: *"Skyrail service is suspended. Clear the platform."* | A train pulls into the station |
| Announced | The station's hex flashes blue; the flashers swap like crossing lights, faster as departure nears | Crossing bells (TODO) | Departures board: next station and a countdown |
| Travelling | A comet with a fading tail runs along the hexes to the station, easing in and out | Train rushing past (stand-in: `woosh-light`) | — |
| Arrived | The comet fades out as its tail catches up; the station's hex is solid blue | Brakes and a chime (stand-in: `latch-double`) | — |
| Station destroyed | An explosion: flash, shockwave, debris thrown toward the flippers, the wreck smoldering; the flashers spin and skid to a stop | `city_hit`, rail screech and a crash (TODO); S: *"The 5:15 is delayed."* on the first | A line flips to DELAYED |
| Completion | The last station stops dead and stutters, derails down the playfield to a crash at the flippers, the hexes close one by one, then green spreads from the roots | S: *"Skyrail unavailable."* for a perfect (stand-in), then *"All lines are now closed."* | Every line DELAYED, then roots tear up the track |

#### Apex Terraces — overload, gold

The luxury tower district with its manicured terraced gardens. The Steward's showpiece.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | Gold bands step up the playfield, terrace by terrace, ending on the first lit shot | S: *"Residents of the Apex: please keep off the terraces."* | Terraced towers in gold |
| Attention | The lit shot's center glows gold, and its filled ring segments swell with green. A segment that's about to drain flickers | Clean, expensive chimes | One terrace, with a growth gauge |
| Hit | One more ring segment fills green, and the center flashes. Burst: a green radial wave from the shot, then gold growth travels across the playfield to the next shot | A wet creak per hit, deeper as the ring fills; burst is stone cracking and soil spilling. S: *"Terrace three: structural concern."* | The hedges swell, then the terrace bursts |
| Intensity | Terrace bands rise faster. 75%: the backbox goes gold | The chimes go out of tune | The towers sway |
| Completion | The manicured gold breaks into a wild green noise field over the whole playfield | S: *"This is a… premium… area."* O: *"Nothing is yours."* | The towers are covered in green |

#### Solarium Atrium — crawling vine, purple

The glass dome at the top of the city. A vine crawls across the glass.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | A purple sweep from the top down, then a vine grows from the roots to the first shot | S: *"Atrium integrity: excellent. There is nothing on the glass."* | The dome from below, in clear light |
| Attention | The vine's head (the lit shot) glows purple with a green trail. | Slow glass stress groans | The vine's head moves across the dome |
| Hit | A white shatter flash radiates from the shot; the vine speeds up | Glass cracking; louder each hit | A crack spreads |
| Intensity | Cracks build up: white points flicker across the playfield. 75%: GI shifts to a warm purple-white like sunlight through broken glass | Groans become creaks; S stutters: *"There is nothing on the… glass."* | The dome is crazed with cracks |
| Completion | The dome shatters: a white burst over the whole playfield, then green comes down from the top as a plane sweep | A huge shatter and falling glass. O: *"Open sky."* | The glass falls and the vine fills the frame |

### Mini-wizards

These are the biggest moments in the game. Each has a long intro, music synced to the lights, and a large finish.

| Mini-wizard | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| **Helix Vaults** | A fill rises from the flippers up the playfield with a moving surface; the arc ramp LEDs are the gauge. Super jackpot: a wave crashes over everything from the top down | Gates grinding open; rushing water. O: *"Open the gates."* S: *"Flood barriers… engaged. Engaged. Engaged."* At high water, S: *"Shoot the ramps."* | The gates open; a water-level gauge |
| **Caldera Generator** | GI off; everything is dark except the roots and lit shots. Each hit kills a section. Super jackpot: one second of total darkness, then the roots explode with light from the spinner outward | A descending power-down whine and a clunk for each section; the Steward fragments, with S: *"System 67 invalid."* at 3 of 6 dark. O: *"Darkness."* | The city grid, with sectors going black one by one |
| **Halcyon Vents** | A green noise field climbs the playfield and up into the backbox speaker LEDs. Collects at the ramps send growth up into the backbox | Giant trunks creaking, a deep roar of wind high above the city. O: *"Higher."* | The forest canopy spreads across the skyline |

### Side quests

| Quest | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| **Charge** | Pop hits send packets along the rays to the spinner; spins pulse the roots; the spore bar fills with a flickering edge | A swarm rustle that thickens as the bar fills | Spore bar in the side quest row |
| **Spore burst ready** | The biospore insert heartbeats green and violet | O, whispered: *"Ready."* | — |
| **Spore burst fired** | A violet-green radial wave from the left inlane across the whole playfield | A swarm whoosh and a deep thump. S: *"Airborne contaminant detected."* | A spore cloud bursts across the screen |
| **Surveillance** | Watching pop targets glow a steady red with a slow, scanning pulse. Hit: a static glitch, then the LED goes dark. Set: all 3 flash white, and a packet flows from each pop down its ray into the roots | A camera whir and a lens crack; on a set, static, then S: *"Camera network… offline."* | A grid of camera feeds, with feeds cutting to static |
| **Sentinels** | The drop strips scan red like drone eyes. Down: the strip sparks and goes dark | A drone whine pitching down, then a crash. S: *"Drone three offline."* | A drone falls |
| **Lock lit** | The lift ramp hex's outer ring lights green; the ramp goes up | S: *"Restricted area. Do not use the Skyrail."* | — |
| **Heartwood** | The captive ball bolts heartbeat; each hit strengthens the beat and speeds it up | A sub heartbeat and wood knock | Heart charge in the side quest row |
| **Spread roots** | Each hit grows one segment on the orbit hex's outer ring. Lash: growth runs from the orbits down to the current district on the city map | A dirt crunch per segment; a deep tearing on a spread | The roots' path to the district |

### Combos

The Steward treats combos as traffic violations. All routes share one template, so only the route name and the shots
change.

| Moment | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Combo in progress | A path flow runs along the shot just made toward the flipper it fed, and the shots that would continue a route blink green | A rising engine note per shot | — |
| Route made | Green light races along the whole route in order, shot to shot, then fades | Car horn and a tire screech. S: *"Traffic advisory: unregistered vehicle on the Interchange."* | Route name over a road map, with the path drawn in green |
| Gridlock | All 8 routes light at once, then every main shot goes to a stalled red and pulses before going green | A city-wide horn jam. S: *"All routes… gridlocked."* O: *"Stay."* | A traffic map with every road in green |

### Multiballs

| Multiball | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| **Groundswell** lit | The left scoop lights bronze | S: *"Seismic activity alert."* | — |
| **Groundswell** start | An earthquake: noise-driven strobing across everything, then a plane sweep from the roots up into the backbox | S: *"Seismic event detected."* O: *"Rise."* | The ground splits under the city |
| **Groundswell** jackpot | A green radial wave from the shot | A heavy hit; S counts the damage | JACKPOT with cracks |
| **Groundswell** super jackpot | A foundation breaks: a plane sweep from the drops up into the backbox | The biggest impact in the game | Tower toppling |
| **Groundswell** completion | Green spreads from the drops over the playfield | S: *"Seismic damage identified."* | The city tilts on its broken foundations |
| **Heartwood** start | The whole playfield beats, synced to the music | A heartbeat that drops into the track. O: *"Heartwood."* | The heart of the oak glowing through the bark |
| **Heartwood** jackpot | A heartbeat radiates from the shot | A double-thump stinger | The heart pulses |

### DMD animations to build

Most of these share a template with art swapped in, which cuts down the amount of work.

- **Takeover template** (used 6 times): a district backdrop, then roots cover it. Use it for every district
  completion.
- **District intros** (6): each district's backdrop with the Steward's alert line.
- **Mini-wizard intros and finishes** (4 + 4).
- **Multiball starts** (2), plus **jackpot** / **super jackpot** stingers.
- **Side quests:** spore burst, cameras offline, drone down, ball locked, Heartwood charged.
- **Combos:** one route card template (8 routes), plus Gridlock.
- **Moments:** game start, ball save, extra ball, tilt, game over summary.
- **Attract:** title, the overgrowing skyline, high score pages.
- **Mode select map.**

## DMD

### New screens and widgets

1. **Mode select menu** (at the left scoop)
   - The scoop holds the ball while the menu is up.
   - Lists the available districts, available mini-wizards and unfinished modes. Each row shows a name, a one-line
     summary and the perk it awards.
   - Flippers move the selection. Both flippers (or the start button) choose. A timeout picks the highlighted item,
     with a countdown bar.
   - With only one option, show it briefly and start it.
2. **Status pages** (hold a flipper)
   - Holding a flipper for about 1.5s while the ball is cradled shows status pages. Tapping the other flipper moves
     to the next page. Letting go returns to the score.
   - Pages: side quests, city map progress, perks earned, locks, combo routes made.
3. **Idle rotation**
   - When nothing has scored for a few seconds, the score screen rotates through side quest status and, when there's
     more than one player, the other players' scores.
4. **Score screen side quest row**
   - A row of small icons, each with a few progress pips: Charge, Surveillance, Sentinels, Heartwood, Roots,
     Locks.
5. **Scoop eject summary**
   - A quick quest summary while a scoop holds the ball.

### Framework needs (frontbox-dmd)

- A screen stack with priority, so overlays (status, menu) cover the score and return to it.
- In-game flipper input routed to the active DMD screen, separately from the flipper coils.
- A list/selection widget. The operator menu in `frontbox_dmd::menu` may be a starting point.
- Timed page rotation.

## High scores

Custom categories, stored with `HighScoresSystem::set_custom_score`:

| Category | Measure |
|---|---|
| Spore Champion | Most spores applied in a game |
| Fastest Caldera Generator | Shortest time to complete Caldera Generator |
| Biggest Flood | Highest Helix Vaults total |
| Halcyon Vents Champion | Highest Halcyon Vents total |
| Groundswell Champion | Highest Groundswell total |
| Heartwood Champion | Highest Heartwood multiball total |
| Sentinel Hunter | Most drones downed in a game |
| Deep Roots | Most roots spread in a game |
| Overgrowth | Most districts completed (ties broken by score) |

Notes:
- "Fastest" scores are lower-is-better. The caller must compare before setting, and the DMD must show the value as a
  time.
- Each category is a single entry today. That's fine to start with.
- Fastest Caldera Generator counts only the time it was running, from the dark coming down to the super jackpot.

## Code impact (lotko-homebrew)

- Replace the per-district qualifiers (`ExclusiveModeQualification<T>`) with a single scoop qualifier.
- `ModeManager` gains per-player map state: played / completed / saved progress for each mode.
- Add 4 mini-wizards to `ExclusiveMode`. Nimbus and Apex move from `NonExclusiveMode` to `ExclusiveMode`.
- `LeftScoopStartable` hosts the mode select menu.
- Spore Multiball becomes Heartwood. `SporesSystem` keeps counting spores applied and adds bursts.
- New side quest systems: Charge, Surveillance, Sentinels, Heartwood, Spread roots.
- A combo tracker: listens for the shot hit events, tracks the timing window and the lift ramp state, and stores the
  routes made per player.
- Nimbus, Apex and Solarium are rewritten on the main shots. The pops, pop targets and vertical spinner no longer
  count toward any district.
- A new `BallDraining` participant after `BallSaveSystem`: `OutlaneSaveSystem`, in lotko.

## Build Order

### Prereqs

- Virtual DMD for testing
- improved DMD DSL + Menu


#### Rules

1. Map progression with the single qualifier, played / completed state and resumable modes.
2. Groundswell lock and Sentinels.
3. Extra balls, outlane saves and the district start ball save.
4. Charge, Surveillance, Heartwood, Spread roots, combos, and the status / idle DMD screens.
5. Perks.
6. Mini-wizards.
7. High score categories.
8. Rework Nimbus (choke points), Apex (overload) and Solarium (crawling vine). Nimbus and Apex need to become
   exclusive modes as part of step 2, so do them early.

#### Presentation

1. LED effect primitives in frontbox. Start with the ones most effects depend on: radial wave, growth, path flow,
   and layers and masks. Then plane sweep, noise field, glitch, heartbeat, fill and tempo sync.
2. Steward lines: write the script, record or generate clean takes, then build the integrity processing chain.
   The speaker LED voice meters need the sound system to report the playback level of the voice channel.
3. Oaks lines: record, layer and process.
4. Sound effects for the city and forest palettes.
5. DMD: the takeover template and the district backdrops first, since they're used the most.
6. Music: one track per district, then mini-wizards and multiballs. Stems are a stretch goal.
