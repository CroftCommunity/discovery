// E11. The financing ledger: extinguishing royalty — "The return is bounded
// because the extraction is bounded; the ledger is how we keep that promise."
//
// The cap table is just another balance-forward ledger. An investor invests P
// and receives royalties until cumulative payout reaches m*P, then the
// obligation extinguishes permanently. Several investors hold pro-rata slices of
// one pool and extinguish simultaneously. We model four co-op financial futures
// (flat, linear, S-curve, downside-with-loss-years) across a grid of rates and
// bases, and drive one representative future year-by-year: each year co-signs a
// royalty entry per investor and a statement that commits the royalty figures
// into the E4 chain, with the extinguishment marked as its own signed entry.
//
// Adversarial: (a) a payment after extinguishment is rejected; (b) a rewritten
// historical royalty figure breaks the statement chain at exactly that link;
// (c) a loss year charges zero with the obligation unchanged and no penalty.

import assert from "node:assert/strict";
import { join } from "node:path";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Actor } from "../src/actor.ts";
import {
  flatFuture,
  linearFuture,
  sCurveFuture,
  downsideFuture,
  type Future,
  type RoyaltyBase,
} from "../src/futures.ts";
import {
  computeSchedule,
  flatYearsToExtinguish,
  assertPayable,
  type Investor,
} from "../src/royalty.ts";
import {
  coSignStatement,
  GENESIS_PREV,
  verifyChain,
  verifyStatement,
  type Statement,
  type StatementBody,
} from "../src/statements.ts";
import { DAYS_PER_PERIOD } from "../src/time.ts";

const M = 3; // cap multiple: total obligation is m*P

