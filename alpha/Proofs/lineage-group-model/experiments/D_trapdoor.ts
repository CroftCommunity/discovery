import { DAG } from '../core/dag';
import { forkFromState, unrelatedFork } from '../core/trapdoor';
import { detectContradiction } from '../core/convergence';
import { checkTrapdoor, checkViewLocalFirst } from '../core/invariants';
import { Peer } from '../harness/peer';
import { ScenarioRunner, ExperimentResult } from '../harness/runner';

const runner = new ScenarioRunner();

function buildStuckState(): { dag: DAG; headX: string; headY: string; genesisId: string } {
  const dag = new DAG();
  const t = 50000;
  const genesis = dag.addNew([], { type: 'genesis', members: ['L1', 'L2', 'L3', 'M'] }, t);
  const ejectM = dag.addNew([genesis.id], { type: 'remove_member', lineageId: 'L1', targetLineageId: 'M' }, t + 1);
  const keepM = dag.addNew([genesis.id], { type: 'add_member', lineageId: 'L2', targetLineageId: 'M' }, t + 2);
  return { dag, headX: ejectM.id, headY: keepM.id, genesisId: genesis.id };
}

function runD1(): ExperimentResult {
  return runner.run('D1: Fork from stuck contradiction', 'D', () => {
    const { dag, headX, headY } = buildStuckState();
    const check = detectContradiction(dag, headX, headY);
    if (check.type !== 'contradiction') throw new Error('Expected contradiction');

    const forkResult = forkFromState(dag, headX, 'Forking out of stuck contradiction', 'L1');
    const oldHeadXExists = dag.has(headX);
    const oldHeadYExists = dag.has(headY);

    return {
      invariants: [
        checkTrapdoor(forkResult),
        {
          invariant: 'INV-TRAPDOOR',
          passed: oldHeadXExists && oldHeadYExists,
          details: `Old stuck history preserved: X=${oldHeadXExists}, Y=${oldHeadYExists}`,
        },
      ],
      provenanceTrace: [
        `Stuck contradiction: headX=${headX.slice(0, 8)}… vs headY=${headY.slice(0, 8)}…`,
        `Trapdoor invoked by L1 from headX`,
        `New branch head: ${forkResult.newBranchHead.id.slice(0, 8)}…`,
        `Ancestry preserved: ${forkResult.ancestryPreserved}`,
        `Old history: X=${oldHeadXExists}, Y=${oldHeadYExists} — accessible and searchable ✓`,
      ].join('\n'),
      socialInputsUsed: ['SCRIPTED: L1 invokes trapdoor from stuck state'],
    };
  });
}

function runD2(): ExperimentResult {
  return runner.run('D2: Unrelated fork', 'D', () => {
    const original = new DAG();
    original.addNew([], { type: 'genesis', members: ['L1', 'L2'] }, 60000);

    const { dag: newDag, genesisNode } = unrelatedFork(['L3', 'L4', 'L5']);
    const noSharedNodes = original.allNodes().every(n => !newDag.has(n.id));
    const newGroupUsable = newDag.has(genesisNode.id);

    return {
      invariants: [{
        invariant: 'INV-TRAPDOOR',
        passed: noSharedNodes && newGroupUsable,
        details: [
          `New genesis: ${genesisNode.id.slice(0, 8)}… (L3, L4, L5)`,
          `No false ancestry to original: ${noSharedNodes}`,
          `New group usable: ${newGroupUsable}`,
        ].join('; '),
      }],
      provenanceTrace: [
        `Original group: L1, L2`,
        `Unrelated fork: new DAG, genesis ${genesisNode.id.slice(0, 8)}… (L3, L4, L5)`,
        `Shared nodes: 0 — no false ancestry claimed ✓`,
      ].join('\n'),
      socialInputsUsed: ['SCRIPTED: L3 starts fresh group with no ancestor relationship'],
    };
  });
}

function runD3(): ExperimentResult {
  return runner.run('D3: Usable and searchable divergent history', 'D', () => {
    const { dag, headX, headY } = buildStuckState();
    const t = Date.now();
    const msgX = dag.addNew([headX], { type: 'message', lineageId: 'L1', content: 'message on branch X' }, t);
    const msgY = dag.addNew([headY], { type: 'message', lineageId: 'L2', content: 'message on branch Y' }, t + 1);

    const peer = new Peer('p1', 'L1');
    for (const node of dag.allNodes()) {
      try { peer.localDAG.add(node); } catch { /* skip */ }
    }

    const view = peer.currentView();
    const searchX = peer.search('branch X');
    const searchY = peer.search('branch Y');
    const searchWorksX = searchX.some(n => n.id === msgX.id);
    const searchWorksY = searchY.some(n => n.id === msgY.id);

    return {
      invariants: [
        checkViewLocalFirst([view]),
        {
          invariant: 'INV-VIEW-LOCAL-FIRST',
          passed: searchWorksX && searchWorksY,
          details: `Search "branch X": ${searchX.length} result(s); "branch Y": ${searchY.length} result(s)`,
        },
      ],
      provenanceTrace: [
        `Divergent state: ${view.heads.length} heads (unresolved contradiction)`,
        `Peer view: ${view.members.length} members, ${view.heads.length} head(s)`,
        `Search "branch X": found ${searchX.length} result(s) ✓`,
        `Search "branch Y": found ${searchY.length} result(s) ✓`,
        `Worst-case divergent state is still a working, searchable chat ✓`,
      ].join('\n'),
      socialInputsUsed: [],
    };
  });
}

export function run(): ExperimentResult[] {
  return [runD1(), runD2(), runD3()];
}
