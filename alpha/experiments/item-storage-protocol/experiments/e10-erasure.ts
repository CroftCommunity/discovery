// E10. Stretch: erasure-coded upgrade path — "Erasure-coded retrievability is
// the upgrade path if spot checks ever are not enough."
//
// Split a collection's items into n shares with k-of-n recovery. Drop up to
// n-k shares; recover; verify fingerprints. Then re-run the E5 loss math to show
// how coding changes the story: an item is lost only if MORE than n-k of its n
// shares are lost — a binomial tail far below the uncoded single-copy loss rate.
//
// Assertions: recovery succeeds at the threshold and fails beyond it; audits over
// coded shares still verify against the original manifest.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItem, type NamedItem } from "../src/items.ts";
import { buildSignedManifest } from "../src/manifest.ts";
import { computeCid } from "../src/store.ts";
import { rsEncode, rsDecode, type Share } from "../src/reedsolomon.ts";

function choose(n: number, r: number): number {
  let c = 1;
  for (let i = 0; i < r; i++) c = (c * (n - i)) / (i + 1);
  return Math.round(c);
}

/** P(item lost) = P(more than n-k of n shares lost), each share lost w.p. p. */
function codedLossProb(n: number, k: number, p: number): number {
  let prob = 0;
  for (let j = n - k + 1; j <= n; j++) {
    prob += choose(n, j) * Math.pow(p, j) * Math.pow(1 - p, n - j);
  }
  return prob;
}

export function run(seed: number): ExperimentResult {
  const w = createWorld("E10", seed);
  const { customer, store } = w;
  const prng = new Prng(seed);

  const K = 4;
  const N = 6; // tolerate loss of up to n-k = 2 shares

  // A small collection; the customer's manifest is over the ORIGINAL items.
  const items: NamedItem[] = [];
  for (let i = 0; i < 4; i++) {
    const it = makeItem(prng, `coded-${i}`, 3000 + i * 700);
    store.put(it.bytes);
    items.push(it);
  }
  const manifest = buildSignedManifest(customer, items.map((it) => ({ cid: it.cid, size: it.size })), 0);

  // Encode each item into n shares (any k recover).
  const encoded = items.map((it) => ({ item: it, enc: rsEncode(it.bytes, K, N) }));
  customer.ledger.append("erasure_declare", w.clock.now(), { k: K, n: N, items: items.length });

  // Recovery succeeds at the threshold (drop exactly n-k = 2 shares) for every
  // item, and the recovered bytes re-fingerprint to the manifest cid.
  for (const { item, enc } of encoded) {
    const survivors: Share[] = enc.shares.slice(0, K); // keep exactly k, drop n-k
    const recovered = rsDecode(survivors, K, N, enc.length);
    assert.equal(Buffer.compare(recovered, item.bytes), 0, `item ${item.name} recovers exactly at the threshold`);
    assert.equal(computeCid(recovered), item.cid, `recovered item ${item.name} matches its manifest fingerprint`);
  }

  // A scattered k-subset (non-contiguous share indices) also recovers.
  {
    const { item, enc } = encoded[0];
    const scattered = [enc.shares[1], enc.shares[2], enc.shares[4], enc.shares[5]];
    const recovered = rsDecode(scattered, K, N, enc.length);
    assert.equal(computeCid(recovered), item.cid, "a scattered k-of-n subset recovers");
  }

  // Recovery FAILS beyond the threshold: with only k-1 shares, decode refuses.
  {
    const { enc } = encoded[0];
    assert.throws(
      () => rsDecode(enc.shares.slice(0, K - 1), K, N, enc.length),
      /need at least k/,
      "recovery must fail beyond the n-k loss threshold",
    );
  }

  // Audits over coded shares still verify against the ORIGINAL manifest: recover
  // from k shares, fingerprint, compare to the signed manifest.
  const manifestCids = new Set(manifest.items.map((it) => it.cid));
  for (const { enc, item } of encoded) {
    const recovered = rsDecode(enc.shares.slice(2, 2 + K), K, N, enc.length);
    assert.ok(manifestCids.has(computeCid(recovered)), `coded audit of ${item.name} verifies against the manifest`);
  }

  // Re-run the loss math: coded item-loss (binomial tail) is far below the
  // uncoded single-copy loss rate p, for the same per-share loss probability.
  const ps = [0.01, 0.05, 0.1, 0.2];
  const lossRows = ps.map((p) => ({ p, uncoded: p, coded: codedLossProb(N, K, p) }));
  for (const row of lossRows) {
    assert.ok(row.coded < row.uncoded, `coded loss must beat uncoded at p=${row.p}`);
  }

  const sentence = "Erasure-coded retrievability is the upgrade path if spot checks ever are not enough.";
  const table = [
    "| per-share loss p | uncoded item-loss | coded item-loss (k=4, n=6) |",
    "| ---: | ---: | ---: |",
    ...lossRows.map((r) => `| ${r.p} | ${r.uncoded.toFixed(4)} | ${r.coded.toExponential(2)} |`),
  ].join("\n");
  const reportMd = [
    `Each item was split into n=${N} shares with k=${K}-of-${N} recovery. Every item recovered`,
    "exactly at the loss threshold (drop 2 shares) and from scattered subsets, re-fingerprinting",
    "to its manifest cid; recovery failed beyond the threshold. Audits over coded shares still",
    "verify against the original manifest.",
    "",
    "Coding changes the loss story — an item is lost only if more than n-k shares are lost:",
    "",
    table,
    "",
    "This is the door left open: probabilistic spot checks upgrade to deterministic",
    "retrievability below the loss threshold, without changing the manifest the customer signed.",
  ].join("\n");

  return { id: "E10", title: "Erasure-coded upgrade path", sentence, reportMd };
}
