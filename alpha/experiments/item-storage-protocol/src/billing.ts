// Rent as byte-days: the storage bill integrated over a period from the manifest
// timeline. This is the one piece of the bill that depends on time rather than
// on discrete events, so it gets its own small, exact integrator.
//
// The timeline is a step function: a sorted list of change points, each saying
// "from this day onward, this many bytes are at rest." Rent for a period is the
// sum, over each day in the period, of bytes at rest that day. Because time is
// an integer day counter (time.ts), this is exact integer arithmetic — the
// customer can recompute it to the byte-day.

export interface TimelinePoint {
  day: number;
  bytesAtRest: number;
}

export class ManifestTimeline {
  private readonly points: TimelinePoint[] = [];

  /** Record that from `day` onward, `bytesAtRest` bytes are stored. */
  set(day: number, bytesAtRest: number): void {
    this.points.push({ day, bytesAtRest });
    this.points.sort((a, b) => a.day - b.day);
  }

  /** Bytes at rest on a given day (the most recent change point at or before it). */
  bytesOn(day: number): number {
    let bytes = 0;
    for (const p of this.points) {
      if (p.day <= day) bytes = p.bytesAtRest;
      else break;
    }
    return bytes;
  }

  /** Byte-days over the inclusive day range [start, end]. */
  byteDays(start: number, end: number): number {
    let total = 0;
    for (let d = start; d <= end; d++) total += this.bytesOn(d);
    return total;
  }
}
