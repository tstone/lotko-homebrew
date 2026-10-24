/** Deterministic PRNG (mulberry32) so a given seed always produces the same tree. */
export function createRandom(seed: number) {
  let a = seed >>> 0
  const next = () => {
    a = (a + 0x6d2b79f5) >>> 0
    let t = a
    t = Math.imul(t ^ (t >>> 15), t | 1)
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
  return {
    next,
    range: (min: number, max: number) => min + (max - min) * next(),
    int: (min: number, max: number) => Math.floor(min + (max - min + 1) * next()),
  }
}

export type Random = ReturnType<typeof createRandom>

/**
 * A random stream for one named purpose. Separate streams keep features independent: adding a
 * burl doesn't reshuffle the roots.
 */
export function createNamedRandom(seed: number, name: string) {
  let h = seed >>> 0
  for (let i = 0; i < name.length; i++) h = Math.imul(h ^ name.charCodeAt(i), 0x9e3779b1) >>> 0
  return createRandom(h)
}
