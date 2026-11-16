# Design

One file per place in Cascadia Prime. Each says what the place is, how its mode plays, and how the lights, sound and display sell it. The story behind the city is in [PLACES.md](../PLACES.md).

Work through a place in this order:

1. **Purpose.** What the place is, and the one thing it's known for.
2. **Gameplay.** The rules, built around that one thing.
3. **Lighting.** What the playfield shows while the mode runs.
4. **Sound and display.** A few notes are enough until the first three are settled.

Where these files disagree with `PLAN.md` or `DESIGN.md`, these files win. Those were written before the scene system.

## How the game tells its story

1. **Lighting drives the game.** A player should be able to tell what's happening from the playfield alone: what to shoot, what the Steward is doing, and how close the forest is.
2. **Sound supports it.** Effects for every hit and change, and callouts that give the story. The Steward announces, and the forest answers in a few words.
3. **The display is the finishing touch.** It can't carry a mode, but it rewards and explains.

"Lighting" means the playfield LEDs, and "display" means the DMD, which can show color. Lighting has two parts:

- **Inserts.** The shots, map and other inserts. Together they can play 2D area effects across the playfield.
- **GI.** The general illumination. Most places leave it on as normal so players can see. A few make special use of it.

## Callouts

- A place has a callout at the start and at the end, and that's enough.
- No callouts for progress, such as "X of Y". They interrupt the music and the feel of the game.
- Peak moments and multiball modes are the exception, and can have more.

## Sound effects

- The start and end of a place each have a sound, tied to the display animation.
- A place with an obvious background sound can have it under the mode. One without doesn't need it.
- Light effects here and there on hits are fine, but not a sound for every little thing.

## Shared display moments

The DMD shows the scores while a place runs. Animations are brief, a few seconds each.

- **The Steward speaks.** Whenever the Steward has a callout, the Steward jumps in from the left, like a character, on a layer over whatever the DMD is showing. Its central sphere pulses red while the line plays, then it leaves.
- **Place start.** A short animation that introduces the place, with its sound.
- **Place complete.** A short animation of the forest taking the place, with its sound and the forest's line.

## Lit shots

A lit shot always means "hit me". Design every place so that what the player should hit is lit, and nothing else is.

Shot statuses are a static color or flashing. Anything more than that is a scene.

## Places

| Place | Status | File |
|---|---|---|
| Meridian Basins | Designed, ready to build | [meridian-basins.md](meridian-basins.md) |
| Hydro Core | Keep, move out of tier 1 | [hydro-core.md](hydro-core.md) |
| Nimbus Promenade | Rework with the scan | [nimbus-promenade.md](nimbus-promenade.md) |
| Skyrail Station | Becomes the roving shot | [skyrail-station.md](skyrail-station.md) |
| Apex Terraces | Revisit | [apex-terraces.md](apex-terraces.md) |
| Solarium Atrium | Needs the mystery | [solarium-atrium.md](solarium-atrium.md) |
| Spire Core | New, holding Apex's old rules | [spire-core.md](spire-core.md) |
| Caldera Generator (Blackout) | Place set, geothermal power | [caldera-generator.md](caldera-generator.md) |
| Helix Vaults (Floodgates) | Place set, tier 2 | [helix-vaults.md](helix-vaults.md) |
| Cargo Quay (Derailment) | Proposed | [cargo-quay.md](cargo-quay.md) |
| Halcyon Vents (Canopy) | Sketched | [halcyon-vents.md](halcyon-vents.md) |

## The map

Every mode becomes a place on the city map. Which places run as multiball (the old mini-wizards) is decided later.

The current route is fixed in pairs: Meridian or Hydro, Nimbus, a mini-wizard, Skyrail or Apex, Solarium, a mini-wizard. With more places, the map could open up so the player picks a route instead of going from point A to point B. One shape for that:

- Places are connected, and any place next to one already taken can be started.
- Height still sets difficulty: easy at the river, hard at the peak.
- Nimbus Promenade stays the way from the lower city to the upper city.

The playfield is a whitewood with temporary art, so the next version can have an insert per place.

## Changes to other features

- **Sentinels:** use the pop bumpers as the targets instead of the drop targets. Pops are more dangerous, which fits drones, and it leaves the drops for places such as Cargo Quay. The pops are also the cameras in Surveillance, so the two need to stay distinct.

## Open questions

- How many places, and how they connect.
- Which places run as multiball.
- What the end of the climb is: Solarium, Spire Core, or something past both.
