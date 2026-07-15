export class TransportModel {
  private partitions: Map<string, Set<string>> = new Map();
  private allPeers: Set<string> = new Set();

  registerPeer(peerId: string): void {
    if (this.allPeers.has(peerId)) return;
    this.allPeers.add(peerId);
    this.partitions.set(peerId, new Set(this.allPeers));
    for (const [id, reachable] of this.partitions.entries()) {
      if (id !== peerId) reachable.add(peerId);
    }
  }

  partition(groupA: string[], groupB: string[]): void {
    for (const a of groupA) {
      if (!this.allPeers.has(a)) continue;
      for (const b of groupB) this.partitions.get(a)?.delete(b);
    }
    for (const b of groupB) {
      if (!this.allPeers.has(b)) continue;
      for (const a of groupA) this.partitions.get(b)?.delete(a);
    }
  }

  reconnectAll(): void {
    for (const id of this.allPeers) {
      this.partitions.set(id, new Set(this.allPeers));
    }
  }

  canSee(fromPeer: string, toPeer: string): boolean {
    if (fromPeer === toPeer) return true;
    return this.partitions.get(fromPeer)?.has(toPeer) ?? true;
  }
}
