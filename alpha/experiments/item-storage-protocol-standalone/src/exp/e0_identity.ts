// E0. Identity — both parties exist as keys, nothing more.
// We generate the two keypairs (done in World), derive each identifier from the
// public key, pin each other's key, and prove that recognition (signature check)
// and counting (identifier derivation) rest on the same public key.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { Actor } from "../actor.ts";
import { signMessage, verifyMessage } from "../crypto.ts";

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider } = world;

  // Each actor records a self-describing genesis entry (its id, its public key,
  // and the peer key it has pinned), signed by itself.
  const ts = world.clock.iso();
  customer.ledger.append("genesis", ts, {
    self: customer.id, publicKey: customer.publicKeyHex, pinnedPeer: provider.id,
  }, [customer.signer()]);
  provider.ledger.append("genesis", ts, {
    self: provider.id, publicKey: provider.publicKeyHex, pinnedPeer: customer.id,
  }, [provider.signer()]);

  // A message signed by the customer verifies under the customer's pinned key...
  const msg = "I, Ada, am bringing my items to the co-op.";
  const sig = signMessage(customer.keypair, msg);
  c.ok(
    "customer signature verifies under customer's pinned key",
    verifyMessage(world.keyring[customer.id], msg, sig),
  );
  // ...and fails under the provider's key. Adversarial: a forged attribution.
  c.ok(
    "customer signature does NOT verify under provider's key",
    !verifyMessage(world.keyring[provider.id], msg, sig),
  );

  // Identifier derivation is a deterministic function of the public key alone.
  c.ok(
    "identifier derivation is deterministic",
    Actor.deriveId(customer.publicKeyHex) === customer.id &&
      Actor.deriveId(provider.publicKeyHex) === provider.id,
  );
  // Distinct keys yield distinct identifiers.
  c.ok("distinct keys yield distinct identifiers", customer.id !== provider.id);

  return {
    id: "E0",
    title: "Identity",
    plainSentence: "We recognize you the same way we count you.",
    assertions: c.results,
    tables: [
      {
        title: "Pinned identities",
        headers: ["actor", "id", "public key (hex, truncated)"],
        rows: [
          ["Ada (customer)", customer.id, customer.publicKeyHex.slice(0, 24) + "…"],
          ["Co-op (provider)", provider.id, provider.publicKeyHex.slice(0, 24) + "…"],
        ],
      },
    ],
    notes: [],
  };
}
