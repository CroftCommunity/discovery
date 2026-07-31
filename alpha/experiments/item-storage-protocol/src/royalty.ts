// E11. The extinguishing royalty instrument.
//
// The cap table is just another balance-forward ledger. An investor puts in
// principal P; the co-op returns royalties until cumulative payout reaches a cap
// of m*P, then the obligation extinguishes permanently — no interest, no accrual
// in loss years, no residual claim after extinguishment. Several investors hold
// pro-rata slices of one royalty pool and, under identical terms, all extinguish
// on the same year.
//
// All money is integer cents so cumulative payout lands on exactly m*P, to the
// cent, with a partial final payment where needed.
//
// This module is the ACTORS' implementation of the instrument. The E12 Funder
// re-derives the same schedule independently in funder/ (no shared code) to
// check the co-op's royalty ledger from the files alone.

import { baseAmount, type Future, type RoyaltyBase } from "./futures.ts";

export interface Investor {
  /** Actor id (did:croft-mock:...) of the investor. */
  id: string;
  /** Principal contributed, in cents. */
  principalCents: number;
}

export interface InvestorPayment {
  investorId: string;
  paymentCents: number; // this year's payment to this investor
  cumulativeCents: number; // cumulative to this investor through this year
  capCents: number; // this investor's own cap, m * principal
}

export interface RoyaltyYear {
  year: number;
  base: RoyaltyBase;
  rate: number;
  baseAmountCents: number; // the base the rate was applied to (0 in loss years)
  lossYear: boolean; // profit < 0: royalty due is zero, obligation unchanged
  poolDueCents: number; // r*base before the cap clamp (0 in loss years)
  poolPaidCents: number; // after the cap clamp (partial on the extinguishing year)
  cumulativePoolCents: number;
  extinguished: boolean; // true on the year cumulative reaches the cap
  perInvestor: InvestorPayment[];
}

export interface RoyaltySchedule {
  m: number;
  totalPrincipalCents: number;
  capCents: number; // m * totalPrincipal
  years: RoyaltyYear[];
  extinguishYear: number | null; // year index the cap is reached, or null if never
}

/**
 * Largest-remainder apportionment of `amount` across `weights`, returning
 * integers that sum EXACTLY to `amount`. Deterministic tie-break by index.
 *
 * Applied to the CUMULATIVE pool each year (not the yearly delta), so per-
 * investor cumulative tracks slice_i * cumulativePool with integer rounding.
 * On the final year cumulativePool == cap == m*totalP and slice_i*cap == m*P_i
 * is an integer, so the apportionment returns exactly m*P_i — every investor
 * extinguishes simultaneously, to the cent.
 */
export function largestRemainder(amount: number, weights: number[]): number[] {
  const total = weights.reduce((s, w) => s + w, 0);
  if (total === 0) return weights.map(() => 0);
  const exact = weights.map((w) => (amount * w) / total);
  const floors = exact.map((x) => Math.floor(x));
  let remainder = amount - floors.reduce((s, x) => s + x, 0);
  // Distribute the leftover units to the largest fractional parts.
  const order = exact
    .map((x, i) => ({ i, frac: x - Math.floor(x) }))
    .sort((a, b) => (b.frac - a.frac) || (a.i - b.i));
  const out = floors.slice();
  for (let j = 0; j < order.length && remainder > 0; j++, remainder--) {
    out[order[j].i] += 1;
  }
  return out;
}

/**
 * Compute the whole royalty schedule for one future, one (rate, base) pair, and
 * a set of investors. Pure arithmetic, no I/O, no signing — the ledger side
 * (e11) co-signs each row and chains it into a statement.
 */
export function computeSchedule(
  future: Future,
  rate: number,
  base: RoyaltyBase,
  investors: Investor[],
  m: number,
): RoyaltySchedule {
  const totalPrincipalCents = investors.reduce((s, inv) => s + inv.principalCents, 0);
  const capCents = m * totalPrincipalCents;
  const weights = investors.map((inv) => inv.principalCents);

  const years: RoyaltyYear[] = [];
  let cumulativePool = 0;
  let extinguishYear: number | null = null;
  let prevCumPerInvestor = investors.map(() => 0);

  for (const pt of future.years) {
    const lossYear = pt.profitCents < 0;
    // Loss year: royalty due is zero, obligation unchanged, no penalty. Downside
    // is shared by waiting, not by compounding.
    const rawBase = lossYear ? 0 : baseAmount(pt, base);
    const poolDue = rawBase <= 0 ? 0 : Math.round(rate * rawBase);

    let poolPaid = poolDue;
    if (cumulativePool + poolPaid >= capCents) {
      // Final (partial) payment: clamp so cumulative lands exactly on the cap.
      poolPaid = capCents - cumulativePool;
    }
    if (poolPaid < 0) poolPaid = 0;
    cumulativePool += poolPaid;

    const justExtinguished = extinguishYear === null && cumulativePool === capCents && capCents > 0;
    if (justExtinguished) extinguishYear = pt.year;

    // Apportion the CUMULATIVE pool so per-investor cumulative sums exactly and
    // the final year distributes exactly m*P_i to each.
    const cumPerInvestor = largestRemainder(cumulativePool, weights);
    const perInvestor: InvestorPayment[] = investors.map((inv, i) => ({
      investorId: inv.id,
      paymentCents: cumPerInvestor[i] - prevCumPerInvestor[i],
      cumulativeCents: cumPerInvestor[i],
      capCents: m * inv.principalCents,
    }));
    prevCumPerInvestor = cumPerInvestor;

    years.push({
      year: pt.year,
      base,
      rate,
      baseAmountCents: rawBase,
      lossYear,
      poolDueCents: poolDue,
      poolPaidCents: poolPaid,
      cumulativePoolCents: cumulativePool,
      extinguished: extinguishYear !== null && pt.year >= extinguishYear,
      perInvestor,
    });
  }

  return { m, totalPrincipalCents, capCents, years, extinguishYear };
}

/**
 * Closed form for the flat case: years-to-extinguish = ceil(cap / yearlyDue),
 * where yearlyDue = round(rate * base). The exact real-valued anchor is
 * cap / (rate*base); the integer simulation reaches the cap on the ceiling of
 * that. Returns null if the yearly due is zero (never extinguishes).
 */
export function flatYearsToExtinguish(capCents: number, yearlyDueCents: number): number | null {
  if (yearlyDueCents <= 0) return null;
  return Math.ceil(capCents / yearlyDueCents);
}

/**
 * Guard for adversarial (a): a payment attempted after extinguishment must be
 * rejected. The schedule pays zero once extinguished; this makes an explicit
 * post-extinguishment payment an error rather than a silent no-op.
 */
export function assertPayable(schedule: RoyaltySchedule, year: number, requestedCents: number): void {
  if (schedule.extinguishYear !== null && year > schedule.extinguishYear && requestedCents > 0) {
    throw new Error(
      `royalty payment of ${requestedCents} rejected: instrument extinguished at year ${schedule.extinguishYear}`,
    );
  }
}
