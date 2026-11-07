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

The city's civic AI. It was built to manage the city's green spaces: to trim them, pave them and keep nature
contained. It's the antagonist, and it does most of the talking.

- **Its alerts are the instructions.** Every mode start is a public announcement that tells the player what to shoot,
  without breaking the fiction: *"Attention residents: unauthorized growth detected on the Solarium ramps. Please
  stand clear."*
- **Direction:** calm, polite, corporate, unhurried, like a transit announcement. A soft two-note chime comes before
  each line. It never shouts until late in the game.
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

## Lighting language

- **City:** cold white, cyan, pale blue. Clean and geometric: scanlines, grids, hard on/off, even pulses. City effects
  sweep in straight lines from the top of the playfield, where the skyline is, and from the backbox.
- **Forest:** bioluminescent yellow-green, amber, deep moss, with violet for spores. Organic: noise, spreading,
  uneven, flickering. Forest effects start at **the roots** (the vertical spinner) or at the point a ball hit.
- **The central motif is takeover:** a forest effect spreads over a city effect and replaces it. Every completion
  uses it.
- **District accent colors** stay as they are now (Hydro cyan, Meridian orange, Nimbus magenta, Skyrail blue,
  Solarium purple), except **Apex changes from green to gold**, so green belongs only to the forest.

### Effect primitives (input to the 2D/3D LED roadmap)

The effects below are built from these. Every LED, GI lamp and flasher has a position in 3D space (playfield, backbox
speakers, cabinet), and all of them take part.

| Primitive | What it does | Used by |
|---|---|---|
| Radial wave | A ring expanding from a point, with speed, width and falloff | Hits, spore burst, jackpots |
| Plane sweep | A plane moving through space, e.g. from the playfield up into the backbox | Steward alerts, multiball starts, super jackpots |
| Growth | Spreads from a seed along an LED nearest-neighbor graph, branching randomly; the trail fades or stays | Roots, completions, Charge, Root tunnels |
| Path flow | Packets travel along an ordered path (pop rays, ramps, subway, orbits) | Charge, locks, Hydro, Skyrail |
| Noise field | An animated simplex noise field for color and brightness | Ambient forest, intensity, Canopy |
| Glitch | Random dropouts, stutter, wrong colors, hard cuts | Steward breakdown, Blackout, Hydro intensity |
| Heartbeat | A double pulse at a tempo, local or global | Heartwood, timers, spore burst ready |
| Fill | Fills space up to a height, with a moving surface | Floodgates, mode progress |
| Layers and masks | Effects composited by priority with blend modes, limited to regions (e.g. "all but the roots") | Side quests over modes, Blackout |
| Tempo sync | Effect timing locks to the music's BPM | Multiballs, mini-wizards |

### Backbox speaker LEDs

Each speaker has 25 LEDs. They're in the 3D space, so sweeps, waves and noise fields carry through them like any other
LED. They also have jobs of their own, because they're the one place a player looks up at, next to the DMD. The ideas
below assume a ring around each speaker; adjust them if the layout is different.

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
| Drop target strips (4 LEDs × 3) | Sentinel progress |
| Captive ball bolts | Heartwood charge |
| Left scoop bolts | Scoop lit (district/mini-wizard select) |
| Pop LEDs, pop target LEDs | Charge (pops) and Surveillance (pop targets) |

Ball paths that matter:
- Left (dome) ramp → left flipper.
- Arc ramp → subway → back of the left scoop. The front of the scoop is a harder direct shot.
- Lift ramp down → right flipper. Lift ramp up → rear scoop → VUK → wireform with up-post (holds 3) → upper flipper,
  which aims at the drops.

## Primary progression: overgrow the city

```
                 Solarium
              ↙            ↘
     [Derailment]        [Canopy]
       Skyrail              Apex
              ↖            ↗
                  Nimbus
              ↙            ↘
     [Floodgates]        [Blackout]
       Meridian            Hydro
       [ spores bar — the roots ]
```

