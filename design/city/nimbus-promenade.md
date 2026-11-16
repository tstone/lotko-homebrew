# Nimbus Promenade

**Status:** the gameplay is good. Rework it around the Steward's scan.

## The place

The city's town center and main gathering place: an open-air plaza and concourse of pavilions and shops, strung with lanterns and full of wind chimes. Skyrail Station is next to it, and it's the way from the lower city to the upper city. It was built to hold crowds and to hold back the forest, with crowd barriers, climate scrubbers and structural shielding.

## Signature

The checkpoint. The Steward shuts the walkways, and the forest forces them open.

## Gameplay

**Today:** every shot is an open walkway. Hit one to weaken it, then again to close it. The Steward blocks walkways for a few seconds, flashing them red first. Close 3. This has the player closing walkways, which is what the Steward does in the story.

**Direction:**

- At the start, the Steward closes every walkway: all the shots turn red.
- Hit a red walkway once and it turns pink: forced.
- Hit it again and it goes dark: open.
- The Steward's scan sweeps the playfield (see Lighting). When it stops on a walkway, it shuts it again, back to red.
- Open every walkway within a time limit, or something to that effect.

Optional: something on the playfield represents the Steward, and hitting it stops the scan for a few seconds. The pop bumpers are already the Steward's cameras, which would fit, but that clashes with Surveillance (the cameras come online after 40 switch hits and have their own rules). Either the pops always count during Nimbus, or Surveillance waits until the mode ends.

## Lighting

The scan carries the mode.

- **The scan:** a red rod drawn across the LEDs, anchored at the drain, sweeping the playfield in an arc. Every so often it stops on a shot and shuts that walkway, then carries on.
- **Walkways:** red for shut, pink for forced, dark for open.
- The rod and the red walkways need to stay easy to tell apart. That may mean tuning the rod's brightness or giving it a soft edge.
- **Completion:** with every walkway open, the scan stops at the drain and fades out, and the lights spread up toward the upper city's map inserts.

## Sound

- A low scanning tone that follows the rod, with a heavy latch when it shuts a walkway.
- Splintering and tearing when a walkway is forced and opened.
- Wind chimes under the mode, going out of tune as the walkways open.
- Recorded: `promenade-closed`, `walkways-will-be-sealed`, `restricted-area`, `walkway-4-sealed`, `access-to-upper-city-restricted`, `oak/the-way-is-open`.

## Display

- Start: the shutters dropping across the walkways.
- Complete: the shutters torn open and the way up to the upper city.
- Idea: if the lights alone don't tell the player enough, the DMD can show a walkway being forced or shut again, and which walkways are open.

## Open questions

- The goal: open every walkway, or a set number, and whether there's a time limit.
- Whether a Steward target stops the scan, and which one.
- How fast the scan moves, and how often it stops.
