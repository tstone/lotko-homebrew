import type { Part } from '../../sdf/scene'
import { tube, type TubePoint } from '../../sdf/tube'
import { lerp, smoothstep } from '../../util/math'
import { createNamedRandom } from '../../util/random'
import { rootAngle, rootRadiusAt, type RootPath } from '../rootPaths'
import type { TrunkFrame } from './frame'

/**
 * Roots of a gnarled oak. Each root is one continuous strand of nearly constant thickness: it
 * follows its path along the ground in toward the trunk, bends up through an elbow at the base and
 * spirals up the trunk as a ridge. Strands are packed so neighbours fuse with a groove between
 * them. Keeping the thickness steady through the elbow is what avoids a swollen base.
 */
export function roots(frame: TrunkFrame): Part[] {
  const thickness = frame.strandRadiusAt(0)
  // Enough blend that strands read as one trunk, little enough to keep the grooves between them.
  const blend = thickness * 0.25
  return frame.params.roots.map((root, i) => ({ shapes: tube(rootTubePoints(frame, root, i)), blend }))
}

/** The ground path of a root from its tip in to the elbow, where it turns up into the trunk. */
export function rootGroundPoints(frame: TrunkFrame, root: RootPath): TubePoint[] {
  const thickness = frame.strandRadiusAt(0)
  const radius = thickness * root.scale
  const points: TubePoint[] = root.points
    .map((p, i) => ({ position: [p.x, p.y, p.z] as TubePoint['position'], radius: rootRadiusAt(thickness, root, i) }))
    .reverse()
  points.push({ position: frame.around(radius * 0.9, rootAngle(root), frame.strandOffsetAt(0) + radius * 0.3), radius })
  return points
}

function rootTubePoints(frame: TrunkFrame, root: RootPath, index: number): TubePoint[] {
  const { params, height, baseRadius: R } = frame
  const rand = createNamedRandom(params.seed, `ridge${index}`)
  const twist = params.twist * Math.PI * 2
  const angle = rootAngle(root)
  const radius = frame.strandRadiusAt(0) * root.scale
  const ridgeTop = params.ridgeHeight * height * rand.range(0.85, 1.1)
  const wobblePhase = rand.range(0, Math.PI * 2)

  const points = rootGroundPoints(frame, root)

  // Up the trunk as a ridge, spiraling with the twist and sinking into the core toward ridgeTop.
  const climbStart = radius * 2
  if (ridgeTop > climbStart) {
    const steps = Math.max(3, Math.ceil((ridgeTop - climbStart) / Math.max(R * 0.4, height / 30)))
    for (let s = 0; s <= steps; s++) {
      const y = lerp(climbStart, ridgeTop, s / steps)
      const t = y / height
      const fade = smoothstep(0.85, 1, y / ridgeTop)
      const a = angle + twist * t + 0.1 * Math.sin(t * 7 + wobblePhase)
      const r = frame.strandRadiusAt(y) * root.scale * (1 - 0.6 * fade)
      const out = frame.strandOffsetAt(y) * (1 - 0.3 * fade)
      points.push({ position: frame.around(y, a, out), radius: r })
    }
  }
  return points
}
