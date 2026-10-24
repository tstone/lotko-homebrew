import type { Part } from '../../sdf/scene'
import { tube, type TubePoint } from '../../sdf/tube'
import { lerp, smoothstep } from '../../util/math'
import type { TrunkFrame } from './frame'

/**
 * The central column of the trunk. Where root strands run up the outside, the core only reaches
 * their middles, so it fills the trunk without filling the grooves between them. Above the
 * strands it grows to the full radius. It runs a little past the top so the cut slices through it.
 * At the bottom it starts high enough that its rounded end only dips as low as the undersides of
 * the roots, closing the underside of the trunk with a shallow dome.
 */
export function core(frame: TrunkFrame): Part {
  const { params, height, baseRadius } = frame
  const ridgeTop = params.rootCount > 0 ? params.ridgeHeight * height : 0
  const step = Math.max(baseRadius * 0.4, height / 40)
  const bottom = frame.strandOffsetAt(0) - frame.strandRadiusAt(0)
  const top = height + baseRadius * 0.5

  const points: TubePoint[] = []
  for (let y = bottom; ; y += step) {
    const clamped = Math.min(y, top)
    const uncovered = ridgeTop > 0 ? smoothstep(ridgeTop * 0.85, ridgeTop, clamped) : 1
    points.push({
      position: frame.centerAt(clamped),
      radius: lerp(frame.strandOffsetAt(clamped), frame.radiusAt(clamped), uncovered),
    })
    if (clamped === top) break
  }

  return { shapes: tube(points), blend: 0 }
}
