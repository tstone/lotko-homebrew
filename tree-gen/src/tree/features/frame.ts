import type { Vec3 } from '../../sdf/shapes'
import type { TreeParams } from '../params'
import { skeletonOffsetAt } from '../skeleton'

/** The trunk's overall shape that every feature positions itself against. */
export interface TrunkFrame {
  params: TreeParams
  height: number
  baseRadius: number
  /** Trunk radius at height y including the base flare, before roots, burls or noise. */
  radiusAt(y: number): number
  /**
   * Radius of each root strand where it runs up the trunk at height y. Strands are packed around
   * the trunk so neighbours just fuse, leaving a groove between each pair. Zero with no roots.
   */
  strandRadiusAt(y: number): number
  /** Distance from the centerline to the middle of each root strand at height y. */
  strandOffsetAt(y: number): number
  /** Point on the trunk's centerline at height y. */
  centerAt(y: number): Vec3
  /** Point at height y, `distance` out from the centerline in direction `angle`. */
  around(y: number, angle: number, distance: number): Vec3
}

export function createTrunkFrame(params: TreeParams): TrunkFrame {
  const height = params.height
  const baseRadius = params.baseDiameter / 2

  const taperedRadiusAt = (y: number) => {
    const t = Math.min(Math.max(y / height, 0), 1)
    return baseRadius * (1 - (1 - params.taper) * t)
  }
  const radiusAt = (y: number) =>
    taperedRadiusAt(y) * (1 + params.baseFlare * Math.exp(-Math.max(y, 0) / (baseRadius * 1.2)))

  // With n strands around a circle of radius c, neighbours are 2c·sin(π/n) apart. Strands 1.2× that
  // chord across overlap enough to fuse but leave a groove about a fifth of the radius deep.
  // Strand thickness follows the tapered radius but not the flare, so the flare spreads the
  // strands apart at the base instead of fattening them.
  const packing = params.rootCount > 0 ? 1.2 * Math.sin(Math.PI / Math.max(params.rootCount, 3)) : 0
  const strandRadiusAt = (y: number) =>
    ((taperedRadiusAt(y) * packing) / (1 + packing)) * params.rootThickness
  const strandOffsetAt = (y: number) => radiusAt(y) - strandRadiusAt(y)

  const centerAt = (y: number): Vec3 => {
    const [x, z] = skeletonOffsetAt(params.skeleton, Math.min(y / height, 1.2))
    return [x, y, z]
  }

  const around = (y: number, angle: number, distance: number): Vec3 => {
    const c = centerAt(y)
    return [c[0] + Math.cos(angle) * distance, y, c[2] + Math.sin(angle) * distance]
  }

  return { params, height, baseRadius, radiusAt, strandRadiusAt, strandOffsetAt, centerAt, around }
}

/** Unit vectors pointing outward, up and sideways at an angle around the trunk. */
export function localAxes(angle: number): [Vec3, Vec3, Vec3] {
  return [
    [Math.cos(angle), 0, Math.sin(angle)],
    [0, 1, 0],
    [-Math.sin(angle), 0, Math.cos(angle)],
  ]
}
