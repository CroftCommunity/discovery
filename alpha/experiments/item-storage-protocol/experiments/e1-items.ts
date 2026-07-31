// E1. Items and fingerprints — an item's name is its content.
//
// The Customer creates several items of varied sizes; each is fingerprinted; the
// Provider stores them keyed by fingerprint; the Customer retrieves and
// re-fingerprints. Adversarial: the Provider flips one byte of one stored item;
// retrieval must fail verification for that item and that item only.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItems } from "../src/items.ts";
import { computeCid } from "../src/store.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E1", seed);
  const { customer, provider, store } = w;
  const prng = new Prng(seed);

  const items = makeItems(prng, [
    { name: "photo.jpg", size: 4096 },
    { name: "letter.txt", size: 512 },
    { name: "post.json", size: 1500 },
    { name: "backup.bin", size: 9000 },
    { name: "avatar.png", size: 300 },
  ]);

  // Provider stores each item keyed by its fingerprint.
  for (const it of items) {
    const cid = store.put(it.bytes);
    assert.equal(cid, it.cid, "store must key by the item's own fingerprint");
    provider.ledger.append("item_stored", w.clock.now(), { cid: it.cid, size: it.size, name: it.name });
    customer.ledger.append("item_owed", w.clock.now(), { cid: it.cid, size: it.size, name: it.name });
  }

  // Customer retrieves and re-fingerprints: every untampered item round-trips.
  for (const it of items) {
    const got = store.getVerified(it.cid);
    assert.notEqual(got, undefined, `item ${it.name} must be retrievable`);
    assert.equal(computeCid(got!), it.cid, `item ${it.name} must re-fingerprint to its name`);
    assert.equal(Buffer.compare(got!, it.bytes), 0, `item ${it.name} bytes must be identical`);
  }

  // Adversarial: the Provider flips one byte of exactly one stored item.
  const victim = items[3]; // backup.bin
  const flipIndex = prng.int(victim.size);
  store.tamperFlipByte(victim.cid, flipIndex);

  // Detection: retrieval fails for the victim, and only the victim.
  const failed: string[] = [];
  for (const it of items) {
    if (store.getVerified(it.cid) === undefined) failed.push(it.cid);
  }
  assert.deepEqual(failed, [victim.cid], "exactly the tampered item must fail verification");

  // The raw (unverified) bytes are still there — the store did not lose them; it
  // refuses to serve them under a name they no longer own.
  assert.notEqual(store.getRaw(victim.cid), undefined, "tampered bytes still present");
  assert.notEqual(computeCid(store.getRaw(victim.cid)!), victim.cid, "tampered bytes no longer match the name");

  const sentence = "An item cannot quietly become a different item.";
  const reportMd = [
    `The Customer created ${items.length} items of sizes ${items.map((i) => i.size).join(", ")} bytes.`,
    "Each is stored under its SHA-256 fingerprint; every untampered item round-trips and",
    "re-fingerprints to its own name.",
    "",
    `Adversarial: one byte of \`${victim.name}\` (${victim.cid.slice(0, 12)}…) was flipped at index`,
    `${flipIndex}. Verification then failed for that item and for no other — detection localizes to`,
    "the single changed item.",
  ].join("\n");

  return { id: "E1", title: "Items and fingerprints", sentence, reportMd };
}
