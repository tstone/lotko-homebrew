import type { Part } from '../../sdf/scene'
import { Ellipsoid } from '../../sdf/shapes'
import { createNamedRandom } from '../../util/random'
import { localAxes, type TrunkFrame } from './frame'

/**
 * Hollows: tall, narrow pockets carved into the trunk where old wounds have rotted out. The
 * opening has a softly rounded lip; depth is capped so a hollow never punches through.
 */
export function hollows(frame: TrunkFrame): Part[] {
  const { params, height } = frame
  const rand = createNamedRandom(params.seed, 'hollows')

  return Array.from({ length: params.hollowCount }, () => {
    const y = height * rand.range(0.12, 0.6)
    const angle = rand.range(0, Math.PI * 2)
    const trunkRadius = frame.radiusAt(y)
    const size = params.hollowSize * trunkRadius * rand.range(0.8, 1.2)
    const center = frame.around(y, angle, trunkRadius * 0.95)
    const depth = Math.min(size * 0.75, trunkRadius * 0.7)
    const shape = new Ellipsoid(center, localAxes(angle), [depth, size * 1.5, size * 0.5])
    return { shapes: [shape], blend: size * 0.25 }
  })
}
