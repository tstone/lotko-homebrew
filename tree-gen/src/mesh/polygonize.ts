import type { Evaluator, Field } from '../sdf/scene'

export interface MeshData {
  positions: Float32Array
  indices: Uint32Array
}

// Cells per side of a block. The field is asked once per block whether it contains any surface.
const BLOCK = 8

// Cube corners are numbered by bit: 1 = +x, 2 = +y, 4 = +z.
//
// Each cube is split into six tetrahedra along its main diagonal (Kuhn triangulation). Neighboring
// cubes split shared faces the same way, so the surface has no cracks. Each tetrahedron is a chain
// 0 ⊂ a ⊂ b ⊂ 7, so every edge runs from a corner to one whose bits are a superset of its own.
const TETRAHEDRA = [
  [0, 1, 3, 7],
  [0, 1, 5, 7],
  [0, 2, 3, 7],
  [0, 2, 6, 7],
  [0, 4, 5, 7],
  [0, 4, 6, 7],
]

// The triangles those tetrahedra put on a cube's top and bottom faces, used to build the caps.
const TOP_FACE = [
  [2, 3, 7],
  [2, 7, 6],
]
const BOTTOM_FACE = [
  [0, 1, 5],
  [0, 5, 4],
]

/**
 * Converts a distance field to a closed, consistently wound triangle mesh using marching
 * tetrahedra. `resolution` is the number of cells along the longest side of the field's bounds.
 *
 * Grid layers sit exactly on the bottom and top of the bounds. Wherever the solid reaches them it
 * is closed with a flat cap, built from the same grid edges as the side surface. That gives crisp,
 * exactly flat cuts that share every rim vertex with the sides, so the mesh stays watertight.
 */
