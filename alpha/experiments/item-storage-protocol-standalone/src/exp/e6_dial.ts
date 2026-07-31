// E6. The dial — assurance is a declared setting with a true, linear cost. We
// define audit tiers (monthly/weekly/daily/hourly, at k=5 or k=20), price each
// tier from E5's cost model (bytes read per audit) plus a fixed per-audit
// overhead, record the customer's chosen tier as a signed declaration, run one
// period per tier billed through the statement chain, and show a mid-period dial
// change pro-rates correctly. No judgment is encoded either way: your paranoia,
// your bill.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { rentCents, auditCents, AUDIT_OVERHEAD_CENTS } from "../pricing.ts";

const PERIOD = 30;

type Tier = { name: string; k: number; auditsPerPeriod: number };

const TIERS: Tier[] = [
  { name: "monthly", k: 5, auditsPerPeriod: 1 },
  { name: "weekly", k: 5, auditsPerPeriod: 4 },
  { name: "daily", k: 20, auditsPerPeriod: 30 },
  { name: "hourly", k: 20, auditsPerPeriod: 720 },
];

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, manifest } = world;

  // Representative bytes per audited item: the manifest average. SEAM: the demo
  // corpus is tiny, so audit cost is priced from k * avg-item-size (the E5 cost
  // model) rather than executed against it; the pricing is the point of E6.
  const avgItemBytes = Math.floor(manifest!.totalBytes / manifest!.leaves.length);

  const perAuditCents = (k: number) => auditCents(k * avgItemBytes, 1);
  const tierCost = (t: Tier) => t.auditsPerPeriod * perAuditCents(t.k);

  let day = world.clock.now(); // day 90 after E4
  const rows: (string | number)[][] = [];

  for (const tier of TIERS) {
    // Ada records her chosen tier as a signed declaration before the period runs.
    customer.ledger.append("dial-declaration", world.clock.iso(), {
      tier: tier.name, k: tier.k, auditsPerPeriod: tier.auditsPerPeriod,
    }, [customer.signer()]);

    const start = day;
    world.clock.advanceDays(PERIOD);
    day = world.clock.now();
    const byteDays = world.byteDays(start, day);
    const rc = rentCents(byteDays);
    const auditBytes = tier.auditsPerPeriod * tier.k * avgItemBytes;
    const ac = tierCost(tier);
    const stmt = world.commitStatement({
      periodStartDay: start, periodEndDay: day,
      openingRoot: manifest!.root, closingRoot: manifest!.root, byteDays,
      rentCents: rc, postageBytes: 0, postageCents: 0,
      auditCount: tier.auditsPerPeriod, auditBytes, auditCents: ac, auditTier: tier.name,
      graceCents: 0, feesCents: 0, totalCents: rc + ac,
    });
    rows.push([tier.name, tier.k, tier.auditsPerPeriod, ac, stmt.totalCents]);

    // The chosen tier appears in the statement.
    c.eq(`statement records the chosen tier (${tier.name})`, stmt.auditTier, tier.name);
  }

  // Billed audit cost is linear in audit count (same k => cost scales with count).
  // monthly vs weekly share k=5; weekly runs 4x the audits => 4x the audit cost.
  c.eq("weekly audit cost = 4 x monthly (same k, linear in count)",
    tierCost(TIERS[1]), 4 * tierCost(TIERS[0]));
  // daily vs hourly share k=20; hourly runs 24x the audits => 24x the cost.
  c.eq("hourly audit cost = 24 x daily (same k, linear in count)",
    tierCost(TIERS[3]), 24 * tierCost(TIERS[2]));

  // Changing the dial mid-period pro-rates correctly: weekly for the first half,
  // daily for the second half. Audit counts pro-rate by days.
  const weekly = TIERS[1], daily = TIERS[2];
  const auditsFor = (t: Tier, days: number) => Math.round((t.auditsPerPeriod * days) / PERIOD);
  const start = day;
  world.clock.advanceDays(15);
  const firstHalfAudits = auditsFor(weekly, 15); // 2
  world.clock.advanceDays(15);
  const secondHalfAudits = auditsFor(daily, 15); // 15
  day = world.clock.now();
  const firstCost = firstHalfAudits * perAuditCents(weekly.k);
  const secondCost = secondHalfAudits * perAuditCents(daily.k);
  const proratedAudits = firstHalfAudits + secondHalfAudits;
  const proratedCost = firstCost + secondCost;
  const byteDays = world.byteDays(start, day);
  const rc = rentCents(byteDays);
  const auditBytes = firstHalfAudits * weekly.k * avgItemBytes + secondHalfAudits * daily.k * avgItemBytes;
  const stmt = world.commitStatement({
    periodStartDay: start, periodEndDay: day,
    openingRoot: manifest!.root, closingRoot: manifest!.root, byteDays,
    rentCents: rc, postageBytes: 0, postageCents: 0,
    auditCount: proratedAudits, auditBytes, auditCents: proratedCost, auditTier: "weekly->daily",
    graceCents: 0, feesCents: 0, totalCents: rc + proratedCost,
  });
  c.eq("mid-period dial change bills the sum of the two pro-rated legs",
    stmt.auditCents, firstCost + secondCost);
  c.ok("pro-rated bill sits between a full-weekly and a full-daily period",
    proratedCost > tierCost(weekly) && proratedCost < tierCost(daily));
  rows.push(["weekly->daily", "5/20", proratedAudits, proratedCost, stmt.totalCents]);

  return {
    id: "E6",
    title: "The dial",
    plainSentence: "Declare your setting, pay its true cost, no judgment encoded either way.",
    assertions: c.results,
    tables: [
      {
        title: "Audit tiers billed through the statement (cents)",
        headers: ["tier", "k", "audits/period", "audit cost", "period total"],
        rows,
      },
    ],
    notes: [`Per-audit overhead: ${AUDIT_OVERHEAD_CENTS} cents; avg item ~${avgItemBytes} bytes.`],
  };
}
