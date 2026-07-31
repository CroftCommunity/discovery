// Standalone ledger verifier. Reads every ./ledgers/*.jsonl file and the pinned
// keyring, then re-checks each entry: correct sequence, unbroken hash-chain link,
// hash matches the (possibly re-serialized) body, and every signature valid under
// a pinned public key. This is the "no hidden trust" guarantee — the books are
// recomputable by anyone holding the files, without re-running the simulation.

import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { verifyEntries, type LedgerEntry } from "./ledger.ts";

const ROOT = resolve(import.meta.dirname, "..");
const LEDGER_DIR = resolve(ROOT, "ledgers");

function main(): number {
  let keyring: Record<string, string>;
  try {
    keyring = JSON.parse(readFileSync(resolve(LEDGER_DIR, "keyring.json"), "utf8"));
  } catch {
    console.error("No keyring found. Run `node src/run.ts` first to generate ledgers.");
    return 1;
  }

  const files = readdirSync(LEDGER_DIR).filter((f) => f.endsWith(".jsonl"));
  let totalEntries = 0;
  let totalIssues = 0;

  console.log("Verifying ledgers against the pinned keyring:\n");
  for (const file of files.sort()) {
    const text = readFileSync(resolve(LEDGER_DIR, file), "utf8").trim();
    const entries: LedgerEntry[] = text
      ? text.split("\n").map((line) => JSON.parse(line))
      : [];
    const issues = verifyEntries(entries, keyring);
    totalEntries += entries.length;
    totalIssues += issues.length;
    const status = issues.length === 0 ? "OK" : `${issues.length} ISSUE(S)`;
    console.log(`  ${file}: ${entries.length} entries — ${status}`);
    for (const i of issues) console.log(`     seq ${i.seq}: ${i.problem}`);
  }

  console.log(`\n  ${totalEntries} entries checked, ${totalIssues} issue(s).`);
  return totalIssues === 0 ? 0 : 1;
}

process.exit(main());
