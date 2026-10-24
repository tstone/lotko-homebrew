import { createEffect, createSignal, on, onCleanup, onMount, Show } from 'solid-js'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { TransformControls } from 'three/examples/jsm/controls/TransformControls.js'
import { meshToGeometry } from './mesh/toGeometry'
import { cadToSceneMatrix, placementFromScene, treeSceneTransform, type Placement } from './placement'
import { referenceSize, type ReferenceModel } from './reference/reference'
import { createReferenceObject } from './reference/referenceObject'
import { createSkeletonEditor } from './skeletonEditor'
import { createMeshWorker } from './tree/meshWorker'
import { snapshotParams, type TreeParams } from './tree/params'
import { treeSkeleton, type SkeletonEdits } from './treeSkeleton'

// Cells along the tree's longest dimension. Lower than export so slider changes stay quick.
const PREVIEW_RESOLUTION = 96

interface ViewportProps {
  params: TreeParams
  reference: ReferenceModel | null
  placement: Placement
  /** Called as the tree is moved or turned in the viewport. */
  movePlacement: (change: Pick<Placement, 'x' | 'y' | 'z' | 'rotation'>) => void
  /** Name of the file being imported, while an import runs. */
  importing: string | null
  importError: string | null
  edits: SkeletonEdits
}

// How long a notice about a finished import stays up.
const NOTICE_MS = 5000

