import { DAG } from './dag';
import { MergeResult } from './convergence';
import { ForkResult } from './trapdoor';

export interface InvariantResult {
  invariant: string;
  passed: boolean;
  details: string;
  provenanceTrace?: string;
}

export function checkAncestry(
  dag: DAG,
  nodeA: string,
  nodeB: string,
  peers: DAG[],
): InvariantResult {
  const reference = dag.lca(nodeA, nodeB);
  const traces: string[] = [];
  let allMatch = true;

  for (let i = 0; i < peers.length; i++) {
    const peerResult = peers[i].lca(nodeA, nodeB);
    const match = peerResult === reference;
    if (!match) allMatch = false;
    traces.push(`Peer ${i}: LCA=${peerResult?.slice(0, 8) ?? 'null'}… (${match ? 'match' : 'MISMATCH'})`);
  }

  return {
    invariant: 'INV-ANCESTRY',
    passed: allMatch,
    details: `LCA of ${nodeA.slice(0, 8)}… and ${nodeB.slice(0, 8)}… = ${reference?.slice(0, 8) ?? 'null'}…`,
    provenanceTrace: traces.join('\n'),
  };
}

export function checkHashIntegrity(dag: DAG): InvariantResult {
  const allNodes = dag.allNodes();
  let firstBreak: string | null = null;
  const tampered: string[] = [];

  for (const node of allNodes) {
    const expected = dag.computeHash(node.parentIds, node.payload, node.timestamp);
    if (node.id !== expected) {
      tampered.push(node.id);
      if (!firstBreak) firstBreak = node.id;
    }
  }

  return {
    invariant: 'INV-HASH-INTEGRITY',
    passed: tampered.length === 0,
    details: tampered.length === 0
      ? `All ${allNodes.length} nodes have valid hashes`
      : `${tampered.length} node(s) have invalid hashes`,
    provenanceTrace: firstBreak ? `First hash chain break at: ${firstBreak.slice(0, 8)}…` : undefined,
  };
}

export function checkConvergeComplementary(resultHashes: string[]): InvariantResult {
  const unique = new Set(resultHashes);
  return {
    invariant: 'INV-CONVERGE-COMPLEMENTARY',
    passed: unique.size === 1,
    details: unique.size === 1
      ? `All ${resultHashes.length} merge orders yield identical state hash`
      : `${unique.size} distinct hashes across ${resultHashes.length} merge orders`,
  };
}

export function checkDetectContradiction(result: MergeResult): InvariantResult {
  return {
    invariant: 'INV-DETECT-CONTRADICTION',
    passed: result.type === 'contradiction',
    details: result.type === 'contradiction'
      ? (result.contradiction ? `Contradiction detected for target ${result.contradiction.claim1.targetLineageId.slice(0, 8)}…` : 'Contradiction (details unavailable)')
      : 'No contradiction detected (unexpected)',
    provenanceTrace: result.contradiction?.provenanceTrace,
  };
}

export function checkNoAutoResolve(result: MergeResult): InvariantResult {
  const preserved =
    result.type === 'contradiction' &&
    result.contradiction !== undefined &&
    result.contradiction.claim1 !== undefined &&
    result.contradiction.claim2 !== undefined;

  return {
    invariant: 'INV-NO-AUTO-RESOLVE',
    passed: preserved,
    details: preserved
      ? 'Both claims preserved; mechanism produced no winner'
      : 'Mechanism produced a winner or discarded a claim (VIOLATION)',
  };
}

export function checkTrapdoor(forkResult: ForkResult): InvariantResult {
  return {
    invariant: 'INV-TRAPDOOR',
    passed: forkResult.ancestryPreserved && forkResult.oldHistoryAccessible,
    details: [
      `New head: ${forkResult.newBranchHead.id.slice(0, 8)}…`,
      `Ancestry preserved: ${forkResult.ancestryPreserved}`,
      `Old history accessible: ${forkResult.oldHistoryAccessible}`,
    ].join('; '),
  };
}

export function checkImmutableAdmin(thrown: boolean, errorMessage: string): InvariantResult {
  return {
    invariant: 'INV-IMMUTABLE-ADMIN',
    passed: thrown,
    details: thrown
      ? `Admin mutation correctly rejected: ${errorMessage}`
      : 'Admin mutation was NOT rejected (VIOLATION)',
  };
}

export function checkLineageNotLeaf(
  lineageCountBefore: number,
  lineageCountAfter: number,
  deviceCountBefore: number,
  deviceCountAfter: number,
): InvariantResult {
  const lineageUnchanged = lineageCountBefore === lineageCountAfter;
  const devicesIncreased = deviceCountAfter > deviceCountBefore;

  return {
    invariant: 'INV-LINEAGE-NOT-LEAF',
    passed: lineageUnchanged && devicesIncreased,
    details: [
      `Lineages: ${lineageCountBefore}→${lineageCountAfter} (${lineageUnchanged ? 'unchanged ✓' : 'changed ✗'})`,
      `Devices: ${deviceCountBefore}→${deviceCountAfter} (${devicesIncreased ? 'increased ✓' : 'no change ✗'})`,
    ].join('; '),
  };
}

export function checkViewLocalFirst(
  peerViews: Array<{ members: string[]; heads: string[] }>,
): InvariantResult {
  const allRenderable = peerViews.every(v => v.heads.length > 0);
  return {
    invariant: 'INV-VIEW-LOCAL-FIRST',
    passed: allRenderable,
    details: peerViews
      .map((v, i) => `Peer ${i}: ${v.members.length} members, ${v.heads.length} head(s)`)
      .join('; '),
  };
}
