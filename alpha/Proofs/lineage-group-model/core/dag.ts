import { createHash } from 'crypto';

export type NodePayload =
  | { type: 'message'; lineageId: string; content: string }
  | { type: 'add_member'; lineageId: string; targetLineageId: string }
  | { type: 'remove_member'; lineageId: string; targetLineageId: string }
  | { type: 'dial_change'; lineageId: string; dialKey: string; value: number }
  | { type: 'checkpoint'; members: string[]; settledThrough: string }
  | { type: 'fork'; fromNodeId: string; reason: string; lineageId: string }
  | {
      type: 'genesis';
      members: string[];
      // Social-layer params — genesis-fixed, immutable post-genesis (hash-linked)
      regime?: 'intimate' | 'public';
      outward_propagation_depth?: number;
      inward_visibility?: 'full' | 'partial' | 'none';
      openness_class?: 'closed' | 'open' | 'fully_open';
    }
  | { type: 'social_resolution'; chosenBranchHead: string; rejectedBranchHead: string; decidedBy: string };

export interface DAGNode {
  id: string;        // SHA-256(parentIds + payload + timestamp)
  parentIds: string[];
  payload: NodePayload;
  timestamp: number;
  tier: 'admin' | 'standard';
}

const ADMIN_PAYLOAD_TYPES = new Set([
  'add_member',
  'remove_member',
  'dial_change',
  'genesis',
  'checkpoint',
  'social_resolution',
  'fork',
]);

function isAdminPayload(payload: NodePayload): boolean {
  return ADMIN_PAYLOAD_TYPES.has(payload.type);
}

export class DAG {
  private nodes: Map<string, DAGNode> = new Map();
  // Track children for heads() computation
  private children: Map<string, Set<string>> = new Map();

  computeHash(parentIds: string[], payload: NodePayload, timestamp: number): string {
    return createHash('sha256')
      .update(JSON.stringify({ parentIds, payload, timestamp }))
      .digest('hex');
  }

  add(node: DAGNode): void {
    // Enforce tier ↔ payload.type consistency (prevents crafted nodes bypassing admin classification)
    const expectedTier: 'admin' | 'standard' = isAdminPayload(node.payload) ? 'admin' : 'standard';
    if (node.tier !== expectedTier) {
      throw new Error(
        `INV-TIER-CLASSIFICATION: Node ${node.id} has tier=${node.tier} but payload ${node.payload.type} requires ${expectedTier}`,
      );
    }

    // Reject mutation of already-existing admin ops
    if (this.nodes.has(node.id)) {
      const existing = this.nodes.get(node.id)!;
      if (existing.tier === 'admin') {
        throw new Error(`INV-IMMUTABLE-ADMIN: Cannot re-add or mutate admin node ${node.id}`);
      }
      return; // idempotent for non-admin
    }

    // Validate hash integrity
    const expectedId = this.computeHash(node.parentIds, node.payload, node.timestamp);
    if (node.id !== expectedId) {
      throw new Error(`INV-HASH-INTEGRITY: Node ${node.id} has invalid hash (expected ${expectedId})`);
    }

    // Validate all parents exist (allow genesis with no parents)
    for (const parentId of node.parentIds) {
      if (!this.nodes.has(parentId)) {
        throw new Error(`DAG integrity: Parent ${parentId} not found for node ${node.id}`);
      }
    }

    this.nodes.set(node.id, node);

    // Update children index
    if (!this.children.has(node.id)) {
      this.children.set(node.id, new Set());
    }
    for (const parentId of node.parentIds) {
      if (!this.children.has(parentId)) {
        this.children.set(parentId, new Set());
      }
      this.children.get(parentId)!.add(node.id);
    }
  }

  get(id: string): DAGNode | undefined {
    return this.nodes.get(id);
  }

  heads(): string[] {
    const heads: string[] = [];
    for (const [id, childSet] of this.children.entries()) {
      if (this.nodes.has(id) && childSet.size === 0) {
        heads.push(id);
      }
    }
    return heads;
  }

  ancestors(id: string): Set<string> {
    const visited = new Set<string>();
    const stack = [id];
    while (stack.length > 0) {
      const current = stack.pop()!;
      if (visited.has(current)) continue;
      visited.add(current);
      const node = this.nodes.get(current);
      if (node) {
        for (const p of node.parentIds) {
          if (!visited.has(p)) stack.push(p);
        }
      }
    }
    visited.delete(id);
    return visited;
  }

  lca(a: string, b: string): string | null {
    const ancestorsA = this.ancestors(a);
    ancestorsA.add(a);
    const ancestorsB = this.ancestors(b);
    ancestorsB.add(b);

    // Find common ancestors
    const common: string[] = [];
    for (const id of ancestorsA) {
      if (ancestorsB.has(id)) common.push(id);
    }

    if (common.length === 0) return null;

    // Find the one with no common ancestor as a descendant (lowest = most recent)
    // Use topological depth: ancestor with the greatest depth is LCA
    const depths = new Map<string, number>();
    const getDepth = (nodeId: string): number => {
      if (depths.has(nodeId)) return depths.get(nodeId)!;
      const node = this.nodes.get(nodeId);
      if (!node || node.parentIds.length === 0) {
        depths.set(nodeId, 0);
        return 0;
      }
      const d = 1 + Math.max(...node.parentIds.map(p => getDepth(p)));
      depths.set(nodeId, d);
      return d;
    };

    let lca = common[0];
    let maxDepth = getDepth(common[0]);
    for (const id of common) {
      const d = getDepth(id);
      if (d > maxDepth) {
        maxDepth = d;
        lca = id;
      }
    }
    return lca;
  }

  verify(id: string): boolean {
    const node = this.nodes.get(id);
    if (!node) return false;
    const expected = this.computeHash(node.parentIds, node.payload, node.timestamp);
    if (node.id !== expected) return false;
    for (const parentId of node.parentIds) {
      if (!this.verify(parentId)) return false;
    }
    return true;
  }

  /** For A2 test ONLY — returns tampered node without adding to DAG */
  tamper(id: string, newPayload: NodePayload): DAGNode {
    const original = this.nodes.get(id);
    if (!original) throw new Error(`Node ${id} not found`);
    // Return a node with same id but different payload (hash will not match)
    return {
      ...original,
      payload: newPayload,
      // id is intentionally unchanged to simulate tamper while keeping the ID reference
    };
  }

  allNodes(): DAGNode[] {
    return Array.from(this.nodes.values());
  }

  size(): number {
    return this.nodes.size;
  }

  has(id: string): boolean {
    return this.nodes.has(id);
  }

  /** Build a node (compute hash) and add it */
  addNew(parentIds: string[], payload: NodePayload, timestamp: number): DAGNode {
    const tier: 'admin' | 'standard' = isAdminPayload(payload) ? 'admin' : 'standard';
    const id = this.computeHash(parentIds, payload, timestamp);
    const node: DAGNode = { id, parentIds, payload, timestamp, tier };
    this.add(node);
    return node;
  }
}