export function run(seed: number): ExperimentResult {
  const w = createWorld("E11", seed);
  const { customer, provider, coop } = w;

  // Three investors holding pro-rata slices of one royalty pool. Bram is the
  // early investor from the narrative; principals are round dollar figures (in
  // cents) so m*P_i is an exact integer per investor.
  const investorActors = [
    new Actor(seed, "investor-bram", join(w.dir, "investor-bram.jsonl")),
    new Actor(seed, "investor-cleo", join(w.dir, "investor-cleo.jsonl")),
    new Actor(seed, "investor-dai", join(w.dir, "investor-dai.jsonl")),
  ];
  // Mutual pinning: the co-op and each investor co-sign royalty entries.
  for (const inv of investorActors) {
    coop.pin(inv);
    inv.pin(coop);
  }
  const investors: Investor[] = [
    { id: investorActors[0].id, principalCents: 1_000_000 }, // $10,000
    { id: investorActors[1].id, principalCents: 600_000 }, //  $6,000
    { id: investorActors[2].id, principalCents: 400_000 }, //  $4,000
  ];
  const totalP = investors.reduce((s, i) => s + i.principalCents, 0); // $20,000
  const cap = M * totalP; // $60,000

  // --- The grid of futures × rates × bases (the sensitivity table). ---
  const futures: Future[] = [
    flatFuture(4_000_000, 10_000_000), // $40k profit, $100k revenue, steady
    linearFuture(2_000_000, 300_000, 8_000_000, 700_000),
    sCurveFuture(6_000_000, 15_000_000, 0.6, 8),
    downsideFuture(),
  ];
  const rates = [0.01, 0.05, 0.1];
  const bases: RoyaltyBase[] = ["profit", "revenue"];

  interface GridCell {
    future: string;
    rate: number;
    base: RoyaltyBase;
    yearsToExtinguish: number | null; // 1-indexed count, null = not within horizon
    finalCumulative: number;
  }
  const grid: GridCell[] = [];
  for (const fut of futures) {
    for (const rate of rates) {
      for (const base of bases) {
        const sched = computeSchedule(fut, rate, base, investors, M);
        const yte = sched.extinguishYear === null ? null : sched.extinguishYear + 1;
        grid.push({
          future: fut.name,
          rate,
          base,
          yearsToExtinguish: yte,
          finalCumulative: sched.years[sched.years.length - 1].cumulativePoolCents,
        });

        // Every extinguishing scenario pays exactly m*P, to the cent.
        if (yte !== null) {
          assert.equal(
            sched.years[sched.extinguishYear!].cumulativePoolCents,
            cap,
            `${fut.name} r=${rate} ${base}: cumulative payout must equal exactly m*P at extinguishment`,
          );
        }
        // Pro-rata splits sum exactly to the pool payment every year.
        for (const yr of sched.years) {
          const splitSum = yr.perInvestor.reduce((s, p) => s + p.paymentCents, 0);
          assert.equal(splitSum, yr.poolPaidCents, `${fut.name} r=${rate} ${base} y${yr.year}: splits must sum to the pool payment`);
        }
      }
    }
  }

  // Flat-case closed form as a sanity anchor: years = ceil(cap / (r*base)).
  // Flat revenue base at r=0.05: yearlyDue = 0.05 * $100k = $5,000; cap $60k → 12 years.
  {
    const flat = futures[0];
    const rate = 0.05;
    const yearlyDueRevenue = Math.round(rate * 10_000_000); // 500,000 cents
    const closed = flatYearsToExtinguish(cap, yearlyDueRevenue); // 12
    const sched = computeSchedule(flat, rate, "revenue", investors, M);
    assert.equal(closed, 12, "closed form: flat/revenue/0.05 extinguishes in 12 years");
    assert.equal(sched.extinguishYear! + 1, closed, "flat-case simulation matches the closed form");
  }

  // The sensitivity table must make visible that a low rate on a small base
  // produces multi-decade or non-extinguishing outcomes (the misalignment to
  // avoid): flat/profit at r=0.01 never extinguishes within the 30-year horizon.
  {
    const flat = futures[0];
    const sched = computeSchedule(flat, 0.01, "profit", investors, M);
    // yearlyDue = 0.01 * $40k = $400; cap $60k → 150 years, far past the horizon.
    assert.equal(sched.extinguishYear, null, "low rate on a small base does not extinguish within the horizon");
  }

  // --- Drive one representative future year by year, on-ledger. ---
  // Downside + revenue + r=0.05: revenue is positive so royalties flow, but loss
  // years (3, 4, 8) charge zero — exercising adversarial (c) inside the chain.
  const chosenFuture = futures[3]; // downside
  const chosenRate = 0.05;
  const chosenBase: RoyaltyBase = "revenue";
  const sched = computeSchedule(chosenFuture, chosenRate, chosenBase, investors, M);
  assert.ok(sched.extinguishYear !== null, "the driven scenario extinguishes within the horizon");

  const statements: Statement[] = [];
  let prevHash = GENESIS_PREV;
  // Only simulate through the extinguishment year plus one trailing year (to
  // prove post-extinguishment years pay nothing).
  const lastYear = sched.extinguishYear! + 1;
  for (let y = 0; y <= lastYear; y++) {
    const yr = sched.years[y];
    const closeDay = y * DAYS_PER_PERIOD + DAYS_PER_PERIOD - 1;

    // Loss-year check (c): zero due, obligation unchanged, no penalty entry.
    if (yr.lossYear) {
      assert.equal(yr.poolPaidCents, 0, `loss year ${y}: royalty due is zero`);
      // "obligation unchanged": cumulative did not move across the loss year.
      const prevCum = y > 0 ? sched.years[y - 1].cumulativePoolCents : 0;
      assert.equal(yr.cumulativePoolCents, prevCum, `loss year ${y}: cumulative (obligation remaining) is unchanged`);
    }

    // A co-signed royalty entry per investor per year (skip zero-payment years'
    // empty rows, but always emit on paying years).
    for (const pay of yr.perInvestor) {
      const invActor = investorActors.find((a) => a.id === pay.investorId)!;
      const body = {
        instrument: "extinguishing-royalty",
        year: y,
        base: chosenBase,
        rate: chosenRate,
        baseAmountCents: yr.baseAmountCents,
        poolPaidCents: yr.poolPaidCents,
        investorId: pay.investorId,
        paymentCents: pay.paymentCents,
        cumulativeCents: pay.cumulativeCents,
        capCents: pay.capCents,
      };
      const coopSig = coop.sign(body);
      const invSig = invActor.sign(body);
      const entry = { ...body, coopSig, investorSig: invSig, payer: coop.id, payee: pay.investorId };
      coop.ledger.append("royalty_payment", closeDay, entry);
      invActor.ledger.append("royalty_payment", closeDay, entry);
      // Both signatures verify under the pinned peer keys.
      assert.equal(coop.verifyFrom(pay.investorId, body, invSig), true, "investor co-signature verifies");
      assert.equal(invActor.verifyFrom(coop.id, body, coopSig), true, "co-op signature verifies");
    }

    // The statement for this period commits the royalty figures into the E4
    // chain (so editing a royalty figure later breaks the chain here).
    const body: StatementBody = {
      period: y,
      openingRoot: "royalty-books",
      closingRoot: "royalty-books",
      rentByteDays: 0,
      postageBytes: 0,
      auditCount: 0,
      auditBytes: 0,
      fees: 0,
      graceNet: 0,
      prevStatementHash: prevHash,
      closeDay,
      royaltyPoolCents: yr.poolPaidCents,
      royaltyCumulativeCents: yr.cumulativePoolCents,
      extinguished: yr.extinguished,
    };
    const stmt = coSignStatement(customer, provider, body, closeDay);
    statements.push(stmt);
    prevHash = stmt.hash;

    // Mark the extinguishment event as its own signed entry.
    if (sched.extinguishYear === y) {
      const extBody = { instrument: "extinguishing-royalty", year: y, cumulativeCents: yr.cumulativePoolCents, capCents: cap };
      coop.ledger.append("royalty_extinguished", closeDay, extBody);
      for (const invActor of investorActors) invActor.ledger.append("royalty_extinguished", closeDay, extBody);
    }
  }

  // The chain verifies end to end.
  assert.equal(verifyChain(customer, statements).ok, true, "the royalty statement chain verifies");

  // Cumulative payout equals exactly m*P, to the cent.
  assert.equal(sched.years[sched.extinguishYear!].cumulativePoolCents, cap, "cumulative payout equals exactly m*P");

  // All investors extinguish simultaneously under identical terms: on the
  // extinguishment year each investor's cumulative equals its own cap m*P_i.
  const extYr = sched.years[sched.extinguishYear!];
  for (let i = 0; i < investors.length; i++) {
    assert.equal(
      extYr.perInvestor[i].cumulativeCents,
      M * investors[i].principalCents,
      `investor ${i} reaches exactly m*P_i on the extinguishment year`,
    );
  }
  // Post-extinguishment year pays zero to everyone.
  const afterYr = sched.years[sched.extinguishYear! + 1];
  assert.equal(afterYr.poolPaidCents, 0, "the year after extinguishment pays zero");
  assert.ok(afterYr.perInvestor.every((p) => p.paymentCents === 0), "no investor is paid after extinguishment");

  // Adversarial (a): a payment attempted after extinguishment is rejected.
  assert.throws(
    () => assertPayable(sched, sched.extinguishYear! + 1, 1),
    /extinguished/,
    "a payment after extinguishment must be rejected",
  );

  // Adversarial (b): rewrite a historical royalty figure. The chain verification
  // from genesis fails at exactly that link, and the untouched earlier period
  // still verifies.
  const tampered: Statement[] = statements.map((s) => ({ ...s, body: { ...s.body } }));
  const editYear = 5;
  tampered[editYear].body.royaltyPoolCents = (tampered[editYear].body.royaltyPoolCents ?? 0) + 100;
  const broken = verifyChain(customer, tampered);
  assert.equal(broken.ok, false, "a rewritten royalty figure must break the chain");
  assert.equal(broken.brokenAt, editYear, "the break is located at exactly the edited period");
  assert.equal(verifyStatement(customer, tampered[editYear - 1]), true, "the untouched earlier period still verifies");

  const sentence = "The return is bounded because the extraction is bounded; the ledger is how we keep that promise.";

  // Sensitivity table: years-to-extinguish across the grid.
  const cell = (future: string, base: RoyaltyBase, rate: number): string => {
    const g = grid.find((x) => x.future === future && x.base === base && x.rate === rate)!;
    return g.yearsToExtinguish === null ? "— (>30y)" : `${g.yearsToExtinguish}y`;
  };
  const rows: string[] = [];
  for (const fut of futures) {
    for (const base of bases) {
      rows.push(
        `| ${fut.name} | ${base} | ${cell(fut.name, base, 0.01)} | ${cell(fut.name, base, 0.05)} | ${cell(fut.name, base, 0.1)} |`,
      );
    }
  }
  const table = [
    "| Future | Base | r=0.01 | r=0.05 | r=0.10 |",
    "| --- | --- | ---: | ---: | ---: |",
    ...rows,
  ].join("\n");

  const dollars = (c: number): string => `$${(c / 100).toLocaleString("en-US")}`;
  const reportMd = [
    `Instrument: principal P per investor, cap multiple m=${M} (total obligation ${dollars(cap)} on`,
    `${dollars(totalP)} invested across three pro-rata investors), royalty until the cap, then`,
    "extinguish. Years-to-extinguish across the grid of futures, rates, and bases:",
    "",
    table,
    "",
    "Closed-form anchor (flat / revenue / r=0.05): years = ceil(cap / (r·base)) =",
    `ceil(${dollars(cap)} / ${dollars(500_000)}) = 12y — matched exactly by the simulation. The`,
    "grid makes the misalignment visible: a low rate on a small base (e.g. flat/profit at r=0.01)",
    "runs multi-decade or never extinguishes within the 30-year horizon.",
    "",
    `Driven on-ledger: downside/revenue/r=0.05 extinguished at year ${sched.extinguishYear! + 1}, paying`,
    `exactly ${dollars(cap)} cumulative to the cent. Loss years (3, 4, 8) charged zero with the`,
    "obligation unchanged and no penalty entry; all three investors reached m·P_i on the same year;",
    "a post-extinguishment payment was rejected; and rewriting a historical royalty figure broke",
    `the co-signed statement chain at exactly period ${editYear}.`,
  ].join("\n");

  return { id: "E11", title: "The financing ledger: extinguishing royalty", sentence, reportMd };
}
