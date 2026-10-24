import { keepLargestComponent } from '../mesh/cleanup'
import { polygonize, type MeshData } from '../mesh/polygonize'
import { createScene } from '../sdf/scene'
import { buildOak } from './oak'
import type { TreeParams } from './params'

/** The one entry point from parameters to a closed triangle mesh. Viewport and exporters use only this. */
export function generateTreeMesh(params: TreeParams, resolution: number): MeshData {
  return keepLargestComponent(polygonize(createScene(buildOak(params)), resolution))
}
