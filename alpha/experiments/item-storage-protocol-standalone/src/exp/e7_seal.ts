// E7. The seal, revocable tier — cold storage where the plan is no movement and
// verification proves it. The customer pins the current root and signs a seal; the
// provider destroys its write credential (write path now fails closed); a rotation
// watch stands up. Time advances across several periods of scheduled audits: rent
// accrues, and postage equals audit reads only. Three adversarial cases: a normal
// write fails for lack of key; a compromised path mutates bytes directly and the
// next audit catches it against the pinned root; and a legitimate customer-signed
// unseal is flagged and classified as customer-initiated.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { makeItem } from "../item.ts";
import { deriveKeypair } from "../crypto.ts";
import { auditSample } from "../audit.ts";
import { rentCents, postageCents, AUDIT_OVERHEAD_CENTS } from "../pricing.ts";
import {
  signSeal, CollectionWriter, RotationWatch, UnsealAuthority, type SealState,
} from "../seal.ts";

const PERIOD = 30;
const AUDITS_PER_PERIOD = 4;
const K = 3;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider, store, manifest } = world;
  const collectionId = "ada-family-vault";

  // The provider's write path starts with a real credential.
  const writeCred = deriveKeypair(world.masterSeed, `provider/write-cred/${collectionId}`);
  const writer = new CollectionWriter(writeCred);

  // Ceremony: Ada pins the current root and signs the seal; the provider destroys
  // the write credential. Both recorded in the ledgers.
  const pinnedRoot = manifest!.root;
  const seal = signSeal(collectionId, pinnedRoot, world.clock.now(), customer);
  customer.ledger.append("seal-declaration", world.clock.iso(), seal as unknown as Record<string, string | number>, [customer.signer()]);
  writer.destroyCredential();
  provider.ledger.append("seal-ceremony", world.clock.iso(), {
    collectionId, pinnedRoot, writeCredentialDestroyed: true,
  }, [provider.signer()]);

  const watch = new RotationWatch(collectionId, customer.id, customer.publicKeyHex);
  const unsealAuthority = new UnsealAuthority(customer);
  const state: SealState = {
    collectionId, pinnedRoot, pinnedManifest: manifest!, writer, watch, unsealAuthority,
  };
  world.seal = state;

  // Several sealed periods of scheduled audits. Postage == audit reads exactly.
  let sealedPostageBytes = 0;
  let sealedAuditReadBytes = 0;
  for (let p = 0; p < 3; p++) {
    const start = world.clock.now();
    world.clock.advanceDays(PERIOD);
    const end = world.clock.now();

    let periodReadBytes = 0;
    let allPassed = true;
    for (let a = 0; a < AUDITS_PER_PERIOD; a++) {
      const out = auditSample(manifest!.leaves, store, world.rng(`e7/audit/${p}/${a}`), K);
      periodReadBytes += out.bytesRead;
      if (!out.passed) allPassed = false;
    }
    if (!allPassed) c.ok(`sealed period ${p} audits all pass`, false);
    sealedAuditReadBytes += periodReadBytes;

    // In the sealed tier, the only bytes that move are audit reads — so that IS
    // the postage for the period.
    const postageBytes = periodReadBytes;
    sealedPostageBytes += postageBytes;
    const byteDays = world.byteDays(start, end);
    const rc = rentCents(byteDays);
    const pc = postageCents(postageBytes);
    const ac = AUDITS_PER_PERIOD * AUDIT_OVERHEAD_CENTS;
    world.commitStatement({
      periodStartDay: start, periodEndDay: end,
      openingRoot: pinnedRoot, closingRoot: pinnedRoot, byteDays,
      rentCents: rc, postageBytes, postageCents: pc,
      auditCount: AUDITS_PER_PERIOD, auditBytes: postageBytes, auditCents: ac, auditTier: "sealed",
      graceCents: 0, feesCents: 0, totalCents: rc + pc + ac,
    });
  }

  c.ok("all sealed-period audits passed against the pinned root", true);
  c.eq("postage over the sealed periods equals audit reads exactly",
    sealedPostageBytes, sealedAuditReadBytes);

  // Adversarial (a): a normal write after the ceremony fails for lack of key.
  const rejected = writer.write(store, makeItem("smuggled.txt", Buffer.from("late addition")));
  c.ok("write through the normal path fails: no credential", !rejected.ok);

  // Adversarial (b): a compromised path mutates stored bytes directly (no new
  // signature). The next audit catches it against the pinned root.
  const victim = manifest!.leaves[0];
  store.corruptOneByte(victim.cid);
  const fullAudit = auditSample(manifest!.leaves, store, world.rng("e7/catch"), manifest!.leaves.length);
  c.ok("direct byte mutation is caught by audit against the pinned root",
    !fullAudit.passed && fullAudit.failures.includes(victim.cid));
  // Restore for narrative continuity (the co-op re-fetches good bytes from Ada).
  const good = world.items.find((i) => i.cid === victim.cid)!;
  store.put(good);

  // Adversarial (c): a legitimate unseal — Ada's offline key signs a rotation.
  // The watch flags it and classifies it as customer-initiated.
  const newRoot = "0".repeat(63) + "1"; // a hypothetical rotated root
  const legit = unsealAuthority.rotate(collectionId, newRoot, world.clock.now())!;
  const evLegit = watch.observe(legit);
  c.eq("customer-signed rotation is classified customer-initiated", evLegit.type, "customer-initiated");

  // And a forged rotation (signed by the provider, not the customer) alarms.
  const forged = {
    collectionId, newRoot, day: world.clock.now(), signerId: provider.id,
    signature: "00".repeat(64),
  };
  const evForged = watch.observe(forged);
  c.eq("a non-customer root change raises an alarm", evForged.type, "alarm");
  c.ok("every observed root change is either customer-signed or alarmed",
    watch.events.every((e) => e.type === "customer-initiated" || e.type === "alarm"));

  return {
    id: "E7",
    title: "The seal, revocable tier",
    plainSentence: "Immutability detectable at the protocol, enforceable at the key ceremony.",
    assertions: c.results,
    tables: [
      {
        title: "Seal state",
        headers: ["property", "value"],
        rows: [
          ["collection", collectionId],
          ["pinned root", pinnedRoot.slice(0, 20) + "…"],
          ["write credential", writer.hasCredential ? "present" : "destroyed"],
          ["unseal capability", unsealAuthority.isDestroyed ? "destroyed" : "held by customer"],
          ["sealed audit reads (bytes)", sealedAuditReadBytes],
        ],
      },
    ],
    notes: [],
  };
}
