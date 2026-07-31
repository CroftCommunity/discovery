// The financing instrument: an "invest P, receive up to m*P, then stop forever"
// royalty. The cap table is just another balance-forward ledger. The return is
// bounded because the extraction is bounded: royalty due each year is r * base
// (profit or revenue), clamped so cumulative payout never exceeds m*P, with no
// interest, no accrual in loss years, and no residual claim after extinguishment.
//
// All amounts are integer cents so cumulative payout equals exactly m*P to the
// cent, with a partial final payment when the last year would overshoot.

export type YearPoint = { profit: number; revenue: number }; // cents
export type Base = "profit" | "revenue";

export type RoyaltyYear = {
  year: number;
  baseValue: number;
  due: number;
  cumulative: number;
  extinguished: boolean;
  lossYear: boolean;
};

export type RoyaltySim = {
  years: RoyaltyYear[];
  cumulative: number;
  yearsToExtinguish: number | null; // null = never within the horizon
};

/**
 * Pure year-by-year simulation of the pool. Royalty due = r * base, zero in loss
 * years (base <= 0), clamped so cumulative never exceeds the cap. Extinguishes
 * permanently once the cap is reached.
 */
export function simulateRoyalty(
  curve: YearPoint[],
  r: number,
  base: Base,
  cap: number,
): RoyaltySim {
  const years: RoyaltyYear[] = [];
  let cumulative = 0;
  let yearsToExtinguish: number | null = null;
  for (let y = 0; y < curve.length; y++) {
    const baseValue = base === "profit" ? curve[y].profit : curve[y].revenue;
    let due = 0;
    let lossYear = false;
    if (cumulative >= cap) {
      due = 0; // already extinguished: no further payments
    } else if (baseValue <= 0) {
      lossYear = true; // no accrual in loss years; obligation unchanged, no penalty
      due = 0;
    } else {
      due = Math.round(r * baseValue);
      if (cumulative + due > cap) due = cap - cumulative; // partial final payment
    }
    cumulative += due;
    const extinguished = cumulative >= cap;
    if (extinguished && yearsToExtinguish === null) yearsToExtinguish = y + 1;
    years.push({ year: y + 1, baseValue, due, cumulative, extinguished, lossYear });
  }
  return { years, cumulative, yearsToExtinguish };
}

/** Closed form for the flat case: years = cap / (r * base). */
export function flatClosedFormYears(cap: number, r: number, base: number): number {
  return cap / (r * base);
}

/**
 * Split an integer-cent amount across shares (fractions summing to 1) so the
 * parts sum EXACTLY to the amount (largest-remainder apportionment).
 */
export function apportion(amount: number, shares: number[]): number[] {
  const raw = shares.map((s) => amount * s);
  const floors = raw.map((x) => Math.floor(x));
  let remainder = amount - floors.reduce((a, b) => a + b, 0);
  const order = raw
    .map((x, i) => ({ i, frac: x - Math.floor(x) }))
    .sort((a, b) => b.frac - a.frac);
  const out = floors.slice();
  for (let n = 0; n < remainder; n++) out[order[n % order.length].i] += 1;
  return out;
}

// --- Four co-op financial futures, as yearly (profit, revenue) curves in cents ---

export function buildFutures(horizon: number): Record<string, YearPoint[]> {
  const flat: YearPoint[] = [];
  const linear: YearPoint[] = [];
  const scurve: YearPoint[] = [];
  const downside: YearPoint[] = [];
  for (let t = 1; t <= horizon; t++) {
    // Flat: steady.
    flat.push({ profit: 150_000, revenue: 190_000 });
    // Linear growth.
    linear.push({ profit: 30_000 + 6_000 * t, revenue: 60_000 + 12_000 * t });
    // S-curve (logistic) revenue; profit a fraction of it.
    const L = 500_000, k = 0.3, mid = 15;
    const rev = Math.round(L / (1 + Math.exp(-k * (t - mid))));
    scurve.push({ profit: Math.round(0.35 * rev), revenue: rev });
    // Downside: every fourth year is a loss year (negative profit), revenue steady.
    downside.push({ profit: t % 4 === 0 ? -40_000 : 55_000, revenue: 120_000 });
  }
  return { flat, linear, scurve, downside };
}
