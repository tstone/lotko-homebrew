# Meridian Basins

**Status:** built, except the DMD animations. Scenes are first drafts to tune by hand.

## The place

The hydroponic farms where Cascadia Prime grows its food. Sterile crops grow in shallow water basins, without soil or seasons, tended entirely by the Steward. Nothing grows here that the city didn't plan.

It sits low by the river, it's wet, and it's made for growing. It's the easiest place in the city for the forest to take root, which is why it's one of the first places the player reaches.

## Signature

Planting seeds. The forest plants its own seeds in the city's basins, and they burst up through the sterile crops.

## Gameplay

1. At the start, 4 of the 6 main shots light up green, chosen at random. Each is a basin.
2. Hit a lit basin once to plant a seed. It starts to flash.
3. Hit it again and the seed bursts: the area around that shot explodes with green energy, and the shot goes out.
4. Burst 3 basins to complete the mode. Basins can be worked in any order, so the player can have more than one flashing at a time.

Every hit fills the arc ramp's lights by a sixth, from the left. 3 basins of 2 hits fill it completely, and the mode completes.

A lit shot is always something to hit: green for a basin to plant, flashing for one to burst.

## Lighting

- **Basins:** a basin's hex is solid green when lit, and flashes green once planted. Both are shot statuses, not scenes.
- **Progress:** the arc ramp's lights fill from the left, a sixth per hit.
- **Burst:** a scene of green energy exploding out from the shot.
- **Completion:** a scene of 7 to 10 bursts firing all over the playfield, in a wondrous display of green.

No intensity scenes as the mode goes on. The bursts and the arc ramp carry it.

## Sound

The farms have no obvious background sound, so there's no bed under the mode. The sound sits on the start and completion sequences, with a light touch on the hits.

**Effects:**

- Start: an air pressure release as the farm's systems react (`sfx/pressure-release-light`).
- Burst: a short swell of growth, with the burst scene.
- Completion: the bursts building into one swell, with the completion scene and the DMD animation.

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

- New code for the mode: 4 random basins, 2 hits each, 3 to complete, with progress on the arc ramp.
- Lit basins green, as a shot status.
- Planted basins flashing, as a shot status. A status should be either a static color or flashing, and anything more is a scene. If the status API can't flash yet, add it.
- A scene for a basin bursting.
- A scene for the completion.
- Drop the old mode's intensity scene and its progress callouts.
- DMD animations for the start (rows of plants fading in) and the completion (roots wrapping the plants), each with its sound.

## Open questions

- Whether its multiball companion (today Floodgates) still fits once it's no longer about water channels.
