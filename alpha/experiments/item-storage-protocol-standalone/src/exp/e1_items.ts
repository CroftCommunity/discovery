// E1. Items and fingerprints — an item's name is its content.
// Ada creates several items of varied sizes; each is fingerprinted; the provider
// stores them keyed by fingerprint; Ada retrieves and re-fingerprints. Then the
// provider flips one byte of one stored item, and retrieval must fail for that
// item and that item only.

import type { World } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { makeItem } from "../item.ts";

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { store } = world;

  // Ada creates items of varied sizes. Content is deterministic (seeded) so the
  // fingerprints are stable across runs.
  const rng = world.rng("e1/item-bytes");
  const specs = [
    { label: "wedding-photo.jpg", size: 4096 },
    { label: "will.pdf", size: 512 },
    { label: "family-post.txt", size: 128 },
    { label: "backup.tar", size: 8192 },
    { label: "voice-note.ogg", size: 1500 },
  ];
  const items = specs.map((s) => {
    const bytes = Buffer.alloc(s.size);
    for (let i = 0; i < s.size; i++) bytes[i] = rng.int(256);
    return makeItem(s.label, bytes);
  });
  world.items = items;
  for (const it of items) store.put(it);

  // Round-trip: retrieve every item and re-fingerprint. All must match.
  let allRoundTrip = true;
  for (const it of items) {
    const r = store.retrieveVerified(it.cid);
    if (!r.ok) allRoundTrip = false;
  }
  c.ok("every untampered item round-trips (retrieve + re-fingerprint)", allRoundTrip);

  // Fingerprints are unique across distinct content.
  const cids = new Set(items.map((i) => i.cid));
  c.eq("all fingerprints are distinct", cids.size, items.length);

  // Adversarial: the provider flips one byte of one stored item at rest.
  const victim = items[2];
  store.corruptOneByte(victim.cid);
  const victimResult = store.retrieveVerified(victim.cid);
  c.ok("tampered item fails verification", !victimResult.ok);
  c.ok(
    "detection identifies exactly which item was tampered",
    !victimResult.ok && victimResult.reason.includes(victim.cid),
  );

  // Every OTHER item still round-trips — the damage is localized to one item.
  let othersOk = true;
  for (const it of items) {
    if (it.cid === victim.cid) continue;
    if (!store.retrieveVerified(it.cid).ok) othersOk = false;
  }
  c.ok("tamper is localized: all other items still verify", othersOk);

  // Repair for the rest of the narrative: restore the good bytes.
  store.put(victim);
  c.ok("restored item verifies again", store.retrieveVerified(victim.cid).ok);

  return {
    id: "E1",
    title: "Items and fingerprints",
    plainSentence: "An item cannot quietly become a different item.",
    assertions: c.results,
    tables: [
      {
        title: "Ada's items",
        headers: ["label", "size (bytes)", "fingerprint (truncated)"],
        rows: items.map((i) => [i.label, i.size, i.cid.slice(0, 20) + "…"]),
      },
    ],
    notes: [],
  };
}
