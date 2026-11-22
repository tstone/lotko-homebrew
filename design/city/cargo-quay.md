# Cargo Quay

**Status:** built as pushing containers to the crane, except the sounds, its own callouts and the DMD. Scenes are first
drafts to tune by hand. A tier 1 district, with Meridian Basins.

## The place

The city's river freight terminal, where everything Cascadia Prime doesn't make for itself comes in. Autonomous barges move along the river, cargo drones fly overhead, gantry cranes lift containers onto the quay, and lift locks raise vessels between the river and the city. The Steward routes every container.

## Signature

The gantry crane. The forest pushes the city's own containers onto it.

## Gameplay

The lift ramp is the crane. It stays down until both containers are in, as its coil buzzes while holding it up.
The other five main shots are the quay, in a row toward the crane: left orbit, dome ramp, arc ramp, center orbit, and the right orbit on the far side.

1. Two containers start on main shots, in one of three layouts. Each takes 5 pushes to get both to the crane:
   - left orbit and center orbit
   - left orbit and right orbit
   - dome ramp and arc ramp
2. Hit a container's shot to push it one shot toward the crane. Once a mode, a hit has a 1 in 3 chance to push it two
   shots instead. It's only rolled 2 or more shots from the crane, so it isn't spent on the last step.
3. A container pushed onto the other one's shot stacks on it, and from then on one hit pushes both.
4. Once both are in, the ramp goes up and the crane flashes. Shoot the lift ramp to lift them and complete the mode.
5. Each hit of a Root Lash pushes a container at random, or makes the crane shot once both are in. A shot during a
   spore burst pushes twice.

Single ball, no timer, and nothing pushes back. It's a tier 1 mode, so it's kept straightforward.

## Lighting

All in `assets/leds/cargo_quay/`.

- **Containers:** a container lights its hex center orange, and a stack lights the whole hex. Shot statuses, not
  scenes.
- **Push** (`push.scene`): the container slides from one hex to the next with a short tail. The city map shows the
  pushes so far.
- **Crane ready:** the lift ramp goes up, and its hex center and the bolt under it flash orange.
- **Completion** (`complete.scene`): a white flash and an orange shockwave from the crane, then green spreads over the
  playfield from it.

## Sound

The quay has an obvious background sound: cranes, horns, and water against the hulls. Stand-ins are in place:
`latch-heavy1` for a push, `motor1` when the crane is ready. Still needed (see TODO.md, Sound effects needed):

- A heavy metal clang for each push, and a crash when a stack forms.
- The crane's whine when it's ready, and as it lifts.
- The quay's ambience under the mode.

**Callouts:** the start says `generic/biological-contaminent-identified` as a stand-in, and the completion only the
Oaks' `oak/fall`. Cargo Quay needs its own Steward lines for both.

## Display

- **Start:** the gantry cranes over the stacks of containers.
- **Completion:** a stack falling.

## Open questions

- Whether 5 pushes plus the crane is the right length for tier 1.
- Its insert on the city map, which has six for seven districts. Its perk is pops charging spores twice as fast,
  taken from Nimbus Promenade.
