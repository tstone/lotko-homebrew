export type Vec3 = [number, number, number]

export interface Bounds {
  min: Vec3
  max: Vec3
}

/**
 * A solid described by its signed distance: negative inside, positive outside. `distance` should
 * be (close to) a true Euclidean distance, since the mesher relies on that to skip empty space.
 */
export interface Shape {
  distance(x: number, y: number, z: number): number
  bounds(): Bounds
}

/** Polynomial smooth minimum: a union that fillets the seam over a band roughly `k` wide. */
export function smoothMin(a: number, b: number, k: number): number {
  if (k <= 0) return Math.min(a, b)
  const h = Math.max(k - Math.abs(a - b), 0) / k
  return Math.min(a, b) - h * h * k * 0.25
}

export function smoothMax(a: number, b: number, k: number): number {
  return -smoothMin(-a, -b, k)
}

/** A capsule whose radius changes linearly from one end to the other (iq's "round cone"). */
export class RoundCone implements Shape {
  private readonly ba: Vec3
  private readonly l2: number
  private readonly rr: number
  private readonly a2: number
  private readonly il2: number

  constructor(
    private readonly a: Vec3,
    private readonly b: Vec3,
    private readonly r1: number,
    private readonly r2: number,
  ) {
    this.ba = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
    this.l2 = Math.max(dot(this.ba, this.ba), 1e-12)
    this.rr = r1 - r2
    this.a2 = this.l2 - this.rr * this.rr
    this.il2 = 1 / this.l2
  }

  distance(x: number, y: number, z: number): number {
    const { a, ba, l2, rr, a2, il2, r1, r2 } = this
    // One end sphere swallows the other: the shape is just that sphere.
    if (a2 <= 0) {
      const big = r1 >= r2 ? a : this.b
      return Math.hypot(x - big[0], y - big[1], z - big[2]) - Math.max(r1, r2)
    }
    const pax = x - a[0]
    const pay = y - a[1]
    const paz = z - a[2]
    const py = pax * ba[0] + pay * ba[1] + paz * ba[2]
    const pz = py - l2
    const qx = pax * l2 - ba[0] * py
    const qy = pay * l2 - ba[1] * py
    const qz = paz * l2 - ba[2] * py
    const x2 = qx * qx + qy * qy + qz * qz
    const y2 = py * py * l2
    const z2 = pz * pz * l2
    const k = Math.sign(rr) * rr * rr * x2
    if (Math.sign(pz) * a2 * z2 > k) return Math.sqrt(x2 + z2) * il2 - r2
    if (Math.sign(py) * a2 * y2 < k) return Math.sqrt(x2 + y2) * il2 - r1
    return (Math.sqrt(x2 * a2 * il2) + py * rr) * il2 - r1
  }

  bounds(): Bounds {
    const { a, b, r1, r2 } = this
    return {
      min: [Math.min(a[0] - r1, b[0] - r2), Math.min(a[1] - r1, b[1] - r2), Math.min(a[2] - r1, b[2] - r2)],
      max: [Math.max(a[0] + r1, b[0] + r2), Math.max(a[1] + r1, b[1] + r2), Math.max(a[2] + r1, b[2] + r2)],
    }
  }
}

/**
 * An ellipsoid with arbitrary orientation. `axes` must be orthonormal; `radii` are the semi-axis
 * lengths along each. Uses iq's distance approximation, which is close but not exact.
 */
export class Ellipsoid implements Shape {
  constructor(
    private readonly center: Vec3,
    private readonly axes: [Vec3, Vec3, Vec3],
    private readonly radii: Vec3,
  ) {}

  distance(x: number, y: number, z: number): number {
    const p: Vec3 = [x - this.center[0], y - this.center[1], z - this.center[2]]
    let k0 = 0
    let k1 = 0
    for (let i = 0; i < 3; i++) {
      const local = dot(p, this.axes[i])
      const r = this.radii[i]
      k0 += (local / r) ** 2
      k1 += (local / (r * r)) ** 2
    }
    k0 = Math.sqrt(k0)
    k1 = Math.sqrt(k1)
    if (k1 === 0) return -Math.min(...this.radii)
    return (k0 * (k0 - 1)) / k1
  }

  bounds(): Bounds {
    const r = Math.max(...this.radii)
    const c = this.center
    return { min: [c[0] - r, c[1] - r, c[2] - r], max: [c[0] + r, c[1] + r, c[2] + r] }
  }
}

function dot(a: Vec3, b: Vec3) {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