10 modes: 6 districts + 4 mini-wizards. One game covers at most 6 of them:

1. Meridian **or** Hydro
2. Nimbus
3. Tier-1 mini-wizard: Floodgates (if Meridian) or Blackout (if Hydro)
4. Skyrail **or** Apex
5. Solarium
6. Tier-2 mini-wizard: Derailment (if Skyrail) or Canopy (if Apex)

### Rules

- **Playing a mode moves you forward.** Starting a mode marks it *played*, and its next step becomes available.
  A mode only ends by completing it or by draining.
- **No going back or sideways.** Choosing Meridian closes Hydro for the rest of the game, and so on.
- **Exception: unfinished modes can be resumed.** A mode that was played but not completed stays in the menu, and
  resuming it keeps its progress.
- **Completing a district earns a persistent perk** for the rest of the game (see Perks).
- **Starting modes:** lit shots (white hex centers) light the left scoop. A shot into the scoop, from the arc ramp or
  the front, opens the mode select menu. Mini-wizards are lit as soon as their step is reached and need no qualifying.
- Qualifying gets harder each time (e.g. 2, 3, 4… lit shots).
- Each player has their own map state.

Open:
- Is the mini-wizard a required step, or can a player skip it and go up a tier? Draft: required, since it's the
  climax of the tier and keeps the route to 6 modes.
- Final wizard after the tier-2 mini-wizard ("Last of the Kilmore Oaks"). Out of scope for now.
- Hydro's timer currently restarts when it runs out. Leave it, and revisit once modes can be resumed.

### District modes

Keep the strong modes. Rework the thin ones to the level of Hydro (combo timing) and Skyrail (alternating ramp and
drops). Every district now plays on the 6 main shots (left orbit, dome ramp, arc ramp, center orbit, lift ramp, right
orbit) plus the drops. The pops, pop targets and vertical spinner are left to the side quests.

| District | Current | Plan |
|---|---|---|
| Meridian Basins | Sweeping window of 3 lit shots | Keep; trim the sweep to the main shots |
| Hydro Core | Timed 5-shot combo | Keep; hexes flash faster as time runs out (TODO) |
| Nimbus Promenade | Pops (non-exclusive) | Rework as **choke points** |
| Skyrail Station | Lift ramp and drops in turn, then 2-ball | Keep |
| Apex Terraces | Pop targets + charged center spinner (non-exclusive) | Rework as **overload** |
| Solarium Atrium | Ramps — too thin | Rework as **crawling vine** |

**Meridian Basins: sweeping basins.** The forest floods the basins one channel at a time, and you ride the current.
- 3 neighboring shots are lit. Every couple of seconds the window moves one shot to the right, starting at the left
  orbit. After the far right it wraps back to the left.
- Hitting a lit shot is a hit. Complete: 6 hits.
- Today the sweep also passes over the pops and the captive ball. Trim it to the 6 main shots so the pops stay with
  the side quests.

**Hydro Core: coolant combo.** The forest drinks the core dry, one pipe at a time, before the Steward can reroute.
- A timed combo: lift ramp → arc ramp → left orbit → center orbit → dome ramp. Each shot has 25s.
- When time runs out the combo restarts from the lift ramp, and each failed attempt adds 5s to the timer.
- Complete: make the full combo.

**Skyrail Station: signal failure.** The forest shuts down the line, alternating between the tracks and the signals.
- Shots go lift ramp, drops, lift ramp, drops, lift ramp.
- The third ramp shot adds a ball for a 2-ball finale, with a ball save until the mode ends.
- Complete: 5 drop target hits during the finale.

