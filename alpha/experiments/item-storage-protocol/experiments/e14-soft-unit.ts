// E14. The soft-unit counterexample: proving the scope condition — "The
// signature attests the count, and the count disciplines the signature; without
// a countable unit, it's signed vibes."
//
// The ledger is trustworthy only where the unit is countable at the boundary by
// both sides. This experiment adds a "consulting hours" ledger where both
// parties sign every entry, but an hour of advice has no boundary-observable
// count. The funder's verifier must still classify every consulting entry as
// attested-but-unverifiable — signatures valid, count not — distinct from the
// verified co-attested counts, so the standard's boundary is demonstrated rather
// than claimed.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { buildCoopYear, type CoopYear } from "../src/coop-scenario.ts";
import { classifyLedgers, type FunderInputs } from "../funder/verifier.ts";

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
  const y = buildCoopYear(seed, "E14-soft-unit", "none");
  const classes = classifyLedgers(inputsFor(y));

  const consulting = classes.find((c) => c.type === "consulting_hours");
  assert.ok(consulting, "the consulting-hours ledger is present");
  // Signatures are valid — both parties signed every entry...
  assert.equal(consulting!.signaturesValid, true, "every consulting entry's signatures verify");
  // ...but the unit is not boundary-countable, so it is NOT verified.
  assert.equal(consulting!.countVerifiable, false, "an hour of advice has no boundary-observable count");
  assert.equal(consulting!.verdict, "attested-but-unverifiable", "consulting hours are attested-but-unverifiable");

  // The co-attested counts remain distinct: verified, not merely signed.
  const verifiedTypes = ["revenue", "audit_transcript", "royalty_payment"];
  for (const t of verifiedTypes) {
    const c = classes.find((x) => x.type === t)!;
    assert.equal(c.signaturesValid, true, `${t} signatures verify`);
    assert.equal(c.countVerifiable, true, `${t} count is boundary-verifiable`);
    assert.equal(c.verdict, "verified", `${t} is verified, distinct from attested-but-unverifiable`);
  }

  // The distinction is real: at least one ledger of each class, side by side.
  const verified = classes.filter((c) => c.verdict === "verified");
  const unverifiable = classes.filter((c) => c.verdict === "attested-but-unverifiable");
  assert.ok(verified.length >= 1 && unverifiable.length >= 1, "both classes are represented on one page");
  // A valid signature is necessary but NOT sufficient for 'verified'.
  assert.ok(
    unverifiable.every((c) => c.signaturesValid),
    "the unverifiable ledger still has valid signatures — the gap is the missing countable unit, not a bad signature",
  );

  const sentence = "The signature attests the count, and the count disciplines the signature; without a countable unit, it's signed vibes.";
  const table = [
    "| Ledger | Signatures | Count verifiable | Verdict |",
    "| --- | :---: | :---: | --- |",
    ...classes.map(
      (c) => `| \`${c.type}\` | ${c.signaturesValid ? "✓" : "✗"} | ${c.countVerifiable ? "✓" : "✗"} | ${c.verdict === "verified" ? "verified" : "attested-but-unverifiable"} |`,
    ),
  ].join("\n");
  const reportMd = [
    "The same funder verifier that blesses the co-attested ledgers refuses to bless a signed ledger",
    "of consulting hours: the signatures verify, but an hour of advice has no boundary-observable",
    "count, so it is classified attested-but-unverifiable — distinct from verified — on one page:",
    "",
    table,
    "",
    "A valid signature is necessary but not sufficient. Where the unit is countable at the boundary",
    "(bytes, fingerprints, cap dollars), the signature disciplines a real count; where it isn't, the",
    "signature is all there is. The standard stops, honestly, at the countable boundary.",
  ].join("\n");

  return { id: "E14", title: "The soft-unit counterexample: proving the scope condition", sentence, reportMd };
}
