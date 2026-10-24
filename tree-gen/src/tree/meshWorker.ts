import type { MeshData } from '../mesh/polygonize'
import { snapshotParams, type TreeParams } from './params'
import type { MeshRequest } from './tree.worker'

/**
 * Runs tree generation on a background thread so the UI stays responsive. Only the newest request
 * matters: while one is running, later requests replace each other in a one-slot queue, and the
 * promises of superseded requests never settle, so stale meshes are never applied.
 */
export function createMeshWorker() {
  const worker = new Worker(new URL('./tree.worker.ts', import.meta.url), { type: 'module' })
  let latest = 0
  let busy = false
  let queued: MeshRequest | null = null
  let pending: { id: number; resolve: (mesh: MeshData) => void; reject: (error: unknown) => void } | null = null

  const send = (request: MeshRequest) => {
    busy = true
    worker.postMessage(request)
  }

  const next = () => {
    busy = false
    if (queued) {
      send(queued)
      queued = null
    }
  }

  worker.onmessage = (event: MessageEvent<{ id: number; mesh: MeshData }>) => {
    if (pending?.id === event.data.id) {
      pending.resolve(event.data.mesh)
      pending = null
    }
    next()
  }
  worker.onerror = (event) => {
    pending?.reject(event.error ?? new Error(event.message))
    pending = null
    next()
  }

  return {
    generate(params: TreeParams, resolution: number): Promise<MeshData> {
      // Copy so a reactive store proxy can be sent across threads.
      const request: MeshRequest = { id: ++latest, params: snapshotParams(params), resolution }
      return new Promise((resolve, reject) => {
        pending = { id: request.id, resolve, reject }
        if (busy) queued = request
        else send(request)
      })
    },
    dispose() {
      worker.terminate()
    },
  }
}
