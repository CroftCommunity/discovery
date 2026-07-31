// Deterministic simulated time.
//
// SEAM: production timestamps come from wall clocks and are compared with skew
// tolerance. Here time is an integer day counter advanced explicitly by the
// experiment, so runs are reproducible and byte-day rent is exact arithmetic
// rather than a floating wall-clock integral.

export const DAYS_PER_PERIOD = 30;

/** A fixed, arbitrary epoch. Day 0 of the whole simulation. */
export const EPOCH_DAY = 0;

export class SimClock {
  private day: number;

  constructor(startDay: number = EPOCH_DAY) {
    this.day = startDay;
  }

  now(): number {
    return this.day;
  }

  advance(days: number): number {
    if (days < 0) throw new Error("time only moves forward");
    this.day += days;
    return this.day;
  }

  /** Which billing period (0-indexed) the current day falls in. */
  period(): number {
    return Math.floor((this.day - EPOCH_DAY) / DAYS_PER_PERIOD);
  }

  /** First and last day (inclusive) of a given period index. */
  static periodBounds(period: number): { start: number; end: number } {
    const start = EPOCH_DAY + period * DAYS_PER_PERIOD;
    return { start, end: start + DAYS_PER_PERIOD - 1 };
  }
}
