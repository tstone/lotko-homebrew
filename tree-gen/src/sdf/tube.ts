import { RoundCone, type Shape, type Vec3 } from './shapes'

export interface TubePoint {
  position: Vec3
  radius: number
}

/**
 * A smooth tube through the given control points: a Catmull-Rom curve through the positions, with
 * radius interpolated linearly, built from short tapered capsules. Segments are kept shorter than
 * the local radius so the chain reads as one smooth surface.
 */
export function tube(points: TubePoint[]): Shape[] {
  if (points.length < 2) return []
  const positions = points.map((p) => p.position)
  const samples: TubePoint[] = [points[0]]
  for (let i = 0; i < points.length - 1; i++) {
    const p1 = points[i].position
    const p2 = points[i + 1].position
    const r1 = points[i].radius
    const r2 = points[i + 1].radius
    const span = Math.hypot(p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2])
    const steps = Math.min(16, Math.max(1, Math.ceil(span / (0.5 * Math.min(r1, r2)))))
    for (let s = 1; s <= steps; s++) {
      const t = s / steps
      samples.push({ position: curveAt(positions, i + t), radius: r1 + (r2 - r1) * t })
    }
  }

  const shapes: Shape[] = []
  for (let i = 0; i < samples.length - 1; i++) {
    shapes.push(
      new RoundCone(samples[i].position, samples[i + 1].position, samples[i].radius, samples[i + 1].radius),
    )
  }
  return shapes
}

/**
 * Point on the Catmull-Rom curve through `positions` that `tube` follows, at `u` from 0 at the first
 * position to `positions.length - 1` at the last. The ends are clamped.
 */
export function curveAt(positions: Vec3[], u: number): Vec3 {
  const last = positions.length - 1
  const clamped = Math.min(Math.max(u, 0), last)
  const i = Math.min(Math.floor(clamped), Math.max(last - 1, 0))
  const t = clamped - i
  const p0 = positions[Math.max(i - 1, 0)]
  const p1 = positions[i]
  const p2 = positions[Math.min(i + 1, last)]
  const p3 = positions[Math.min(i + 2, last)]
  const t2 = t * t
  const t3 = t2 * t
  const out: Vec3 = [0, 0, 0]
  for (let k = 0; k < 3; k++) {
    out[k] =
      0.5 *
      (2 * p1[k] +
        (p2[k] - p0[k]) * t +
        (2 * p0[k] - 5 * p1[k] + 4 * p2[k] - p3[k]) * t2 +
        (3 * p1[k] - p0[k] - 3 * p2[k] + p3[k]) * t3)
  }
  return out
}