**Nimbus Promenade: choke points.** The Steward is sealing the walkways between the pavilions to keep the forest out
of the upper city.
- All 6 main shots start lit. These are the open walkways.
- Every few seconds the Steward seals one lit shot, and it goes dark. It never seals the last lit shot.
- A lit shot is a hit: roots choke the walkway, and the shot stays green for the rest of the mode.
- A sealed shot can't be choked. Hitting it reopens it and relights it.
- Complete: choke 4 walkways. Nimbus is the gateway, so it's the easiest district.

**Apex Terraces: overload.** The forest feeds one terrace until it bursts.
- One shot is lit gold. Each hit on it fills one segment of its hex's outer ring.
- A full ring bursts the terrace for a big award, and the lit shot moves to the shot farthest away.
- Each terrace needs more hits than the last: 3, then 4, then 5.
- Pressure: the ring loses one segment after a few seconds without a hit.
- Complete: burst 3 terraces.
- While the mode runs, the lit shot's outer ring shows overload instead of Root tunnel progress.

**Solarium Atrium: crawling vine.** A vine crawls across the glass dome, and you chase its head.
- One shot is lit. Every few seconds the lit shot moves to a neighboring shot, and the next one pre-lights dimly.
- Each hit speeds the vine up.
- Complete: 6 hits. Solarium is the last district before the tier-2 mini-wizard, so it's the hardest.

### Perks (from completing a district)

Each perk boosts a side quest, so the progression and the side quests feed each other.

| District | Perk |
|---|---|
| Meridian | Root tunnel hits count double |
| Hydro | +5s on all mode timers and hurry-ups |
| Nimbus | Pops charge spores twice as fast |
| Skyrail | One Sentinel clear lights a lock (normally two) |
| Apex | Center spinner values ×2 |
| Solarium | Captive ball hits charge Heartwood twice as fast |

Completing a district may also give its mini-wizard a bonus (e.g. a longer ball save). Optional.

### Mini-wizards

Tier 1 is 2-ball. Tier 2 is 3–4 ball with higher values.

- **Floodgates** (Meridian). The forest holds the basins and opens the gates. The arc ramp LEDs show a rising water
  level. Orbit hits raise it, and at high water the ramps become super jackpots.
- **Blackout** (Hydro). The forest cuts the city's power. GI goes off and the playfield goes dark except for the
  roots and lit shots. Each hit darkens another shot. When they're all dark, the scoop is the super jackpot.
- **Derailment** (Skyrail). The skyrail is cut. The balls on the wireform drop like falling railcars and feed the
  upper flipper into the drops. Locked balls the player had are kept as virtual locks.
- **Canopy** (Apex). The forest reaches the tops of the towers. The spinner and pops build value; the ramps collect
  it.

## Side quests

Side quests run all the time, on top of district modes. The DMD shows their status (see DMD).

### Charge (spores)

- Vertical spinner spins and pop hits fill the spore bar. The pop rays animate toward the spinner.
- A full bar lights the biospore insert: **spore burst ready**.
- Hitting the left inlane target fires the burst. For a few seconds every mode shot counts double, both for spores
  applied and for mode progress. With no mode running, it scores big points instead.
- The bar empties, and each charge after that needs more.
- **Spores applied:** every mode shot applies one spore, which a burst doubles. This replaces `SporeApplied` as it is
  today and feeds the high score.

### Surveillance (pop targets)

The Steward watches the pops through three cameras, one on each pop target.
- The pop target LEDs are lit red while their camera is watching. Hitting a target blinds that camera, and its LED
  goes dark.
- Blinding all 3 is a set. The cameras come back on, and each set needs one more hit per camera than the last.
- Each set counts as one lit shot toward lighting the left scoop, so the pops help you reach the next district.
- Every third set lights an outlane save on a side that isn't already lit (see Outlane saves).

### Sentinels (city drones)

- Each drop bank clear downs a drone. The target strips show progress to the next drone.
- Every 2 drones lights a lock (1 with the Skyrail perk).
- Awards at 5 / 10 / 15 drones: points, then extra ball lit (see Extra balls), then TBD.

