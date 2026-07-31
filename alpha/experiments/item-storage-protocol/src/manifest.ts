// The manifest: the customer's signed list of what the provider is supposed to
// be keeping. Each item is a (fingerprint, size) pair; the list is sorted by
// fingerprint, reduced to a single root hash over a Merkle tree, and signed by
// the customer over that root.
//
// Two properties matter:
//   1. Expected bytes at rest is a PURE FUNCTION of the manifest (sum of sizes) —
//      the storage bill is computable by the customer with no retrieval and no
//      trust in the provider's claims.
//   2. The provider can recompute the SAME root from the copies it actually
//      holds; a mismatch (missing item, wrong size, tamper) is arithmetic-visible
//      before a single byte is retrieved.

import type { Json } from "./canonical.ts";
import { sha256hex } from "./crypto.ts";
import type { Actor } from "./actor.ts";

export interface ManifestItem {
  cid: string;
  size: number;
}

export interface SignedManifest {
  /** Sorted-by-cid list of (cid, size). */
  items: ManifestItem[];
  /** Merkle root over the items. */
  root: string;
  /** Customer id and signature over { root, itemCount, generatedDay }. */
  customer: string;
  generatedDay: number;
  sig: string;
}

/** Leaf hash binds both the fingerprint AND the claimed size. */
function leafHash(item: ManifestItem): string {
  return sha256hex(`leaf:${item.cid}:${item.size}`);
}

/**
 * Merkle root over the item leaves. Odd nodes at a level are promoted (the last
 * leaf is duplicated) — a standard, deterministic pairing. Empty list hashes to
 * a fixed sentinel.
 */
export function merkleRoot(items: ManifestItem[]): string {
  if (items.length === 0) return sha256hex("empty-manifest");
  let level = items.map(leafHash);
  while (level.length > 1) {
    const next: string[] = [];
    for (let i = 0; i < level.length; i += 2) {
      const a = level[i];
      const b = i + 1 < level.length ? level[i + 1] : level[i];
      next.push(sha256hex(`node:${a}:${b}`));
    }
    level = next;
  }
  return level[0];
}

/** Canonicalize the items list by sorting on cid. */
export function sortItems(items: ManifestItem[]): ManifestItem[] {
  return [...items].sort((x, y) => (x.cid < y.cid ? -1 : x.cid > y.cid ? 1 : 0));
}

/** The signed core: what the customer's signature actually covers. */
function manifestCore(root: string, itemCount: number, generatedDay: number): Json {
  return { root, itemCount, generatedDay };
}

/** Customer builds and signs a manifest over a set of items. */
export function buildSignedManifest(
  customer: Actor,
  rawItems: ManifestItem[],
  generatedDay: number,
): SignedManifest {
  const items = sortItems(rawItems);
  const root = merkleRoot(items);
  const sig = customer.sign(manifestCore(root, items.length, generatedDay));
  return { items, root, customer: customer.id, generatedDay, sig };
}

/** Verify the customer's signature over the manifest root, using a pinned key. */
export function verifyManifestSig(verifier: Actor, m: SignedManifest): boolean {
  // The root must actually be the root of the items it ships with...
  if (merkleRoot(m.items) !== m.root) return false;
  // ...and the customer must have signed that root.
  return verifier.verifyFrom(m.customer, manifestCore(m.root, m.items.length, m.generatedDay), m.sig);
}

/** Expected bytes at rest — a pure function of the manifest, no retrieval. */
export function expectedBytesAtRest(m: SignedManifest): number {
  return m.items.reduce((sum, it) => sum + it.size, 0);
}

/**
 * Provider recomputes the root from the copies it actually holds. Sizes come
 * from the manifest's claimed sizes for the cids the provider can produce; any
 * missing cid is simply absent, which changes the item set and therefore the
 * root.
 */
export function providerComputedRoot(
  m: SignedManifest,
  heldCids: Set<string>,
): string {
  const held = m.items.filter((it) => heldCids.has(it.cid));
  return merkleRoot(held);
}
