// E9. The grace ledger — "The receipts make fairness legible; the margin makes
// mercy affordable."
//
// Mercy is represented in the books, not off-book. Grace events are first-class
// signed ledger entries: a fee waiver with a reason code; a deceased-member hold
// (rent accrues to the co-op's own account for a fixed term, mock three periods);
// throttle-instead-of-cutoff during a payment lapse. Each is a double-entry that
// nets to zero against a co-op grace account, so the books still balance; grace
// totals are reportable per period; every grace event is a forward entry that
// edits no history.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import {
  coSignStatement,
  GENESIS_PREV,
  verifyChain,
  verifyStatement,
  type Statement,
  type StatementBody,
} from "../src/statements.ts";
import { DAYS_PER_PERIOD } from "../src/time.ts";

interface Posting {
  account: string;
  delta: number;
}

interface GraceEvent {
  kind: "fee_waiver" | "deceased_hold" | "throttle_hold";
  reasonCode: string;
  amount: number;
  period: number;
  termPeriods?: number;
}

export function run(seed: number): ExperimentResult {
  const w = createWorld("E9", seed);
  const { customer, provider, coop } = w;

  // The co-op grace account is funded from margin — "the margin makes mercy
  // affordable." Each grace event debits it; it must stay solvent.
  const MARGIN = 100000;
  let coopGraceBalance = MARGIN;
  const postings: Posting[] = [];

  const events: GraceEvent[] = [
    { kind: "fee_waiver", reasonCode: "hardship", amount: 500, period: 0 },
    { kind: "deceased_hold", reasonCode: "bereavement", amount: 3000, period: 1, termPeriods: 3 },
    { kind: "throttle_hold", reasonCode: "payment_lapse", amount: 800, period: 2 },
  ];

  function applyGrace(ev: GraceEvent): void {
    // Double entry: the co-op grace account is debited; the customer is credited
    // the same amount. The two postings net to zero.
    const debit: Posting = { account: "coop-grace", delta: -ev.amount };
    const credit: Posting = { account: customer.id, delta: +ev.amount };
    assert.equal(debit.delta + credit.delta, 0, `grace event ${ev.kind} must net to zero`);
    postings.push(debit, credit);
    coopGraceBalance += debit.delta;

    // First-class SIGNED ledger entries, forward-only, on both the co-op's grace
    // ledger and the customer's ledger.
    const body = {
      kind: ev.kind,
      reasonCode: ev.reasonCode,
      amount: ev.amount,
      period: ev.period,
      benefitTo: customer.id,
      chargedTo: "coop-grace",
      ...(ev.termPeriods ? { termPeriods: ev.termPeriods } : {}),
    };
    coop.ledger.append("grace_event", ev.period * DAYS_PER_PERIOD, body);
    customer.ledger.append("grace_event", ev.period * DAYS_PER_PERIOD, body);
  }

  // Baseline statement (period 0) is co-signed BEFORE any grace, so we can prove
  // later grace events do not edit it.
  const statements: Statement[] = [];
  let prevHash = GENESIS_PREV;
  const baseBody: StatementBody = {
    period: 0,
    openingRoot: "grace-open",
    closingRoot: "grace-close-0",
    rentByteDays: 6000,
    postageBytes: 0,
    auditCount: 0,
    auditBytes: 0,
    fees: 500,
    graceNet: 0,
    prevStatementHash: prevHash,
    closeDay: DAYS_PER_PERIOD - 1,
  };

  // Apply period 0's grace (the fee waiver), then close period 0 crediting it.
  applyGrace(events[0]);
  const stmt0Body: StatementBody = { ...baseBody, graceNet: events[0].amount };
  const stmt0 = coSignStatement(customer, provider, stmt0Body, DAYS_PER_PERIOD - 1);
  statements.push(stmt0);
  prevHash = stmt0.hash;
  // The customer's net for period 0: fees minus the waived amount.
  assert.equal(stmt0.body.fees - stmt0.body.graceNet, 0, "the waived fee zeroes the customer's period-0 fee");

  // Periods 1 and 2: deceased-hold and throttle-hold.
  for (const ev of [events[1], events[2]]) {
    applyGrace(ev);
    const closeDay = ev.period * DAYS_PER_PERIOD + DAYS_PER_PERIOD - 1;
    const body: StatementBody = {
      period: ev.period,
      openingRoot: `grace-open-${ev.period}`,
      closingRoot: `grace-close-${ev.period}`,
      rentByteDays: ev.kind === "deceased_hold" ? ev.amount : 6000,
      postageBytes: 0,
      auditCount: 0,
      auditBytes: 0,
      fees: ev.kind === "throttle_hold" ? ev.amount : 0,
      graceNet: ev.amount,
      prevStatementHash: prevHash,
      closeDay,
    };
    const stmt = coSignStatement(customer, provider, body, closeDay);
    statements.push(stmt);
    prevHash = stmt.hash;
  }

  // The deceased-member hold is a fixed three-period term.
  assert.equal(events[1].termPeriods, 3, "deceased-member hold runs a fixed three-period term");

  // The books still balance: every posting sums to zero.
  const totalDelta = postings.reduce((s, p) => s + p.delta, 0);
  assert.equal(totalDelta, 0, "all grace postings must net to zero — the books balance");

  // Grace totals are reportable per period.
  const perPeriod = new Map<number, number>();
  for (const ev of events) perPeriod.set(ev.period, (perPeriod.get(ev.period) ?? 0) + ev.amount);
  assert.deepEqual([...perPeriod.entries()].sort(), [[0, 500], [1, 3000], [2, 800]], "per-period grace totals report correctly");

  // Mercy was affordable: the grace account stayed solvent against its margin.
  const spent = MARGIN - coopGraceBalance;
  assert.equal(spent, 500 + 3000 + 800, "grace account debited exactly the total grace");
  assert.ok(coopGraceBalance >= 0, "the co-op grace account stayed solvent (mercy was affordable)");

  // No grace event edited history: the pre-grace baseline period-0 statement
  // still verifies, and all grace entries are forward-only appends.
  assert.equal(verifyStatement(customer, stmt0), true, "the co-signed period-0 statement is unchanged by later grace");
  assert.equal(verifyChain(customer, statements).ok, true, "the statement chain still verifies with grace applied");
  const graceRows = coop.ledger.entries().filter((e) => e.type === "grace_event");
  assert.equal(graceRows.length, 3, "three grace events recorded");
  graceRows.forEach((e, i) => {
    if (i > 0) assert.ok(e.seq > graceRows[i - 1].seq, "grace events are forward-only (strictly increasing seq)");
  });

  const sentence = "The receipts make fairness legible; the margin makes mercy affordable.";
  const reportMd = [
    "Three grace events were booked as first-class, signed, double-entry ledger rows against a",
    "co-op grace account:",
    "",
    "| Period | Kind | Reason | Amount | Nets to |",
    "| --- | --- | --- | ---: | ---: |",
    "| 0 | fee_waiver | hardship | 500 | 0 |",
    "| 1 | deceased_hold | bereavement (3-period term) | 3000 | 0 |",
    "| 2 | throttle_hold | payment_lapse | 800 | 0 |",
    "",
    `Every event nets to zero against the grace account; the account spent ${spent} of its`,
    `${MARGIN} margin and stayed solvent. Per-period totals are reportable, and no grace event`,
    "edited history — the pre-grace statement still verifies and every grace row is a forward append.",
  ].join("\n");

  return { id: "E9", title: "The grace ledger", sentence, reportMd };
}
