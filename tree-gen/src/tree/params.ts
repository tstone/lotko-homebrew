import { fitRoots, type RootPath } from './rootPaths'
import type { SkeletonPoint } from './skeleton'

export interface TreeParams {
  // Trunk
  /** Height of the flat cut at the top of the trunk, in meters. */
  height: number
  /** Trunk diameter just above the roots, in meters. */
  baseDiameter: number
  /** Top diameter as a fraction of the base diameter. */
  taper: number
  /** How much the trunk swells out just above the ground, as a fraction of the base radius. */
  baseFlare: number
  /** Number of control points on the skeleton. */
  skeletonPoints: number
  /** Control points the trunk's centerline runs through, in order up the trunk. Kept `skeletonPoints` long. */
  skeleton: SkeletonPoint[]
  /** How far the root ridges spiral around the trunk from bottom to top, in turns. */
  twist: number

  // Roots
  rootCount: number
  /** Number of control points along each root. */
  rootPoints: number
  /** Each root's path along the ground. Kept `rootCount` long, each with `rootPoints` points. */
  roots: RootPath[]
  /** Root thickness relative to a snug fit around the trunk. Above 1 fills in the grooves; below opens them up. */
  rootThickness: number
  /** How far up the trunk the root ridges carry, as a fraction of the height. */
  ridgeHeight: number
  // Gnarl
  /** Large-scale lumpiness of the whole surface. */
  gnarl: number
  burlCount: number
  /** Burl size as a fraction of the local trunk radius. */
  burlSize: number
  hollowCount: number
  /** Hollow size as a fraction of the local trunk radius. */
  hollowSize: number

  seed: number
}

const defaultsWithoutRoots: TreeParams = {
  height: 4,
  baseDiameter: 1.2,
  taper: 0.75,
  baseFlare: 0.3,
  skeletonPoints: 3,
  // A gentle lean, curving away from vertical toward the top.
  skeleton: [
    { t: 1 / 3, x: 0.03, z: 0 },
    { t: 2 / 3, x: 0.13, z: 0 },
    { t: 1, x: 0.3, z: 0 },
  ],
  twist: 0.25,

  rootCount: 7,
  rootPoints: 4,
  roots: [],
  rootThickness: 1,
  ridgeHeight: 1.1,

  gnarl: 0.5,
  burlCount: 3,
  burlSize: 0.5,
  hollowCount: 1,
  hollowSize: 0.6,

  seed: 1,
}

export const defaultTreeParams: TreeParams = { ...defaultsWithoutRoots, roots: fitRoots(defaultsWithoutRoots) }

/**
 * Copies the params into plain objects, e.g. to send them to a worker. Reads every value, so a
 * reactive caller depends on all of them.
 */
export function snapshotParams(params: TreeParams): TreeParams {
  return {
    ...params,
    skeleton: params.skeleton.map((p) => ({ ...p })),
    roots: params.roots.map((root) => ({ scale: root.scale, points: root.points.map((p) => ({ ...p })) })),
  }
}

type NumericParamKey = { [K in keyof TreeParams]: TreeParams[K] extends number ? K : never }[keyof TreeParams]

export interface ParamSpec {
  key: NumericParamKey
  label: string
  min: number
  max: number
  step: number
  unit?: string
}

export interface ParamGroup {
  title: string
  params: ParamSpec[]
}

export const treeParamGroups: ParamGroup[] = [
  {
    title: 'Trunk',
    params: [
      { key: 'height', label: 'Height', min: 0.5, max: 15, step: 0.1, unit: 'm' },
      { key: 'baseDiameter', label: 'Base diameter', min: 0.2, max: 3, step: 0.05, unit: 'm' },
      { key: 'taper', label: 'Taper', min: 0.75, max: 1.25, step: 0.01 },
      { key: 'baseFlare', label: 'Base flare', min: 0, max: 1, step: 0.01 },
      { key: 'skeletonPoints', label: 'Control points', min: 1, max: 8, step: 1 },
      { key: 'twist', label: 'Twist', min: -1, max: 1, step: 0.01, unit: 'turns' },
    ],
  },
  {
    title: 'Roots',
    params: [
      { key: 'rootCount', label: 'Count', min: 0, max: 12, step: 1 },
      { key: 'rootPoints', label: 'Control points', min: 2, max: 8, step: 1 },
      { key: 'rootThickness', label: 'Thickness', min: 1, max: 2, step: 0.01 },
      { key: 'ridgeHeight', label: 'Ridge height', min: 0, max: 1.2, step: 0.01 },
    ],
  },
  {
    title: 'Gnarl',
    params: [
      { key: 'gnarl', label: 'Lumpiness', min: 0, max: 1, step: 0.01 },
      { key: 'burlCount', label: 'Burls', min: 0, max: 10, step: 1 },
      { key: 'burlSize', label: 'Burl size', min: 0.1, max: 1, step: 0.01 },
      { key: 'hollowCount', label: 'Hollows', min: 0, max: 5, step: 1 },
      { key: 'hollowSize', label: 'Hollow size', min: 0.1, max: 1, step: 0.01 },
    ],
  },
  {
    title: 'Variation',
    params: [{ key: 'seed', label: 'Seed', min: 1, max: 1000, step: 1 }],
  },
]
