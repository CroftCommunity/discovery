export interface Epoch {
  id: number;
  parentEpochId: number | null;
  commitNodeId: string;    // DAG node that created this epoch
  members: string[];       // lineage IDs (not device IDs)
  branchId: string;        // which branch this epoch belongs to
}

export interface ForkDetection {
  isFork: boolean;
  forkPoint: number | null;
  branches: string[];
}

export class EpochChain {
  private epochs: Epoch[] = [];
  private epochsByParent: Map<number | null, Epoch[]> = new Map();
  private epochsById: Map<number, Epoch> = new Map();

  private snapshot(epoch: Epoch): Epoch {
    return { ...epoch, members: [...epoch.members] };
  }

  addEpoch(epoch: Epoch): void {
    if (this.epochsById.has(epoch.id)) {
      throw new Error(`Duplicate epoch id: ${epoch.id}`);
    }
    if (epoch.parentEpochId !== null && !this.epochsById.has(epoch.parentEpochId)) {
      throw new Error(`Parent epoch ${epoch.parentEpochId} not found`);
    }

    const stored = this.snapshot(epoch);
    this.epochs.push(stored);
    this.epochsById.set(stored.id, stored);

    const key = stored.parentEpochId;
    if (!this.epochsByParent.has(key)) {
      this.epochsByParent.set(key, []);
    }
    this.epochsByParent.get(key)!.push(stored);
  }

  detectFork(): ForkDetection {
    for (const [parentId, children] of this.epochsByParent.entries()) {
      if (children.length > 1) {
        return {
          isFork: true,
          forkPoint: parentId,
          branches: children.map(e => e.branchId),
        };
      }
    }
    return { isFork: false, forkPoint: null, branches: [] };
  }

  getLatestEpoch(branchId: string): Epoch | undefined {
    const branchEpochs = this.epochs
      .filter(e => e.branchId === branchId)
      .sort((a, b) => b.id - a.id);
    return branchEpochs[0] ? this.snapshot(branchEpochs[0]) : undefined;
  }

  getAllEpochs(): Epoch[] {
    return this.epochs.map(e => this.snapshot(e));
  }

  getEpochById(id: number): Epoch | undefined {
    const e = this.epochsById.get(id);
    return e ? this.snapshot(e) : undefined;
  }
}
