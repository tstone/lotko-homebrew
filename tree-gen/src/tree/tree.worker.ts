import { generateTreeMesh } from './generateTree'
import type { TreeParams } from './params'

export interface MeshRequest {
  id: number
  params: TreeParams
  resolution: number
}

self.onmessage = (event: MessageEvent<MeshRequest>) => {
  const { id, params, resolution } = event.data
  const mesh = generateTreeMesh(params, resolution)
  self.postMessage({ id, mesh }, { transfer: [mesh.positions.buffer, mesh.indices.buffer] })
}
