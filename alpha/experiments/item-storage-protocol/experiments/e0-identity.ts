// E0. Identity — "We recognize you the same way we count you."
//
// Both parties exist as keys, nothing more. Each derives a stable identifier
// from its public key; they exchange and pin each other's keys. A message signed
// by the Customer verifies under the Customer's pinned key and fails under the
// Provider's. Identifier derivation is deterministic.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import {
  identifierFromRawHex,
  keypairFromSeedHex,
  deriveSeedHex,
  verifyJson,
} from "../src/crypto.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E0", seed);
  const { customer, provider } = w;

  // Record the key exchange as ledger entries (inspectable after the run).
  customer.ledger.append("key_exchange", w.clock.now(), {
    self: customer.id,
    pinnedPeer: provider.id,
    pinnedKey: provider.publicRawHex,
  });
  provider.ledger.append("key_exchange", w.clock.now(), {
    self: provider.id,
    pinnedPeer: customer.id,
    pinnedKey: customer.publicRawHex,
  });

  // A message signed by the Customer...
  const message = { note: "these are the items I am entrusting to you", day: w.clock.now() };
  const sig = customer.sign(message);

  // ...verifies under the Customer's pinned key (the Provider recognizes Ada).
  assert.equal(provider.verifyFrom(customer.id, message, sig), true, "customer sig must verify under pinned customer key");

  // ...and fails under the Provider's key (a different party did not sign it).
  assert.equal(verifyJson(provider.publicRawHex, message, sig), false, "customer sig must NOT verify under provider key");

  // Verifying as though it were from the Provider fails (wrong pinned key).
  assert.equal(provider.verifyFrom(provider.id, message, sig), false, "must not attribute customer sig to provider");

  // An unpinned stranger cannot be verified at all.
  const stranger = keypairFromSeedHex(deriveSeedHex(seed, "stranger"));
  const strangerId = identifierFromRawHex(stranger.publicRawHex);
  assert.equal(provider.verifyFrom(strangerId, message, sig), false, "unpinned party does not verify");

  // Identifier derivation is deterministic: rebuild the Customer's key from the
  // same seed and get the same identifier, byte for byte.
  const rebuilt = keypairFromSeedHex(deriveSeedHex(seed, "customer"));
  assert.equal(rebuilt.publicRawHex, customer.publicRawHex, "key derivation must be deterministic");
  assert.equal(identifierFromRawHex(rebuilt.publicRawHex), customer.id, "identifier derivation must be deterministic");

  // Distinct labels yield distinct identities.
  assert.notEqual(customer.id, provider.id, "customer and provider are distinct identities");

  const sentence = "We recognize you the same way we count you.";
  const reportMd = [
    "Both parties exist as keys and nothing more. Each identifier is a pure function of a",
    "public key, so recognition and counting use the same handle.",
    "",
    `- Customer (Ada): \`${customer.id}\``,
    `- Provider:       \`${provider.id}\``,
    "",
    "A Customer-signed message verifies under the Customer's pinned key and fails under the",
    "Provider's; an unpinned stranger cannot be verified at all; key and identifier derivation",
    "are deterministic across rebuilds.",
  ].join("\n");

  return { id: "E0", title: "Identity", sentence, reportMd };
}
