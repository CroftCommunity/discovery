// A simulated clock. Everything time-stamped in the ledgers reads its "now" from
// here, and time only advances when an experiment explicitly advances it. That
// keeps timestamps reproducible (no Date.now()) and lets rent — measured in
// byte-days — be integrated over a known, deterministic timeline.

const MS_PER_DAY = 24 * 60 * 60 * 1000;

export class SimClock {
  /** Whole days elapsed since the fixed epoch. */
  private day: number;
  private readonly epochMs: number;

  constructor(epochISO = "2020-01-01T00:00:00.000Z") {
    this.epochMs = Date.parse(epochISO);
    this.day = 0;
  }

  now(): number {
    return this.day;
  }

  iso(): string {
    return new Date(this.epochMs + this.day * MS_PER_DAY).toISOString();
  }

  advanceDays(n: number): void {
    if (n < 0) throw new Error("time does not run backward in this simulation");
    this.day += n;
  }
}
