import { DAG } from './dag';

export interface AncestryPath {
  from: string;
  to: string;
  path: string[];  // node IDs from 'from' to LCA
}

export function computeLCA(dag: DAG, a: string, b: string): string | null {
  return dag.lca(a, b);
}

export function ancestryPath(dag: DAG, from: string, to: string): AncestryPath {
  // BFS from 'from' toward root, collecting path to 'to' (or LCA)
  const visited = new Map<string, string | null>(); // nodeId -> parent in BFS
  const queue = [from];
  visited.set(from, null);

  while (queue.length > 0) {
    const current = queue.shift()!;
    if (current === to) {
      // Reconstruct path
      const path: string[] = [];
      let node: string | null = current;
      while (node !== null) {
        path.unshift(node);
        node = visited.get(node) ?? null;
      }
      return { from, to, path };
    }
    const node = dag.get(current);
    if (node) {
      for (const parentId of node.parentIds) {
        if (!visited.has(parentId)) {
          visited.set(parentId, current);
          queue.push(parentId);
        }
      }
    }
  }

  // 'to' not found as ancestor of 'from'; return path to LCA instead
  const lca = dag.lca(from, to);
  if (!lca) return { from, to, path: [from] };

  // BFS to lca
  const visited2 = new Map<string, string | null>();
  const queue2 = [from];
  visited2.set(from, null);
  while (queue2.length > 0) {
    const current = queue2.shift()!;
    if (current === lca) {
      const path: string[] = [];
      let node: string | null = current;
      while (node !== null) {
        path.unshift(node);
        node = visited2.get(node) ?? null;
      }
      return { from, to: lca, path };
    }
    const node = dag.get(current);
    if (node) {
      for (const parentId of node.parentIds) {
        if (!visited2.has(parentId)) {
          visited2.set(parentId, current);
          queue2.push(parentId);
        }
      }
    }
  }

  return { from, to, path: [from] };
}

export function commonAncestors(dag: DAG, a: string, b: string): Set<string> {
  const ancestorsA = dag.ancestors(a);
  ancestorsA.add(a);
  const ancestorsB = dag.ancestors(b);
  ancestorsB.add(b);

  const common = new Set<string>();
  for (const id of ancestorsA) {
    if (ancestorsB.has(id)) common.add(id);
  }
  return common;
}