### Heartwood (captive ball)

- Captive ball hits charge the heart; the bolts show the charge.
- Fully charged starts **Heartwood multiball** (replaces Spore Multiball). It needs more charge each time.

### Root tunnels (orbits)

- Orbit hits dig tunnels. The orbit hexes' outer rings fill one segment per hit.
- A full ring collects the tunnel's value and adds one hit of progress to the current district mode.

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

## Multiballs

### Groundswell (main lock)

- Sentinels light locks. When a lock is lit, the lift ramp goes up and the shot feeds the rear scoop → VUK → wireform.
  The post holds up to 3 balls.
- Lock 3 to start a 3-ball multiball. Jackpots are on the main shots. The super jackpot is the lift ramp: the ball goes
  up to the upper flipper, then a shot at the drops.
- Relighting gets harder each time, and jackpot values reset after each run, so it can't be replayed for points over
  and over.
- Can stack with a district mode. Can't stack with a mini-wizard.

**Multiplayer.** Each player has their own lock count, and the balls on the post are shared. If a player locks while
the post holds more balls than that player's count, the post releases one ball. When multiball starts, it tops up from
the trough. Counts are read from the trough's occupancy, not kept as running totals.

### Heartwood multiball

- 2-ball, from the captive ball.
- Cannot stack with Groundswell but can stack with any mode

## Extra balls and saves

### Extra balls

An extra ball is first **lit** and then **collected** with a shot, so earning one always takes a second shot.

Ways to light one, each once per game:

| Source | When |
|---|---|
| Progression | Completing your first district |
| Sentinels | 10 drones down |
| Groundswell | Super jackpot in your first Groundswell |

- **Collect at the front of the left scoop.** It's the hardest shot on the playfield, so it suits the biggest award.
  A ball coming in from the arc ramp doesn't collect, since the subway opto shows it came in the back. The scoop then
  holds the ball for the animation.
- If the extra ball and a mode are lit together, the extra ball is collected first, then the mode select menu opens.
- **Operator settings:** most extra balls per game (default 2), and extra balls awarded as points instead (for
  competition). Past the limit, each source awards points.
- The drain LED (shoot again) stays lit while an extra ball is waiting.

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
- Each use raises the hits needed to relight that side (e.g. 12, then 18, then 24).
- **Only outlane drains count.** A drain is an outlane drain if that outlane's switch fired just before it. A center
  drain isn't saved.
- During multiball the outlane save isn't used and stays lit; multiball has its own timed save.

Passing through the right pass lane grants a hidden, temporary right outlane ball save if the ball accidentally doesn't make it all the way through.

### Drain order

Following the framework's `BallDraining` routine, the systems get the chance to save a drain in this order:

1. `BallSaveSystem`: timed saves.
2. Outlane save, so a lit outlane isn't spent while a timed save would have covered the drain.
3. Extra ball.

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
| Attract | Slow forest noise field. Every so often roots grow out of the spinner over the city map and pull back. City scan lines sweep down from the top | Rare Oaks whispers under ambient city hum | Title; city skyline slowly covered by roots; high scores |
| Game start | Roots ignite at the spinner, then a wave runs down to the flippers | S: *"Good morning, residents. The city is clean."* O: *"Wake."* | Roots crack through the title |
| Ball start | City-white scan down the playfield, then the forest ambient for the current overgrowth level | S: a status line, e.g. *"Overgrowth at twelve percent."* | Player / ball / overgrowth % |
| Mode select | The scoop bolts pulse. Available districts on the city map pulse in their colors; the highlighted one is solid | Wooden clicks to move; a deep impact to choose | The city map. Roots reach toward the highlighted district; the row shows name, summary and perk |
| Ball save | A green radial pulse from the drain | O: *"Again."* | Roots catch a falling ball |
| Extra ball lit | The left scoop bolts flash white; the drain LED pulses | S: *"Unregistered resident detected near the scoop."* | EXTRA BALL LIT |
| Extra ball collected | A growth from the roots to the drain LED, which stays lit | S: *"Error. Resident count incorrect."* | A second root breaks the surface |
| Outlane save growing | Each sling hit grows the roots a step up that side's post rings | A short creak | — |
| Outlane save lit | Both post rings on that side are full; the outlane LED glows green with a slow heartbeat | O, whispered: *"Hold."* | — |
| Outlane save used | Roots surge from the outlane up to the plunger lane; the post rings empty | O: *"Not yet."* S: *"Retrieval… failed."* | Roots grab the ball as it falls and throw it back |
| Drain | Forest effects pull back into the roots; GI dims | S: *"Growth contained."* As integrity drops, it sounds less sure | Roots withdraw underground |
| Tilt | Everything snaps to city white, then goes off | S, smug: *"Thank you for your cooperation."* | TILT in a clean city font |
| Game over | Growth spreads over the city map up to the districts the player took, then holds | S: *"Final overgrowth: thirty-four percent."* O (at high overgrowth): *"Soon."* | Per-player summary: score, districts, spores applied |

