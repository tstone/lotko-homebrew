/** A control point on the trunk's skeleton. */
export interface SkeletonPoint {
  /** Height as a fraction of the trunk height, from 0 at the ground to 1 at the top. */
  t: number
  /** Horizontal offset of the centerline from the base, in meters. */
  x: number
  z: number
}

/**
 * Smallest vertical gap between neighbouring control points, as a fraction of the trunk height.
 * Keeping points in order up the trunk keeps the centerline a function of height.
 */
export const MIN_SKELETON_GAP = 0.03

/**
 * Horizontal offset [x, z] of the trunk's centerline at t, from 0 at the ground to 1 at the top.
 * The skeleton is a fixed base at the origin plus control points in order up the trunk, joined by
 * a Catmull-Rom style curve that allows uneven spacing. The slope is zero at the base, so the trunk
 * leaves the ground vertically, and above the last point the curve carries on in a straight line.
 */
export function skeletonOffsetAt(points: SkeletonPoint[], t: number): [number, number] {
  const n = points.length
  if (n === 0 || t <= 0) return [0, 0]
  const knots: SkeletonPoint[] = [{ t: 0, x: 0, z: 0 }, ...points]

  // Rate of change of the offset per unit t at each knot: zero at the base, one-sided at the last
  // point and centred in between.
  const slope = (i: number): [number, number] => {
    if (i === 0) return [0, 0]
    const a = knots[i - 1]
    const b = knots[Math.min(i + 1, n)]
    const dt = b.t - a.t
    return [(b.x - a.x) / dt, (b.z - a.z) / dt]
  }

  const last = knots[n]
  if (t >= last.t) {
    const [sx, sz] = slope(n)
    return [last.x + sx * (t - last.t), last.z + sz * (t - last.t)]
  }

  let i = 0
  while (knots[i + 1].t <= t) i++
  const a = knots[i]
  const b = knots[i + 1]
  const span = b.t - a.t
  const s = (t - a.t) / span
  const [ax, az] = slope(i)
  const [bx, bz] = slope(i + 1)
  return [hermite(a.x, ax * span, b.x, bx * span, s), hermite(a.z, az * span, b.z, bz * span, s)]
}

/** Control points evenly spaced up the same curve, so changing the count keeps its shape. */
export function resampleSkeleton(points: SkeletonPoint[], count: number): SkeletonPoint[] {
  return Array.from({ length: count }, (_, i) => {
    const t = (i + 1) / count
    const [x, z] = skeletonOffsetAt(points, t)
    return { t, x, z }
  })
}

/** The range of heights control point `index` can move through without passing its neighbours. */
export function skeletonPointRange(points: SkeletonPoint[], index: number): [number, number] {
  const below = index > 0 ? points[index - 1].t : 0
  const above = index < points.length - 1 ? points[index + 1].t - MIN_SKELETON_GAP : 1
  return [below + MIN_SKELETON_GAP, above]
}

function hermite(p0: number, m0: number, p1: number, m1: number, s: number) {
  const s2 = s * s
  const s3 = s2 * s
  return (2 * s3 - 3 * s2 + 1) * p0 + (s3 - 2 * s2 + s) * m0 + (-2 * s3 + 3 * s2) * p1 + (s3 - s2) * m1
}
