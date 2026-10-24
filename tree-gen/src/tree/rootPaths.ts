import { curveAt } from '../sdf/tube'
import type { Vec3 } from '../sdf/shapes'
import { lerp, smoothstep } from '../util/math'
import { createNamedRandom, type Random } from '../util/random'
import { createTrunkFrame } from './features/frame'
import type { TreeParams } from './params'

/** A control point on a root's path along the ground, in meters. y is the height of the root's middle. */
export interface RootPoint {
  x: number
  y: number
  z: number
}

export interface RootPath {
  /** Thickness relative to the other roots. */
  scale: number
  /** Control points from next to the trunk out to the tip, evenly spaced along the root. */
  points: RootPoint[]
}

// Shape of newly generated roots, in multiples of the base radius.
const SPREAD = 1.6
const MEANDER = 0.45
// From resting on the ground (0) to mostly buried (1).
const BURIAL = 0.4

/** Direction from the trunk the root leaves in. */
export function rootAngle(root: RootPath): number {
  const first = root.points[0]
  return Math.atan2(first.z, first.x)
}

/** Root radius at control point `index`: nearly full thickness at first, tapering off to the tip. */
export function rootRadiusAt(strandRadius: number, root: RootPath, index: number): number {
  return strandRadius * root.scale * rootTaper((index + 1) / root.points.length)
}

/** Fraction of full thickness at `u` along a root, from 0 at the trunk to 1 at the tip. */
function rootTaper(u: number): number {
  return lerp(1, 0.3, u * u)
}

/**
 * The roots fitted to the root count and points per root. Existing roots are kept, with their paths
 * resampled if the point count changed. New roots are generated into the widest gaps between the
 * existing ones; with none to start from they are spread evenly around the trunk.
 */
export function fitRoots(params: TreeParams): RootPath[] {
  const count = Math.max(0, Math.round(params.rootCount))
  const pointCount = Math.max(2, Math.round(params.rootPoints))
  const kept = params.roots
    .slice(0, count)
    .map((root) => (root.points.length === pointCount ? root : resampleRoot(params, root, pointCount)))
  if (kept.length === count) return kept

  const roots = [...kept]
  const rand = createNamedRandom(params.seed, `roots${kept.length}`)
  const offset = rand.range(0, Math.PI * 2)
  for (let i = kept.length; i < count; i++) {
    const angle =
      kept.length === 0
        ? offset + ((i + rand.range(-0.2, 0.2)) / count) * Math.PI * 2
        : widestGap(roots.map(rootAngle))
    roots.push(generateRoot(params, angle, pointCount, rand))
  }
  return roots
}

function generateRoot(params: TreeParams, angle: number, pointCount: number, rand: Random): RootPath {
  const frame = createTrunkFrame(params)
  const R = frame.baseRadius
  const scale = rand.range(0.85, 1.15)
  const radius = frame.strandRadiusAt(0) * scale
  const start = frame.strandOffsetAt(0) + radius
  const reach = start + R * SPREAD * rand.range(0.7, 1.2)
  const meanderFreq = rand.range(2.5, 5)
  const meanderPhase = rand.range(0, Math.PI * 2)
  const humpPhase = rand.range(0, Math.PI * 2)
  const sideways = [-Math.sin(angle), Math.cos(angle)]

  const points = Array.from({ length: pointCount }, (_, i): RootPoint => {
    const u = (i + 1) / pointCount
    const r = radius * rootTaper(u)
    const distance = lerp(start, reach, u)
    const lateral = MEANDER * R * u * Math.sin(u * meanderFreq + meanderPhase)
    // The root sinks gradually along its length so its top just meets the ground at the tip, with a
    // gentle hump or two on the way.
    const sink = smoothstep(0.3, 1, u) * (1.6 - BURIAL)
    return {
      x: Math.cos(angle) * distance + sideways[0] * lateral,
      y: r * (0.5 - BURIAL - sink) + r * 0.2 * Math.sin(u * 11 + humpPhase) * (1 - u),
      z: Math.sin(angle) * distance + sideways[1] * lateral,
    }
  })
  return { scale, points }
}

/** The same root path with a different number of evenly spaced control points. */
function resampleRoot(params: TreeParams, root: RootPath, pointCount: number): RootPath {
  // Anchor the curve where the root meets the trunk, so new points can be placed inside the first.
  const frame = createTrunkFrame(params)
  const angle = rootAngle(root)
  const start = frame.strandOffsetAt(0) + frame.strandRadiusAt(0) * root.scale
  const first = root.points[0]
  const anchor: Vec3 = [Math.cos(angle) * start, first.y, Math.sin(angle) * start]
  const positions: Vec3[] = [anchor, ...root.points.map((p): Vec3 => [p.x, p.y, p.z])]

  const points = Array.from({ length: pointCount }, (_, i): RootPoint => {
    const [x, y, z] = curveAt(positions, ((i + 1) / pointCount) * root.points.length)
    return { x, y, z }
  })
  return { scale: root.scale, points }
}

/** The angle midway across the widest gap between the given angles. */
function widestGap(angles: number[]): number {
  const sorted = angles.map((a) => ((a % (Math.PI * 2)) + Math.PI * 2) % (Math.PI * 2)).sort((a, b) => a - b)
  let best = sorted[0] + Math.PI
  let widest = 0
  sorted.forEach((a, i) => {
    const next = i + 1 < sorted.length ? sorted[i + 1] : sorted[0] + Math.PI * 2
    if (next - a > widest) {
      widest = next - a
      best = a + widest / 2
    }
  })
  return best
}
