// Standalone ledger verifier (Part 5): every entry in every ledger file must be
// signature-checkable with no other state. Walks ./ledgers/**, verifies each
// entry's signature against its own embedded public key, confirms the actor id
// is the key-derived identifier, and checks the append-only sequence.
//
//   node scripts/verify-ledgers.ts    (or: npm run verify)

import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { existsSync, readdirSync, statSync } from "node:fs";
import { verifyLedgerFile } from "../src/ledger.ts";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const LEDGERS = join(ROOT, "ledgers");

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    const full = join(dir, name);
    if (statSync(full).isDirectory()) out.push(...walk(full));
    else if (name.endsWith(".jsonl")) out.push(full);
  }
  return out;
}

if (!existsSync(LEDGERS)) {
  console.error(`No ledgers directory at ${LEDGERS}. Run \`node scripts/run.ts\` first.`);
  process.exit(1);
}

const files = walk(LEDGERS).sort();
let totalEntries = 0;
let badFiles = 0;

console.log(`Verifying ${files.length} ledger file(s) under ./ledgers/\n`);
for (const f of files) {
  const rel = f.slice(ROOT.length + 1);
  const res = verifyLedgerFile(f);
  totalEntries += res.count;
  if (res.ok) {
    console.log(`  [OK]   ${rel}  (${res.count} entries)`);
  } else {
    badFiles++;
    console.log(`  [BAD]  ${rel}  (${res.count} entries)`);
    for (const e of res.errors) console.log(`         - ${e}`);
  }
}

console.log("");
if (badFiles === 0) {
  console.log(`All ${files.length} ledger files verified; ${totalEntries} signed entries checked.`);
  process.exit(0);
} else {
  console.error(`${badFiles} ledger file(s) failed verification.`);
  process.exit(1);
}
