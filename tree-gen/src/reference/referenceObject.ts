import * as THREE from 'three'
import { meshToGeometry } from '../mesh/toGeometry'
import type { ReferenceModel } from './reference'

const DEFAULT_COLOR = new THREE.Color(0x7aa2c8)

/**
 * Shows reference geometry in the viewport. The meshes stay in their file coordinates (CAD
 * millimeters, Z up); the object as a whole is placed with `cadToSceneMatrix`.
 */
export function createReferenceObject() {
  const object = new THREE.Group()
  object.matrixAutoUpdate = false

  const clear = () => {
    for (const child of object.children as THREE.Mesh[]) {
      child.geometry.dispose()
      ;(child.material as THREE.Material).dispose()
    }
    object.clear()
  }

  return {
    object,
    setModel(model: ReferenceModel | null) {
      clear()
      for (const mesh of model?.meshes ?? []) {
        const material = new THREE.MeshStandardMaterial({
          color: mesh.color ? new THREE.Color(...mesh.color) : DEFAULT_COLOR,
          roughness: 0.7,
          transparent: true,
          opacity: 0.6,
          // STEP surfaces are not always closed solids, so draw the back of open shells too.
          side: THREE.DoubleSide,
        })
        object.add(new THREE.Mesh(meshToGeometry(mesh), material))
      }
    },
    setTransform(cadToScene: THREE.Matrix4) {
      object.matrix.copy(cadToScene)
      object.matrixWorldNeedsUpdate = true
    },
    setVisible(visible: boolean) {
      object.visible = visible
    },
    /** The reference's bounds in the scene, or null when nothing is loaded. */
    sceneBounds(): THREE.Box3 | null {
      if (object.children.length === 0) return null
      object.updateMatrixWorld(true)
      return new THREE.Box3().setFromObject(object)
    },
    dispose: clear,
  }
}
