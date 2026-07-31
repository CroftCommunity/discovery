// E12. The outside reader: diligence from the files alone — "Revenue isn't
// asserted, it's co-attested; the loan officer can check it from her desk."
//
// A funder (June) who has never touched the service, holds no keys, and gets no
// private access underwrites the co-op from the published ledger files, the
// actors' public keys, the audit transcripts, and the public randomness source.
// Her verifier (funder/verifier.ts) is INDEPENDENT of the actors' code — it
// re-derives every primitive from the spec. This experiment builds an honest
// co-op year plus four cooked-books years, one per trust problem, and confirms
// the funder passes the honest one and detects-and-classifies each cook — all
// from the files, without asking anyone anything.

import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import type { ExperimentResult } from "../src/experiment.ts";
import { buildCoopYear, type CoopYear, type Defect } from "../src/coop-scenario.ts";
import { runFunder, type FunderInputs, type FindingCode } from "../funder/verifier.ts";

function inputsFor(y: CoopYear): FunderInputs {
  return {
    ledgerDir: y.dir,
    keys: y.keys,
    beaconSeedHex: y.beaconSeedHex,
    periods: y.periods,
    auditK: y.auditK,
    royaltyTerms: y.royaltyTerms,
    covenants: y.covenants,
  };
}

/**
 * Enforce the module boundary structurally (Part 3 E12: "the Funder's verifier
 * shares no code with the actors, enforced by module boundary"): no source file
 * under funder/ may import from ../src or ../experiments.
 */
function assertFunderIsIndependent(): void {
  const here = dirname(fileURLToPath(import.meta.url));
  const funderDir = join(here, "..", "funder");
  const files = readdirSync(funderDir).filter((f) => f.endsWith(".ts"));
  assert.ok(files.length > 0, "funder/ must contain source files");
  const importRe = /\bfrom\s+["']([^"']+)["']/g;
  for (const f of files) {
    const src = readFileSync(join(funderDir, f), "utf8");
    let m: RegExpExecArray | null;
    while ((m = importRe.exec(src)) !== null) {
      const spec = m[1];
      const isNodeBuiltin = spec.startsWith("node:");
      const reachesActors = /(^|\/)(src|experiments)\//.test(spec) || spec.includes("../src") || spec.includes("../experiments");
      assert.ok(
        isNodeBuiltin || (!reachesActors && !spec.startsWith("..")),
        `funder/${f} imports "${spec}" — the funder must share no code with the actors`,
      );
    }
  }
}

export function run(seed: number): ExperimentResult {
  assertFunderIsIndependent();

  // --- Honest year: the funder passes every check with no findings. ---
  const honest = buildCoopYear(seed, "E12-honest", "none");
  const hr = runFunder(inputsFor(honest));
  assert.equal(hr.overallOk, true, `honest year must pass all funder checks (findings: ${JSON.stringify(hr.findings)})`);
  assert.equal(hr.findings.length, 0, "honest year yields no findings");
  assert.equal(hr.revenue.ok, true, "honest revenue is fully co-attested and matches the report");
  assert.equal(hr.service.ok, true, "honest audit transcripts all replay from the public beacon");
  assert.equal(hr.service.passRate, 1, "every honest audit passed");
  assert.equal(hr.chain.ok, true, "honest statement chain is intact from genesis");
  assert.equal(hr.grace.ok, true, "honest grace is fully on-book");
  assert.equal(hr.royalty.ok, true, "honest royalty ledger matches the instrument terms");
  assert.equal(hr.royalty.paidCents, hr.royalty.capCents, "royalty paid exactly the cap");

  // --- One cooked-books scenario per trust problem. ---
  const cooks: { defect: Defect; code: FindingCode; label: string }[] = [
    { defect: "revenue", code: "uncosigned-revenue", label: "inflated revenue lacking a customer signature" },
    { defect: "waiver", code: "off-book-waiver", label: "a fee waived off-book (no grace entry)" },
    { defect: "retro", code: "retro-edit", label: "a retroactive edit to a closed period" },
    { defect: "audit", code: "bad-audit-challenge", label: "a fabricated audit transcript" },
  ];

  const cookRows: { label: string; code: string; detected: boolean; period: number | null }[] = [];
  for (const c of cooks) {
    // Adversarial fixtures live under ledgers/adversarial/ so the standalone
    // verifier can hold them to a different bar: a detected tamper here is the
    // point, not a corpus failure.
    const cooked = buildCoopYear(seed + 1, `adversarial/E12-cooked-${c.defect}`, c.defect);
    const r = runFunder(inputsFor(cooked));
    assert.equal(r.overallOk, false, `cooked year (${c.defect}) must fail`);
    const hit = r.findings.find((f) => f.code === c.code);
    assert.ok(hit, `cooked year (${c.defect}) must be classified as ${c.code}; got ${JSON.stringify(r.findings.map((f) => f.code))}`);
    // Each cook trips exactly its own trust problem — no misclassification.
    assert.ok(
      r.findings.every((f) => f.code === c.code),
      `cooked year (${c.defect}) should raise only ${c.code}; got ${JSON.stringify(r.findings.map((f) => f.code))}`,
    );
    cookRows.push({ label: c.label, code: c.code, detected: true, period: hit!.period });
  }

  const sentence = "Revenue isn't asserted, it's co-attested; the loan officer can check it from her desk.";
  const dollars = (c: number): string => `$${Math.round(c / 100).toLocaleString("en-US")}`;
  const table = [
    "| Cooked-books scenario | Classified as | Detected | At |",
    "| --- | --- | :---: | ---: |",
    ...cookRows.map((r) => `| ${r.label} | \`${r.code}\` | ✓ | ${r.period === null ? "—" : `period ${r.period}`} |`),
  ].join("\n");
  const reportMd = [
    "A funder holding only the published files, the public keys, and the public randomness seed ran",
    "an INDEPENDENT verifier (no shared code with the actors, enforced by module boundary) over an",
    `honest co-op year of ${honest.periods} periods. From the files alone she confirmed: revenue is`,
    `co-attested (${dollars(hr.revenue.coAttestedCents)} across signed entries, matching the report);`,
    `service was delivered (every audit transcript's challenges replay from the public beacon, ${(hr.service.passRate * 100).toFixed(0)}%`,
    "pass); the statement chain is intact from genesis; grace is on-book; and the royalty ledger paid",
    `exactly the ${dollars(hr.royalty.capCents)} cap and extinguished on schedule.`,
    "",
    "Then one cooked-books year per trust problem — each detected and correctly classified, from the",
    "files, without asking anyone anything:",
    "",
    table,
  ].join("\n");

  return { id: "E12", title: "The outside reader: diligence from the files alone", sentence, reportMd };
}