### District modes

#### Meridian Basins — sweeping basins, orange

The canal and reservoir district. The lit window is the current moving through the channels, and the roots break the
channel walls.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | An orange current runs across all the main shots from left to right, then settles into the first window of 3 | S: *"Notice: root intrusion in the Meridian channels. Channel gates are closing."* | Overhead view of the basins with channels traced in orange |
| Attention | The 3 lit hexes ripple like water flowing right. As the window moves, the shot leaving drains away and the one arriving fills in | Low water hum; a lock gate clunk each time the window moves | The lit channels flow across the basins |
| Hit | A wave of water from the shot, with green streaks bleeding in. Later hits have more green | A rush of water + concrete cracking; S counts: *"Channel breach. Two of six."* | A crack runs along a channel |
| Intensity | 25%: GI shimmers like light on water (orange/blue noise). 50%: the shimmer speeds up. 75%: flashers pulse with the water | Distant sirens come in | Water level rises in the basins |
| Completion | Roots pour in through every channel and spread across the playfield; the Meridian insert turns green | S: *"Meridian Basins… reclassified."* O: *"Ours."* | Channels overflow with green |

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
| Attention | Open walkways glow magenta with a slow shimmer. Before a seal, the target shot flickers for a moment, then snaps dark like a shutter coming down | Faint wind chimes; a shutter slam on each seal. S: *"Walkway four… sealed."* | Shutters drop over walkways |
| Hit | Choke: growth bursts from the shot's center into its hex, and it stays green. Reopen: a sealed shot cracks open with a white flash and goes back to magenta | A thick wooden thump + the boardwalk splintering; reopen is a shutter tearing | Roots knot across the walkway |
| Intensity | A magenta haze (noise field) drifts over the shots that are still open. Seals come faster. 75%: the backbox speaker LEDs pulse with the music | The chimes detune; seals sound more frantic | Clouds roll in |
| Completion | Every walkway floods green at once. A growth spreads from the roots in every direction, Nimbus lights on the map, and growth climbs to Skyrail and Apex | S: *"Access to the upper city is… restricted."* O: *"The way is open."* | Camera pans up to the upper districts |

#### Skyrail Station — lift ramp + drops, blue

The elevated train hub.

