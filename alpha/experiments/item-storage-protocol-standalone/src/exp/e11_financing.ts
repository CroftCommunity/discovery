// E11. The financing ledger: extinguishing royalty — the cap table is just another
// balance-forward ledger, run on the same receipt machinery as everything else.
// Investors hold pro-rata slices of one royalty pool. Each year the pool pays
// r * base, clamped so cumulative never exceeds m*P; the payout splits pro-rata
// into co-signed ledger entries; when the cap is reached the instrument extinguishes
// permanently. We model four financial futures and sweep (rate, base) to show
// years-to-extinguish, with the flat closed form as a sanity anchor.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult, type ReportTable } from "../types.ts";
import { Actor } from "../actor.ts";
import { verifyEntries } from "../ledger.ts";
import {
  simulateRoyalty, flatClosedFormYears, apportion, buildFutures, type Base,
} from "../financing.ts";
import type { Json } from "../canonical.ts";

const HORIZON = 40; // years
const M = 3;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { provider } = world;

  // Two investors holding pro-rata slices of one pool. Principal in cents.
  const bram = new Actor("Bram", world.masterSeed, "investor/bram");
  const cleo = new Actor("Cleo", world.masterSeed, "investor/cleo");
  world.investors.push(bram, cleo);
  world.pin(bram);
  world.pin(cleo);
  const holders = [
    { actor: bram, principal: 60_000, share: 0.6 },
    { actor: cleo, principal: 40_000, share: 0.4 },
  ];
  const poolPrincipal = holders.reduce((s, h) => s + h.principal, 0); // 100_000
  const cap = M * poolPrincipal; // 300_000 = m*P
  const shares = holders.map((h) => h.share);

  const futures = buildFutures(HORIZON);

  // --- Live scenario: flat future, r = 0.05, base = revenue. ---
  const R = 0.05;
  const BASE: Base = "revenue";
  const sim = simulateRoyalty(futures.flat, R, BASE, cap);

  // Emit co-signed ledger entries per investor per year, into the provider ledger
  // and each investor's ledger. The extinguishing year splits to the exact cap.
  const cumById: Record<string, number> = {};
  const investorCap: Record<string, number> = {};
  for (const h of holders) {
    cumById[h.actor.id] = 0;
    investorCap[h.actor.id] = M * h.principal;
  }
  let poolCumulative = 0;
  let extinguishYear: number | null = null;
  let splitsSumExactly = true;

  for (const yr of sim.years) {
    if (yr.due === 0) continue;
    const willExtinguish = poolCumulative + yr.due >= cap;
    let pays: number[];
    if (willExtinguish) {
      // Final payment: top each investor up to exactly m * their principal.
      pays = holders.map((h) => investorCap[h.actor.id] - cumById[h.actor.id]);
    } else {
      pays = apportion(yr.due, shares);
    }
    if (pays.reduce((a, b) => a + b, 0) !== yr.due) splitsSumExactly = false;
    holders.forEach((h, i) => {
      cumById[h.actor.id] += pays[i];
      const body: Json = {
        instrument: "extinguishing-royalty", year: yr.year, r: R, base: BASE,
        poolBaseValue: yr.baseValue, poolDue: yr.due,
        investorId: h.actor.id, payment: pays[i], investorCumulative: cumById[h.actor.id],
        investorCap: investorCap[h.actor.id],
      };
      // Co-signed: the co-op pays, the investor acknowledges receipt.
      provider.ledger.append("royalty", world.clock.iso(), body, [provider.signer(), h.actor.signer()]);
      h.actor.ledger.append("royalty", world.clock.iso(), body, [h.actor.signer(), provider.signer()]);
    });
    poolCumulative += yr.due;
    if (poolCumulative >= cap && extinguishYear === null) {
      extinguishYear = yr.year;
      provider.ledger.append("extinguishment", world.clock.iso(), {
        instrument: "extinguishing-royalty", year: yr.year, cumulative: poolCumulative, cap,
        note: "obligation fully satisfied; the instrument extinguishes permanently",
      }, [provider.signer(), bram.signer(), cleo.signer()]);
    }
  }

  // Cumulative payout equals exactly m*P, to the cent, final payment partial.
  c.eq("cumulative payout equals exactly m*P to the cent", poolCumulative, cap);
  c.eq("investors' cumulatives sum exactly to the cap",
    cumById[bram.id] + cumById[cleo.id], cap);
  c.eq("Bram extinguishes at exactly m * his principal", cumById[bram.id], M * 60_000);
  c.eq("Cleo extinguishes at exactly m * her principal", cumById[cleo.id], M * 40_000);
  c.ok("all investors extinguish simultaneously (one event)", extinguishYear !== null);
  c.ok("pro-rata splits sum exactly every year", splitsSumExactly);
  const finalYear = sim.years.find((y) => y.extinguished && y.due > 0)!;
  c.ok("final payment is partial (clamped to the cap)", finalYear.due < Math.round(R * 190_000));

  // Flat-case simulation matches the closed form (years = cap / (r*base)).
  const closed = flatClosedFormYears(cap, R, 190_000);
  c.eq("flat-case years-to-extinguish matches ceil(closed form)",
    sim.yearsToExtinguish, Math.ceil(closed));

  // Adversarial (a): a payment after extinguishment is rejected.
  const afterExtinguish = sim.years.filter((y) => y.year > (extinguishYear ?? 0));
  c.ok("no payment accrues after extinguishment",
    afterExtinguish.every((y) => y.due === 0));

  // Adversarial (b): rewrite a historical royalty figure; chain verification locates it.
  const clone = provider.ledger.entries.map((e) => ({ ...e, body: { ...(e.body as object) } }));
  const target = clone.findIndex((e) => e.kind === "royalty");
  (clone[target].body as Record<string, unknown>).payment = 999_999;
  const issues = verifyEntries(clone as typeof provider.ledger.entries, world.keyring);
  c.ok("editing a historical royalty figure breaks chain verification at that link",
    issues.some((i) => i.seq === target));

  // Adversarial (c): loss year (downside future, profit base) => due 0, no penalty.
  const downsideProfit = simulateRoyalty(futures.downside, 0.05, "profit", cap);
  const lossYears = downsideProfit.years.filter((y) => y.lossYear);
  c.ok("loss years exist in the downside future", lossYears.length > 0);
  c.ok("in a loss year royalty due is zero and the obligation is unchanged (no penalty)",
    lossYears.every((y, idx) => y.due === 0 && (idx === 0 || y.cumulative >= lossYears[idx - 1].cumulative)));

  // --- Sensitivity sweep: years-to-extinguish across futures x rates x bases. ---
  const rates = [0.02, 0.05, 0.1];
  const bases: Base[] = ["profit", "revenue"];
  const rows: (string | number)[][] = [];
  let sawNonExtinguishing = false;
  for (const fname of Object.keys(futures)) {
    for (const base of bases) {
      const cells: (string | number)[] = [fname, base];
      for (const r of rates) {
        const s = simulateRoyalty(futures[fname], r, base, cap);
        if (s.yearsToExtinguish === null) {
          cells.push(`>${HORIZON}`);
          sawNonExtinguishing = true;
        } else {
          cells.push(s.yearsToExtinguish);
        }
      }
      rows.push(cells);
    }
  }
  c.ok("a low rate on a small base fails to extinguish within the horizon (visible misalignment)",
    sawNonExtinguishing);

  const table: ReportTable = {
    title: `Years-to-extinguish (cap = m*P = ${cap} cents; horizon ${HORIZON}y)`,
    headers: ["future", "base", "r=0.02", "r=0.05", "r=0.10"],
    rows,
  };

  return {
    id: "E11",
    title: "The financing ledger: extinguishing royalty",
    plainSentence: "The return is bounded because the extraction is bounded; the ledger is how we keep that promise.",
    assertions: c.results,
    tables: [table],
    notes: [
      `Live scenario: flat future, r=${R}, base=${BASE}; extinguished at year ${extinguishYear} ` +
        `(closed form ${closed.toFixed(2)}). Pool cap ${cap} cents = m*P.`,
    ],
  };
}
