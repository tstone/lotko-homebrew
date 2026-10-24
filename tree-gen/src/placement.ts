import * as THREE from 'three'

/**
 * Where the tree goes in the CAD model it is designed for: its real size, how it is turned and
 * where it stands. CAD coordinates are millimeters with Z up.
 */
export interface Placement {
  /** The trunk's height from the ground to the top cut. Fixes the scale from the tree's meters to millimeters. */
  heightMm: number
  /** Turn about the vertical axis, in degrees, counterclockwise seen from above. */
  rotation: number
  /** Where the center of the trunk's base sits. */
  x: number
  y: number
  z: number
}

export const defaultPlacement: Placement = { heightMm: 100, rotation: 0, x: 0, y: 0, z: 0 }

/**
 * The transform from the tree's own coordinates (meters, Y up) to CAD coordinates. The STL export
 * applies it and the reference geometry is shown through its inverse, so anything that lines up in
 * the viewport lines up in CAD.
 */
export function placementMatrix(placement: Placement, treeHeight: number): THREE.Matrix4 {
  const scale = placement.heightMm / treeHeight
  return new THREE.Matrix4()
    .makeTranslation(placement.x, placement.y, placement.z)
    .multiply(new THREE.Matrix4().makeRotationZ(THREE.MathUtils.degToRad(placement.rotation)))
    .multiply(new THREE.Matrix4().makeScale(scale, scale, scale))
    // +90° about X maps (x, y, z) to (x, -z, y): Y-up becomes Z-up and handedness is preserved.
    .multiply(new THREE.Matrix4().makeRotationX(Math.PI / 2))
}

/**
 * The viewport shows CAD space at the tree's scale: CAD millimeters shrunk to tree meters and turned
 * from Z up to Y up. The reference is drawn through this matrix; the tree is placed in it with
 * `treeSceneTransform`. Together they agree with `placementMatrix`.
 */
export function cadToSceneMatrix(placement: Placement, treeHeight: number): THREE.Matrix4 {
  const scale = treeHeight / placement.heightMm
  return new THREE.Matrix4().makeRotationX(-Math.PI / 2).multiply(new THREE.Matrix4().makeScale(scale, scale, scale))
}

/** Where the tree stands in the viewport: CAD (x, y, z) becomes (x, z, -y), and its turn is about Y. */
export function treeSceneTransform(placement: Placement, treeHeight: number) {
  const scale = treeHeight / placement.heightMm
  return {
    position: new THREE.Vector3(placement.x, placement.z, -placement.y).multiplyScalar(scale),
    rotationY: THREE.MathUtils.degToRad(placement.rotation),
  }
}

/** The inverse of `treeSceneTransform`, for when the tree is moved in the viewport. */
export function placementFromScene(
  position: THREE.Vector3,
  rotationY: number,
  placement: Placement,
  treeHeight: number,
): Pick<Placement, 'x' | 'y' | 'z' | 'rotation'> {
  const scale = placement.heightMm / treeHeight
  const round = (v: number) => Math.round(v * 10) / 10
  // Wrapped into -180..180 to stay within the rotation slider.
  const degrees = THREE.MathUtils.euclideanModulo(THREE.MathUtils.radToDeg(rotationY) + 180, 360) - 180
  return {
    x: round(position.x * scale),
    y: round(-position.z * scale),
    z: round(position.y * scale),
    rotation: round(degrees),
  }
}