| Phase | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| Start | A blue chase climbs the lift ramp like an arriving train | S: *"Skyrail service is suspended. Clear the platforms and signals."* | A train pulls into the station |
| Attention | Lift ramp hex chases; the drop target strips glow like blue signal lights | The station platform's ambient hum | Departures board |
| Hit | Ramp: a fast chase as a train flies past. Drops: a signal turns red | Train rushing past / signal clank; S: *"The 5:15 is delayed."* | Departures board rows switch to DELAYED |
| Intensity | The drop strips flash like crossing signals. 75%: flashers pulse with the crossing bells | Crossing bells | Board gets glitchy |
| Completion | The chase stops dead; blue scatters and green takes over from the ramp down | A rail screech and crash. S: *"All lines… closed."* | Roots tear up the track |

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
| Attention | The vine's head (the lit shot) glows purple with a green trail. The next shot it will move to pre-lights dimly | Slow glass stress groans | The vine's head moves across the dome |
| Hit | A white shatter flash radiates from the shot; the vine speeds up | Glass cracking; louder each hit | A crack spreads |
| Intensity | Cracks build up: white points flicker across the playfield. 75%: GI shifts to a warm purple-white like sunlight through broken glass | Groans become creaks; S stutters: *"There is nothing on the… glass."* | The dome is crazed with cracks |
| Completion | The dome shatters: a white burst over the whole playfield, then green comes down from the top as a plane sweep | A huge shatter and falling glass. O: *"Open sky."* | The glass falls and the vine fills the frame |

### Mini-wizards

These are the biggest moments in the game. Each has a long intro, music synced to the lights, and a large finish.

| Mini-wizard | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| **Floodgates** | A fill rises from the flippers up the playfield with a moving surface; the arc ramp LEDs are the gauge. Super jackpot: a wave crashes over everything from the top down | Gates grinding open; rushing water. O: *"Open the gates."* S: *"Flood barriers… engaged. Engaged. Engaged."* | The gates open; a water-level gauge |
| **Blackout** | GI off; everything is dark except the roots and lit shots. Each hit kills a section. Super jackpot: one second of total darkness, then the roots explode with light from the spinner outward | A descending power-down whine and a clunk for each section; the Steward fragments. O: *"Darkness."* | The city grid, with sectors going black one by one |
| **Derailment** | The lock balls drop like falling railcars, a chase down the wireform for each ball. Jackpots at the drops shatter blue into green | Rail screech, metal tearing, crashes. S: *"Train… train… train."* O: *"Fall."* | Train cars tumbling off the elevated track |
| **Canopy** | A green noise field climbs the playfield and up into the backbox speaker LEDs. Collects at the ramps send growth up into the backbox | Giant trunks creaking, a deep roar of wind high above the city. O: *"Higher."* | The forest canopy spreads across the skyline |

### Side quests

| Quest | LEDs | Sound / callouts | DMD |
|---|---|---|---|
| **Charge** | Pop hits send packets along the rays to the spinner; spins pulse the roots; the spore bar fills with a flickering edge | A swarm rustle that thickens as the bar fills | Spore bar in the side quest row |
| **Spore burst ready** | The biospore insert heartbeats green and violet | O, whispered: *"Ready."* | — |
| **Spore burst fired** | A violet-green radial wave from the left inlane across the whole playfield | A swarm whoosh and a deep thump. S: *"Airborne contaminant detected."* | A spore cloud bursts across the screen |
| **Surveillance** | Watching pop targets glow a steady red with a slow, scanning pulse. Hit: a static glitch, then the LED goes dark. Set: all 3 flash white, and a packet flows from each pop down its ray into the roots | A camera whir and a lens crack; on a set, static, then S: *"Camera network… offline."* | A grid of camera feeds, with feeds cutting to static |
| **Sentinels** | The drop strips scan red like drone eyes. Down: the strip sparks and goes dark | A drone whine pitching down, then a crash. S: *"Drone three offline."* | A drone falls |
| **Lock lit** | The lift ramp hex's outer ring lights green; the ramp goes up | S: *"Restricted area. Do not use the Skyrail."* | — |
| **Ball locked** | A pulse up the lift ramp to the wireform; the playfield's ambient throb gets stronger with each lock | A deep earth rumble that gets heavier with each lock | Lock count, with a seismograph |
| **Heartwood** | The captive ball bolts heartbeat; each hit strengthens the beat and speeds it up | A sub heartbeat and wood knock | Heart charge in the side quest row |
| **Root tunnels** | Each hit grows one segment on the orbit hex's outer ring. Collect: growth runs from the orbit down to the current district on the city map | A dirt crunch per segment; a deep tearing on collect | The tunnel's path to the district |

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
| **Groundswell** start | An earthquake: noise-driven strobing across everything, then a plane sweep from the roots up into the backbox | S: *"Seismic event detected."* O: *"Rise."* | The ground splits under the city |
| **Groundswell** jackpot | A green radial wave from the shot | A heavy hit; S counts the damage | JACKPOT with cracks |
| **Groundswell** super jackpot | A plane sweep from the drops up into the backbox | The biggest impact in the game | Tower toppling |
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
   - A row of small icons, each with a few progress pips: Charge, Surveillance, Sentinels, Heartwood, Tunnels,
     Locks.
