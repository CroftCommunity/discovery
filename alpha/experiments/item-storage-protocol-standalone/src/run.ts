// The single command. Runs the whole suite deterministically, writes each actor's
// append-only ledger to ./ledgers/, writes the pinned keyring, generates
// RUN_REPORT.md, prints each experiment's plain-language sentence in order, and
// exits non-zero if any assertion failed. A green run of this is the demo.

import { mkdirSync, writeFileSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import { runSuite, tally } from "./suite.ts";
import { generateReport } from "./report.ts";
import type { Actor } from "./actor.ts";

const ROOT = resolve(import.meta.dirname, "..");
const LEDGER_DIR = resolve(ROOT, "ledgers");

function fileNameFor(actor: Actor): string {
  return actor.name.toLowerCase().replace(/[^a-z0-9]+/g, "-") + ".jsonl";
}

function main(): number {
  const world = runSuite();

  // Write ledgers (fresh each run — deterministic, so regeneration is safe).
  rmSync(LEDGER_DIR, { recursive: true, force: true });
  mkdirSync(LEDGER_DIR, { recursive: true });
  const actors: Actor[] = [world.customer, world.provider, ...world.investors];
  for (const a of actors) {
    writeFileSync(resolve(LEDGER_DIR, fileNameFor(a)), a.ledger.toJSONL());
  }
  // The pinned keyring the standalone verifier checks signatures against.
  writeFileSync(resolve(LEDGER_DIR, "keyring.json"), JSON.stringify(world.keyring, null, 2) + "\n");

  // The report.
  writeFileSync(resolve(ROOT, "RUN_REPORT.md"), generateReport(world));

  // Print the narrative, in order.
  console.log("Item Storage Protocol — suite run\n");
  for (const r of world.results) {
    const passed = r.assertions.filter((a) => a.ok).length;
    const flag = passed === r.assertions.length ? "ok " : "FAIL";
    console.log(`  [${flag}] ${r.id} ${r.title}  (${passed}/${r.assertions.length})`);
    console.log(`         "${r.plainSentence}"`);
    for (const a of r.assertions.filter((x) => !x.ok)) {
      console.log(`         FAIL: ${a.name} — ${a.detail ?? ""}`);
    }
  }

  const { total, failed } = tally(world);
  console.log(`\n  ${total - failed}/${total} assertions passed across ${world.results.length} experiments.`);
  console.log(`  Ledgers written to ${LEDGER_DIR}`);
  console.log(`  Report written to ${resolve(ROOT, "RUN_REPORT.md")}`);
  return failed === 0 ? 0 : 1;
}

process.exit(main());
