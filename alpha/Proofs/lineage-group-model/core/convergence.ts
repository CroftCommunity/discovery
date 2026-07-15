import { DAG, DAGNode } from './dag';

export interface ContradictionClaim {
  nodeId: string;
  lineageId: string;
  targetLineageId: string;
  operation: 'add' | 'remove';
}

export interface MergeResult {
  type: 'converged' | 'contradiction';
  convergenceNode?: DAGNode;
  contradiction?: {
    claim1: ContradictionClaim;
    claim2: ContradictionClaim;
    sharedAncestorId: string;
    provenanceTrace: string;
  };
}

/**
 * Merge complementary (non-contradicting) branches.
 * Creates a merge node whose parents are the given heads.
 * Returns a 'converged' result with a new merge node.
 * Heads are canonically sorted so the same set always produces the same node.
 */
export function mergeComplementary(dag: DAG, heads: string[]): MergeResult {
  if (heads.length === 0) {
    throw new Error('mergeComplementary requires at least one head');
  }
  const canonicalHeads = Array.from(new Set(heads)).sort();

  if (canonicalHeads.length === 1) {
    const node = dag.get(canonicalHeads[0]);
    if (!node) throw new Error(`Head node ${canonicalHeads[0]} not found`);
    return { type: 'converged', convergenceNode: node };
  }

  // Check for contradictions first
  for (let i = 0; i < canonicalHeads.length; i++) {
    for (let j = i + 1; j < canonicalHeads.length; j++) {
      const contradiction = detectContradiction(dag, canonicalHeads[i], canonicalHeads[j]);
      if (contradiction.type === 'contradiction') {
        return contradiction;
      }
    }
  }

  // Create a merge node
  const timestamp = Date.now();
  const payload = {
    type: 'checkpoint' as const,
    members: computeMembersFromHeads(dag, canonicalHeads),
    settledThrough: canonicalHeads[canonicalHeads.length - 1],
  };

  const convergenceNode = dag.addNew(canonicalHeads, payload, timestamp);
  return { type: 'converged', convergenceNode };
}

/** Topological sort of a node set so ops are replayed in causal order. */
function topoSort(dag: DAG, nodeIds: Set<string>): DAGNode[] {
  const result: DAGNode[] = [];
  const visited = new Set<string>();

  function visit(id: string): void {
    if (visited.has(id) || !nodeIds.has(id)) return;
    visited.add(id);
    const node = dag.get(id);
    if (!node) return;
    for (const parentId of node.parentIds) visit(parentId);
    result.push(node);
  }

  for (const id of nodeIds) visit(id);
  return result;
}

function computeMembersFromHeads(dag: DAG, heads: string[]): string[] {
  const nodeIds = new Set<string>();
  for (const head of heads) {
    const ancestors = dag.ancestors(head);
    ancestors.add(head);
    for (const id of ancestors) nodeIds.add(id);
  }

  const members = new Set<string>();
  for (const node of topoSort(dag, nodeIds)) {
    if (node.payload.type === 'genesis') {
      for (const m of node.payload.members) members.add(m);
    } else if (node.payload.type === 'add_member') {
      members.add(node.payload.targetLineageId);
    } else if (node.payload.type === 'remove_member') {
      members.delete(node.payload.targetLineageId);
    }
  }
  return Array.from(members).sort();
}

/**
 * Detect a contradiction between two DAG heads.
 * A contradiction occurs when one branch adds and another removes the same member,
 * or vice versa, diverging from the same ancestor state.
 * The mechanism NEVER auto-resolves — it returns the contradiction for human decision.
 */
export function detectContradiction(dag: DAG, headA: string, headB: string): MergeResult {
  const lca = dag.lca(headA, headB);
  if (!lca) {
    // No common ancestor; these are unrelated forks — not a contradiction
    return { type: 'converged' };
  }

  // Collect operations in branch A (from LCA to headA)
  const opsA = collectOpsFromTo(dag, lca, headA);
  const opsB = collectOpsFromTo(dag, lca, headB);

  // Build maps: targetLineageId -> operation for each branch
  const memberOpsA = extractMemberOps(dag, opsA);
  const memberOpsB = extractMemberOps(dag, opsB);

  for (const [target, claimA] of memberOpsA.entries()) {
    const claimB = memberOpsB.get(target);
    if (claimB && claimA.operation !== claimB.operation) {
      const trace = buildProvenanceTrace(dag, lca, headA, headB, claimA, claimB);
      return {
        type: 'contradiction',
        contradiction: {
          claim1: claimA,
          claim2: claimB,
          sharedAncestorId: lca,
          provenanceTrace: trace,
        },
      };
    }
  }

  return { type: 'converged' };
}

function collectOpsFromTo(dag: DAG, from: string, to: string): DAGNode[] {
  // Collect all nodes that are ancestors of 'to' but not ancestors of 'from' (exclusive)
  const ancestorsOfTo = dag.ancestors(to);
  ancestorsOfTo.add(to);
  const ancestorsOfFrom = dag.ancestors(from);
  ancestorsOfFrom.add(from);

  const ops: DAGNode[] = [];
  for (const id of ancestorsOfTo) {
    if (!ancestorsOfFrom.has(id)) {
      const node = dag.get(id);
      if (node) ops.push(node);
    }
  }
  return ops;
}

function extractMemberOps(dag: DAG, ops: DAGNode[]): Map<string, ContradictionClaim> {
  const result = new Map<string, ContradictionClaim>();
  // Use causal (topological) order so the last op per target is the causally latest one
  const nodeIds = new Set(ops.map(n => n.id));
  const ordered = topoSort(dag, nodeIds);
  for (const node of ordered) {
    if (node.payload.type === 'add_member') {
      result.set(node.payload.targetLineageId, {
        nodeId: node.id,
        lineageId: node.payload.lineageId,
        targetLineageId: node.payload.targetLineageId,
        operation: 'add',
      });
    } else if (node.payload.type === 'remove_member') {
      result.set(node.payload.targetLineageId, {
        nodeId: node.id,
        lineageId: node.payload.lineageId,
        targetLineageId: node.payload.targetLineageId,
        operation: 'remove',
      });
    }
  }
  return result;
}

function maskId(id: string): string {
  return id.slice(0, 6) + (id.length > 6 ? `…[${id.length}]` : '');
}

function buildProvenanceTrace(
  dag: DAG,
  lca: string,
  headA: string,
  headB: string,
  claimA: ContradictionClaim,
  claimB: ContradictionClaim,
): string {
  const lcaNode = dag.get(lca);
  const nodeA = dag.get(claimA.nodeId);
  const nodeB = dag.get(claimB.nodeId);
  return [
    `CONTRADICTION DETECTED`,
    `  Shared ancestor: ${lca} (t=${lcaNode?.timestamp ?? 'unknown'})`,
    `  Branch A head: ${headA}`,
    `    Claim: ${maskId(claimA.lineageId)} ${claimA.operation}s ${maskId(claimA.targetLineageId)} at node ${claimA.nodeId} (t=${nodeA?.timestamp ?? 'unknown'})`,
    `  Branch B head: ${headB}`,
    `    Claim: ${maskId(claimB.lineageId)} ${claimB.operation}s ${maskId(claimB.targetLineageId)} at node ${claimB.nodeId} (t=${nodeB?.timestamp ?? 'unknown'})`,
    `  Status: HALTED — awaiting social resolution input`,
  ].join('\n');
}
