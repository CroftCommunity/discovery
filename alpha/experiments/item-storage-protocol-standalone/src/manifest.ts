// The manifest: the customer's signed list of what the provider is supposed to be
// keeping. It is a sorted list of (fingerprint, size) pairs, a Merkle root over
// that list, and the customer's signature over the root. Because the customer
// signs it, "what we owe them" is written in their handwriting — and because the
// sizes are in it, the storage bill (byte-days) is a pure function of a document
// the customer authored, computable by the customer without trusting the provider.

import { sha256hex, signMessage, verifyMessage, type Keypair } from "./crypto.ts";

export type ManifestLeaf = { cid: string; size: number };

export type Manifest = {
  /** Sorted by cid, so the root is canonical for a given set of items. */
  leaves: ManifestLeaf[];
  root: string;
  /** Total bytes at rest implied by the list — the rent base. */
  totalBytes: number;
  /** Customer id + signature over the root. */
  signerId: string;
  signature: string;
};

/** Hash of a single leaf: binds the fingerprint to its claimed size. */
function leafHash(leaf: ManifestLeaf): string {
  return sha256hex(`leaf:${leaf.cid}:${leaf.size}`);
}

/** Merkle root over the leaf hashes (duplicate-last padding for odd levels). */
export function merkleRoot(leaves: ManifestLeaf[]): string {
  if (leaves.length === 0) return sha256hex("empty-manifest");
  let level = leaves.map(leafHash);
  while (level.length > 1) {
    const next: string[] = [];
    for (let i = 0; i < level.length; i += 2) {
      const left = level[i];
      const right = i + 1 < level.length ? level[i + 1] : level[i];
      next.push(sha256hex(`node:${left}:${right}`));
    }
    level = next;
  }
  return level[0];
}

/** Build and sign a manifest from a set of (cid, size) pairs. */
export function buildManifest(
  items: ManifestLeaf[],
  customerId: string,
  customerKey: Keypair,
): Manifest {
  const leaves = [...items].sort((a, b) => (a.cid < b.cid ? -1 : a.cid > b.cid ? 1 : 0));
  const root = merkleRoot(leaves);
  const totalBytes = leaves.reduce((s, l) => s + l.size, 0);
  const signature = signMessage(customerKey, root);
  return { leaves, root, totalBytes, signerId: customerId, signature };
}

/** Verify the customer's signature over the manifest root. */
export function verifyManifest(manifest: Manifest, customerPublicKeyHex: string): boolean {
  const recomputedRoot = merkleRoot(manifest.leaves);
  if (recomputedRoot !== manifest.root) return false;
  return verifyMessage(customerPublicKeyHex, manifest.root, manifest.signature);
}

/** Expected bytes at rest — a pure function of the manifest, no retrieval needed. */
export function expectedBytes(manifest: Manifest): number {
  return manifest.leaves.reduce((s, l) => s + l.size, 0);
}
