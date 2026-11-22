# Meridian Basins

**Status:** built as spreading growth, except the DMD animations and a wither sound. Scenes are first drafts to tune by
hand, and the wither time is a first guess to tune by playing.

## The place

The hydroponic farms where Cascadia Prime grows its food. Sterile crops grow in shallow water basins, without soil or seasons, tended entirely by the Steward. Nothing grows here that the city didn't plan.

It sits low by the river, it's wet, and it's made for growing. It's the easiest place in the city for the forest to take root, which is why it's one of the first places the player reaches.

## Signature

Growth spreading. The forest's seeds take root in the city's basins, and spread from one basin to the next.

## Gameplay

1. At the start, 2 of the 6 main shots are seeded at random. They flash yellow, the farm's grow lights.
2. Hit a flashing shot and it blooms: green energy bursts from it, and its hex goes dark, as it's done.
3. A bloom spreads to the shots on either side, which start flashing. The outer orbits only have one neighbor, so the
   middle shots spread further.
4. Bloom all 6 shots to complete the mode. That's always 6 hits, and there's always a flashing shot until then.
5. Going 15 seconds without a bloom has an even chance to wither the last one: it goes back to flashing, and the
   shots it spread to go dark unless another bloom or a seed still lights them. If it's spared, it gets another chance
   15 seconds later. This keeps on until there's a bloom, down to the two seeds.

Every bloom fills the arc ramp's lights by a sixth, from the left, and a wither takes a sixth back.

A flashing shot is always something to hit, and a dark one is either done or not reached yet.

Each hit of a Root Lash blooms a flashing shot at random. A shot made during a spore burst blooms it and another
flashing shot.

## Lighting

- **Basins:** a hex flashes yellow when it can be hit, and goes dark once it has bloomed. Both are shot statuses, not
  scenes.
- **Progress:** the arc ramp's lights fill from the left, a sixth per bloom.
- **Bloom** (`bloom.scene`): green energy exploding out from the shot.
- **Completion:** a scene of 7 to 10 bursts firing all over the playfield, in a wondrous display of green.

No intensity scenes as the mode goes on. The blooms and the arc ramp carry it.

## Sound

The farms have no obvious background sound, so there's no bed under the mode. The sound sits on the start and completion sequences, with a light touch on the hits.

**Effects:**

- Start: an air pressure release as the farm's systems react (`sfx/pressure-release-light`).
- Bloom: a short swell of growth, with the bloom scene.
- Wither: none yet (TODO.md, Sound effects needed).
- Completion: the blooms building into one swell, with the completion scene and the DMD animation.

**Callouts:** only at the start and the end.

| Moment | Voice | Line | Recorded |
|---|---|---|---|
| Start | Steward | "Crop contamination detected." | `meridian/crop-contamination-detected` |
| Completion | Steward | "Meridian Basins uninhabitable." | `meridian/meridian-basins-uninhabitable` |
| Completion | Forest | "Ours." | `oak/ours` |

## Display

The DMD shows the scores while the mode runs. It only plays an animation at the start and the completion, a few seconds each.

- **Start:** rows of plants fade in, the farm's neat crops in their basins. Plays with the pressure release and the Steward's line.
- **Completion:** roots wrap around those same rows of plants. Plays with the completion swell and the forest's line.

## To build

- A wither sound, and maybe a scene of the bloom draining away.
- DMD animations for the start (rows of plants fading in) and the completion (roots wrapping the plants), each with its sound.

## Open questions

- Whether 15 seconds to wither is too harsh or too soft.
