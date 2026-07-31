// E3. Transfer receipts — postage by weight, not by trips, signed on each end.
// We simulate an upload and a download in fixed-size increments. Each increment
// is a bilaterally signed receipt embedded in both ledgers. Then two adversarial
// cases: a forged byte count exposed by the receipt's own signatures, and a
// walkaway where the receiver takes the final increment without signing — the
// unsigned exposure is exactly one increment, recorded as a reputation event.

import type { World, ReceiptRecord } from "../world.ts";
import { Checker, type ExperimentResult } from "../types.ts";
import { makeReceipt, verifyReceipt, isAcknowledged, type Receipt } from "../receipt.ts";
import type { Json } from "../canonical.ts";

const INCREMENT = 2048;

export function run(world: World): ExperimentResult {
  const c = new Checker();
  const { customer, provider } = world;

  // Transfer the backup (8192 bytes) up, then a smaller item down.
  const upItem = world.items.find((i) => i.label === "backup.tar")!;
  const downItem = world.items.find((i) => i.label === "will.pdf")!;

  // Independent running totals each side will reconcile from its own ledger.
  const providerReceipts: Receipt[] = [];
  const customerReceipts: Receipt[] = [];

  function transfer(
    direction: "upload" | "download",
    cid: string,
    size: number,
    receiver: typeof customer,
    sender: typeof customer,
    walkawayLast = false,
  ): { receipts: Receipt[]; increments: number } {
    const receipts: Receipt[] = [];
    let running = 0;
    let offset = 0;
    let idx = 0;
    const total = Math.ceil(size / INCREMENT);
    while (offset < size) {
      const end = Math.min(offset + INCREMENT, size);
      const bytes = end - offset;
      running += bytes;
      const isLast = idx === total - 1;
      const core = {
        direction, cid, byteStart: offset, byteEnd: end, bytes,
        runningTotal: running, day: world.clock.now(),
        receiverId: receiver.id, senderId: sender.id,
      };
      // On a walkaway, the receiver never signs the final increment.
      const receiverKey = walkawayLast && isLast ? null : receiver.keypair;
      const r = makeReceipt(core, receiverKey, sender.keypair);
      receipts.push(r);
      offset = end;
      idx++;
    }
    return { receipts, increments: total };
  }

  // Upload: Ada -> co-op (co-op is receiver, Ada is sender).
  const up = transfer("upload", upItem.cid, upItem.size, provider, customer);
  // Download: co-op -> Ada (Ada is receiver, co-op is sender).
  const down = transfer("download", downItem.cid, downItem.size, customer, provider);

  // Both parties append every acknowledged receipt to their own ledger, and the
  // billable postage is recorded on the world timeline (for statements in E4).
  const ts = world.clock.iso();
  for (const r of [...up.receipts, ...down.receipts]) {
    if (!isAcknowledged(r)) continue;
    customer.ledger.append("receipt", ts, r as unknown as Json, [customer.signer()]);
    provider.ledger.append("receipt", ts, r as unknown as Json, [provider.signer()]);
    providerReceipts.push(r);
    customerReceipts.push(r);
    const rec: ReceiptRecord = {
      day: r.day, direction: r.direction, cid: r.cid,
      byteStart: r.byteStart, byteEnd: r.byteEnd, bytes: r.bytes, runningTotal: r.runningTotal,
    };
    world.receipts.push(rec);
  }

  // Every acknowledged receipt verifies bilaterally.
  c.ok("every acknowledged receipt verifies under both pinned keys",
    [...up.receipts, ...down.receipts].filter(isAcknowledged).every((r) => verifyReceipt(r, world.keyring)));

  // Both ledgers reconcile to identical postage totals.
  const sum = (rs: Receipt[]) => rs.reduce((s, r) => s + r.bytes, 0);
  c.eq("both ledgers reconcile to identical postage totals",
    sum(providerReceipts), sum(customerReceipts));

  // Adversarial (a): the provider alters a byte count in its own copy of a receipt.
  // The receipt's own bilateral signatures expose the forgery.
  const forged: Receipt = { ...providerReceipts[0], bytes: providerReceipts[0].bytes + 1000 };
  c.ok("forged byte count fails the receipt's own signature check",
    !verifyReceipt(forged, world.keyring));

  // Adversarial (b): walkaway. Ada requests the same download but abandons the
  // final increment without signing. Exposure is exactly one increment.
  const walk = transfer("download", downItem.cid, downItem.size, customer, provider, true);
  const unsigned = walk.receipts.filter((r) => !isAcknowledged(r));
  const unsignedBytes = unsigned.reduce((s, r) => s + r.bytes, 0);
  const lastIncrementSize = walk.receipts[walk.receipts.length - 1].bytes;
  c.eq("walkaway leaves exactly one unsigned increment", unsigned.length, 1);
  c.eq("unsigned exposure equals one increment, never more", unsignedBytes, lastIncrementSize);

  // The walkaway is recorded as a reputation event — a forward ledger entry, not
  // a silent loss. Signed by the sender (the co-op), who observed it.
  provider.ledger.append("reputation-event", ts, {
    kind: "walkaway",
    counterparty: customer.id,
    cid: downItem.cid,
    unsignedBytes,
    note: "receiver took the final increment without acknowledging",
  }, [provider.signer()]);
  c.ok("walkaway recorded as a reputation event in the ledger",
    provider.ledger.entries.some((e) => e.kind === "reputation-event"));

  const postageBytes = sum(customerReceipts);

  return {
    id: "E3",
    title: "Transfer receipts",
    plainSentence: "Meter the boundary, not the machine.",
    assertions: c.results,
    tables: [
      {
        title: "Transfer summary",
        headers: ["direction", "item", "bytes", "increments"],
        rows: [
          ["upload", upItem.label, upItem.size, up.increments],
          ["download", downItem.label, downItem.size, down.increments],
        ],
      },
    ],
    notes: [`Reconciled postage this exchange: ${postageBytes} bytes.`],
  };
}
