// The audit (spot check): pick k items uniformly at random from the manifest,
// retrieve them, re-fingerprint, and verify against the signed list. Its power is
// quantifiable: if a provider has silently dropped a fraction f of items, a
// k-item audit detects it with probability 1 - (1 - f)^k. Checking costs real work
// (bytes retrieved), and that cost scales with k and item sizes, not with the size
// of the whole corpus — which is what makes the assurance dial cheap and honest.

import { BlobStore } from "./item.ts";
import type { ManifestLeaf } from "./manifest.ts";
import type { Rng } from "./rng.ts";

/** Closed form: probability a k-item audit detects a fraction-f loss. */
export function detectionProbability(f: number, k: number): number {
  return 1 - Math.pow(1 - f, k);
}

export type AuditOutcome = {
  sampled: string[];
  passed: boolean;
  bytesRead: number;
  failures: string[];
};

/** Run one k-item audit against a store, verifying each sampled item's bytes. */
export function auditSample(
  leaves: ManifestLeaf[],
  store: BlobStore,
  rng: Rng,
  k: number,
): AuditOutcome {
  const idxs = rng.sampleIndices(leaves.length, k);
  const sampled = idxs.map((i) => leaves[i].cid);
  const failures: string[] = [];
  for (const cid of sampled) {
    const r = store.retrieveVerified(cid);
    if (!r.ok) failures.push(cid);
  }
  return {
    sampled,
    passed: failures.length === 0,
    bytesRead: store.auditReadCost(sampled),
    failures,
  };
}
