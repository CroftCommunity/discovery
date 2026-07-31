// Shared harness for the per-experiment `node --test` wrappers.
//
// Each wrapper registers one test that runs its experiment (which asserts
// internally, throwing on failure) and confirms it returns the expected result
// shape. Running from the package root keeps ./ledgers where the experiments
// expect it.

import { test } from "node:test";
import assert from "node:assert/strict";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import type { ExperimentResult } from "../src/experiment.ts";

process.chdir(join(dirname(fileURLToPath(import.meta.url)), ".."));

const BASE_SEED = 424242;

export function registerExperiment(
  id: string,
  title: string,
  fn: (seed: number) => ExperimentResult,
  seedOffset: number,
): void {
  test(`${id} — ${title}`, () => {
    const r = fn(BASE_SEED + seedOffset);
    assert.equal(r.id, id, "experiment returns its own id");
    assert.ok(r.sentence.length > 0, "experiment returns a plain-language sentence");
    assert.ok(r.reportMd.length > 0, "experiment returns a report section");
  });
}
