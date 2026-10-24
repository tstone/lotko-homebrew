import type { Part } from '../../sdf/scene'
import { Ellipsoid, type Shape } from '../../sdf/shapes'
import { createNamedRandom } from '../../util/random'
import { localAxes, type TrunkFrame } from './frame'

/**
 * Burls: knobbly swellings on the trunk. Each is a main lobe with a few smaller lobes clustered on
 * it, so it reads as a lumpy growth rather than a smooth balloon.
 */
export function burls(frame: TrunkFrame): Part[] {
  const { params, height } = frame
  const rand = createNamedRandom(params.seed, 'burls')

  return Array.from({ length: params.burlCount }, () => {
    const y = height * rand.range(0.25, 0.85)
    const angle = rand.range(0, Math.PI * 2)
    const trunkRadius = frame.radiusAt(y)
    const size = params.burlSize * trunkRadius * rand.range(0.6, 1.3)

    const shapes: Shape[] = [
      new Ellipsoid(frame.around(y, angle, trunkRadius * 0.8), localAxes(angle), [size * 0.6, size, size * 0.8]),
    ]
    const lobes = rand.int(2, 4)
    for (let i = 0; i < lobes; i++) {
      const lobeY = y + rand.range(-0.7, 0.7) * size
      const lobeAngle = angle + (rand.range(-0.6, 0.6) * size) / trunkRadius
      const lobeSize = size * rand.range(0.35, 0.6)
      const center = frame.around(lobeY, lobeAngle, trunkRadius * 0.8 + size * 0.45)
      shapes.push(new Ellipsoid(center, localAxes(lobeAngle), [lobeSize * 0.7, lobeSize, lobeSize * 0.9]))
    }
    return { shapes, blend: size * 0.35 }
  })
}
