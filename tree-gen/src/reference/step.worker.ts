import occtimportjs from 'occt-import-js'
import wasmUrl from 'occt-import-js/dist/occt-import-js.wasm?url'
import type { ReferenceMesh } from './reference'

export type StepResponse = { meshes: ReferenceMesh[] } | { error: string }

self.onmessage = async (event: MessageEvent<ArrayBuffer>) => {
  try {
    const occt = await occtimportjs({ locateFile: () => wasmUrl })
    // OCCT converts whatever unit the file was authored in, so the output is always millimeters.
    const result = occt.ReadStepFile(new Uint8Array(event.data), { linearUnit: 'millimeter' })
    if (!result.success) throw new Error('Not a readable STEP file')
    if (result.meshes.length === 0) throw new Error('The file has no solids or surfaces to show')
    const meshes: ReferenceMesh[] = result.meshes.map((mesh) => ({
      positions: new Float32Array(mesh.attributes.position.array),
      indices: new Uint32Array(mesh.index.array),
      color: mesh.color ?? null,
    }))
    const transfer = meshes.flatMap((mesh) => [mesh.positions.buffer, mesh.indices.buffer])
    self.postMessage({ meshes } satisfies StepResponse, { transfer })
  } catch (error) {
    self.postMessage({ error: error instanceof Error ? error.message : String(error) } satisfies StepResponse)
  }
}
