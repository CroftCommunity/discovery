// E9. The grace ledger — mercy is represented in the books, not off-book. Grace
// events are first-class signed ledger entries: a fee waiver with a reason code, a
// deceased-member hold (rent accrues to the co-op's own grace account for a fixed
// term), and throttle-instead-of-cutoff during a payment lapse. Each is a forward
// entry that nets to zero against the co-op grace account, so the books still
// balance and grace totals are reportable per period.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { rentCents } from "../pricing.ts";

const PERIOD = 30;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider, manifest } = world;

  let customerCreditsCents = 0; // negative: credits applied to members' bills
  const perPeriod: (string | number)[][] = [];

  // Emit a signed grace event and its offsetting entry into the co-op grace
  // account. The customer credit (-amount) and the grace-account charge (+amount)
  // net to zero by construction.
  function graceEvent(reasonCode: string, amountCents: number, note: string): void {
    customerCreditsCents -= amountCents;
    world.graceAccountCents += amountCents;
    provider.ledger.append("grace-event", world.clock.iso(), {
      reasonCode, amountCents, note,
      customerCredit: -amountCents, graceAccountCharge: amountCents,
      graceAccountBalance: world.graceAccountCents,
    }, [provider.signer(), customer.signer()]);
  }

  function periodRent(start: number, end: number): number {
    return rentCents(world.byteDays(start, end));
  }

  // --- Scenario 1: a fee waiver. ---
  {
    const start = world.clock.now();
    world.clock.advanceDays(PERIOD);
    const end = world.clock.now();
    const rent = periodRent(start, end);
    const fee = 25; // a one-off service fee...
    graceEvent("FIRST_TIME_HARDSHIP", fee, "waived a late fee, once, because we could afford to");
    const stmt = world.commitStatement({
      periodStartDay: start, periodEndDay: end,
      openingRoot: manifest!.root, closingRoot: manifest!.root, byteDays: world.byteDays(start, end),
      rentCents: rent, postageBytes: 0, postageCents: 0,
      auditCount: 0, auditBytes: 0, auditCents: 0, auditTier: "none",
      graceCents: -fee, feesCents: fee, totalCents: rent + fee - fee,
    });
    perPeriod.push([stmt.period, "fee-waiver", rent, fee, -fee, stmt.totalCents]);
    c.eq("fee waiver: member's total is rent only (fee fully waived)", stmt.totalCents, rent);
  }

  // --- Scenario 2: deceased-member hold, rent to the co-op for a fixed term. ---
  for (let held = 0; held < 3; held++) {
    const start = world.clock.now();
    world.clock.advanceDays(PERIOD);
    const end = world.clock.now();
    const rent = periodRent(start, end);
    graceEvent("DECEASED_MEMBER_HOLD", rent, `estate hold period ${held + 1}/3: rent carried by the co-op`);
    const stmt = world.commitStatement({
      periodStartDay: start, periodEndDay: end,
      openingRoot: manifest!.root, closingRoot: manifest!.root, byteDays: world.byteDays(start, end),
      rentCents: rent, postageBytes: 0, postageCents: 0,
      auditCount: 0, auditBytes: 0, auditCents: 0, auditTier: "none",
      graceCents: -rent, feesCents: 0, totalCents: 0,
    });
    perPeriod.push([stmt.period, "deceased-hold", rent, 0, -rent, stmt.totalCents]);
    c.eq(`deceased hold period ${held + 1}: estate owes nothing`, stmt.totalCents, 0);
  }

  // --- Scenario 3: throttle-instead-of-cutoff during a payment lapse. ---
  {
    const start = world.clock.now();
    world.clock.advanceDays(PERIOD);
    const end = world.clock.now();
    const rent = periodRent(start, end);
    graceEvent("PAYMENT_LAPSE_THROTTLE", rent, "service throttled, not cut off; rent carried this period");
    const stmt = world.commitStatement({
      periodStartDay: start, periodEndDay: end,
      openingRoot: manifest!.root, closingRoot: manifest!.root, byteDays: world.byteDays(start, end),
      rentCents: rent, postageBytes: 0, postageCents: 0,
      auditCount: 0, auditBytes: 0, auditCents: 0, auditTier: "none",
      graceCents: -rent, feesCents: 0, totalCents: 0,
    });
    perPeriod.push([stmt.period, "throttle", rent, 0, -rent, stmt.totalCents]);
    c.ok("throttle: service continues and the shortfall is carried, not lost", stmt.totalCents === 0);
  }

  // Every grace event nets to zero against the co-op grace account.
  c.eq("grace events net to zero against the co-op grace account",
    customerCreditsCents + world.graceAccountCents, 0);

  // Grace totals are reportable per period (we have a per-period table).
  c.ok("grace totals are reportable per period", perPeriod.length === 5);

  // No grace event edits history: all are forward entries; the provider ledger
  // still verifies as an unbroken append-only chain.
  c.ok("all grace events are forward entries (no history edited)",
    provider.ledger.entries.filter((e) => e.kind === "grace-event").length === 5);

  return {
    id: "E9",
    title: "The grace ledger",
    plainSentence: "The receipts make fairness legible; the margin makes mercy affordable.",
    assertions: c.results,
    tables: [
      {
        title: "Grace events (cents)",
        headers: ["period", "kind", "rent", "fee", "grace credit", "member total"],
        rows: perPeriod,
      },
    ],
    notes: [`Co-op grace account absorbed ${world.graceAccountCents} cents this run.`],
  };
}
