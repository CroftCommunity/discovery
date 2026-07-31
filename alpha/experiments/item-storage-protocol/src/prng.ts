// Deterministic, seedable pseudo-random generator.
//
// Determinism is a ground rule of this suite (Part 2): every experiment run must
// be byte-for-byte reproducible, so nothing may touch Math.random() or any OS
// entropy source. All randomness — item contents, audit sampling, Monte Carlo
// trials, which byte an adversary flips — flows from a seed through this class.
//
// Algorithm: mulberry32, a small fast 32-bit generator. It is NOT
// cryptographically secure and is not used for anything security-bearing; keys
// and signatures come from Node's crypto (see crypto.ts). This is only for
// reproducible experiment scenarios.

export class Prng {
  private state: number;

  constructor(seed: number) {
    // Force to uint32.
    this.state = seed >>> 0;
  }

  /** Next float in [0, 1). */
  float(): number {
    // mulberry32
    this.state = (this.state + 0x6d2b79f5) >>> 0;
    let t = this.state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  }

  /** Integer in [0, n). */
  int(n: number): number {
    return Math.floor(this.float() * n);
  }

  /** True with probability p. */
  bool(p: number): boolean {
    return this.float() < p;
  }

  /** Uniform pick from an array. */
  pick<T>(items: readonly T[]): T {
    return items[this.int(items.length)];
  }

  /**
   * k distinct indices in [0, n) chosen uniformly WITHOUT replacement
   * (partial Fisher–Yates). Used by the honest spot-check audit, which never
   * checks the same item twice. If k >= n, returns all indices.
   */
  sampleIndices(n: number, k: number): number[] {
    const idx = Array.from({ length: n }, (_, i) => i);
    const take = Math.min(k, n);
    for (let i = 0; i < take; i++) {
      const j = i + this.int(n - i);
      [idx[i], idx[j]] = [idx[j], idx[i]];
    }
    return idx.slice(0, take);
  }

  /**
   * k indices in [0, n) chosen uniformly WITH replacement. Used by the E5 Monte
   * Carlo so measured detection lines up with the closed form 1-(1-f)^k, whose
   * derivation assumes k independent draws.
   */
  sampleWithReplacement(n: number, k: number): number[] {
    const out: number[] = [];
    for (let i = 0; i < k; i++) out.push(this.int(n));
    return out;
  }

  /** Deterministic bytes of a given length. */
  bytes(length: number): Buffer {
    const b = Buffer.alloc(length);
    for (let i = 0; i < length; i++) b[i] = this.int(256);
    return b;
  }
}
