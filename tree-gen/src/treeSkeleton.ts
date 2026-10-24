import type { Vec3 } from './sdf/shapes'
import { curveAt } from './sdf/tube'
import type { SkeletonContent } from './skeletonEditor'
import { createTrunkFrame } from './tree/features/frame'
import { rootGroundPoints } from './tree/features/roots'
import type { TreeParams } from './tree/params'
import { rootAngle, type RootPoint } from './tree/rootPaths'
import { skeletonOffsetAt, skeletonPointRange, type SkeletonPoint } from './tree/skeleton'

const TRUNK_COLOR = 0xef4444
const ROOT_COLOR = 0xf59e0b
const THICKNESS_COLOR = 0x38bdf8
const TRUNK_SAMPLES = 64
const ROOT_SAMPLES_PER_SPAN = 8
const RING_SEGMENTS = 32
// Dragging a thickness handle this many pixels up doubles the root's thickness.
const PIXELS_PER_DOUBLING = 100
const MIN_ROOT_SCALE = 0.3
const MAX_ROOT_SCALE = 3

export interface SkeletonEdits {
  moveTrunkPoint(index: number, point: SkeletonPoint): void
  moveRootPoint(root: number, index: number, point: RootPoint): void
  setRootScale(root: number, scale: number): void
}

/** The trunk and root skeletons of a tree as editable curves and handles. */
export function treeSkeleton(params: TreeParams, edits: SkeletonEdits): SkeletonContent {
  const { height, skeleton } = params
  const content: SkeletonContent = { curves: [], handles: [] }

  const trunk: Vec3[] = []
  for (let s = 0; s <= TRUNK_SAMPLES; s++) {
    const t = s / TRUNK_SAMPLES
    const [x, z] = skeletonOffsetAt(skeleton, t)
    trunk.push([x, t * height, z])
  }
  content.curves.push({ points: trunk, color: TRUNK_COLOR })
  content.handles.push({ key: 'trunk-base', position: [0, 0, 0], color: TRUNK_COLOR })
  skeleton.forEach((p, i) => {
    content.handles.push({
      key: `trunk-${i}`,
      position: [p.x, p.t * height, p.z],
      color: TRUNK_COLOR,
      move([x, y, z]) {
        const [low, high] = skeletonPointRange(skeleton, i)
        edits.moveTrunkPoint(i, { t: Math.min(Math.max(y / height, low), high), x, z })
      },
    })
  })

  const frame = createTrunkFrame(params)
  params.roots.forEach((root, r) => {
    const ground = rootGroundPoints(frame, root)
    const positions = ground.map((p) => p.position)
    const curve: Vec3[] = []
    const end = positions.length - 1
    for (let s = 0; s <= end * ROOT_SAMPLES_PER_SPAN; s++) curve.push(curveAt(positions, s / ROOT_SAMPLES_PER_SPAN))
    content.curves.push({ points: curve, color: ROOT_COLOR })
    root.points.forEach((p, i) => {
      content.handles.push({
        key: `root-${r}-${i}`,
        position: [p.x, p.y, p.z],
        color: ROOT_COLOR,
        move: ([x, y, z]) => edits.moveRootPoint(r, i, { x, y, z }),
      })
    })

    // Where the root meets the trunk: a handle that stays put and sets the root's thickness when
    // dragged up or down, with a ring showing the root's cross-section there.
    const elbow = ground[ground.length - 1]
    const angle = rootAngle(root)
    const across: Vec3 = [-Math.sin(angle), 0, Math.cos(angle)]
    const ring: Vec3[] = []
    for (let s = 0; s <= RING_SEGMENTS; s++) {
      const a = (s / RING_SEGMENTS) * Math.PI * 2
      const [c, d] = [Math.cos(a) * elbow.radius, Math.sin(a) * elbow.radius]
      ring.push([elbow.position[0] + across[0] * c, elbow.position[1] + d, elbow.position[2] + across[2] * c])
    }
    content.curves.push({ points: ring, color: ROOT_COLOR })
    content.handles.push({
      key: `root-${r}-thickness`,
      position: elbow.position,
      color: THICKNESS_COLOR,
      cursor: 'ns-resize',
      move(_, [, dy]) {
        const scale = root.scale * 2 ** (-dy / PIXELS_PER_DOUBLING)
        edits.setRootScale(r, Math.min(Math.max(scale, MIN_ROOT_SCALE), MAX_ROOT_SCALE))
      },
    })
  })

  return content
}
