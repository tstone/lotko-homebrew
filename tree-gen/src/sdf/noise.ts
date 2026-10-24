import { createRandom } from '../util/random'

/** Seeded 3D value noise in [-1, 1], smoothly interpolated between integer lattice points. */
export function createNoise3(seed: number) {
  const rand = createRandom(seed)
  const perm = new Uint8Array(256)
  for (let i = 0; i < 256; i++) perm[i] = i
  for (let i = 255; i > 0; i--) {
    const j = Math.floor(rand.next() * (i + 1))
    ;[perm[i], perm[j]] = [perm[j], perm[i]]
  }
  const values = new Float32Array(256)
  for (let i = 0; i < 256; i++) values[i] = rand.next() * 2 - 1

  const lattice = (x: number, y: number, z: number) =>
    values[perm[(perm[(perm[x & 255] + y) & 255] + z) & 255]]

  return (x: number, y: number, z: number) => {
    const ix = Math.floor(x)
    const iy = Math.floor(y)
    const iz = Math.floor(z)
    const u = fade(x - ix)
    const v = fade(y - iy)
    const w = fade(z - iz)
    const x00 = lerp(lattice(ix, iy, iz), lattice(ix + 1, iy, iz), u)
    const x10 = lerp(lattice(ix, iy + 1, iz), lattice(ix + 1, iy + 1, iz), u)
    const x01 = lerp(lattice(ix, iy, iz + 1), lattice(ix + 1, iy, iz + 1), u)
    const x11 = lerp(lattice(ix, iy + 1, iz + 1), lattice(ix + 1, iy + 1, iz + 1), u)
    return lerp(lerp(x00, x10, v), lerp(x01, x11, v), w)
  }
}

export type Noise3 = ReturnType<typeof createNoise3>

/** Fractal noise: `octaves` layers, each at double the frequency and half the amplitude. Stays in [-1, 1]. */
export function fbm(noise: Noise3, x: number, y: number, z: number, octaves: number) {
  let sum = 0
  let amp = 1
  let norm = 0
  let freq = 1
  for (let i = 0; i < octaves; i++) {
    // Offset each octave so their lattices don't line up.
    sum += amp * noise(x * freq + i * 17.1, y * freq + i * 5.3, z * freq + i * 11.7)
    norm += amp
    amp *= 0.5
    freq *= 2
  }
  return sum / norm
}

function fade(t: number) {
  return t * t * t * (t * (t * 6 - 15) + 10)
}

function lerp(a: number, b: number, t: number) {
  return a + (b - a) * t
}
