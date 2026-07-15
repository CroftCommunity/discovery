import { DAG, DAGNode, NodePayload } from '../core/dag';
import { TransportModel } from './transport';

function topoSortNodes(nodes: DAGNode[]): DAGNode[] {
  const nodeMap = new Map(nodes.map(n => [n.id, n]));
  const visited = new Set<string>();
  const result: DAGNode[] = [];
  function visit(id: string): void {
    if (visited.has(id) || !nodeMap.has(id)) return;
    visited.add(id);
    for (const parentId of nodeMap.get(id)!.parentIds) visit(parentId);
    result.push(nodeMap.get(id)!);
  }
  for (const id of nodeMap.keys()) visit(id);
  return result;
}

export class Peer {
  id: string;
  lineageId: string;
  localDAG: DAG;

  constructor(id: string, lineageId: string) {
    this.id = id;
    this.lineageId = lineageId;
    this.localDAG = new DAG();
  }

  applyOp(node: DAGNode): void {
    try { this.localDAG.add(node); } catch { /* skip duplicates/invalid */ }
  }

  sendOp(payload: NodePayload, parentIds?: string[]): DAGNode {
    const parents = parentIds ?? this.localDAG.heads();
    return this.localDAG.addNew(parents, payload, Date.now());
  }

  sync(other: Peer, transport: TransportModel): void {
    if (!transport.canSee(this.id, other.id)) return;
    for (const node of topoSortNodes(other.localDAG.allNodes())) {
      try { this.localDAG.add(node); } catch { /* skip */ }
    }
    for (const node of topoSortNodes(this.localDAG.allNodes())) {
      try { other.localDAG.add(node); } catch { /* skip */ }
    }
  }

  currentView(): { members: string[]; heads: string[] } {
    const members = new Set<string>();
    for (const node of topoSortNodes(this.localDAG.allNodes())) {
      if (node.payload.type === 'genesis') {
        members.clear();
        for (const m of node.payload.members) members.add(m);
      } else if (node.payload.type === 'add_member') {
        members.add(node.payload.targetLineageId);
      } else if (node.payload.type === 'remove_member') {
        members.delete(node.payload.targetLineageId);
      }
    }
    return { members: Array.from(members), heads: this.localDAG.heads() };
  }

  search(query: string): DAGNode[] {
    const q = query.toLowerCase();
    return this.localDAG.allNodes().filter(node => {
      if (node.payload.type === 'message') {
        return node.payload.content.toLowerCase().includes(q);
      }
      return JSON.stringify(node.payload).toLowerCase().includes(q);
    });
  }
}
