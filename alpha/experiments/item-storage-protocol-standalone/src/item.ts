// Items and their fingerprints, and the provider's content-addressed store.
// An item is any bytes a person asked us to keep. Its name IS its fingerprint,
// so an item cannot quietly become a different item: change one byte and the
// fingerprint changes, and the store is keyed by that fingerprint.

import { fingerprint } from "./crypto.ts";

export type Item = {
  /** A human label, purely for narration; identity is the fingerprint. */
  label: string;
  bytes: Buffer;
  cid: string; // the fingerprint (hex SHA-256). SEAM: stands in for a CIDv1.
  size: number;
};

/** Make an item from raw bytes; its cid is computed, not assigned. */
export function makeItem(label: string, bytes: Buffer): Item {
  return { label, bytes, cid: fingerprint(bytes), size: bytes.length };
}

/**
 * The provider's store: a map keyed by fingerprint. Retrieval re-fingerprints
 * the stored bytes and compares, so a byte-flip at rest is caught on the way out.
 */
export class BlobStore {
  private readonly blobs = new Map<string, Buffer>();

  put(item: Item): void {
    this.blobs.set(item.cid, Buffer.from(item.bytes));
  }

  has(cid: string): boolean {
    return this.blobs.has(cid);
  }

  /** Total bytes actually held, summed from stored blobs. */
  storedBytes(): number {
    let total = 0;
    for (const b of this.blobs.values()) total += b.length;
    return total;
  }

  /**
   * Retrieve by fingerprint and verify. Returns the bytes only if the stored
   * bytes still fingerprint to the requested cid; otherwise reports a mismatch
   * that names exactly which item failed.
   */
  retrieveVerified(cid: string): { ok: true; bytes: Buffer } | { ok: false; reason: string } {
    const stored = this.blobs.get(cid);
    if (!stored) return { ok: false, reason: `missing item ${cid}` };
    const actual = fingerprint(stored);
    if (actual !== cid) {
      return { ok: false, reason: `tampered item ${cid} (now fingerprints as ${actual})` };
    }
    return { ok: true, bytes: stored };
  }

  /** Adversarial helper: flip one byte of a stored item, in place, at rest. */
  corruptOneByte(cid: string): void {
    const stored = this.blobs.get(cid);
    if (!stored || stored.length === 0) throw new Error(`cannot corrupt ${cid}`);
    stored[0] = stored[0] ^ 0x01;
  }

  /** Adversarial helper: silently drop an item entirely (loss, not tamper). */
  drop(cid: string): void {
    this.blobs.delete(cid);
  }

  /** Bytes retrieved for an audit — the audit's true cost. */
  auditReadCost(cids: string[]): number {
    let total = 0;
    for (const cid of cids) {
      const b = this.blobs.get(cid);
      if (b) total += b.length;
    }
    return total;
  }
}
