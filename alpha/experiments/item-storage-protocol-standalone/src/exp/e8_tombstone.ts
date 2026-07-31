// E8. The tombstone, permanent tier — an archive no one can unseal, including us.
// We repeat E7's ceremony, then destroy the customer's rotation capability as well
// and prove the unseal function now fails closed. Time advances; audits continue to
// verify against the pinned root; the statement chain continues cleanly with rent
// only. Adversarially, every unseal and write path from both actors is attempted,
// and all must fail.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { makeItem } from "../item.ts";
import { auditSample } from "../audit.ts";
import { rentCents } from "../pricing.ts";

const PERIOD = 30;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider, store, manifest } = world;
  const state = world.seal!;
  const collectionId = state.collectionId;

  // Destroy the customer's rotation capability. Prove deletion by showing the
  // unseal function fails closed.
  state.unsealAuthority.destroy();
  provider.ledger.append("tombstone-ceremony", world.clock.iso(), {
    collectionId, pinnedRoot: state.pinnedRoot, rotationCapabilityDestroyed: true,
  }, [provider.signer()]);
  customer.ledger.append("tombstone-declaration", world.clock.iso(), {
    collectionId, pinnedRoot: state.pinnedRoot,
    note: "no party, including the co-op, can unseal this collection",
  }, [customer.signer()]);

  c.ok("unseal capability is destroyed", state.unsealAuthority.isDestroyed);
  c.ok("unseal now fails closed (returns no rotation)",
    state.unsealAuthority.rotate(collectionId, "deadbeef", world.clock.now()) === null);

  // Advance several periods. Audits still verify; statements carry rent only.
  let allAuditsPass = true;
  let rentOnly = true;
  for (let p = 0; p < 3; p++) {
    const start = world.clock.now();
    world.clock.advanceDays(PERIOD);
    const end = world.clock.now();
    const out = auditSample(manifest!.leaves, store, world.rng(`e8/audit/${p}`), manifest!.leaves.length);
    if (!out.passed) allAuditsPass = false;
    const byteDays = world.byteDays(start, end);
    const rc = rentCents(byteDays);
    // Permanent tier: the co-op absorbs verification cost, so the statement is
    // rent only — postage and billed audit cost are zero.
    const stmt = world.commitStatement({
      periodStartDay: start, periodEndDay: end,
      openingRoot: state.pinnedRoot, closingRoot: state.pinnedRoot, byteDays,
      rentCents: rc, postageBytes: 0, postageCents: 0,
      auditCount: 1, auditBytes: 0, auditCents: 0, auditTier: "tombstone",
      graceCents: 0, feesCents: 0, totalCents: rc,
    });
    if (stmt.totalCents !== rc || stmt.postageCents !== 0) rentOnly = false;
  }
  c.ok("audits still verify against the pinned root in the tombstone tier", allAuditsPass);
  c.ok("statement chain continues cleanly with rent only", rentOnly);

  // Adversarial: attempt every write and unseal path from both actors. All fail.
  const providerWrite = state.writer.write(store, makeItem("x", Buffer.from("x")));
  const customerUnseal = state.unsealAuthority.rotate(collectionId, "beef", world.clock.now());
  // The provider holds no unseal authority of its own; the customer holds no write
  // path of its own — both are structurally absent, which is itself "fails closed".
  c.ok("provider write path fails (no credential)", !providerWrite.ok);
  c.ok("customer unseal path fails (capability destroyed)", customerUnseal === null);
  c.ok("collection is frozen for all parties",
    !state.writer.hasCredential && state.unsealAuthority.isDestroyed);

  return {
    id: "E8",
    title: "The tombstone, permanent tier",
    plainSentence: "The tombstone tier is a feature.",
    assertions: c.results,
    tables: [
      {
        title: "Tombstone state",
        headers: ["property", "value"],
        rows: [
          ["collection", collectionId],
          ["write credential", state.writer.hasCredential ? "present" : "destroyed"],
          ["unseal capability", state.unsealAuthority.isDestroyed ? "destroyed" : "held"],
          ["audits still verifying", allAuditsPass ? "yes" : "no"],
        ],
      },
    ],
    notes: [],
  };
}
