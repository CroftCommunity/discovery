// E5. Spot checks and the detection math — "Spot checks are probabilistic, and
// we know exactly how probabilistic."
//
// An audit picks k items at random, retrieves, fingerprints, verifies. A Monte
// Carlo sweep drops a fraction f of items and measures the detection rate of a
// k-item audit, comparing it against the closed form P = 1-(1-f)^k over f in
// {0.001, 0.01, 0.05, 0.2} and k in {1, 5, 20, 100}. An honest provider passes
// all audits; audit cost (bytes retrieved) scales with k and item sizes,
// independent of total corpus size.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItem, type NamedItem } from "../src/items.ts";
import { buildSignedManifest } from "../src/manifest.ts";
import { audit, monteCarloDetection, type MonteCarloRow } from "../src/audit.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E5", seed);
  const { customer, store } = w;
  const prng = new Prng(seed);

  // A corpus of uniform-size items, so audit cost is exactly k * itemSize.
  const ITEM_SIZE = 1024;
  const N = 200;
  const items: NamedItem[] = [];
  for (let i = 0; i < N; i++) {
    const it = makeItem(prng, `item-${i}`, ITEM_SIZE);
    store.put(it.bytes);
    items.push(it);
  }
  const manifest = buildSignedManifest(customer, items.map((it) => ({ cid: it.cid, size: it.size })), 0);

  // An honest provider passes every audit, at every k.
  for (const k of [1, 5, 20, 100]) {
    const r = audit(manifest, store, k, prng);
    assert.equal(r.passed, true, `honest provider must pass a k=${k} audit`);
    assert.equal(r.failures.length, 0);
    assert.equal(r.bytesRetrieved, k * ITEM_SIZE, `audit cost must be exactly k*itemSize for k=${k}`);
  }

  // Audit cost is independent of corpus size: a second, larger corpus with the
  // same item size and same k costs the same bytes.
  const w2 = createWorld("E5-big", seed + 1);
  const prng2 = new Prng(seed + 1);
  const bigN = 5000;
  const bigItems: NamedItem[] = [];
  for (let i = 0; i < bigN; i++) {
    const it = makeItem(prng2, `big-${i}`, ITEM_SIZE);
    w2.store.put(it.bytes);
    bigItems.push(it);
  }
  const bigManifest = buildSignedManifest(w2.customer, bigItems.map((it) => ({ cid: it.cid, size: it.size })), 0);
  const kFixed = 20;
  const smallCost = audit(manifest, store, kFixed, new Prng(99)).bytesRetrieved;
  const bigCost = audit(bigManifest, w2.store, kFixed, new Prng(99)).bytesRetrieved;
  assert.equal(smallCost, bigCost, "audit cost must not depend on corpus size (200 vs 5000 items)");
  assert.equal(smallCost, kFixed * ITEM_SIZE);

  // Monte Carlo sweep. N=1000 makes round(f*N)/N == f exactly for these f.
  const MC_N = 1000;
  const TRIALS = 5000;
  const TOLERANCE = 0.035; // comfortably above Monte Carlo noise at 5000 trials
  const fs = [0.001, 0.01, 0.05, 0.2];
  const ks = [1, 5, 20, 100];
  const rows: MonteCarloRow[] = [];
  const mcPrng = new Prng(seed ^ 0x5eed);
  for (const f of fs) {
    for (const k of ks) {
      const row = monteCarloDetection(MC_N, f, k, TRIALS, mcPrng);
      rows.push(row);
      assert.ok(
        row.absError <= TOLERANCE,
        `f=${f} k=${k}: measured ${row.measured.toFixed(4)} vs predicted ${row.predicted.toFixed(4)} (|err| ${row.absError.toFixed(4)} > ${TOLERANCE})`,
      );
    }
  }

  // A provider that actually drops items is caught: drop 5% and audit k=100.
  const dropCount = Math.round(0.05 * N);
  for (let i = 0; i < dropCount; i++) store.delete(items[i].cid);
  const caught = audit(manifest, store, 100, new Prng(7));
  assert.equal(caught.passed, false, "a provider dropping 5% must be caught by a k=100 audit");
  assert.ok(caught.failures.every((fl) => fl.reason === "missing"), "dropped items report as missing");

  const table = [
    "| f (fraction dropped) | k=1 | k=5 | k=20 | k=100 |",
    "| --- | --- | --- | --- | --- |",
    ...fs.map((f) => {
      const cells = ks.map((k) => {
        const r = rows.find((x) => x.f === f && x.k === k)!;
        return `${r.measured.toFixed(3)} / ${r.predicted.toFixed(3)}`;
      });
      return `| ${f} | ${cells.join(" | ")} |`;
    }),
  ].join("\n");

  const sentence = "Spot checks are probabilistic, and we know exactly how probabilistic.";
  const reportMd = [
    "Measured vs. predicted detection probability (measured / predicted = 1-(1-f)^k), from",
    `${TRIALS} Monte Carlo trials per cell; all cells within ${TOLERANCE} of the closed form.`,
    "",
    table,
    "",
    `An honest provider passed every audit (k = 1, 5, 20, 100). Audit cost was exactly`,
    `k * ${ITEM_SIZE} bytes and did not change between a 200-item and a 5000-item corpus. A`,
    "provider that dropped 5% of items was caught by a k=100 audit.",
  ].join("\n");

  return { id: "E5", title: "Spot checks and the detection math", sentence, reportMd };
}
