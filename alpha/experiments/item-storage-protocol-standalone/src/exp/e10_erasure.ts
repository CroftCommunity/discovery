// E10. Stretch: erasure-coded upgrade path — the road from spot checks to a real
// retrievability guarantee, shown but deliberately not made the default. We split
// one collection into n shares with k-of-n recovery, drop up to n-k shares and
// recover, verify the reconstructed items against the original manifest, and show
// (with the E5 math re-run) how coding changes the loss story. A tampered share is
// caught by its fingerprint while recovery still succeeds from the good shares.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { fingerprint } from "../crypto.ts";
import { encode, decode, codedLossProbability, type Share } from "../erasure.ts";
import { detectionProbability } from "../audit.ts";

const N = 6;
const K = 4;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { items, manifest } = world;

  // Serialize the collection into one blob, remembering each item's slice so we
  // can re-derive items after recovery and verify them against the manifest.
  const layout: { cid: string; offset: number; size: number }[] = [];
  let offset = 0;
  const parts: Buffer[] = [];
  for (const it of items) {
    layout.push({ cid: it.cid, offset, size: it.size });
    parts.push(it.bytes);
    offset += it.size;
  }
  const blob = Buffer.concat(parts);
  const originalFp = fingerprint(blob);

  // Encode into n shares; each share gets its own fingerprint (for share audits).
  const { shares, originalLength } = encode(blob, N, K);
  const shareFp = shares.map((s) => fingerprint(s.bytes));

  // Recovery at the threshold: drop exactly n-k shares (all such drop-sets).
  let thresholdOk = true;
  for (let a = 0; a < N; a++) {
    for (let b = a + 1; b < N; b++) {
      const avail = shares.filter((s) => s.index !== a && s.index !== b);
      const rec = decode(avail, N, K, originalLength);
      if (fingerprint(rec) !== originalFp) thresholdOk = false;
    }
  }
  c.ok(`recovery succeeds from any ${K} of ${N} shares (drop up to ${N - K})`, thresholdOk);

  // Recovery beyond the threshold: only k-1 shares -> impossible.
  let beyondFails = false;
  try {
    decode(shares.slice(0, K - 1), N, K, originalLength);
  } catch {
    beyondFails = true;
  }
  c.ok(`recovery fails beyond the threshold (${K - 1} shares)`, beyondFails);

  // Audits over coded shares still verify against the original manifest: recover,
  // re-derive items, and confirm each item's fingerprint matches the signed list.
  const recovered = decode(shares.slice(0, K), N, K, originalLength);
  let manifestVerified = true;
  const manifestCids = new Set(manifest!.leaves.map((l) => l.cid));
  for (const part of layout) {
    const slice = recovered.subarray(part.offset, part.offset + part.size);
    if (fingerprint(slice) !== part.cid || !manifestCids.has(part.cid)) manifestVerified = false;
  }
  c.ok("recovered items verify against the original manifest", manifestVerified);

  // A tampered share is caught by its fingerprint; recovery still succeeds from
  // the remaining good shares (coding + fingerprinting compose).
  const tampered: Share = { index: shares[0].index, bytes: Buffer.from(shares[0].bytes) };
  tampered.bytes[0] ^= 0x01;
  c.ok("tampered share is detected by its fingerprint",
    fingerprint(tampered.bytes) !== shareFp[0]);
  const goodShares = shares.filter((s) => s.index !== shares[0].index).slice(0, K);
  const recAfter = decode(goodShares, N, K, originalLength);
  c.ok("recovery still succeeds using only the good shares",
    fingerprint(recAfter) === originalFp);

  // Re-run the E5 math beside the redundancy math: for a per-item/per-share loss
  // rate p, uncoded loss is p; coded(n,k) loss is P(>n-k shares lost).
  const rows: (string | number)[][] = [];
  for (const p of [0.001, 0.01, 0.05, 0.2]) {
    // Detection is the spot-check story (single item, k audit draws would be the
    // dial); here we contrast the *loss* story: uncoded vs coded.
    const uncoded = p; // a single copy is lost with probability p
    const coded = codedLossProbability(p, N, K);
    rows.push([
      p,
      uncoded.toFixed(4),
      coded.toExponential(3),
      detectionProbability(p, K).toFixed(4),
    ]);
  }

  return {
    id: "E10",
    title: "Stretch: erasure-coded upgrade path",
    plainSentence: "Erasure-coded retrievability is the upgrade path if spot checks ever are not enough.",
    assertions: c.results,
    tables: [
      {
        title: `Loss story: uncoded vs ${K}-of-${N} coding (detection math beside it)`,
        headers: ["p (per-share loss)", "uncoded loss", "coded loss", "detect@k=" + K],
        rows,
      },
    ],
    notes: [
      `Scope note: any tier marketed as "archive" ships with a labeled redundancy floor ` +
        `(e.g. ${K}-of-${N} coding or 3 copies); E10 is a stretch for the general tiers only.`,
    ],
  };
}
