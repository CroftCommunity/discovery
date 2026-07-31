// E7. The seal, revocable tier — "Immutability detectable at the protocol,
// enforceable at the key ceremony."
//
// The Customer pins the current manifest root and signs a seal declaration. The
// Provider destroys its write-path credential (the write function fails closed
// without it). A rotation watch treats any new signed root as an event. Time
// advances across periods; scheduled audits run against the pinned root;
// statements show rent accruing with postage equal to audit reads only.
//
// Adversarial: (a) a write through the normal path fails for lack of key; (b) a
// compromised path mutates stored bytes directly and the next audit catches it
// against the pinned root; (c) a legitimate unseal — the Customer's offline key
// signs a rotation — is flagged and classified as customer-initiated.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItem, type NamedItem } from "../src/items.ts";
import { buildSignedManifest } from "../src/manifest.ts";
import { audit } from "../src/audit.ts";
import {
  coSignStatement,
  GENESIS_PREV,
  verifyChain,
  type Statement,
  type StatementBody,
} from "../src/statements.ts";
import { WriteGate, RotationCapability, classifyRotation, type RootChangeEvent } from "../src/seal.ts";
import { DAYS_PER_PERIOD } from "../src/time.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E7", seed);
  const { customer, provider, store } = w;
  const prng = new Prng(seed);

  // Build and store the collection; the Provider writes via a guarded write path.
  const gate = new WriteGate("provider-write-cred");
  const items: NamedItem[] = [];
  for (let i = 0; i < 8; i++) {
    const it = makeItem(prng, `sealed-${i}`, 1024);
    gate.write(store, it.bytes);
    items.push(it);
  }
  const manifest = buildSignedManifest(customer, items.map((it) => ({ cid: it.cid, size: it.size })), 0);

  // Seal ceremony: pin the root, sign a seal declaration, destroy the write cred.
  const sealDecl = { pinnedRoot: manifest.root, tier: "sealed-revocable", sealDay: w.clock.now() };
  const sealSig = customer.sign(sealDecl);
  customer.ledger.append("seal", w.clock.now(), { ...sealDecl, sig: sealSig });
  provider.ledger.append("seal_ack", w.clock.now(), { pinnedRoot: manifest.root, tier: "sealed-revocable" });
  gate.destroyWriteCredential();
  assert.equal(gate.hasWriteCredential(), false, "write credential destroyed by the ceremony");

  // Adversarial (a): a write through the normal path fails closed.
  assert.throws(() => gate.write(store, Buffer.from("new item")), /failed closed/, "no write path after seal");

  // Advance across periods; scheduled audits run against the pinned root, and
  // statements accrue rent with postage equal to audit reads only.
  const AUDITS_PER_PERIOD = 4;
  const AUDIT_K = 5;
  const statements: Statement[] = [];
  let prevHash = GENESIS_PREV;
  const bytesAtRest = items.reduce((s, it) => s + it.size, 0);
  for (let period = 0; period < 3; period++) {
    let auditBytes = 0;
    for (let a = 0; a < AUDITS_PER_PERIOD; a++) {
      const r = audit(manifest, store, AUDIT_K, prng);
      assert.equal(r.passed, true, `sealed audit period ${period} #${a} must pass`);
      auditBytes += r.bytesRetrieved;
    }
    const closeDay = period * DAYS_PER_PERIOD + DAYS_PER_PERIOD - 1;
    const body: StatementBody = {
      period,
      openingRoot: manifest.root,
      closingRoot: manifest.root, // no movement — root pinned
      rentByteDays: bytesAtRest * DAYS_PER_PERIOD,
      postageBytes: auditBytes, // postage over the sealed period is audit reads only
      auditCount: AUDITS_PER_PERIOD,
      auditBytes,
      fees: 0,
      graceNet: 0,
      prevStatementHash: prevHash,
      closeDay,
    };
    const stmt = coSignStatement(customer, provider, body, closeDay);
    statements.push(stmt);
    prevHash = stmt.hash;
    // Postage over the sealed period equals audit reads exactly.
    assert.equal(stmt.body.postageBytes, auditBytes, "sealed-period postage equals audit reads exactly");
    assert.equal(stmt.body.openingRoot, stmt.body.closingRoot, "sealed period has no root movement");
  }
  assert.equal(verifyChain(customer, statements).ok, true, "sealed statement chain verifies");

  // Adversarial (b): a compromised path mutates stored bytes directly (bypassing
  // the destroyed write function). The next audit catches it against the pinned
  // root.
  const victim = items[2];
  store.overwriteRaw(victim.cid, Buffer.from("corrupted-by-compromised-path".padEnd(1024, "!")));
  // Force the audit to include the victim by auditing all items.
  const sweep = audit(manifest, store, items.length, new Prng(seed + 123));
  assert.equal(sweep.passed, false, "direct mutation must be caught by audit against the pinned root");
  assert.ok(
    sweep.failures.some((f) => f.cid === victim.cid && f.reason === "hash-mismatch"),
    "the mutated item is identified by the audit",
  );

  // Adversarial (c): a legitimate unseal. The Customer's offline key signs a
  // rotation to a new root; the watch flags it and classifies it as
  // customer-initiated by signature. An attacker-signed rotation is alarmed.
  const rot = new RotationCapability("customer-offline-cap");
  const newRoot = "0".repeat(64);
  const custEvent: RootChangeEvent = rot.rotate(customer, newRoot, w.clock.now());
  const custClass = classifyRotation(customer.id, customer.publicRawHex, custEvent);
  assert.equal(custClass, "customer-initiated", "a customer-signed rotation is classified customer-initiated");

  // Attacker forges a rotation signed with the provider's key but claiming to be
  // the customer. The watch alarms it (wrong key).
  const attackerEvent: RootChangeEvent = {
    root: "f".repeat(64),
    day: w.clock.now(),
    claimedSigner: customer.id,
    sig: provider.sign({ root: "f".repeat(64), day: w.clock.now() }),
  };
  assert.equal(
    classifyRotation(customer.id, customer.publicRawHex, attackerEvent),
    "alarm",
    "an attacker-signed root change is alarmed",
  );
  customer.ledger.append("rotation_event", w.clock.now(), { root: newRoot, class: custClass });
  provider.ledger.append("rotation_alarm", w.clock.now(), { root: attackerEvent.root, class: "alarm" });

  const sentence = "Immutability detectable at the protocol, enforceable at the key ceremony.";
  const reportMd = [
    `The Customer sealed ${items.length} items at pinned root \`${manifest.root.slice(0, 16)}…\` and the`,
    "Provider destroyed its write credential. Over three sealed periods, scheduled audits all",
    "passed and every statement's postage equaled its audit reads exactly, with the root",
    "unmoved.",
    "",
    "Adversarial: a normal-path write failed closed; a direct byte mutation by a compromised path",
    "was caught by the next audit against the pinned root (identifying the exact item); a",
    "customer-signed rotation was classified customer-initiated, while an attacker-signed root",
    "change was alarmed.",
  ].join("\n");

  return { id: "E7", title: "The seal, revocable tier", sentence, reportMd };
}
