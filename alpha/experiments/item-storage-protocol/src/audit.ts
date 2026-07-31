// Spot checks and the detection math.
//
// An audit picks k items at random from the manifest, retrieves the bytes,
// re-fingerprints, and compares against the signed manifest. Its power is
// quantifiable: if a provider has silently dropped or tampered a fraction f of
// items, one k-item audit detects it with probability 1-(1-f)^k. E5 confirms the
// measured detection rate against this closed form by Monte Carlo.
//
// Audit cost is bytes retrieved: it scales with k and with item sizes, and is
// independent of the total corpus size.

import type { Prng } from "./prng.ts";
import type { Store } from "./store.ts";
import type { SignedManifest } from "./manifest.ts";
import { computeCid } from "./store.ts";

export interface AuditFailure {
  cid: string;
  reason: "missing" | "hash-mismatch";
}

export interface AuditResult {
  k: number;
  checkedCids: string[];
  failures: AuditFailure[];
  bytesRetrieved: number;
  passed: boolean;
}

/**
 * Honest audit against the signed manifest. Samples k items WITHOUT replacement
 * (you never spend a check on the same item twice), retrieves each, and
 * verifies the bytes fingerprint to the manifest cid.
 */
export function audit(
  manifest: SignedManifest,
  store: Store,
  k: number,
  prng: Prng,
): AuditResult {
  const items = manifest.items;
  const idxs = prng.sampleIndices(items.length, k);
  const checkedCids: string[] = [];
  const failures: AuditFailure[] = [];
  let bytesRetrieved = 0;

  for (const i of idxs) {
    const { cid } = items[i];
    checkedCids.push(cid);
    const raw = store.getRaw(cid);
    if (raw === undefined) {
      failures.push({ cid, reason: "missing" });
      continue;
    }
    bytesRetrieved += raw.length; // bytes actually moved off the store
    if (computeCid(raw) !== cid) {
      failures.push({ cid, reason: "hash-mismatch" });
    }
  }

  return { k, checkedCids, failures, bytesRetrieved, passed: failures.length === 0 };
}

export interface MonteCarloRow {
  f: number;
  k: number;
  trials: number;
  measured: number;
  predicted: number;
  absError: number;
}

/**
 * Monte Carlo of detection probability.
 *
 * Model: a corpus of N items with exactly round(f*N) of them dropped. Each trial
 * runs a k-draw audit WITH replacement (the draws the closed form assumes), and
 * detects if any draw lands on a dropped item. With N chosen so round(f*N)/N == f
 * exactly, the per-trial detection expectation is exactly 1-(1-f)^k, and the
 * measured rate converges to it.
 *
 * SEAM: the real audit samples without replacement; with-replacement here keeps
 * the demonstration aligned to the textbook closed form. For small f and large N
 * the two are indistinguishable within Monte Carlo tolerance.
 */
export function monteCarloDetection(
  N: number,
  f: number,
  k: number,
  trials: number,
  prng: Prng,
): MonteCarloRow {
  const droppedCount = Math.round(f * N);
  // Items [0, droppedCount) are the dropped ones (deterministic, order-agnostic
  // because draws are uniform).
  let detections = 0;
  for (let t = 0; t < trials; t++) {
    const draws = prng.sampleWithReplacement(N, k);
    if (draws.some((idx) => idx < droppedCount)) detections++;
  }
  const measured = detections / trials;
  const predicted = 1 - Math.pow(1 - f, k);
  return { f, k, trials, measured, predicted, absError: Math.abs(measured - predicted) };
}
