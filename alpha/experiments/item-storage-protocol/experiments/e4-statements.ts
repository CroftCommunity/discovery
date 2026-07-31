// E4. Balance-forward statements — "Last month was agreed, so this month only
// has to explain the difference."
//
// Three billing periods with mixed activity (adds, deletes via manifest update,
// transfers). Each close co-signs a statement: opening/closing roots, rent
// (byte-days integrated from the manifest timeline), postage (summed receipts),
// fees. Statement N+1 references N by hash — a chain from genesis. Adversarial:
// rewrite a figure in a historical statement (chain fails at exactly that link);
// fabricate an extra period (fails to attach).

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItem, type NamedItem } from "../src/items.ts";
import { merkleRoot, sortItems } from "../src/manifest.ts";
import { ManifestTimeline } from "../src/billing.ts";
import { runTransfer } from "../src/receipts.ts";
import {
  coSignStatement,
  verifyChain,
  verifyStatement,
  hashStatement,
  GENESIS_PREV,
  type Statement,
  type StatementBody,
} from "../src/statements.ts";
import { DAYS_PER_PERIOD } from "../src/time.ts";

function rootOfItems(items: NamedItem[]): string {
  return merkleRoot(sortItems(items.map((it) => ({ cid: it.cid, size: it.size }))));
}

