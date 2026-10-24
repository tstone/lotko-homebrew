import type { MeshData } from './polygonize'

/**
 * Keeps only the connected piece with the most triangles. Thin features close to the grid
 * resolution can leave stray crumbs of surface; dropping them guarantees a single solid body,
 * which is what CAD tools expect.
 */
export function keepLargestComponent(mesh: MeshData): MeshData {
  const { positions, indices } = mesh
  const vertexCount = positions.length / 3
  const parent = new Int32Array(vertexCount)
  for (let i = 0; i < vertexCount; i++) parent[i] = i
  const find = (i: number) => {
    while (parent[i] !== i) {
      parent[i] = parent[parent[i]]
      i = parent[i]
    }
    return i
  }
  for (let t = 0; t < indices.length; t += 3) {
    const a = find(indices[t])
    parent[find(indices[t + 1])] = a
    parent[find(indices[t + 2])] = a
  }

  const triangleCount = new Map<number, number>()
  for (let t = 0; t < indices.length; t += 3) {
    const root = find(indices[t])
    triangleCount.set(root, (triangleCount.get(root) ?? 0) + 1)
  }
  if (triangleCount.size <= 1) return mesh
  let largest = -1
  let most = 0
  for (const [root, count] of triangleCount) {
    if (count > most) {
      most = count
      largest = root
    }
  }

  const remap = new Int32Array(vertexCount).fill(-1)
  const keptPositions: number[] = []
  const keptIndices = new Uint32Array(most * 3)
  let k = 0
  for (let t = 0; t < indices.length; t += 3) {
    if (find(indices[t]) !== largest) continue
    for (let j = 0; j < 3; j++) {
      const v = indices[t + j]
      if (remap[v] === -1) {
        remap[v] = keptPositions.length / 3
        keptPositions.push(positions[v * 3], positions[v * 3 + 1], positions[v * 3 + 2])
      }
      keptIndices[k++] = remap[v]
    }
  }
  return { positions: new Float32Array(keptPositions), indices: keptIndices }
}
