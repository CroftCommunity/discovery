// E4. Balance-forward statements — each month stands on the last, so disputes are
// bounded to one period. We run three billing periods with mixed activity (an add
// and a delete via manifest update, plus transfers), close each into a co-signed
// statement that chains to the previous by hash, then show that editing a
// historical figure breaks the chain at exactly that link and that a fabricated
// extra period cannot be inserted.

import type { World, ReceiptRecord } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { makeItem } from "../item.ts";
import { makeReceipt, isAcknowledged } from "../receipt.ts";
import { buildStatement, verifyChain, type Statement } from "../statement.ts";
import { rentCents, postageCents } from "../pricing.ts";
import type { Json } from "../canonical.ts";

const PERIOD = 30;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider } = world;

  // A billable upload of a whole item: a bilateral receipt + a postage record.
  function billableUpload(cid: string, size: number): void {
    const core = {
      direction: "upload" as const, cid, byteStart: 0, byteEnd: size, bytes: size,
      runningTotal: size, day: world.clock.now(), receiverId: provider.id, senderId: customer.id,
    };
    const r = makeReceipt(core, provider.keypair, customer.keypair);
    if (!isAcknowledged(r)) throw new Error("upload receipt must be acknowledged");
    customer.ledger.append("receipt", world.clock.iso(), r as unknown as Json, [customer.signer()]);
    provider.ledger.append("receipt", world.clock.iso(), r as unknown as Json, [provider.signer()]);
    const rec: ReceiptRecord = {
      day: r.day, direction: r.direction, cid: r.cid, byteStart: 0, byteEnd: size,
      bytes: size, runningTotal: size,
    };
    world.receipts.push(rec);
  }

  // Close a period into a co-signed, chained statement (no audits yet — E5/E6).
  function close(startDay: number, endDay: number, openingRoot: string): Statement {
    const byteDays = world.byteDays(startDay, endDay);
    const postageBytes = world.receipts
      .filter((r) => r.day >= startDay && r.day < endDay)
      .reduce((s, r) => s + r.bytes, 0);
    const rc = rentCents(byteDays);
    const pc = postageCents(postageBytes);
    return world.commitStatement({
      periodStartDay: startDay, periodEndDay: endDay,
      openingRoot, closingRoot: world.manifest!.root, byteDays,
      rentCents: rc, postageBytes, postageCents: pc,
      auditCount: 0, auditBytes: 0, auditCents: 0, auditTier: "none",
      graceCents: 0, feesCents: 0, totalCents: rc + pc,
    });
  }

  // --- Period 0: steady state. Opening root = the E2 manifest; no change. ---
  const rootP0 = world.manifest!.root;
  world.clock.advanceDays(PERIOD); // day 0 -> 30
  const s0 = close(0, PERIOD, rootP0);

  // --- Period 1: Ada adds an item mid-period. ---
  const rootP1open = world.manifest!.root;
  world.clock.advanceDays(5); // day 30 -> 35
  const rng = world.rng("e4/new-item");
  const newBytes = Buffer.alloc(3000);
  for (let i = 0; i < newBytes.length; i++) newBytes[i] = rng.int(256);
  const added = makeItem("new-photo.jpg", newBytes);
  world.store.put(added);
  world.updateManifest([...world.items, added], "add new-photo.jpg");
  billableUpload(added.cid, added.size);
  world.clock.advanceDays(25); // day 35 -> 60
  const s1 = close(30, 60, rootP1open);

  // --- Period 2: Ada deletes an item mid-period. ---
  const rootP2open = world.manifest!.root;
  world.clock.advanceDays(10); // day 60 -> 70
  const removed = world.items.find((i) => i.label === "voice-note.ogg")!;
  world.store.drop(removed.cid);
  world.updateManifest(world.items.filter((i) => i.cid !== removed.cid), "delete voice-note.ogg");
  world.clock.advanceDays(20); // day 70 -> 90
  const s2 = close(60, 90, rootP2open);

  // The chain verifies end to end.
  c.ok("statement chain verifies from genesis", verifyChain(world.statements).ok);

  // Each statement's opening root equals the prior closing root (balance-forward).
  c.eq("period 1 opens where period 0 closed", s1.openingRoot, s0.closingRoot);
  c.eq("period 2 opens where period 1 closed", s2.openingRoot, s1.closingRoot);

  // Rent equals the independently recomputed byte-day integral.
  let rentMatches = true;
  for (const s of world.statements) {
    const recomputed = rentCents(world.byteDays(s.periodStartDay, s.periodEndDay));
    if (recomputed !== s.rentCents) rentMatches = false;
  }
  c.ok("rent equals the independently recomputed byte-day integral", rentMatches);

  // Adversarial: rewrite a figure inside statement 1. Chain must fail at link 1.
  const tampered: Statement[] = world.statements.map((s) => ({ ...s }));
  tampered[1] = { ...tampered[1], rentCents: tampered[1].rentCents + 100 };
  const tamperResult = verifyChain(tampered);
  c.ok("editing a historical figure fails chain verification", !tamperResult.ok);
  c.ok("chain failure is located at exactly the edited link",
    !tamperResult.ok && tamperResult.failedAt === 1);

  // Adversarial: insert a fabricated extra period between 1 and 2.
  const fabricated: Statement[] = [
    world.statements[0], world.statements[1],
    buildStatement({
      ...world.statements[2], period: 2, prevStatementHash: world.statements[1].hash,
      rentCents: 9999,
    }),
    world.statements[2],
  ];
  c.ok("a fabricated inserted period cannot pass chain verification",
    !verifyChain(fabricated).ok);

  return {
    id: "E4",
    title: "Balance-forward statements",
    plainSentence: "Last month was agreed, so this month only has to explain the difference.",
    assertions: c.results,
    tables: [
      {
        title: "Statement chain (cents)",
        headers: ["period", "byte-days", "rent", "postage", "total", "closing root"],
        rows: world.statements.map((s) => [
          s.period, s.byteDays, s.rentCents, s.postageCents, s.totalCents,
          s.closingRoot.slice(0, 12) + "…",
        ]),
      },
    ],
    notes: [],
  };
}