export function polygonize(field: Field, resolution: number): MeshData {
  const { min, max } = field.bounds
  const longest = Math.max(max[0] - min[0], max[1] - min[1], max[2] - min[2])
  const cell = longest / resolution
  const height = max[1] - min[1]
  const nx = Math.ceil((max[0] - min[0]) / cell) + 2
  const ny = Math.max(1, Math.round(height / cell))
  const nz = Math.ceil((max[2] - min[2]) / cell) + 2
  // Cells are cubes in x and z, padded by one so the field is outside at the grid's sides. In y
  // the cell height is stretched slightly so whole cells span exactly from bottom to top.
  const size = [cell, height / ny, cell]
  const origin = [min[0] - cell, min[1], min[2] - cell]

  const px = nx + 1
  const pxy = px * (ny + 1)
  const pointX = (i: number) => i % px
  const pointY = (i: number) => Math.floor(i / px) % (ny + 1)
  const pointZ = (i: number) => Math.floor(i / pxy)
  const cornerOffset = (bits: number) => (bits & 1) + (bits & 2 ? px : 0) + (bits & 4 ? pxy : 0)

  // One value per grid point, computed at most once so neighboring blocks always agree.
  const values = new Float32Array(px * (ny + 1) * (nz + 1)).fill(NaN)
  const positions: number[] = []
  const indices: number[] = []
  const edgeVertices = new Map<number, number>()
  const pointVertices = new Map<number, number>()

  const sample = (i: number, evaluate: Evaluator) => {
    if (!Number.isNaN(values[i])) return
    const v = evaluate(
      origin[0] + pointX(i) * size[0],
      origin[1] + pointY(i) * size[1],
      origin[2] + pointZ(i) * size[2],
    )
    // Never exactly zero, so no vertex lands on a grid point and no triangle degenerates.
    values[i] = v === 0 ? 1e-9 : v
  }

  /** Vertex where the surface crosses the edge from corner `lo` to corner `hi` (hi ⊇ lo). */
  const edgeVertex = (base: number, lo: number, hi: number) => {
    const a = base + cornerOffset(lo)
    const key = a * 8 + (lo ^ hi)
    let index = edgeVertices.get(key)
    if (index === undefined) {
      const dir = lo ^ hi
      const va = values[a]
      const t = va / (va - values[base + cornerOffset(hi)])
      positions.push(
        origin[0] + (pointX(a) + (dir & 1 ? t : 0)) * size[0],
        origin[1] + (pointY(a) + (dir & 2 ? t : 0)) * size[1],
        origin[2] + (pointZ(a) + (dir & 4 ? t : 0)) * size[2],
      )
      index = positions.length / 3 - 1
      edgeVertices.set(key, index)
    }
    return index
  }

  const edge = (base: number, p: number, q: number) =>
    (p & q) === p ? edgeVertex(base, p, q) : edgeVertex(base, q, p)

  /** Vertex at a grid point itself; only cap interiors use these. */
  const pointVertex = (base: number, corner: number) => {
    const i = base + cornerOffset(corner)
    let index = pointVertices.get(i)
    if (index === undefined) {
      positions.push(
        origin[0] + pointX(i) * size[0],
        origin[1] + pointY(i) * size[1],
        origin[2] + pointZ(i) * size[2],
      )
      index = positions.length / 3 - 1
      pointVertices.set(i, index)
    }
    return index
  }

  const isInside = (base: number, corner: number) => values[base + cornerOffset(corner)] < 0

  function polygonizeCube(base: number) {
    let inside = 0
    for (let c = 0; c < 8; c++) if (isInside(base, c)) inside |= 1 << c
    if (inside === 0 || inside === 0xff) return

    for (const tet of TETRAHEDRA) {
      const ins: number[] = []
      const outs: number[] = []
      for (const c of tet) (inside & (1 << c) ? ins : outs).push(c)

      if (ins.length === 1 || ins.length === 3) {
        // One corner on its own side: a single triangle cutting off that corner. Wind it so the
        // normal points from inside to outside, judged from the exact integer corner offsets.
        const [apex, others] = ins.length === 1 ? [ins[0], outs] : [outs[0], ins]
        const [a, b, c] = others
        const awayFromApex = orientation(apex, a, b, c) > 0
        const va = edge(base, apex, a)
        const vb = edge(base, apex, b)
        const vc = edge(base, apex, c)
        if (awayFromApex === (ins.length === 1)) indices.push(va, vb, vc)
        else indices.push(va, vc, vb)
      } else if (ins.length === 2) {
        const [i0, i1] = ins
        const [o0, o1] = outs
        const q0 = edge(base, i0, o0)
        const q1 = edge(base, i0, o1)
        const q2 = edge(base, i1, o1)
        const q3 = edge(base, i1, o0)
        if (orientation(i0, i1, o0, o1) > 0) indices.push(q0, q1, q2, q0, q2, q3)
        else indices.push(q0, q2, q1, q0, q3, q2)
      }
    }
  }

  /** Fills the inside part of a cube's top or bottom face, facing up (+1) or down (-1). */
  function capFace(base: number, face: number[][], facing: 1 | -1) {
    for (const corners of face) {
      // Put the corners in the cyclic order whose normal faces the right way. The cap polygon walks
      // the triangle's boundary in that same order, so it inherits the winding.
      const tri = faceNormalY(corners) === facing ? corners : [corners[0], corners[2], corners[1]]
      const inside = tri.map((c) => isInside(base, c))
      const count = inside.filter(Boolean).length
      if (count === 0) continue
      if (count === 3) {
        indices.push(pointVertex(base, tri[0]), pointVertex(base, tri[1]), pointVertex(base, tri[2]))
        continue
      }
      // Rotate so the lone corner (inside if count is 1, outside if 2) comes first.
      const lone = inside.findIndex((v) => v === (count === 1))
      const [l, a, b] = [tri[lone], tri[(lone + 1) % 3], tri[(lone + 2) % 3]]
      if (count === 1) {
        indices.push(pointVertex(base, l), edge(base, l, a), edge(base, b, l))
      } else {
        const p0 = pointVertex(base, a)
        const p1 = pointVertex(base, b)
        const p2 = edge(base, b, l)
        const p3 = edge(base, l, a)
        indices.push(p0, p1, p2, p0, p2, p3)
      }
    }
  }

  const blockRadius = (BLOCK / 2) * Math.hypot(size[0], size[1], size[2])

  for (let z0 = 0; z0 < nz; z0 += BLOCK) {
    for (let y0 = 0; y0 < ny; y0 += BLOCK) {
      for (let x0 = 0; x0 < nx; x0 += BLOCK) {
        const x1 = Math.min(x0 + BLOCK, nx)
        const y1 = Math.min(y0 + BLOCK, ny)
        const z1 = Math.min(z0 + BLOCK, nz)
        const region = field.region(
          origin[0] + ((x0 + x1) / 2) * size[0],
          origin[1] + ((y0 + y1) / 2) * size[1],
          origin[2] + ((z0 + z1) / 2) * size[2],
          blockRadius,
        )
        if (region.kind === 'outside') continue

        // Solid blocks have no side surface, but still need caps where they touch the cut planes.
        const evaluate = region.kind === 'surface' ? region.evaluate : field.evaluate
        const layers: number[] = []
        if (region.kind === 'surface') for (let y = y0; y <= y1; y++) layers.push(y)
        else {
          if (y0 === 0) layers.push(0)
          if (y1 === ny) layers.push(ny)
        }
        for (const y of layers) {
          for (let z = z0; z <= z1; z++) {
            for (let x = x0; x <= x1; x++) sample(x + y * px + z * pxy, evaluate)
          }
        }

        for (let z = z0; z < z1; z++) {
          for (let x = x0; x < x1; x++) {
            if (region.kind === 'surface') {
              for (let y = y0; y < y1; y++) polygonizeCube(x + y * px + z * pxy)
            }
            if (y0 === 0) capFace(x + z * pxy, BOTTOM_FACE, -1)
            if (y1 === ny) capFace(x + (ny - 1) * px + z * pxy, TOP_FACE, 1)
          }
        }
      }
    }
  }

  return { positions: new Float32Array(positions), indices: new Uint32Array(indices) }
}

/** Sign of det(b - a, c - a, d - a) for cube corners given by bit index. */
function orientation(a: number, b: number, c: number, d: number) {
  const [ax, ay, az] = corner(a)
  const [bx, by, bz] = corner(b)
  const [cx, cy, cz] = corner(c)
  const [dx, dy, dz] = corner(d)
  const ux = bx - ax
  const uy = by - ay
  const uz = bz - az
  const vx = cx - ax
  const vy = cy - ay
  const vz = cz - az
  const wx = dx - ax
  const wy = dy - ay
  const wz = dz - az
  return ux * (vy * wz - vz * wy) - uy * (vx * wz - vz * wx) + uz * (vx * wy - vy * wx)
}

/** Sign of the y component of the normal of a triangle of cube corners. */
function faceNormalY([a, b, c]: number[]): 1 | -1 {
  const [ax, , az] = corner(a)
  const [bx, , bz] = corner(b)
  const [cx, , cz] = corner(c)
  return (bz - az) * (cx - ax) - (bx - ax) * (cz - az) > 0 ? 1 : -1
}

function corner(bits: number): [number, number, number] {
  return [bits & 1, (bits >> 1) & 1, (bits >> 2) & 1]
}
