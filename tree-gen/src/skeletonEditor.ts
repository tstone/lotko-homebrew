import * as THREE from 'three'
import type { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import type { Vec3 } from './sdf/shapes'

// How close the cursor must be to a handle to grab it, in CSS pixels.
const PICK_RADIUS = 14

const ACTIVE_COLOR = new THREE.Color(0xf4f4f5)
const FIXED_COLOR = new THREE.Color(0x71717a)

export interface SkeletonCurve {
  points: Vec3[]
  color: number
}

export interface SkeletonHandle {
  /** Identifies the handle across updates, so a drag survives the content being rebuilt. */
  key: string
  position: Vec3
  color: number
  /** Cursor shown over the handle. Defaults to a grab hand. */
  cursor?: string
  /**
   * Called while the handle is dragged, on the handle as it was when the drag started. `position` is
   * where the cursor puts the handle; `screenDelta` is how far the cursor has moved since the drag
   * started, in CSS pixels. Omit for a point that can't be dragged.
   */
  move?(position: Vec3, screenDelta: [number, number]): void
}

export interface SkeletonContent {
  curves: SkeletonCurve[]
  handles: SkeletonHandle[]
}

interface SkeletonEditorOptions {
  camera: THREE.Camera
  domElement: HTMLElement
  controls: OrbitControls
}

/**
 * Curves drawn over the tree with draggable handles on them. It ignores depth so it stays visible
 * inside the trunk. What a handle controls, and how its movement is constrained, is up to its
 * `move` callback. Positions are in the coordinates of whatever `object` is added to, so it can
 * be moved along with the tree; that parent may be moved and turned about Y, but not tilted.
 */
export function createSkeletonEditor({ camera, domElement, controls }: SkeletonEditorOptions) {
  const object = new THREE.Group()
  object.visible = false

  const lineGeometry = new THREE.BufferGeometry()
  const lineMaterial = new THREE.LineBasicMaterial({ vertexColors: true, depthTest: false, transparent: true })
  const lines = new THREE.LineSegments(lineGeometry, lineMaterial)

  const dot = dotTexture()
  const pointGeometry = new THREE.BufferGeometry()
  const pointMaterial = new THREE.PointsMaterial({
    size: 14,
    sizeAttenuation: false,
    vertexColors: true,
    map: dot,
    alphaTest: 0.5,
    depthTest: false,
    transparent: true,
  })
  const points = new THREE.Points(pointGeometry, pointMaterial)

  for (const child of [lines, points]) {
    // Geometry is rebuilt on every change, so skip keeping bounding spheres up to date.
    child.frustumCulled = false
    object.add(child)
  }
  lines.renderOrder = 1
  points.renderOrder = 2

  let content: SkeletonContent = { curves: [], handles: [] }
  let hovered: SkeletonHandle | undefined
  let dragging: {
    handle: SkeletonHandle
    plane: THREE.Plane
    grab: THREE.Vector3
    start: { x: number; y: number }
  } | null = null
  const raycaster = new THREE.Raycaster()

  const rebuild = () => {
    const linePositions: number[] = []
    const lineColors: number[] = []
    for (const curve of content.curves) {
      const c = new THREE.Color(curve.color)
      for (let i = 0; i < curve.points.length - 1; i++) {
        linePositions.push(...curve.points[i], ...curve.points[i + 1])
        lineColors.push(c.r, c.g, c.b, c.r, c.g, c.b)
      }
    }
    lineGeometry.setAttribute('position', new THREE.Float32BufferAttribute(linePositions, 3))
    lineGeometry.setAttribute('color', new THREE.Float32BufferAttribute(lineColors, 3))

    const active = (dragging?.handle ?? hovered)?.key
    const positions: number[] = []
    const colors: number[] = []
    for (const handle of content.handles) {
      const c = !handle.move ? FIXED_COLOR : handle.key === active ? ACTIVE_COLOR : new THREE.Color(handle.color)
      positions.push(...handle.position)
      colors.push(c.r, c.g, c.b)
    }
    pointGeometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
    pointGeometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  }

  const cursorIn = (event: PointerEvent) => {
    const rect = domElement.getBoundingClientRect()
    return { x: event.clientX - rect.left, y: event.clientY - rect.top, width: rect.width, height: rect.height }
  }

  /** The movable handle under the cursor, or undefined. */
  const pick = (event: PointerEvent) => {
    const cursor = cursorIn(event)
    let best: SkeletonHandle | undefined
    let bestDistance = PICK_RADIUS
    for (const handle of content.handles) {
      if (!handle.move) continue
      const p = new THREE.Vector3(...handle.position).applyMatrix4(object.matrixWorld).project(camera)
      if (p.z > 1) continue
      const x = ((p.x + 1) / 2) * cursor.width
      const y = ((1 - p.y) / 2) * cursor.height
      const distance = Math.hypot(x - cursor.x, y - cursor.y)
      if (distance < bestDistance) {
        bestDistance = distance
        best = handle
      }
    }
    return best
  }

  const hitOn = (event: PointerEvent, plane: THREE.Plane) => {
    const cursor = cursorIn(event)
    const ndc = new THREE.Vector2((cursor.x / cursor.width) * 2 - 1, 1 - (cursor.y / cursor.height) * 2)
    raycaster.setFromCamera(ndc, camera)
    return raycaster.ray.intersectPlane(plane, new THREE.Vector3())
  }

  const setHovered = (handle: SkeletonHandle | undefined) => {
    if (handle?.key === hovered?.key) return
    hovered = handle
    domElement.style.cursor = handle ? (handle.cursor ?? 'grab') : ''
    rebuild()
  }

  const onPointerDown = (event: PointerEvent) => {
    if (!object.visible || event.button !== 0) return
    const handle = pick(event)
    if (!handle) return
    const point = new THREE.Vector3(...handle.position).applyMatrix4(object.matrixWorld)
    // Looking down, the handle slides over a horizontal plane at its height; from the side, over an
    // upright plane facing the camera, so it moves across and up or down with the cursor.
    const view = camera.getWorldDirection(new THREE.Vector3())
    const normal =
      Math.abs(view.y) <= 0.6 ? new THREE.Vector3(view.x, 0, view.z).normalize() : new THREE.Vector3(0, 1, 0)
    const plane = new THREE.Plane().setFromNormalAndCoplanarPoint(normal, point)
    const hit = hitOn(event, plane)
    if (!hit) return

    // Registered in the capture phase, so this runs before OrbitControls sees the event.
    event.stopImmediatePropagation()
    controls.enabled = false
    domElement.setPointerCapture(event.pointerId)
    domElement.style.cursor = handle.cursor ?? 'grabbing'
    dragging = { handle, plane, grab: point.sub(hit), start: { x: event.clientX, y: event.clientY } }
    rebuild()
  }

  const onPointerMove = (event: PointerEvent) => {
    if (!dragging) {
      if (object.visible) setHovered(pick(event))
      return
    }
    const hit = hitOn(event, dragging.plane)
    if (!hit) return
    object.worldToLocal(hit.add(dragging.grab))
    const { handle, start } = dragging
    handle.move?.([hit.x, hit.y, hit.z], [event.clientX - start.x, event.clientY - start.y])
  }

  const onPointerUp = (event: PointerEvent) => {
    if (!dragging) return
    dragging = null
    controls.enabled = true
    domElement.releasePointerCapture(event.pointerId)
    hovered = pick(event)
    domElement.style.cursor = hovered ? (hovered.cursor ?? 'grab') : ''
    rebuild()
  }

  domElement.addEventListener('pointerdown', onPointerDown, { capture: true })
  domElement.addEventListener('pointermove', onPointerMove)
  domElement.addEventListener('pointerup', onPointerUp)
  domElement.addEventListener('pointercancel', onPointerUp)

  return {
    object,
    setContent(next: SkeletonContent) {
      content = next
      rebuild()
    },
    setVisible(visible: boolean) {
      object.visible = visible
      if (!visible) setHovered(undefined)
    },
    dispose() {
      domElement.removeEventListener('pointerdown', onPointerDown, { capture: true })
      domElement.removeEventListener('pointermove', onPointerMove)
      domElement.removeEventListener('pointerup', onPointerUp)
      domElement.removeEventListener('pointercancel', onPointerUp)
      lineGeometry.dispose()
      lineMaterial.dispose()
      pointGeometry.dispose()
      pointMaterial.dispose()
      dot.dispose()
    },
  }
}

/** A round dot with a dark outline, so points read against both the bark and the background. */
function dotTexture() {
  const size = 64
  const canvas = document.createElement('canvas')
  canvas.width = size
  canvas.height = size
  const context = canvas.getContext('2d')!
  context.beginPath()
  context.arc(size / 2, size / 2, size / 2 - 4, 0, Math.PI * 2)
  context.fillStyle = '#fff'
  context.fill()
  context.lineWidth = 6
  context.strokeStyle = '#18181b'
  context.stroke()
  return new THREE.CanvasTexture(canvas)
}
