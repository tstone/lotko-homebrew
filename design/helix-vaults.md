# Helix Vaults

**Status:** place and gameplay set, lighting being worked through. Tier 2.

## The place

The city's store of living raw material for genetic enhancement. Cascadia Prime's residents aren't augmented with machines, they're edited: their bodies are improved with engineered tissue grown here. Huge glass cylinders hold it, full of bubbling neon yellow-green growth medium, behind containment seals. The Steward keeps it all sterile and controlled.

Words to sell it: biomatter, splice stock, somatic culture, growth medium, nutrient gel.

## Signature

The vats: huge cylinders of bubbling neon goo. The forest drains them.

## The mystery

This is where the mystery starts to come out, so it sits in tier 2. The biomatter is grown from Kilmore cells. The city that cleared the forest has been putting the forest into its residents all along, and that's why the forest can take it back.

The place should hint at this without saying it outright. For example, the Steward refers to the vats' contents by a specimen line beginning with "K".

## Gameplay

A 2-ball multiball, reworked from Floodgates. The vat starts full, and the goal is to empty it.

1. The vat is full. The orbits are lit yellow-green.
2. Each orbit hit depletes matter from the vat.
3. After N orbit hits, the containment gives: a ramp lights lime for a jackpot.
4. Hit it for a rupture, worth big points. Goo splatters over the playfield and the DMD.
5. The orbits take over again for the next rupture.
6. The third rupture empties the vat and completes it. Each rupture is worth more than the last.
7. It ends when one ball is left.

Root Lash on an orbit counts as orbit hits.

## Lighting

The mode color is yellow-green.

- **The vat:** the subway's LED strip (11 LEDs) is the vat. It starts full of yellow-green and empties as goo is discharged. The arc ramp feeds the subway, so a ball on the arc ramp passes right through the vat.
- **Discharge:** on an orbit hit, a blob of goo travels from the vat to the shot along the LED graph (the graph mask's pathfinding), and the vat drops a step.
- **Orbits:** lit yellow-green, with a bubbling effect.
- **Ramps:** a ramp lights lime when a rupture is ready, so it reads as different from the orbits.
- **Rupture:** a splatter scene. Yellow-green dots splatter across the playfield on impact, then slowly fade. It uses the 2D lighting, and the same splatter plays on the DMD.
- **Completion:** the splatters land everywhere and keep coming until they cover the playfield.

**To evaluate:** the bubbling on the lane lights. Check whether an existing curve or `Prop::Random` can make it, or whether the scene system needs something new.

## Sound

**Music:** "Syntax" by Substain.

The vats have an obvious sound, so there's a bed under the mode: bubbling.

- Rupture: a goopy splat, with the splatter.
- Completion: a glug, glug of thick liquid draining away.

**Callouts:** start and end, plus each rupture, since it's a multiball.

| Moment | Voice | Line | Recorded |
|---|---|---|---|
| Start | Steward | "Containment nominal." | No |
| Rupture ready | Oak | "Shoot the ramps." | `shoot-the-ramps` |
| Rupture | Steward | "Containment breach. Specimen line K…" (cuts off) | No |
| Completion | Steward | To decide: a line that says too much | No |
| Completion | Oak | "Open the gates." or a new line | `oak/open-the-gates` |

## Display

- **Start:** rows of glowing cylinders, bubbling.
- **Rupture:** the splatter from the playfield, landing over the DMD and slowly fading.
- **Completion:** the vats drain empty, and what's left is the forest's green.

## Open questions

- N, the orbit hits per rupture. Floodgates used 8.
- Which ramp lights for a rupture: a set one, a random one, or each ramp once.
- Lime is Heartwood's color. Either the ramps need another shade, or Heartwood shares it.
- The completion lines, and how much they reveal.
- Whether it stays a multiball.
