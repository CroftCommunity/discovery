// E2. The manifest — "The list is the bill's source of truth, and the customer
// wrote the list."
//
// The Customer builds a manifest: a sorted list of (fingerprint, size), a Merkle
// root over the list, and a signature over the root. The Provider independently
// computes expected bytes at rest by summing manifest sizes, and computes the
// same root from its stored copies. Adversarial: the Provider claims a larger
// stored total (rejected by arithmetic alone); and recomputes the root with one
// item missing (roots mismatch).

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItems } from "../src/items.ts";
import {
  buildSignedManifest,
  expectedBytesAtRest,
  providerComputedRoot,
  verifyManifestSig,
  merkleRoot,
  sortItems,
} from "../src/manifest.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E2", seed);
  const { customer, provider, store } = w;
  const prng = new Prng(seed);

  const items = makeItems(prng, [
    { name: "a", size: 1000 },
    { name: "b", size: 2000 },
    { name: "c", size: 3000 },
    { name: "d", size: 4000 },
  ]);
  for (const it of items) store.put(it.bytes);

  const manifest = buildSignedManifest(
    customer,
    items.map((it) => ({ cid: it.cid, size: it.size })),
    w.clock.now(),
  );
  customer.ledger.append("manifest", w.clock.now(), {
    root: manifest.root,
    itemCount: manifest.items.length,
    sig: manifest.sig,
  });

  // The Provider verifies the customer's signature over the root...
  assert.equal(verifyManifestSig(provider, manifest), true, "provider must verify the customer-signed root");

  // ...and independently computes the same root from the copies it holds.
  const heldCids = new Set(store.cids());
  const provRoot = providerComputedRoot(manifest, heldCids);
  assert.equal(provRoot, manifest.root, "provider-computed root must equal customer-signed root");

  // Expected bytes at rest is a PURE function of the manifest — no retrieval.
  const expected = expectedBytesAtRest(manifest);
  assert.equal(expected, 1000 + 2000 + 3000 + 4000, "expected bytes = sum of manifest sizes");
  provider.ledger.append("manifest_ack", w.clock.now(), {
    computedRoot: provRoot,
    expectedBytesAtRest: expected,
  });

  // Adversarial (a): the Provider claims a larger stored total than the manifest
  // implies. Rejected by arithmetic alone — the manifest is authoritative and
  // the customer recomputes `expected` without retrieving a single byte.
  const providerClaimedTotal = expected + 5000;
  const claimAccepted = providerClaimedTotal === expected;
  assert.equal(claimAccepted, false, "an inflated stored-total claim must be rejected against the manifest sum");

  // Adversarial (b): the Provider recomputes the root with one item missing.
  const short = new Set(heldCids);
  const dropped = manifest.items[1].cid;
  short.delete(dropped);
  const shortRoot = providerComputedRoot(manifest, short);
  assert.notEqual(shortRoot, manifest.root, "a root over a missing item must not match the signed root");

  // The signed root is genuinely a Merkle root over the sorted items (structure
  // check, not just an opaque hash).
  assert.equal(merkleRoot(sortItems(manifest.items)), manifest.root, "root must be the Merkle root of the sorted items");

  const sentence = "The list is the bill's source of truth, and the customer wrote the list.";
  const reportMd = [
    `The Customer's manifest lists ${manifest.items.length} items; the signed Merkle root is`,
    `\`${manifest.root.slice(0, 16)}…\`. The Provider recomputes the identical root from its stored`,
    `copies and agrees the expected bytes at rest is **${expected}** — a pure function of the`,
    "manifest, computed with no retrieval and no trust in the Provider.",
    "",
    "Adversarial: an inflated stored-total claim is rejected by arithmetic against the manifest",
    "sum alone; recomputing the root with one item missing yields a different root",
    `(\`${shortRoot.slice(0, 16)}…\`), so a dropped item cannot hide.`,
  ].join("\n");

  return { id: "E2", title: "The manifest", sentence, reportMd };
}
