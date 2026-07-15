import { DAG, DAGNode } from './dag';

export interface ForkResult {
  newBranchHead: DAGNode;
  ancestryPreserved: boolean;  // can reach the original lineage roots
  oldHistoryAccessible: boolean;
}

/**
 * Fork from a given node in the DAG.
 * Creates a new 'fork' node pointing to fromNodeId as a parent.
 * Persists lineageId so fork provenance is verifiable.
 */
export function forkFromState(
  dag: DAG,
  fromNodeId: string,
  reason: string,
  lineageId: string,
): ForkResult {
  const fromNode = dag.get(fromNodeId);
  if (!fromNode) throw new Error(`Node ${fromNodeId} not found`);

  const timestamp = Date.now();
  const forkNode = dag.addNew(
    [fromNodeId],
    { type: 'fork', fromNodeId, reason, lineageId },
    timestamp,
  );

  const ancestors = dag.ancestors(forkNode.id);
  ancestors.add(forkNode.id);

  // Ancestry is preserved relative to the lineage of fromNodeId specifically.
  // Only check roots that are ancestors of fromNodeId, not unrelated DAG roots.
  const fromAncestors = dag.ancestors(fromNodeId);
  fromAncestors.add(fromNodeId);
  const allNodes = dag.allNodes();
  const lineageRoots = allNodes.filter(
    n => n.parentIds.length === 0 && fromAncestors.has(n.id),
  );
  const ancestryPreserved = lineageRoots.every(root => ancestors.has(root.id));

  // Tautologically true: forkNode has fromNodeId as a parent so fromNodeId is always an ancestor.
  // Kept as an explicit, named invariant check to make the guarantee visible in ForkResult.
  const oldHistoryAccessible = ancestors.has(fromNodeId);

  return { newBranchHead: forkNode, ancestryPreserved, oldHistoryAccessible };
}

/**
 * Create a completely unrelated fork with a new genesis node.
 * No shared ancestry with the original DAG.
 */
export function unrelatedFork(members: string[]): { dag: DAG; genesisNode: DAGNode } {
  const dag = new DAG();
  const timestamp = Date.now();
  const genesisNode = dag.addNew([], { type: 'genesis', members }, timestamp);
  return { dag, genesisNode };
}
