import { createNoise3, fbm } from '../../sdf/noise'
import type { Displacement } from '../../sdf/scene'
import { smoothstep } from '../../util/math'
import { createNamedRandom } from '../../util/random'
import type { TrunkFrame } from './frame'

/**
 * Large-scale lumpiness over the whole surface. The noise is stretched vertically so lumps run with
 * the grain, and twisted with height so they follow the spiral of the root ridges.
 */
export function gnarl(frame: TrunkFrame): Displacement | undefined {
  const { params, height, baseRadius } = frame
  if (params.gnarl <= 0) return undefined
  const noise = createNoise3(createNamedRandom(params.seed, 'gnarl').int(0, 2 ** 31))
  const amplitude = params.gnarl * baseRadius * 0.2
  const frequency = 1.1 / baseRadius
  const twist = params.twist * Math.PI * 2

  return {
    amplitude,
    fn(x, y, z) {
      const a = (-twist * y) / height
      const cos = Math.cos(a)
      const sin = Math.sin(a)
      const rx = x * cos - z * sin
      const rz = x * sin + z * cos
      // Calmer near the ground, where the thin ends of the roots would otherwise break up.
      const calm = 0.35 + 0.65 * smoothstep(0, baseRadius * 0.8, y)
      return amplitude * calm * fbm(noise, rx * frequency, y * frequency * 0.3, rz * frequency, 4)
    },
  }
}