export function run(seed: number): ExperimentResult {
  const w = createWorld("E4", seed);
  const { customer, provider, store } = w;
  const prng = new Prng(seed);
  const INC = 1000;

  const timeline = new ManifestTimeline();
  const held = new Map<string, NamedItem>();
  const statements: Statement[] = [];
  let prevHash = GENESIS_PREV;
  const summaries: { period: number; rent: number; postage: number; opening: string; closing: string }[] = [];

  // Independent, segment-by-segment rent oracle, computed a different way than
  // the timeline integrator so the two must agree.
  function independentRent(segments: { days: number; bytes: number }[]): number {
    return segments.reduce((sum, s) => sum + s.days * s.bytes, 0);
  }

  function closePeriod(
    period: number,
    opening: string,
    postage: number,
    fees: number,
    expectedRentSegments: { days: number; bytes: number }[],
  ): void {
    const { start, end } = { start: period * DAYS_PER_PERIOD, end: period * DAYS_PER_PERIOD + DAYS_PER_PERIOD - 1 };
    const rent = timeline.byteDays(start, end);
    assert.equal(
      rent,
      independentRent(expectedRentSegments),
      `period ${period}: timeline rent must equal the independent byte-day integral`,
    );
    const closing = rootOfItems([...held.values()]);
    const body: StatementBody = {
      period,
      openingRoot: opening,
      closingRoot: closing,
      rentByteDays: rent,
      postageBytes: postage,
      auditCount: 0,
      auditBytes: 0,
      fees,
      graceNet: 0,
      prevStatementHash: prevHash,
      closeDay: end,
    };
    const stmt = coSignStatement(customer, provider, body, end);
    statements.push(stmt);
    prevHash = stmt.hash;
    summaries.push({ period, rent, postage, opening: opening.slice(0, 10), closing: closing.slice(0, 10) });
  }

  // --- Period 0: three adds on day 0 ---
  const emptyRoot = rootOfItems([]);
  {
    const opening = emptyRoot;
    let postage = 0;
    for (const spec of [
      { name: "A", size: 1000 },
      { name: "B", size: 2000 },
      { name: "C", size: 3000 },
    ]) {
      const it = makeItem(prng, spec.name, spec.size);
      store.put(it.bytes);
      held.set(it.name, it);
      postage += runTransfer("upload", it.cid, it.size, INC, customer, provider, 0).total;
    }
    timeline.set(0, 6000); // A+B+C
    closePeriod(0, opening, postage, 0, [{ days: 30, bytes: 6000 }]);
  }

  // --- Period 1: add D on day 30, delete B on day 45 ---
  {
    const opening = statements[0].body.closingRoot;
    let postage = 0;
    const d = makeItem(prng, "D", 4000);
    store.put(d.bytes);
    held.set(d.name, d);
    postage += runTransfer("upload", d.cid, d.size, INC, customer, provider, 30).total;
    timeline.set(30, 10000); // A+B+C+D
    held.delete("B");
    timeline.set(45, 8000); // A+C+D
    closePeriod(1, opening, postage, 0, [
      { days: 15, bytes: 10000 },
      { days: 15, bytes: 8000 },
    ]);
  }

  // --- Period 2: downloads on day 60, delete C on day 75 ---
  {
    const opening = statements[1].body.closingRoot;
    let postage = 0;
    for (const name of ["A", "C"]) {
      const it = held.get(name)!;
      postage += runTransfer("download", it.cid, it.size, INC, provider, customer, 60).total;
    }
    held.delete("C");
    timeline.set(75, 5000); // A+D
    closePeriod(2, opening, postage, 0, [
      { days: 15, bytes: 8000 },
      { days: 15, bytes: 5000 },
    ]);
  }

  // The chain verifies end to end.
  const chain = verifyChain(customer, statements);
  assert.equal(chain.ok, true, `chain must verify: ${chain.reason ?? ""}`);
  assert.equal(chain.brokenAt, null);

  // Adversarial (1): rewrite a figure inside period 1. Chain verification from
  // genesis fails at exactly that link; the untouched period 0 still verifies.
  const tampered: Statement[] = statements.map((s) => ({ ...s, body: { ...s.body } }));
  tampered[1].body.rentByteDays += 1; // stored hash left stale
  const brokenChain = verifyChain(customer, tampered);
  assert.equal(brokenChain.ok, false, "an edited historical figure must break the chain");
  assert.equal(brokenChain.brokenAt, 1, "the break must be located at exactly the edited period");
  assert.equal(verifyStatement(customer, tampered[0]), true, "the untouched earlier period still verifies");

  // Adversarial (2): fabricate an extra period. Without both keys it cannot be
  // co-signed, so it fails to attach even with a correct prev-hash.
  const fakeBody: StatementBody = {
    period: 3,
    openingRoot: statements[2].body.closingRoot,
    closingRoot: rootOfItems([...held.values()]),
    rentByteDays: 0,
    postageBytes: 0,
    auditCount: 0,
    auditBytes: 0,
    fees: 0,
    graceNet: 0,
    prevStatementHash: statements[2].hash,
    closeDay: 119,
  };
  // The fabricator can sign as the provider but cannot forge the customer's sig,
  // so it hands over a bogus customer signature.
  const fabricated: Statement = {
    body: fakeBody,
    hash: hashStatement(fakeBody),
    customer: customer.id,
    provider: provider.id,
    customerSig: provider.sign(fakeBody), // wrong key — not the customer's
    providerSig: provider.sign(fakeBody),
  };
  assert.equal(verifyStatement(customer, fabricated), false, "a fabricated period cannot be co-signed into validity");
  const withFake = verifyChain(customer, [...statements, fabricated]);
  assert.equal(withFake.ok, false, "appending a fabricated period must fail the chain");
  assert.equal(withFake.brokenAt, 3, "the fabricated period is rejected at its own link");

  const sentence = "Last month was agreed, so this month only has to explain the difference.";
  const table = [
    "| Period | Opening root | Closing root | Rent (byte-days) | Postage (bytes) |",
    "| --- | --- | --- | ---: | ---: |",
    ...summaries.map(
      (s) => `| ${s.period} | \`${s.opening}…\` | \`${s.closing}…\` | ${s.rent} | ${s.postage} |`,
    ),
  ].join("\n");
  const reportMd = [
    "Three periods closed into a co-signed, hash-linked chain. Each period's rent (byte-days)",
    "equals an independent segment-by-segment integral of the manifest timeline; postage is the",
    "sum of that period's signed receipts.",
    "",
    table,
    "",
    "Adversarial: rewriting period 1's rent breaks the chain at exactly period 1 while period 0",
    "still verifies; a fabricated period 3 cannot be co-signed and is rejected at its own link.",
  ].join("\n");

  return { id: "E4", title: "Balance-forward statements", sentence, reportMd };
}
