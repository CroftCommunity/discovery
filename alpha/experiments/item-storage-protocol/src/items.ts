// Helper for fabricating deterministic items ("a photo, a document, a post, a
// backup") of varied sizes. Content is seeded PRNG bytes, so a run is
// reproducible; the fingerprint is computed the same way the store computes it.

import type { Prng } from "./prng.ts";
import { computeCid } from "./store.ts";

export interface NamedItem {
  name: string;
  bytes: Buffer;
  cid: string;
  size: number;
}

export function makeItem(prng: Prng, name: string, size: number): NamedItem {
  const bytes = prng.bytes(size);
  return { name, bytes, cid: computeCid(bytes), size };
}

export function makeItems(prng: Prng, specs: { name: string; size: number }[]): NamedItem[] {
  return specs.map((s) => makeItem(prng, s.name, s.size));
}
