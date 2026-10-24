import * as THREE from 'three'
import { STLExporter } from 'three/examples/jsm/exporters/STLExporter.js'

/**
 * Encodes a geometry as binary STL for CAD. `placement` takes the tree to CAD coordinates (see
 * `placementMatrix`); STL has no units, and CAD programs read it as millimeters.
 */
export function geometryToStl(geometry: THREE.BufferGeometry, placement: THREE.Matrix4): Blob {
  const exported = geometry.clone()
  exported.applyMatrix4(placement)

  const data = new STLExporter().parse(new THREE.Mesh(exported), { binary: true })
  exported.dispose()
  return new Blob([data], { type: 'model/stl' })
}

export function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  link.click()
  URL.revokeObjectURL(url)
}
