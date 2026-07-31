// The single command: run every experiment in dependency order, deterministically,
// asserting as it goes; print each experiment's plain-language sentence; then
// generate RUN_REPORT.md. Exit non-zero if any assertion fails.
//
//   node scripts/run.ts    (or: npm run run)

import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { rmSync } from "node:fs";
import type { ExperimentResult } from "../src/experiment.ts";
import { writeReport } from "../src/report.ts";

// Run from the package root so ./ledgers and ./RUN_REPORT.md land there
// regardless of the caller's working directory.
const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
process.chdir(ROOT);

import { run as e0 } from "../experiments/e0-identity.ts";
import { run as e1 } from "../experiments/e1-items.ts";
import { run as e2 } from "../experiments/e2-manifest.ts";
import { run as e3 } from "../experiments/e3-receipts.ts";
import { run as e4 } from "../experiments/e4-statements.ts";
import { run as e5 } from "../experiments/e5-spotcheck.ts";
import { run as e6 } from "../experiments/e6-dial.ts";
import { run as e7 } from "../experiments/e7-seal.ts";
import { run as e8 } from "../experiments/e8-tombstone.ts";
import { run as e9 } from "../experiments/e9-grace.ts";
import { run as e10 } from "../experiments/e10-erasure.ts";
import { run as e11 } from "../experiments/e11-royalty.ts";
import { run as e12 } from "../experiments/e12-outside-reader.ts";
import { run as e13 } from "../experiments/e13-covenants.ts";
import { run as e14 } from "../experiments/e14-soft-unit.ts";

const BASE_SEED = 424242;

const suite: { fn: (seed: number) => ExperimentResult; seed: number }[] = [
  { fn: e0, seed: BASE_SEED + 0 },
  { fn: e1, seed: BASE_SEED + 10 },
  { fn: e2, seed: BASE_SEED + 20 },
  { fn: e3, seed: BASE_SEED + 30 },
  { fn: e4, seed: BASE_SEED + 40 },
  { fn: e5, seed: BASE_SEED + 50 },
  { fn: e6, seed: BASE_SEED + 60 },
  { fn: e7, seed: BASE_SEED + 70 },
  { fn: e8, seed: BASE_SEED + 80 },
  { fn: e9, seed: BASE_SEED + 90 },
  { fn: e10, seed: BASE_SEED + 100 },
  { fn: e11, seed: BASE_SEED + 110 },
  { fn: e12, seed: BASE_SEED + 120 },
  { fn: e13, seed: BASE_SEED + 130 },
  { fn: e14, seed: BASE_SEED + 140 },
];

// Fresh ledger tree each run.
rmSync(join(ROOT, "ledgers"), { recursive: true, force: true });

const results: ExperimentResult[] = [];
let failures = 0;

console.log("Item Storage Protocol — running the experiment suite\n");
for (const { fn, seed } of suite) {
  try {
    const r = fn(seed);
    results.push(r);
    console.log(`  [PASS] ${r.id}  ${r.title}`);
    console.log(`         "${r.sentence}"`);
  } catch (err) {
    failures++;
    const e = err as Error;
    console.error(`  [FAIL] experiment threw: ${e.message}`);
    console.error(e.stack);
    break; // experiments are ordered by dependency; stop at the first failure
  }
}

console.log("");
if (failures === 0) {
  writeReport(join(ROOT, "RUN_REPORT.md"), results, {
    seed: BASE_SEED,
    generatedNote:
      "All assertions green. Ledgers are under `./ledgers/` (one JSONL file per actor per " +
      "experiment); verify every signature with `node scripts/verify-ledgers.ts`.",
  });
  console.log(
    `All ${results.length} experiments green. Wrote RUN_REPORT.md, DILIGENCE_REPORT.md, and ./ledgers/.`,
  );
  process.exit(0);
} else {
  console.error(`${failures} experiment(s) failed.`);
  process.exit(1);
}