export function Viewport(props: ViewportProps) {
  let container!: HTMLDivElement
  const [generating, setGenerating] = createSignal(false)
  // At most one way of editing in the viewport at a time, so their drag handles never compete.
  const [tool, setTool] = createSignal<'skeleton' | 'move' | 'rotate' | null>(null)
  const repositioning = () => tool() === 'move' || tool() === 'rotate'
  const [showReference, setShowReference] = createSignal(true)
  const [notice, setNotice] = createSignal<{ text: string; error: boolean } | null>(null)
  let noticeTimer: ReturnType<typeof setTimeout> | undefined
  const notify = (text: string, error = false) => {
    clearTimeout(noticeTimer)
    setNotice({ text, error })
    noticeTimer = setTimeout(() => setNotice(null), NOTICE_MS)
  }
  onCleanup(() => clearTimeout(noticeTimer))
  createEffect(() => {
    if (props.importError) notify(`Import failed: ${props.importError}`, true)
  })

  // Seconds since the current import started, so a long import visibly keeps going.
  const [importSeconds, setImportSeconds] = createSignal(0)
  createEffect(() => {
    if (!props.importing) return
    const start = performance.now()
    setImportSeconds(0)
    const timer = setInterval(() => setImportSeconds(Math.floor((performance.now() - start) / 1000)), 1000)
    onCleanup(() => clearInterval(timer))
  })

  onMount(() => {
    const renderer = new THREE.WebGLRenderer({ antialias: true })
    renderer.setPixelRatio(window.devicePixelRatio)
    container.appendChild(renderer.domElement)

    const scene = new THREE.Scene()
    scene.background = new THREE.Color(0x1e1e22)

    scene.add(new THREE.HemisphereLight(0xdfe8ff, 0x3a3226, 1.2))
    const sun = new THREE.DirectionalLight(0xfff4e0, 2.2)
    sun.position.set(5, 10, 4)
    scene.add(sun)
    // Fill from the opposite side so form stays readable when orbiting.
    const fill = new THREE.DirectionalLight(0xdfe8ff, 0.9)
    fill.position.set(-6, 4, -5)
    scene.add(fill)

    // 1m grid for a sense of scale. It stays at the CAD origin while the tree is moved.
    const grid = new THREE.GridHelper(20, 20, 0x55555f, 0x35353c)
    scene.add(grid)

    const camera = new THREE.PerspectiveCamera(45, 1, 0.05, 500)
    camera.position.set(4.5, 3, 5.5)

    const controls = new OrbitControls(camera, renderer.domElement)
    controls.enableDamping = true
    controls.target.set(0, props.params.height / 2, 0)
    controls.update()

    // The scene is CAD space at the tree's scale (see cadToSceneMatrix). The tree and everything
    // drawn in its coordinates go in this group, which the placement moves and turns.
    const treeGroup = new THREE.Group()
    scene.add(treeGroup)
    createEffect(() => {
      const { position, rotationY } = treeSceneTransform({ ...props.placement }, props.params.height)
      treeGroup.position.copy(position)
      treeGroup.rotation.set(0, rotationY, 0)
    })

    const barkMaterial = new THREE.MeshStandardMaterial({ color: 0x5b4a3a, roughness: 0.95 })
    const tree = new THREE.Mesh(new THREE.BufferGeometry(), barkMaterial)
    treeGroup.add(tree)

    const gizmo = new TransformControls(camera, renderer.domElement)
    gizmo.setRotationSnap(THREE.MathUtils.degToRad(1))
    scene.add(gizmo.getHelper())
    gizmo.addEventListener('dragging-changed', (event) => {
      controls.enabled = !event.value
    })
    gizmo.addEventListener('objectChange', () => {
      // Read the turn from the quaternion: Euler angles flip past ±90°.
      const facing = new THREE.Vector3(1, 0, 0).applyQuaternion(treeGroup.quaternion)
      const rotationY = Math.atan2(-facing.z, facing.x)
      // Keep the tree upright even if a free-rotation handle tilted it.
      treeGroup.rotation.set(0, rotationY, 0)
      props.movePlacement(placementFromScene(treeGroup.position, rotationY, props.placement, props.params.height))
    })
    createEffect(() => {
      const current = tool()
      if (current !== 'move' && current !== 'rotate') {
        gizmo.detach()
        return
      }
      gizmo.setMode(current === 'move' ? 'translate' : 'rotate')
      // Only turning about the vertical axis makes sense for a tree standing on the ground.
      gizmo.showX = gizmo.showZ = gizmo.showE = gizmo.showXYZE = current === 'move'
      gizmo.attach(treeGroup)
    })

    // Points the camera at a box from the current viewing direction, close enough to fit it in view.
    const frame = (box: THREE.Box3) => {
      const sphere = box.getBoundingSphere(new THREE.Sphere())
      const fov = THREE.MathUtils.degToRad(camera.fov) * Math.min(1, camera.aspect)
      const distance = sphere.radius / Math.sin(fov / 2)
      const direction = camera.position.clone().sub(controls.target).normalize()
      controls.target.copy(sphere.center)
      camera.position.copy(sphere.center).addScaledVector(direction, distance)
      camera.near = Math.min(0.05, distance / 1000)
      camera.far = Math.max(500, distance * 20)
      camera.updateProjectionMatrix()
      controls.update()
    }

    const reference = createReferenceObject()
    scene.add(reference.object)
    createEffect(() => reference.setTransform(cadToSceneMatrix(props.placement, props.params.height)))
    createEffect(() => reference.setVisible(showReference()))
    createEffect(
      on(
        () => props.reference,
        (model) => {
          reference.setModel(model)
          if (!model) return
          const [x, y, z] = referenceSize(model).map((v) => v.toFixed(0))
          notify(`Imported ${model.name}: ${x} × ${y} × ${z} mm`)
          setShowReference(true)
          // Frame the tree and the reference together, so the new geometry is in view however
          // large it is or wherever it sits.
          frame(reference.sceneBounds()!.union(new THREE.Box3().setFromObject(tree)))
        },
      ),
    )

    const skeletonEditor = createSkeletonEditor({ camera, domElement: renderer.domElement, controls })
    treeGroup.add(skeletonEditor.object)
    createEffect(() => skeletonEditor.setVisible(tool() === 'skeleton'))
    createEffect(() => skeletonEditor.setContent(treeSkeleton(snapshotParams(props.params), props.edits)))

    const meshWorker = createMeshWorker()
    // generate() copies every param synchronously, so the effect subscribes to all of them.
    createEffect(() => {
      setGenerating(true)
      meshWorker.generate(props.params, PREVIEW_RESOLUTION).then(
        (mesh) => {
          tree.geometry.dispose()
          tree.geometry = meshToGeometry(mesh)
          setGenerating(false)
        },
        (error) => {
          console.error('Tree generation failed', error)
          setGenerating(false)
        },
      )
    })

    // Track the container rather than the window so the canvas follows the splitter.
    const resize = () => {
      const { clientWidth: w, clientHeight: h } = container
      if (w === 0 || h === 0) return
      renderer.setSize(w, h)
      camera.aspect = w / h
      camera.updateProjectionMatrix()
    }
    const observer = new ResizeObserver(resize)
    observer.observe(container)
    resize()

    renderer.setAnimationLoop(() => {
      controls.update()
      renderer.render(scene, camera)
    })

    onCleanup(() => {
      observer.disconnect()
      renderer.setAnimationLoop(null)
      controls.dispose()
      gizmo.dispose()
      skeletonEditor.dispose()
      reference.dispose()
      meshWorker.dispose()
      tree.geometry.dispose()
      barkMaterial.dispose()
      renderer.dispose()
      renderer.domElement.remove()
    })
  })

  return (
    <div class="viewport">
      {/* The canvas gets its own element: Solid owns the children of elements it renders into. */}
      <div ref={container} class="viewport-canvas" />
      <div class="viewport-toolbar">
        <button
          type="button"
          class="toggle"
          aria-pressed={tool() === 'skeleton'}
          title="Show the trunk's skeleton and drag its control points"
          onClick={() => setTool(tool() === 'skeleton' ? null : 'skeleton')}
        >
          Edit skeleton
        </button>
        <div class="toggle-group">
          <button
            type="button"
            class="toggle"
            aria-pressed={repositioning()}
            title="Drag the tree to move or turn it. Updates the placement."
            onClick={() => setTool(repositioning() ? null : 'move')}
          >
            Reposition
          </button>
          <Show when={repositioning()}>
            <button
              type="button"
              class="toggle"
              aria-pressed={tool() === 'move'}
              title="Drag the arrows to move along an axis, or the squares to move across a plane"
              onClick={() => setTool('move')}
            >
              Move
            </button>
            <button
              type="button"
              class="toggle"
              aria-pressed={tool() === 'rotate'}
              title="Drag the ring to turn the tree about its vertical axis"
              onClick={() => setTool('rotate')}
            >
              Rotate
            </button>
          </Show>
        </div>
        <Show when={props.reference}>
          <button
            type="button"
            class="toggle"
            aria-pressed={showReference()}
            title="Show the imported reference geometry"
            onClick={() => setShowReference(!showReference())}
          >
            Reference
          </button>
        </Show>
      </div>
      <div class="viewport-status">
        <Show when={props.importing}>
          {(name) => (
            <div class="status-badge">
              <span class="spinner" aria-hidden="true" />
              Importing {name()}… {importSeconds()}s
            </div>
          )}
        </Show>
        <Show when={generating()}>
          <div class="status-badge">Generating…</div>
        </Show>
        <Show when={notice()}>
          {(n) => (
            <div class="status-badge" classList={{ error: n().error }} role="status">
              {n().text}
            </div>
          )}
        </Show>
      </div>
    </div>
  )
}
