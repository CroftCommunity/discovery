// The public randomness source, and the audit-challenge derivation on top of it.
//
// Spot-check audits (E5/E6) sampled from the private PRNG. For a third party to
// confirm "service was delivered" WITHOUT trusting the co-op (E12), the audit
// challenges must come from a source the outsider can reproduce: a public
// randomness beacon. Each round publishes a value; the items challenged that
// round are a pure, published function of the beacon value and the customer's
// signed manifest. Anyone holding the beacon and the manifest can recompute
// exactly which items should have been checked and confirm the transcript.
//
// SEAM: production would use a real public randomness beacon (e.g. drand / a
// verifiable delay function). Here a beacon value is sha256(publicSeed|round) —
// public and reproducible, but not unpredictable-in-advance the way a real
// beacon is. The property E12 needs (an outsider can recompute the challenge) is
// identical; only the unpredictability guarantee is mocked.
//
// The derivation below is deliberately simple and self-contained so the E12
// Funder can reimplement it from the spec with no shared code (funder/verifier).

import { createHash } from "node:crypto";

/** The published beacon value for a round: sha256(publicSeed | round), hex. */
export function beaconValue(publicSeedHex: string, round: number): string {
  return createHash("sha256").update(`${publicSeedHex}|beacon|${round}`).digest("hex");
}

/**
 * Derive the k challenged indices in [0, n) from a beacon value, WITHOUT
 * replacement, deterministically. Expansion: for counter i = 0, 1, 2, …, take
 * idx = sha256(beacon|":"|i) mod n (over the first 15 hex digits, < 2^60, safe
 * in a JS number), skipping indices already chosen, until k distinct are found.
 * Returns them sorted ascending.
 */
export function deriveChallengeIndices(beaconHex: string, n: number, k: number): number[] {
  const take = Math.min(k, n);
  const chosen: number[] = [];
  const seen = new Set<number>();
  let i = 0;
  while (chosen.length < take) {
    const d = createHash("sha256").update(`${beaconHex}:${i}`).digest("hex").slice(0, 15);
    const idx = Number(BigInt("0x" + d) % BigInt(n));
    if (!seen.has(idx)) {
      seen.add(idx);
      chosen.push(idx);
    }
    i++;
  }
  return chosen.sort((a, b) => a - b);
}
