// The Provider's content-addressed store: bytes keyed by their own fingerprint.
//
// An item's key is sha256(bytes). Retrieval re-fingerprints and compares, so a
// tampered item cannot masquerade as the item it replaced — its fingerprint no
// longer matches the key it is filed under.
//
// The adversarial helpers (`tamperFlipByte`, `overwriteRaw`) model a Provider or
// a compromised write path mutating stored bytes. They deliberately keep the old
// key, which is exactly the situation an audit must catch.

import { sha256hex } from "./crypto.ts";

export function computeCid(bytes: Buffer): string {
  return sha256hex(bytes);
}

export class Store {
  private readonly blobs = new Map<string, Buffer>();

  /** Store bytes under their fingerprint; returns the fingerprint (cid). */
  put(bytes: Buffer): string {
    const cid = computeCid(bytes);
    this.blobs.set(cid, Buffer.from(bytes));
    return cid;
  }

  has(cid: string): boolean {
    return this.blobs.has(cid);
  }

  /** Raw stored bytes for a cid (what is physically on disk), or undefined. */
  getRaw(cid: string): Buffer | undefined {
    const b = this.blobs.get(cid);
    return b ? Buffer.from(b) : undefined;
  }

  /**
   * Honest retrieval: return bytes only if they still fingerprint to the
   * requested cid. A tampered or corrupted blob returns undefined — the store
   * refuses to hand back bytes under a name they no longer own.
   */
  getVerified(cid: string): Buffer | undefined {
    const b = this.blobs.get(cid);
    if (!b) return undefined;
    return computeCid(b) === cid ? Buffer.from(b) : undefined;
  }

  delete(cid: string): boolean {
    return this.blobs.delete(cid);
  }

  cids(): string[] {
    return [...this.blobs.keys()];
  }

  size(): number {
    return this.blobs.size;
  }

  totalStoredBytes(): number {
    let total = 0;
    for (const b of this.blobs.values()) total += b.length;
    return total;
  }

  // --- adversarial mutators (model tamper; they keep the old key on purpose) ---

  /** Flip one byte of a stored blob, leaving it filed under the old cid. */
  tamperFlipByte(cid: string, index: number): void {
    const b = this.blobs.get(cid);
    if (!b) throw new Error(`cannot tamper unknown cid ${cid}`);
    b[index] = b[index] ^ 0xff;
  }

  /** Replace a stored blob's bytes wholesale, keeping the old cid key. */
  overwriteRaw(cid: string, bytes: Buffer): void {
    if (!this.blobs.has(cid)) throw new Error(`cannot overwrite unknown cid ${cid}`);
    this.blobs.set(cid, Buffer.from(bytes));
  }
}
