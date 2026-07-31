// E13. Covenants as code, and the underwriting packet — "The loan application
// is a build artifact."
//
// Loan covenants become machine-checkable rules over the ledger: salary ratio
// within the chartered cap, surplus by a fixed published formula, repayment
// priority (workers before investors), grace within a declared band. One
// compliant year passes every covenant; one violation scenario per covenant is
// flagged with the exact entries responsible. The diligence report is then
// generated ENTIRELY by the funder-side verifier — the co-op cannot influence
// its contents except by changing the underlying signed facts.

import assert from "node:assert/strict";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import type { ExperimentResult } from "../src/experiment.ts";
import { buildCoopYear, type CoopYear, type Defect } from "../src/coop-scenario.ts";
import { runFunder, checkCovenants, classifyLedgers, type FunderInputs } from "../funder/verifier.ts";
import { writeDiligenceReport } from "../funder/report.ts";

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

export function run(seed: number): ExperimentResult {
  // --- Compliant year: every covenant passes. ---
  const compliant = buildCoopYear(seed, "E13-compliant", "none");
  const cov = checkCovenants(inputsFor(compliant));
  assert.equal(cov.ok, true, `compliant year must pass every covenant (got ${JSON.stringify(cov.covenants.filter((c) => !c.ok))})`);
  for (const c of cov.covenants) assert.equal(c.ok, true, `covenant ${c.name} must pass in the compliant year`);

  // --- One violation scenario per covenant, each flagged with responsible entries. ---
  const violations: { defect: Defect; covenant: string }[] = [
    { defect: "salary", covenant: "salary-ratio" },
    { defect: "surplus", covenant: "surplus-floor" },
    { defect: "priority", covenant: "repayment-priority" },
    { defect: "grace-band", covenant: "grace-band" },
  ];
  const rows: { covenant: string; defect: string; flagged: boolean; refs: number }[] = [];
  for (const v of violations) {
    // Adversarial fixtures live under ledgers/adversarial/ (see E12).
    const bad = buildCoopYear(seed + 7, `adversarial/E13-violate-${v.defect}`, v.defect);
    const r = checkCovenants(inputsFor(bad));
    assert.equal(r.ok, false, `violation year (${v.defect}) must fail covenants`);
    const failed = r.covenants.filter((c) => !c.ok);
    // The named covenant must be among the failures.
    const target = failed.find((c) => c.name === v.covenant);
    assert.ok(target, `violation (${v.defect}) must flag covenant ${v.covenant}; failed: ${JSON.stringify(failed.map((c) => c.name))}`);
    // Each violation is isolated to exactly its covenant.
    assert.equal(failed.length, 1, `violation (${v.defect}) should flag only ${v.covenant}; got ${JSON.stringify(failed.map((c) => c.name))}`);
    // It cites the exact ledger entries responsible.
    assert.ok(target!.ledgerRefs.length > 0, `flagged covenant ${v.covenant} must cite the responsible ledger entries`);
    rows.push({ covenant: v.covenant, defect: v.defect, flagged: true, refs: target!.ledgerRefs.length });
  }

  // --- Generate the diligence packet from the compliant year, funder-side only. ---
  const fr = runFunder(inputsFor(compliant));
  const ledgers = classifyLedgers(inputsFor(compliant));
  const root = join(dirname(fileURLToPath(import.meta.url)), "..");
  const reportPath = writeDiligenceReport(join(root, "DILIGENCE_REPORT.md"), {
    coopName: "Croft storage co-op (mock)",
    periods: compliant.periods,
    funder: fr,
    covenants: cov,
    ledgers,
    generatedNote:
      "This document is a build artifact: re-running the suite regenerates it byte-for-byte from the " +
      "signed ledgers. Nothing here is asserted by the co-op; every line is recomputed from the files.",
  });
  assert.ok(reportPath.endsWith("DILIGENCE_REPORT.md"), "the diligence report is written as a build artifact");

  const sentence = "The loan application is a build artifact.";
  const table = [
    "| Covenant | Violation scenario | Flagged | Entries cited |",
    "| --- | --- | :---: | ---: |",
    ...rows.map((r) => `| \`${r.covenant}\` | ${r.defect} | ✓ | ${r.refs} |`),
  ].join("\n");
  const reportMd = [
    "Four loan covenants were expressed as executable checks over the ledger and run against a",
    "compliant year (all pass) and one violation scenario each (each flagged with the exact entries",
    "responsible, and isolated to its own covenant):",
    "",
    table,
    "",
    "The diligence packet `DILIGENCE_REPORT.md` is generated entirely by the funder-side verifier —",
    "covenant status, co-attested revenue, audit pass rate, royalty position, and the scope-condition",
    "page — so the co-op cannot influence the report except by changing the underlying signed facts.",
  ].join("\n");

  return { id: "E13", title: "Covenants as code, and the underwriting packet", sentence, reportMd };
}
