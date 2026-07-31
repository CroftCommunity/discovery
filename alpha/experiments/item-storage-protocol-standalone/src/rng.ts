// A seeded, deterministic pseudo-random generator. Every place the suite needs
// "randomness" (which items an audit samples, which items a provider secretly
// drops) draws from one of these, seeded from a string. Same seed in, same draws
// out — so a Monte Carlo experiment is exactly reproducible and its assertions
// can be tight rather than "usually passes".
//
// We deliberately do NOT use Math.random(): it cannot be seeded and would make
// runs non-reproducible.

import { createHash } from "node:crypto";

/** mulberry32 — a small, fast, well-distributed 32-bit PRNG. */
export class Rng {
  private state: number;

  constructor(seed: string) {
    // Fold a string seed down to 32 bits via SHA-256 so any label seeds cleanly.
    const digest = createHash("sha256").update(seed).digest();
    this.state = digest.readUInt32LE(0);
  }

  /** Next float in [0, 1). */
  next(): number {
    this.state |= 0;
    this.state = (this.state + 0x6d2b79f5) | 0;
    let t = Math.imul(this.state ^ (this.state >>> 15), 1 | this.state);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  }

  /** Integer in [0, n). */
  int(n: number): number {
    return Math.floor(this.next() * n);
  }

  /** Uniform sample of k distinct indices from [0, n) (partial Fisher-Yates). */
  sampleIndices(n: number, k: number): number[] {
    const count = Math.min(k, n);
    const pool = Array.from({ length: n }, (_, i) => i);
    for (let i = 0; i < count; i++) {
      const j = i + this.int(n - i);
      const tmp = pool[i];
      pool[i] = pool[j];
      pool[j] = tmp;
    }
    return pool.slice(0, count).sort((a, b) => a - b);
  }
}
