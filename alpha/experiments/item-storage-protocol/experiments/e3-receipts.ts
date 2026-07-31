// E3. Transfer receipts — "Meter the boundary, not the machine."
//
// Postage is by weight, not by trips, signed on each end. An upload and a
// download run in fixed-size increments; each increment is a co-signed receipt in
// both ledgers. Adversarial: (a) a party alters a byte count in its own ledger
// copy — the embedded counterparty signature exposes it; (b) walkaway — the
// receiver takes the final increment and never signs, and the unsigned exposure
// is exactly one increment, recorded as a reputation event.

import assert from "node:assert/strict";
import type { ExperimentResult } from "../src/experiment.ts";
import { createWorld } from "../src/world.ts";
import { Prng } from "../src/prng.ts";
import { makeItem } from "../src/items.ts";
import { runTransfer, reconcile, verifyReceipt, type Ack } from "../src/receipts.ts";

export function run(seed: number): ExperimentResult {
  const w = createWorld("E3", seed);
  const { customer, provider, store } = w;
  const prng = new Prng(seed);

  const item = makeItem(prng, "backup.bin", 4000);
  store.put(item.bytes);
  const INC = 1000;

  // Upload (customer -> provider) and download (provider -> customer), metered.
  const up = runTransfer("upload", item.cid, item.size, INC, customer, provider, w.clock.now());
  w.clock.advance(1);
  const down = runTransfer("download", item.cid, item.size, INC, provider, customer, w.clock.now());

  assert.equal(up.total, item.size, "upload total equals item size");
  assert.equal(down.total, item.size, "download total equals item size");

  // Both ledgers reconcile to identical totals, and every embedded signature
  // verifies.
  const rec = reconcile(customer, provider);
  assert.equal(rec.ok, true, `ledgers must reconcile: ${rec.errors.join("; ")}`);
  assert.equal(rec.customerTotal, rec.providerTotal, "customer and provider totals must match");
  assert.equal(rec.customerTotal, item.size * 2, "total postage = one upload + one download");

  // Adversarial (a): the Provider alters a byte count in its own copy of a
  // receipt. The Customer's embedded signature no longer covers the altered ack,
  // so the forgery fails the signature check.
  const provReceipts = provider.ledger.entries().filter((e) => e.type === "receipt");
  const target = provReceipts[0].body as unknown as {
    ack: Ack;
    sender: string;
    receiver: string;
    senderSig: string;
    receiverSig: string;
  };
  const forged = {
    ack: { ...target.ack, incBytes: target.ack.incBytes + 999 },
    sender: target.sender,
    receiver: target.receiver,
    senderSig: target.senderSig,
    receiverSig: target.receiverSig,
  };
  assert.equal(verifyReceipt(customer, target), true, "the genuine receipt verifies");
  assert.equal(verifyReceipt(customer, forged), false, "the altered receipt must fail signature check");

  // Adversarial (b): walkaway. The receiver takes the final increment without
  // signing. Exposure is exactly one increment — never more.
  const item2 = makeItem(prng, "handoff.bin", 4000);
  store.put(item2.bytes);
  const walk = runTransfer("upload", item2.cid, item2.size, INC, customer, provider, w.clock.now(), {
    walkawayOnLast: true,
  });
  assert.equal(walk.unsignedExposureBytes, INC, "unsigned exposure equals exactly one increment");
  assert.ok(walk.unsignedExposureBytes <= INC, "exposure never exceeds one increment");
  // Billable total excludes the unsigned increment (you bill what was agreed).
  assert.equal(walk.total, item2.size - INC, "unsigned increment is not billed as postage");

  const repEvents = customer.ledger.entries().filter((e) => e.type === "reputation_event");
  assert.equal(repEvents.length, 1, "exactly one reputation event recorded for the walkaway");
  assert.equal(
    (repEvents[0].body as { exposureBytes: number }).exposureBytes,
    INC,
    "the reputation event records the one-increment exposure",
  );

  const sentence = "Meter the boundary, not the machine.";
  const reportMd = [
    `An upload and a download of a ${item.size}-byte item ran in ${INC}-byte increments; each`,
    "increment is a co-signed receipt in both ledgers. The ledgers reconcile to identical",
    `totals (${rec.customerTotal} bytes of postage), and every embedded signature verifies.`,
    "",
    "Adversarial: altering a byte count in one ledger's own copy of a receipt is caught because",
    "the counterparty's signature no longer covers the altered ack. On a walkaway, the receiver",
    `took the final ${INC}-byte increment without signing; the unsigned exposure is exactly one`,
    "increment and is booked as a reputation event, not an off-book loss.",
  ].join("\n");

  return { id: "E3", title: "Transfer receipts", sentence, reportMd };
}
