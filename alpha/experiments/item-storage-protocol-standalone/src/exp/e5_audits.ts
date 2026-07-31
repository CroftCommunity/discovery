// E5. Spot checks and the detection math — audits are cheap, and their power is
// quantifiable. An honest provider passes every audit. Then, over a large
// synthetic corpus, the provider secretly drops a fraction f of items; we run many
// seeded k-item audits and measure the detection rate, comparing it to the closed
// form 1 - (1 - f)^k across a sweep of f and k. Finally we show audit cost scales
// with k and item sizes, not with the size of the corpus.

import type { World, AuditRecord } from "../world.ts";
import { Checker, type ExperimentResult, type ReportTable } from "../types.ts";
import { BlobStore, makeItem } from "../item.ts";
import type { ManifestLeaf } from "../manifest.ts";
import type { Rng } from "../rng.ts";
import { auditSample, detectionProbability } from "../audit.ts";

const F_SWEEP = [0.001, 0.01, 0.05, 0.2];
const K_SWEEP = [1, 5, 20, 100];
const TRIALS = 5000;
const N = 5000; // synthetic corpus size (large, so hypergeometric ~ binomial)
const ITEM_SIZE = 256;
const TOLERANCE = 0.03;

/** Sample k distinct indices in [0,N); return true if any lands in `dropped`. */
function sampleHitsDropped(n: number, k: number, dropped: Set<number>, rng: Rng): boolean {
  const seen = new Set<number>();
  const count = Math.min(k, n);
  while (seen.size < count) {
    const idx = rng.int(n);
    if (seen.has(idx)) continue;
    seen.add(idx);
    if (dropped.has(idx)) return true; // a dropped item was sampled: detected
  }
  return false;
}

export function run(world: World): ExperimentResult {
  const c = new Checker();

  // 1) Honest provider: audits over the real (current) manifest always pass.
  const realLeaves = world.manifest!.leaves;
  const honestRng = world.rng("e5/honest");
  let honestPass = true;
  for (let t = 0; t < 50; t++) {
    const k = 1 + honestRng.int(realLeaves.length);
    if (!auditSample(realLeaves, world.store, honestRng, k).passed) honestPass = false;
  }
  c.ok("an honest provider passes every audit", honestPass);

  // Record one honest audit in the ledgers (proof-of-presence), for the narrative.
  const shown = auditSample(realLeaves, world.store, world.rng("e5/ledger-audit"), 3);
  const auditRec: AuditRecord = {
    day: world.clock.now(), k: 3, bytesRead: shown.bytesRead, passed: shown.passed,
  };
  world.audits.push(auditRec);
  world.customer.ledger.append("audit", world.clock.iso(), {
    k: 3, sampled: shown.sampled, passed: shown.passed, bytesRead: shown.bytesRead,
  }, [world.customer.signer()]);

  // 2) Monte Carlo detection sweep. A silently DROPPED item is detected simply by
  // its absence when sampled, so the sweep models detection as set membership over
  // an N-item corpus with round(f*N) items dropped once — no per-trial hashing
  // needed (the honest-audit and cost checks above exercise real retrieval).
  const rows: (string | number)[][] = [];
  let allWithinTolerance = true;
  let maxErr = 0;
  for (const f of F_SWEEP) {
    const m = Math.round(f * N);
    const dropIdx = new Set(world.rng(`e5/drop/${f}`).sampleIndices(N, m));
    for (const k of K_SWEEP) {
      const trialRng = world.rng(`e5/trials/${f}/${k}`);
      let detected = 0;
      for (let t = 0; t < TRIALS; t++) {
        if (sampleHitsDropped(N, k, dropIdx, trialRng)) detected++;
      }
      const measured = detected / TRIALS;
      const predicted = detectionProbability(f, k);
      const err = Math.abs(measured - predicted);
      maxErr = Math.max(maxErr, err);
      if (err > TOLERANCE) allWithinTolerance = false;
      rows.push([f, k, predicted.toFixed(4), measured.toFixed(4), err.toFixed(4)]);
    }
  }
  c.ok(`measured detection matches 1-(1-f)^k within ${TOLERANCE} (max err ${maxErr.toFixed(4)})`,
    allWithinTolerance);

  // 3) Audit cost scales with k and item sizes, independent of corpus size.
  const small = buildSameSizeCorpus(world, 500, ITEM_SIZE, "e5/small");
  const big = buildSameSizeCorpus(world, 5000, ITEM_SIZE, "e5/big");
  const costSmall = auditSample(small.leaves, small.store, world.rng("e5/cost-a"), 10).bytesRead;
  const costBig = auditSample(big.leaves, big.store, world.rng("e5/cost-b"), 10).bytesRead;
  c.eq("audit cost is independent of corpus size (same k, same item size)", costSmall, costBig);
  c.eq("audit cost equals k * item size", costBig, 10 * ITEM_SIZE);

  const table: ReportTable = {
    title: "E5 detection: measured vs predicted (1-(1-f)^k)",
    headers: ["f", "k", "predicted", "measured", "abs err"],
    rows,
  };

  return {
    id: "E5",
    title: "Spot checks and the detection math",
    plainSentence: "Spot checks are probabilistic, and we know exactly how probabilistic.",
    assertions: c.results,
    tables: [table],
    notes: [`Monte Carlo: ${TRIALS} trials per cell, corpus N=${N}, tolerance ${TOLERANCE}.`],
  };
}

function buildSameSizeCorpus(world: World, n: number, size: number, seed: string) {
  const rng = world.rng(seed);
  const store = new BlobStore();
  const leaves: ManifestLeaf[] = [];
  for (let i = 0; i < n; i++) {
    const bytes = Buffer.alloc(size);
    bytes.writeUInt32LE(i, 0);
    for (let j = 4; j < size; j++) bytes[j] = rng.int(256);
    const item = makeItem(`s-${i}`, bytes);
    store.put(item);
    leaves.push({ cid: item.cid, size });
  }
  return { leaves, store };
}
