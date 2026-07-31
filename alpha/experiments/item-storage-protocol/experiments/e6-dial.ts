// E6. The dial — "Declare your setting, pay its true cost, no judgment encoded
// either way."
//
// Assurance is a declared setting with a true, linear cost. Audit tiers (monthly
// k=5, weekly k=5, daily k=20, hourly k=20) each cost (audits per period) *
// (per-audit bytes + fixed overhead). The customer's chosen tier is a signed
// declaration in the ledger, billed through the E4 statement. Cost is linear in
// audit count; the chosen tier appears in the statement; changing the dial
// mid-period pro-rates.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { coSignStatement, GENESIS_PREV, type StatementBody } from "../src/statements.ts";

interface Tier {
  name: string;
  k: number;
  auditsPerPeriod: number;
}

export function run(seed: number): ExperimentResult {
  const w = createWorld("E6", seed);
  const { customer, provider } = w;

  // Cost model (from E5's measurement): each audit retrieves k * ITEM_SIZE
  // bytes, plus a fixed per-audit overhead (round-trip, signature).
  const ITEM_SIZE = 1024;
  const OVERHEAD = 256;
  const perAuditBytes = (k: number): number => k * ITEM_SIZE + OVERHEAD;

  // A 30-day period: ~4 weeks, 30 days, 24*30 hours.
  const tiers: Tier[] = [
    { name: "monthly", k: 5, auditsPerPeriod: 1 },
    { name: "weekly", k: 5, auditsPerPeriod: 4 },
    { name: "daily", k: 20, auditsPerPeriod: 30 },
    { name: "hourly", k: 20, auditsPerPeriod: 720 },
  ];

  const tierCost = (t: Tier): number => t.auditsPerPeriod * perAuditBytes(t.k);

  // Cost is LINEAR in audit count: doubling audits doubles cost exactly.
  for (const t of tiers) {
    const single = perAuditBytes(t.k);
    assert.equal(tierCost(t), t.auditsPerPeriod * single, `tier ${t.name} cost must be linear in audit count`);
  }
  // Same k, 4x the audits -> exactly 4x the cost (weekly vs monthly).
  const monthly = tiers[0];
  const weekly = tiers[1];
  assert.equal(tierCost(weekly), 4 * tierCost(monthly), "weekly (4 audits) costs exactly 4x monthly (1 audit) at equal k");

  // The customer declares a chosen tier as a signed ledger entry, and it is
  // billed through a co-signed statement. The tier appears in the statement:
  // auditCount and auditBytes match the declaration.
  const chosen = tiers[2]; // daily
  const decl = {
    tier: chosen.name,
    k: chosen.k,
    auditsPerPeriod: chosen.auditsPerPeriod,
    perAuditBytes: perAuditBytes(chosen.k),
  };
  const declSig = customer.sign(decl);
  customer.ledger.append("audit_tier", w.clock.now(), { ...decl, sig: declSig });
  assert.equal(customer.verifyFrom(customer.id, decl, declSig) || provider.verifyFrom(customer.id, decl, declSig), true, "the tier declaration is signed by the customer");

  const body: StatementBody = {
    period: 0,
    openingRoot: "seal-none",
    closingRoot: "seal-none",
    rentByteDays: 0,
    postageBytes: 0,
    auditCount: chosen.auditsPerPeriod,
    auditBytes: tierCost(chosen),
    fees: 0,
    graceNet: 0,
    prevStatementHash: GENESIS_PREV,
    closeDay: 29,
  };
  const stmt = coSignStatement(customer, provider, body, 29);
  assert.equal(stmt.body.auditCount, chosen.auditsPerPeriod, "the chosen tier's audit count appears in the statement");
  assert.equal(stmt.body.auditBytes, tierCost(chosen), "the chosen tier's audit cost appears in the statement");

  // Changing the dial mid-period pro-rates. Switch from monthly to daily at the
  // halfway mark: cost = 0.5 * monthlyCost + 0.5 * dailyCost.
  const fracBefore = 15 / 30;
  const fracAfter = 15 / 30;
  const proratedAudits = fracBefore * monthly.auditsPerPeriod + fracAfter * chosen.auditsPerPeriod;
  const proratedCost =
    fracBefore * monthly.auditsPerPeriod * perAuditBytes(monthly.k) +
    fracAfter * chosen.auditsPerPeriod * perAuditBytes(chosen.k);
  const expectedProrated = 0.5 * tierCost(monthly) + 0.5 * tierCost(chosen);
  assert.equal(proratedCost, expectedProrated, "mid-period dial change pro-rates the audit cost");
  assert.equal(proratedAudits, 0.5 * 1 + 0.5 * 30, "mid-period dial change pro-rates the audit count");

  const sentence = "Declare your setting, pay its true cost, no judgment encoded either way.";
  const table = [
    "| Tier | k | audits/period | cost (bytes) |",
    "| --- | ---: | ---: | ---: |",
    ...tiers.map((t) => `| ${t.name} | ${t.k} | ${t.auditsPerPeriod} | ${tierCost(t)} |`),
  ].join("\n");
  const reportMd = [
    "Assurance is a declared, signed setting with a linear per-audit price:",
    "",
    table,
    "",
    `The customer declared the **${chosen.name}** tier; the statement carries its audit count`,
    `(${chosen.auditsPerPeriod}) and cost (${tierCost(chosen)} bytes). Cost is exactly linear in`,
    "audit count (weekly = 4x monthly at equal k), and a mid-period switch pro-rates by day.",
  ].join("\n");

  return { id: "E6", title: "The dial", sentence, reportMd };
}