5. **Scoop eject summary**
   - A quick quest summary while a scoop holds the ball.

### Framework needs (frontbox-pin2dmd)

- A screen stack with priority, so overlays (status, menu) cover the score and return to it.
- In-game flipper input routed to the active DMD screen, separately from the flipper coils.
- A list/selection widget. The operator menu in `frontbox_pin2dmd::menu` may be a starting point.
- Timed page rotation.

## High scores

Custom categories, stored with `HighScoresSystem::set_custom_score`:

| Category | Measure |
|---|---|
| Spore Champion | Most spores applied in a game |
| Fastest Blackout | Shortest time to complete Blackout |
| Biggest Flood | Highest Floodgates total |
| Derailment Champion | Highest Derailment total |
| Canopy Champion | Highest Canopy total |
| Groundswell Champion | Highest Groundswell total |
| Heartwood Champion | Highest Heartwood multiball total |
| Sentinel Hunter | Most drones downed in a game |
| Tunnel Rat | Most root tunnels collected in a game |
| Overgrowth | Most districts completed (ties broken by score) |

Notes:
- "Fastest" scores are lower-is-better. The caller must compare before setting, and the DMD must show the value as a
  time.
- Each category is a single entry today. That's fine to start with.

## Code impact (lotko-homebrew)

- Replace the per-district qualifiers (`ExclusiveModeQualification<T>`) with a single scoop qualifier.
- `ModeManager` gains per-player map state: played / completed / saved progress for each mode.
- Add 4 mini-wizards to `ExclusiveMode`. Nimbus and Apex move from `NonExclusiveMode` to `ExclusiveMode`.
- `LeftScoopStartable` hosts the mode select menu. `LiftRampStartable` becomes the Groundswell lock.
- Spore Multiball becomes Heartwood. `SporesSystem` keeps counting spores applied and adds bursts.
- New side quest systems: Charge, Surveillance, Sentinels, Heartwood, Root tunnels.
- A combo tracker: listens for the shot hit events, tracks the timing window and the lift ramp state, and stores the
  routes made per player.
- Nimbus, Apex and Solarium are rewritten on the main shots. The pops, pop targets and vertical spinner no longer
  count toward any district.
- New `BallDraining` participants after `BallSaveSystem`: an outlane save and an extra ball. The framework's
  `BallDraining` docs already list both as examples, but neither exists yet. Decide whether they belong in
  frontbox-pinball or in lotko.
- Open: an extra ball that halts the drain would skip end-of-ball bonus. Decide whether an extra ball is a drain save
  or a repeat of the turn after bonus. Standard pinball counts bonus, then gives the player the next ball.

## Build Order

### Prereqs

- Virtual DMD for testing
- improved DMD DSL + Menu


#### Rules

1. Map progression with the single qualifier, played / completed state and resumable modes.
2. Groundswell lock and Sentinels.
3. Extra balls, outlane saves and the district start ball save.
4. Charge, Surveillance, Heartwood, Root tunnels, combos, and the status / idle DMD screens.
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
