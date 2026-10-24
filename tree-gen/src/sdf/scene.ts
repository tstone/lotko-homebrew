import { smoothMax, smoothMin, type Bounds, type Shape } from './shapes'

/**
 * A group of shapes joined with a plain union (e.g. the capsules that make up one root), which is
 * then blended into the rest of the scene with a fillet `blend` wide.
 */
export interface Part {
  shapes: Shape[]
  blend: number
}

export interface Displacement {
  /** Largest absolute value `fn` can return. */
  amplitude: number
  fn(x: number, y: number, z: number): number
}

export interface SceneDescription {
  /** Smoothly unioned, in order. */
  add: Part[]
  /** Smoothly carved out of the union, in order. */
  subtract: Part[]
  /** Added to the distance after the union and carving, e.g. noise for an irregular surface. */
  displacement?: Displacement
  /** Flat cuts: the solid is kept only between these heights. Without `minY` the bottom is left uncut. */
  clip: { minY?: number; maxY: number }
}

export type Evaluator = (x: number, y: number, z: number) => number

export type Region =
  | { kind: 'outside' }
  | { kind: 'inside' }
  | { kind: 'surface'; evaluate: Evaluator }

/** A distance field the mesher can sample. */
export interface Field {
  /**
   * The field is outside the solid on the x and z faces of these bounds. On the y faces it may be
   * inside: the mesher closes the solid there with flat caps, which is how flat cuts are made.
   */
  bounds: Bounds
  evaluate: Evaluator
  /** Classifies everything within `radius` of the given center, so empty and solid space can be skipped. */
  region(cx: number, cy: number, cz: number, radius: number): Region
}

// Slack for shapes whose distance is only approximately Euclidean (ellipsoids, smooth blends).
const LIPSCHITZ_SLACK = 1.3

export function createScene(scene: SceneDescription): Field {
  const { add, subtract, displacement } = scene
  const amplitude = displacement?.amplitude ?? 0
  const maxAddBlend = Math.max(0, ...add.map((p) => p.blend))

  const evaluatorFor = (addParts: Part[], subParts: Part[]): Evaluator => (x, y, z) => {
    const v = carve(union(addParts, x, y, z), subParts, x, y, z)
    return displacement ? v + displacement.fn(x, y, z) : v
  }

  return {
    bounds: sceneBounds(scene),
    evaluate: evaluatorFor(add, subtract),

    region(cx, cy, cz, radius) {
      const reach = radius * LIPSCHITZ_SLACK

      // Only shapes that can come within one blend width of the nearest shape anywhere in the
      // region affect the union there, so the rest are dropped for this region.
      const addDistances = add.map((part) => part.shapes.map((s) => s.distance(cx, cy, cz)))
      const nearest = Math.min(...addDistances.flat())
      const nearAdd = filterParts(add, addDistances, () => nearest + 2 * reach + maxAddBlend)
      const d0 = union(nearAdd, cx, cy, cz)

      // A carve only matters where the point is inside it or within its blend width of its edge.
      const subDistances = subtract.map((part) => part.shapes.map((s) => s.distance(cx, cy, cz)))
      const nearSub = filterParts(subtract, subDistances, (part) => part.blend + 2 * reach + Math.max(0, -d0))
      const d = carve(d0, nearSub, cx, cy, cz)

      if (d - reach > amplitude) return { kind: 'outside' }
      if (d + reach < -amplitude) return { kind: 'inside' }
      return { kind: 'surface', evaluate: evaluatorFor(nearAdd, nearSub) }
    },
  }
}

function filterParts(parts: Part[], distances: number[][], limit: (part: Part) => number): Part[] {
  return parts
    .map((part, i) => ({ ...part, shapes: part.shapes.filter((_, j) => distances[i][j] <= limit(part)) }))
    .filter((part) => part.shapes.length > 0)
}

function partDistance(part: Part, x: number, y: number, z: number) {
  let d = Infinity
  for (const s of part.shapes) d = Math.min(d, s.distance(x, y, z))
  return d
}

function union(parts: Part[], x: number, y: number, z: number) {
  let d = Infinity
  for (const part of parts) d = smoothMin(d, partDistance(part, x, y, z), part.blend)
  return d
}

function carve(d: number, parts: Part[], x: number, y: number, z: number) {
  for (const part of parts) d = smoothMax(d, -partDistance(part, x, y, z), part.blend)
  return d
}

function sceneBounds({ add, displacement, clip }: SceneDescription): Bounds {
  const pad = Math.max(0, ...add.map((p) => p.blend)) + (displacement?.amplitude ?? 0)
  const min: Bounds['min'] = [Infinity, Infinity, Infinity]
  const max: Bounds['max'] = [-Infinity, -Infinity, -Infinity]
  for (const part of add) {
    for (const shape of part.shapes) {
      const b = shape.bounds()
      for (let i = 0; i < 3; i++) {
        min[i] = Math.min(min[i], b.min[i] - pad)
        max[i] = Math.max(max[i], b.max[i] + pad)
      }
    }
  }
  // The cut heights are the bounds in y, so the mesher caps the solid exactly there. Uncut, the
  // padded shape bounds keep the solid clear of the bottom face, so nothing is capped.
  if (clip.minY !== undefined) min[1] = clip.minY
  max[1] = clip.maxY
  return { min, max }
}
