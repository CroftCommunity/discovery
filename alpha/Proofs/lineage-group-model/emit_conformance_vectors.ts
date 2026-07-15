// emit_conformance_vectors.ts — emit language-neutral verdict snapshots for the
// Rust conformance suite's categories 8 (visibility V1..V9 + S2) and 9
// (freshness E2.16). The TS model is the AUTHORITATIVE runner for these
// categories; the Rust suite cannot re-prove the TS logic, so it validates only
// the STRUCTURE of these snapshots + their manifest hashes.
//
// Run from the lineage-group-model dir:
//   npm run emit-conformance
// which writes:
//   ../lineage-groups/conformance/vectors/visibility/visibility.json
//   ../lineage-groups/conformance/vectors/freshness.json
//
// No verdict is hand-written: each experiment's invariant pass/fail is taken
// straight from the real experiment run() functions.

import { writeFileSync, mkdirSync } from 'fs';
import { dirname, join } from 'path';
import { ExperimentResult } from './harness/runner';
import * as V from './experiments/V_visibility';
import * as S2 from './experiments/S2_scoped_visibility';
import * as E216 from './experiments/E216_tier_degradation';

interface TsInvariant {
  invariant: string;
  passed: boolean;
}
interface TsExperiment {
  name: string;
  invariants: TsInvariant[];
  all_passed: boolean;
}

function toExperiments(results: ExperimentResult[]): TsExperiment[] {
  return results.map(r => {
    const invariants = r.invariants.map(i => ({ invariant: i.invariant, passed: i.passed }));
    return {
      name: r.name,
      invariants,
      all_passed: invariants.every(i => i.passed),
    };
  });
}

function header(category: number, specSection: string, note: string) {
  return { spec_section: specSection, category, note };
}

const provenance =
  'lineage-group-model: npm run emit-conformance (experiments V/S2/E216 run())';
const authoritativeRunner =
  'TS model in Proofs/lineage-group-model (authoritative; Rust suite validates structure only)';

const vectorsRoot = join(__dirname, '..', 'lineage-groups', 'conformance', 'vectors');

const visibilityExperiments = [...toExperiments(V.run()), ...toExperiments(S2.run())];
const visibilityFile = {
  header: header(
    8,
    'CROFT visibility V1..V9 + S2 (scoped visibility)',
    'Language-neutral snapshot of the TS model verdicts for visibility/propagation ' +
      'geometry (V1..V9) and scoped visibility (S2a/S2b). The AUTHORITATIVE runner is ' +
      'the TS model in Proofs/lineage-group-model; the Rust conformance suite cannot ' +
      're-prove TS logic and validates only structure + manifest hash.',
  ),
  authoritative_runner: authoritativeRunner,
  provenance,
  experiments: visibilityExperiments,
};

const freshnessExperiments = toExperiments(E216.run());
const freshnessFile = {
  header: header(
    9,
    'CROFT freshness E2.16 (tier-degradation visibility)',
    'Language-neutral snapshot of the TS model verdicts for the E2.16 freshness ' +
      'signal (availability with no superpeer, no-false-current, tier degrades ' +
      'visibly). AUTHORITATIVE runner is the TS model; the Rust suite validates only ' +
      'structure + manifest hash.',
  ),
  authoritative_runner: authoritativeRunner,
  provenance,
  experiments: freshnessExperiments,
};

function writePretty(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  // Match serde_json::to_string_pretty: 2-space indent, trailing newline omitted
  // (serde writes no trailing newline). The Rust manifest hashes the exact bytes
  // the Rust emitter re-serializes, so this file's bytes are NOT what the manifest
  // hashes — the Rust emitter re-reads + re-serializes via serde before hashing.
  writeFileSync(path, JSON.stringify(value, null, 2));
}

writePretty(join(vectorsRoot, 'visibility', 'visibility.json'), visibilityFile);
writePretty(join(vectorsRoot, 'freshness.json'), freshnessFile);

const vCount = visibilityExperiments.length;
const fCount = freshnessExperiments.length;
const vAllPass = visibilityExperiments.every(e => e.all_passed);
const fAllPass = freshnessExperiments.every(e => e.all_passed);
console.log(
  `emitted visibility.json (${vCount} experiments, all_passed=${vAllPass}) ` +
    `+ freshness.json (${fCount} experiments, all_passed=${fAllPass})`,
);
if (!vAllPass || !fAllPass) {
  console.error('TS model reported a failing experiment — not emitting a green snapshot');
  process.exit(1);
}
