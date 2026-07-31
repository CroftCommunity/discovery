// Co-op financial futures: yearly profit-and-revenue curves the E11 royalty
// instrument is simulated against.
//
// A "future" is one hypothetical trajectory for the co-op's books — a series of
// years, each carrying that year's profit and revenue. All figures are in
// integer cents so the royalty arithmetic downstream is exact to the cent.
//
// Four shapes are modelled, per Part 3 E11:
//   - flat:    steady profit and revenue every year (the closed-form anchor).
//   - linear:  profit and revenue grow by a fixed step each year.
//   - s-curve: slow start, steep middle, saturating tail (logistic-ish).
//   - downside: contains genuine loss years (negative profit), to exercise the
//               "share downside by waiting, not by compounding" property.
//
// These are inputs, not secrets: in E12 the Funder receives the same curves as
// public diligence material and recomputes the royalty schedule from them.

export interface YearPoint {
  year: number; // 0-indexed
  profitCents: number; // may be negative in a loss year
  revenueCents: number; // gross revenue, always >= 0
}

export interface Future {
  name: string;
  years: YearPoint[];
}

const HORIZON = 30; // simulate a 30-year horizon; long enough to show non-extinguishing cases

/** Steady state: same profit and revenue every year. */
export function flatFuture(profitCents: number, revenueCents: number, years = HORIZON): Future {
  const pts: YearPoint[] = [];
  for (let y = 0; y < years; y++) pts.push({ year: y, profitCents, revenueCents });
  return { name: "flat", years: pts };
}

/** Linear growth: profit and revenue rise by a fixed step from a base. */
export function linearFuture(
  baseProfit: number,
  profitStep: number,
  baseRevenue: number,
  revenueStep: number,
  years = HORIZON,
): Future {
  const pts: YearPoint[] = [];
  for (let y = 0; y < years; y++) {
    pts.push({
      year: y,
      profitCents: baseProfit + profitStep * y,
      revenueCents: baseRevenue + revenueStep * y,
    });
  }
  return { name: "linear", years: pts };
}

/**
 * S-curve (logistic) growth toward a ceiling. Slow early years, a steep middle,
 * a saturating tail. Integer-cent outputs via rounding; deterministic (no PRNG).
 */
export function sCurveFuture(
  profitCeiling: number,
  revenueCeiling: number,
  steepness: number,
  midpointYear: number,
  years = HORIZON,
): Future {
  const pts: YearPoint[] = [];
  for (let y = 0; y < years; y++) {
    const logistic = 1 / (1 + Math.exp(-steepness * (y - midpointYear)));
    pts.push({
      year: y,
      profitCents: Math.round(profitCeiling * logistic),
      revenueCents: Math.round(revenueCeiling * logistic),
    });
  }
  return { name: "s-curve", years: pts };
}

/**
 * A future containing loss years. Revenue is always positive (the co-op still
 * takes money in), but some years post negative profit. The royalty instrument
 * must charge zero in loss years without any penalty entry — downside is shared
 * by waiting, not by compounding.
 */
export function downsideFuture(years = HORIZON): Future {
  const pts: YearPoint[] = [];
  for (let y = 0; y < years; y++) {
    // Years 3, 4, 8 are loss years (negative profit); revenue dips but stays > 0.
    const lossYear = y === 3 || y === 4 || y === 8;
    const profitCents = lossYear ? -2_000_000 : 3_000_000 + y * 200_000;
    const revenueCents = lossYear ? 6_000_000 : 12_000_000 + y * 300_000;
    pts.push({ year: y, profitCents, revenueCents });
  }
  return { name: "downside", years: pts };
}

export type RoyaltyBase = "profit" | "revenue";

/** The base amount the royalty rate is applied to in a given year. */
export function baseAmount(pt: YearPoint, base: RoyaltyBase): number {
  if (base === "profit") return pt.profitCents;
  return pt.revenueCents;
}
