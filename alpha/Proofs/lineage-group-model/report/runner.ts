import { ExperimentResult } from '../harness/runner';
import * as A from '../experiments/A_ancestry_integrity';
import * as B from '../experiments/B_complementary_convergence';
import * as C from '../experiments/C_contradiction';
import * as D from '../experiments/D_trapdoor';
import * as E from '../experiments/E_governance_dials';
import * as F from '../experiments/F_rollups';
import * as G from '../experiments/G_soak';
import * as H from '../experiments/H_adversarial';
import * as V from '../experiments/V_visibility';
import * as S2 from '../experiments/S2_scoped_visibility';
import * as E216 from '../experiments/E216_tier_degradation';

const ALL_INVARIANTS = [
  'INV-ANCESTRY',
  'INV-HASH-INTEGRITY',
  'INV-CONVERGE-COMPLEMENTARY',
  'INV-DETECT-CONTRADICTION',
  'INV-NO-AUTO-RESOLVE',
  'INV-TRAPDOOR',
  'INV-IMMUTABLE-ADMIN',
  'INV-LINEAGE-NOT-LEAF',
  'INV-VIEW-LOCAL-FIRST',
  // Social-layer / visibility invariants (V1–V9)
  'INV-REGIME-IMMUTABLE',
  'INV-REGIME-IN-CONTENT',
  'INV-NO-SILENT-CROSSING',
  'INV-REPUBLISH-DISTINCT',
  'INV-DEPTH-ENFORCED',
  'INV-OPENNESS-CAPS-DEPTH',
  'INV-INWARD-OUTWARD-INDEPENDENT',
  'INV-PUBLIC-MEMBERSHIP-BOUNDED',
  'INV-FREEZE-BY-DEFAULT',
  // S2 — scoped visibility (structure deanonymizes; only consented-distance sharing is safe)
  'INV-STRUCTURE-LEAKS-IDENTITY',
  'INV-SCOPED-CONSENT',
  // E2.16 — tier-degradation visibility via the freshness signal
  'INV-AVAILABILITY-NO-SUPERPEER',
  'INV-NO-FALSE-CURRENT',
  'INV-TIER-DEGRADES-VISIBLY',
] as const;

function pad(s: string, width: number): string {
  return s.length >= width ? s : s + ' '.repeat(width - s.length);
}

function printTable(results: ExperimentResult[]): void {
  const COL_NAME = 52;
  const COL_STATUS = 8;
  const COL_INV = 34;

  const header = pad('Experiment', COL_NAME) + pad('Status', COL_STATUS) + 'Invariants';
  const divider = '-'.repeat(header.length + 20);

  console.log('\n' + divider);
  console.log('EXPERIMENT RESULTS');
  console.log(divider);
  console.log(header);
  console.log(divider);

  let passed = 0;
  let failed = 0;

  for (const r of results) {
    const allPass = r.invariants.every(i => i.passed);
    if (allPass) passed++; else failed++;

    const status = allPass ? 'PASS' : 'FAIL';
    const invList = r.invariants.map(i => `${i.passed ? '✓' : '✗'} ${i.invariant}`).join(', ');
    console.log(pad(r.name, COL_NAME) + pad(status, COL_STATUS) + invList);

    for (const inv of r.invariants) {
      if (!inv.passed) {
        console.log('  ' + pad('', COL_NAME) + '    ↳ FAIL: ' + inv.details);
      }
    }
  }

  console.log(divider);
  console.log(`Total: ${results.length} experiments — ${passed} passed, ${failed} failed`);
  console.log(divider);
}

function printCoverageMatrix(results: ExperimentResult[]): void {
  console.log('\n' + '='.repeat(80));
  console.log('INV-* COVERAGE MATRIX');
  console.log('='.repeat(80));

  const COL_INV = 34;

  // Header row: experiment names abbreviated
  const names = results.map(r => r.name.slice(0, 4));
  console.log(pad('Invariant', COL_INV) + names.join(' '));

  for (const inv of ALL_INVARIANTS) {
    const row = results.map(r => {
      const hit = r.invariants.find(i => i.invariant === inv);
      if (!hit) return '   ';
      return hit.passed ? ' ✓ ' : ' ✗ ';
    });
    console.log(pad(inv, COL_INV) + row.join(''));
  }

  // Summary: which invariants have zero coverage
  const uncovered = ALL_INVARIANTS.filter(inv =>
    !results.some(r => r.invariants.some(i => i.invariant === inv))
  );
  if (uncovered.length > 0) {
    console.log('\nUNCOVERED INVARIANTS: ' + uncovered.join(', '));
  } else {
    console.log('\nAll invariants covered ✓');
  }
}

function printProvenanceTraces(results: ExperimentResult[]): void {
  console.log('\n' + '='.repeat(80));
  console.log('PROVENANCE TRACES');
  console.log('='.repeat(80));

  for (const r of results) {
    const allPass = r.invariants.every(i => i.passed);
    console.log(`\n[${allPass ? 'PASS' : 'FAIL'}] ${r.name}`);
    console.log('-'.repeat(60));
    if (typeof r.provenanceTrace === 'string') {
      console.log(r.provenanceTrace);
    } else {
      console.log(JSON.stringify(r.provenanceTrace, null, 2));
    }
  }
}

function printSocialInputs(results: ExperimentResult[]): void {
  console.log('\n' + '='.repeat(80));
  console.log('SOCIAL INPUTS USED (mechanism never computed these)');
  console.log('='.repeat(80));

  for (const r of results) {
    if (r.socialInputsUsed.length === 0) continue;
    console.log(`\n${r.name}:`);
    for (const s of r.socialInputsUsed) {
      console.log('  • ' + s);
    }
  }
}

function printMetrics(results: ExperimentResult[]): void {
  const withMetrics = results.filter(r => r.metrics && Object.keys(r.metrics).length > 0);
  if (withMetrics.length === 0) return;

  console.log('\n' + '='.repeat(80));
  console.log('METRICS');
  console.log('='.repeat(80));

  for (const r of withMetrics) {
    console.log(`\n${r.name}:`);
    for (const [k, v] of Object.entries(r.metrics!)) {
      console.log(`  ${k}: ${v}`);
    }
  }
}

export function runAll(): void {
  console.log('Running lineage-group experiment suite…\n');

  const groups: Array<[string, ExperimentResult[]]> = [
    ['A — Ancestry & Hash Integrity', A.run()],
    ['B — Complementary Convergence', B.run()],
    ['C — Contradiction Hard Stop', C.run()],
    ['D — Trapdoor Fork', D.run()],
    ['E — Governance Dials', E.run()],
    ['F — Roll-ups & Checkpoints', F.run()],
    ['G — Soak / Long-horizon', G.run()],
    ['H — Adversarial / Boundary', H.run()],
    ['V — Visibility & Propagation Geometry', V.run()],
    ['S — Scoped Visibility (S2)', S2.run()],
    ['E2.16 — Tier-degradation Visibility', E216.run()],
  ];

  const all: ExperimentResult[] = [];
  for (const [, results] of groups) {
    for (const r of results) all.push(r);
  }

  printTable(all);
  printCoverageMatrix(all);
  printProvenanceTraces(all);
  printSocialInputs(all);
  printMetrics(all);

  console.log('\n' + '='.repeat(80));
  console.log('Done.');
  console.log('='.repeat(80) + '\n');

  const anyFailed = all.some(r => r.invariants.some(i => !i.passed));
  if (anyFailed) process.exit(1);
}
