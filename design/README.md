# Design

How Last of the Kilmore Oaks plays, and how the lights, sound and display tell its story. The story behind the city is in [PLACES.md](../PLACES.md).

Where these files disagree with `PLAN.md` or `DESIGN.md`, these files win. Those were written before the scene system.

| Section | What it covers |
|---|---|
| [city/](city/README.md) | The main progression: the forest climbs the city one place at a time |
| [roots/](roots/README.md) | The forest's second progression: spreading its roots |
| [steward/](steward/README.md) | The negative progression: the Steward watching, then sending the Sentinels |
| [shots/](shots/README.md) | Skill shots that run alongside everything: combos and reflex shots |
| [mystery.md](mystery.md) | The secrets the climb reveals, and the clues along the way |

Not yet placed in a section: Charge (spores and the spore burst), Heartwood, Groundswell, outlane saves, skill shots at the plunge, and extra balls.

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
