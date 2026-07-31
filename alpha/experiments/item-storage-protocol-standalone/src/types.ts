// Shared shapes for the experiment harness. Every experiment returns an
// ExperimentResult: a list of assertions (the deliverable), optional tables for
// the report, and the one plain-language sentence that closes it — printed in
// order, the sentences read as the Part 4 narrative.

export type Assertion = {
  name: string;
  ok: boolean;
  detail?: string;
};

export type ReportTable = {
  title: string;
  headers: string[];
  rows: (string | number)[][];
};

export type ExperimentResult = {
  id: string;
  title: string;
  plainSentence: string;
  assertions: Assertion[];
  tables: ReportTable[];
  notes: string[];
};

/** Records assertions as an experiment runs; the array is the deliverable. */
export class Checker {
  readonly results: Assertion[] = [];

  ok(name: string, cond: unknown, detail?: string): boolean {
    const pass = !!cond;
    this.results.push({ name, ok: pass, detail });
    return pass;
  }

  /** Assert two values are strictly equal (with a helpful detail on failure). */
  eq(name: string, actual: unknown, expected: unknown): boolean {
    const pass = actual === expected;
    return this.ok(
      name,
      pass,
      pass ? undefined : `expected ${String(expected)}, got ${String(actual)}`,
    );
  }

  get allOk(): boolean {
    return this.results.every((r) => r.ok);
  }
}
