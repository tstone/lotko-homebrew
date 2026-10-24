import type { MeshData } from '../mesh/polygonize'
import type { StepResponse } from './step.worker'

/** One solid or body from the reference file, in millimeters with Z up as CAD tools use. */
export interface ReferenceMesh extends MeshData {
  /** RGB in 0–1, if the file gave the body a color. */
  color: [number, number, number] | null
}

/** Geometry the tree is designed against, e.g. the parts it has to fit around. Display only. */
export interface ReferenceModel {
  name: string
  meshes: ReferenceMesh[]
}

/** The size of the model's bounding box along X, Y and Z, in millimeters. */
export function referenceSize(model: ReferenceModel): [number, number, number] {
  const min = [Infinity, Infinity, Infinity]
  const max = [-Infinity, -Infinity, -Infinity]
  for (const { positions } of model.meshes) {
    for (let i = 0; i < positions.length; i++) {
      min[i % 3] = Math.min(min[i % 3], positions[i])
      max[i % 3] = Math.max(max[i % 3], positions[i])
    }
  }
  return [max[0] - min[0], max[1] - min[1], max[2] - min[2]]
}

/**
 * Reads and triangulates a STEP file. OpenCascade is several megabytes of WebAssembly, so it runs
 * on a worker that only exists for the duration of the load.
 */
export function loadStepFile(file: File): Promise<ReferenceModel> {
  return file.arrayBuffer().then(
    (buffer) =>
      new Promise((resolve, reject) => {
        const worker = new Worker(new URL('./step.worker.ts', import.meta.url), { type: 'module' })
        worker.onmessage = (event: MessageEvent<StepResponse>) => {
          worker.terminate()
          if ('error' in event.data) reject(new Error(event.data.error))
          else resolve({ name: file.name, meshes: event.data.meshes })
        }
        worker.onerror = (event) => {
          worker.terminate()
          reject(event.error ?? new Error(event.message || 'The STEP reader failed to start'))
        }
        worker.postMessage(buffer, [buffer])
      }),
  )
}
