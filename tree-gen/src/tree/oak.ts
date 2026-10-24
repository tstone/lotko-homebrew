import type { SceneDescription } from '../sdf/scene'
import { burls } from './features/burls'
import { core } from './features/core'
import { createTrunkFrame } from './features/frame'
import { gnarl } from './features/gnarl'
import { hollows } from './features/hollows'
import { roots } from './features/roots'
import type { TreeParams } from './params'

/**
 * Assembles a gnarled oak from independent features. Each feature only adds or carves shapes, so
 * new ones can be added here without touching the others, the mesher or the exporters.
 */
export function buildOak(params: TreeParams): SceneDescription {
  const frame = createTrunkFrame(params)
  return {
    add: [core(frame), ...roots(frame), ...burls(frame)],
    subtract: hollows(frame),
    displacement: gnarl(frame),
    // Only the top is cut, so the roots are left whole down to their tips.
    clip: { maxY: params.height },
  }
}
