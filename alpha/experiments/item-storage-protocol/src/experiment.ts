// Shared shape for an experiment result and the report it feeds.
//
// Every experiment (E0..E10) runs its steps, makes its assertions (throwing on
// failure), and returns this: the plain-language sentence Part 1/Part 4 promise,
// plus a Markdown section for RUN_REPORT.md. A green run of the whole suite is
// the demo; the report is the narrative.

export interface ExperimentResult {
  id: string;
  title: string;
  /** The one-line plain-language sentence, printed and collected into the report. */
  sentence: string;
  /** Markdown body for this experiment's RUN_REPORT.md section (no heading). */
  reportMd: string;
}

export type Experiment = (seed: number) => ExperimentResult;
