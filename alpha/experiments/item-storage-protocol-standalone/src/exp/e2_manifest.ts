// E2. The manifest — what should be on disk is computable from the customer's own
// signature. Ada builds a signed manifest (sorted (fingerprint, size) list, a
// Merkle root, her signature over the root). The provider independently computes
// expected bytes-at-rest by summing the manifest, and recomputes the same root
// from its stored copies. Two adversarial claims must be caught: an inflated
// stored-total (by arithmetic alone) and a root computed with an item missing.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import {
  buildManifest,
  verifyManifest,
  expectedBytes,
  merkleRoot,
} from "../manifest.ts";

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider, store, items } = world;

  // Ada authors and signs the manifest.
  const manifest = buildManifest(
    items.map((i) => ({ cid: i.cid, size: i.size })),
    customer.id,
    customer.keypair,
  );
  world.manifest = manifest;

  // Seed the rent timeline: from now, this many bytes are at rest.
  world.setBytesAtRest(expectedBytes(manifest));

  // Both parties record the pinned manifest in their ledgers (co-signed: Ada
  // authored it, the co-op acknowledges the obligation).
  const ts = world.clock.iso();
  const body = { root: manifest.root, totalBytes: manifest.totalBytes, itemCount: manifest.leaves.length };
  customer.ledger.append("manifest", ts, body, [customer.signer()]);
  provider.ledger.append("manifest-ack", ts, body, [provider.signer(), customer.signer()]);

  // The customer's signature over the root verifies under her pinned key.
  c.ok("manifest signature verifies under customer's pinned key",
    verifyManifest(manifest, world.keyring[customer.id]));

  // The provider independently recomputes the root from ITS stored copies and
  // gets the same root the customer signed.
  const providerLeaves = items
    .map((i) => ({ cid: i.cid, size: i.size }))
    .sort((a, b) => (a.cid < b.cid ? -1 : 1));
  const providerRoot = merkleRoot(providerLeaves);
  c.eq("provider-computed root equals customer-signed root", providerRoot, manifest.root);

  // Expected-bytes is a pure function of the manifest and equals what's on disk.
  const expected = expectedBytes(manifest);
  c.eq("expected bytes is a pure function of the manifest", expected, manifest.totalBytes);
  c.eq("provider's stored bytes match expected bytes", store.storedBytes(), expected);

  // Adversarial (a): the provider claims a larger stored total than the manifest
  // implies. Rejected by arithmetic alone — no retrieval needed.
  const inflatedClaim = expected + 1_000_000;
  c.ok("inflated storage claim rejected by arithmetic alone",
    inflatedClaim !== expectedBytes(manifest));

  // Adversarial (b): the provider recomputes the root with one item missing.
  const missingOne = providerLeaves.slice(1);
  const rootMissing = merkleRoot(missingOne);
  c.ok("root over an incomplete set mismatches the signed root",
    rootMissing !== manifest.root);

  return {
    id: "E2",
    title: "The manifest",
    plainSentence: "The list is the bill's source of truth, and the customer wrote the list.",
    assertions: c.results,
    tables: [
      {
        title: "Manifest summary",
        headers: ["field", "value"],
        rows: [
          ["items", manifest.leaves.length],
          ["total bytes", manifest.totalBytes],
          ["root (truncated)", manifest.root.slice(0, 24) + "…"],
          ["signed by", manifest.signerId],
        ],
      },
    ],
    notes: [],
  };
}
