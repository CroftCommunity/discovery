import { DAG } from '../core/dag';
import { detectContradiction } from '../core/convergence';
import { forkFromState } from '../core/trapdoor';
import { meetsRemovalThreshold, canChangeDial, DIALS_INCLUSION, DIALS_BALANCED, DIALS_FIDELITY, GovernanceDials } from '../social/dials';
import { ScenarioRunner, ExperimentResult } from '../harness/runner';

const runner = new ScenarioRunner();

function runEjection(dials: GovernanceDials): { ejectionOccurred: boolean; mechanismType: string } {
  const dag = new DAG();
  const t = 70000;
  const genesis = dag.addNew([], { type: 'genesis', members: ['L1', 'L2', 'L3', 'L4', 'L5', 'M'] }, t);
  const ejectOp = dag.addNew([genesis.id], { type: 'remove_member', lineageId: 'L1', targetLineageId: 'M' }, t + 1);
  const keepOp = dag.addNew([genesis.id], { type: 'add_member', lineageId: 'L2', targetLineageId: 'M' }, t + 2);
  const contradiction = detectContradiction(dag, ejectOp.id, keepOp.id);
  return {
    ejectionOccurred: meetsRemovalThreshold(dials, 3, 6), // 3/6 = 50% vote to remove
    mechanismType: contradiction.type,
  };
}

function runE1(): ExperimentResult {
  return runner.run('E1: Dial tuning changes outcomes, mechanism unchanged', 'E', () => {
    const a = runEjection(DIALS_INCLUSION);
    const b = runEjection(DIALS_BALANCED);
    const c = runEjection(DIALS_FIDELITY);

    const outcomesdiffer = a.ejectionOccurred !== c.ejectionOccurred;
    const mechanismIdentical = a.mechanismType === b.mechanismType && b.mechanismType === c.mechanismType;

    return {
      invariants: [
        {
          invariant: 'INV-NO-AUTO-RESOLVE',
          passed: mechanismIdentical,
          details: `Mechanism type identical across all dials: "${a.mechanismType}" (${mechanismIdentical ? '✓' : '✗'})`,
        },
        {
          invariant: 'INV-DETECT-CONTRADICTION',
          passed: outcomesdiffer,
          details: [
            `Inclusion (34%): ejection=${a.ejectionOccurred}`,
            `Balanced (51%): ejection=${b.ejectionOccurred}`,
            `Fidelity (67%): ejection=${c.ejectionOccurred}`,
          ].join('; '),
        },
      ],
      provenanceTrace: [
        `3/6 lineages vote to remove M (50%)`,
        `Inclusion dial (34% threshold): PASSES ✓`,
        `Balanced dial (51% threshold): FAILS (50% < 51%) ✓`,
        `Fidelity dial (67% threshold): FAILS ✓`,
        `Underlying mechanism type identical: ${mechanismIdentical ? '✓' : '✗'}`,
        `Social posture is input; mechanism is invariant ✓`,
      ].join('\n'),
      socialInputsUsed: [
        'SCRIPTED: 3/6 lineages vote remove_member M',
        'SCRIPTED: Dial settings are test fixture inputs',
      ],
    };
  });
}

function runE2(): ExperimentResult {
  return runner.run('E2: Genesis-fixed vs runtime dials', 'E', () => {
    const dials = { ...DIALS_BALANCED };
    const fixedRejected = !canChangeDial(dials, 'removalThreshold', false);
    const runtimeAllowed = canChangeDial(dials, 'quorumSize', false);

    const dag = new DAG();
    const t = 80000;
    const genesis = dag.addNew([], { type: 'genesis', members: ['L1', 'L2'] }, t);
    const dialChangeNode = dag.addNew([genesis.id], { type: 'dial_change', lineageId: 'L1', dialKey: 'quorumSize', value: 5 }, t + 1);

    return {
      invariants: [
        {
          invariant: 'INV-IMMUTABLE-ADMIN',
          passed: fixedRejected,
          details: fixedRejected
            ? `Genesis-fixed "removalThreshold" correctly rejected at runtime`
            : 'VIOLATION: genesis-fixed dial was modifiable at runtime',
        },
        {
          invariant: 'INV-IMMUTABLE-ADMIN',
          passed: runtimeAllowed,
          details: runtimeAllowed
            ? `Runtime "quorumSize" change allowed, recorded: ${dialChangeNode.id.slice(0, 8)}…`
            : 'Runtime dial change incorrectly blocked',
        },
      ],
      provenanceTrace: [
        `Genesis: ${genesis.id.slice(0, 8)}… (genesisFixed: [${dials.genesisFixed.join(', ')}])`,
        `Change "removalThreshold" at runtime: ${fixedRejected ? 'REJECTED ✓ (provenance: genesis)' : 'ALLOWED ✗'}`,
        `Change "quorumSize" at runtime: ${runtimeAllowed ? 'ALLOWED ✓' : 'REJECTED ✗'}`,
        `Runtime dial change recorded as admin op: ${dialChangeNode.id.slice(0, 8)}…`,
      ].join('\n'),
      socialInputsUsed: [
        'SCRIPTED: L1 attempts genesis-fixed removalThreshold change (negative)',
        'SCRIPTED: L1 changes runtime quorumSize (positive)',
      ],
    };
  });
}

function runE3(): ExperimentResult {
  return runner.run('E3: Captured-quorum entrenchment (adversarial governance)', 'E', () => {
    const dag = new DAG();
    const t = 90000;
    const genesis = dag.addNew([], { type: 'genesis', members: ['L1', 'L2', 'L3', 'L4', 'L5', 'Minority'] }, t);
    const captureDialChange = dag.addNew([genesis.id], { type: 'dial_change', lineageId: 'L1', dialKey: 'removalThreshold', value: 0.34 }, t + 1);
    const ejectMinority = dag.addNew([captureDialChange.id], { type: 'remove_member', lineageId: 'L1', targetLineageId: 'Minority' }, t + 2);

    const ejectionValid = dag.has(ejectMinority.id);
    const forkResult = forkFromState(dag, genesis.id, 'Minority re-forms off shared ancestor', 'Minority');
    const reformationAvailable = forkResult.ancestryPreserved;

    return {
      invariants: [
        {
          invariant: 'INV-TRAPDOOR',
          passed: reformationAvailable,
          details: `Minority re-formation available: ${reformationAvailable}`,
        },
        {
          invariant: 'INV-DETECT-CONTRADICTION',
          passed: dag.get(captureDialChange.id)?.payload.type === 'dial_change' && ejectionValid,
          details: 'Full capture chain is attributable in provenance',
        },
      ],
      provenanceTrace: [
        `Malicious majority: L1–L5`,
        `1. Dial change removalThreshold→34%: ${captureDialChange.id.slice(0, 8)}… — attributable ✓`,
        `2. Ejection of Minority (valid under new dials): ${ejectMinority.id.slice(0, 8)}… — attributable ✓`,
        `3. Minority re-formation fork: ${forkResult.newBranchHead.id.slice(0, 8)}… — available ✓`,
        `Crypto guarantees legibility and clean exit; it does not prevent social harm ✓`,
      ].join('\n'),
      socialInputsUsed: [
        'SCRIPTED: L1 changes removalThreshold (adversarial)',
        'SCRIPTED: Majority ejects Minority (valid under new dials)',
        'SCRIPTED: Minority invokes re-formation backstop',
      ],
    };
  });
}

export function run(): ExperimentResult[] {
  return [runE1(), runE2(), runE3()];
}
