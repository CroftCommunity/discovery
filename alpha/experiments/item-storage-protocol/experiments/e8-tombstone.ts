// E8. The tombstone, permanent tier — "The tombstone tier is a feature."
//
// Repeat the seal ceremony, then destroy the Customer's rotation capability as
// well: the unseal function fails closed. Time advances; audits keep verifying
// against the pinned root. Adversarial: every unseal and write path, from both
// actors, must fail. The statement chain continues cleanly with rent only.

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
import { WriteGate, RotationCapability } from "../src/seal.ts";
import { DAYS_PER_PERIOD } from "../src/time.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E8", seed);
  const { customer, provider, store } = w;
  const prng = new Prng(seed);

  const gate = new WriteGate("provider-write-cred");
  const rot = new RotationCapability("customer-offline-cap");
  const items: NamedItem[] = [];
  for (let i = 0; i < 6; i++) {
    const it = makeItem(prng, `tomb-${i}`, 2048);
    gate.write(store, it.bytes);
    items.push(it);
  }
  const manifest = buildSignedManifest(customer, items.map((it) => ({ cid: it.cid, size: it.size })), 0);

  // Tombstone ceremony: seal (destroy provider write cred) AND destroy the
  // customer's rotation capability. No party can move anything, ever.
  customer.ledger.append("tombstone", w.clock.now(), { pinnedRoot: manifest.root, tier: "tombstone-permanent" });
  provider.ledger.append("tombstone_ack", w.clock.now(), { pinnedRoot: manifest.root, tier: "tombstone-permanent" });
  gate.destroyWriteCredential();
  rot.destroyCapability();
  assert.equal(gate.hasWriteCredential(), false, "provider write credential destroyed");
  assert.equal(rot.hasCapability(), false, "customer rotation capability destroyed");

  // Adversarial: every write and unseal path, from both actors, fails closed.
  assert.throws(() => gate.write(store, Buffer.from("x")), /failed closed/, "provider write path frozen");
  assert.throws(() => rot.rotate(customer, "1".repeat(64), w.clock.now()), /failed closed/, "customer unseal path frozen");
  // The customer cannot write either — there is no write path but the gate; and
  // cannot rotate — the capability is gone.
  assert.throws(() => gate.write(store, Buffer.from("y")), /failed closed/, "no actor has a write path");

  // Advance time. Audits keep verifying against the pinned root. One period runs
  // scheduled audits (postage = audit reads); one period is a pure cold archive
  // (rent only, zero postage) — the chain continues cleanly through both.
  const statements: Statement[] = [];
  let prevHash = GENESIS_PREV;
  const bytesAtRest = items.reduce((s, it) => s + it.size, 0);
  for (let period = 0; period < 3; period++) {
    const coldArchive = period === 1; // middle period: rent only, no audits
    let auditBytes = 0;
    let auditCount = 0;
    if (!coldArchive) {
      auditCount = 3;
      for (let a = 0; a < auditCount; a++) {
        const r = audit(manifest, store, 4, prng);
        assert.equal(r.passed, true, `tombstone audit period ${period} #${a} must pass`);
        auditBytes += r.bytesRetrieved;
      }
    }
    const closeDay = period * DAYS_PER_PERIOD + DAYS_PER_PERIOD - 1;
    const body: StatementBody = {
      period,
      openingRoot: manifest.root,
      closingRoot: manifest.root,
      rentByteDays: bytesAtRest * DAYS_PER_PERIOD,
      postageBytes: auditBytes,
      auditCount,
      auditBytes,
      fees: 0,
      graceNet: 0,
      prevStatementHash: prevHash,
      closeDay,
    };
    const stmt = coSignStatement(customer, provider, body, closeDay);
    statements.push(stmt);
    prevHash = stmt.hash;
    if (coldArchive) {
      // Rent only: the cold-archive period has zero postage.
      assert.equal(stmt.body.postageBytes, 0, "cold-archive period is rent only (zero postage)");
      assert.ok(stmt.body.rentByteDays > 0, "rent still accrues on the frozen archive");
    }
  }
  assert.equal(verifyChain(customer, statements).ok, true, "tombstone statement chain continues cleanly");

  const sentence = "The tombstone tier is a feature.";
  const reportMd = [
    `The Customer tombstoned ${items.length} items at pinned root \`${manifest.root.slice(0, 16)}…\`,`,
    "destroying both the Provider's write credential and the Customer's own rotation capability.",
    "Every write and unseal path, from both actors, then failed closed — the collection is frozen",
    "for all parties, the co-op included.",
    "",
    "Audits kept passing against the pinned root; the statement chain continued cleanly across a",
    "scheduled-audit period and a pure cold-archive period (rent only, zero postage).",
  ].join("\n");

  return { id: "E8", title: "The tombstone, permanent tier", sentence, reportMd };
}
