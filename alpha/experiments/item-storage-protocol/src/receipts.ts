// Transfer receipts: postage by weight, not by trips, signed on each end.
//
// Bytes move in fixed-size increments. After each increment the receiver signs
// an acknowledgment (direction, item, byte range, running total, timestamp) and
// the sender countersigns. Both parties append the same co-signed receipt to
// their own ledgers, so the two ledgers reconcile to identical totals and
// neither side can quietly restate the weight moved.
//
// "Meter the boundary, not the machine": billing is the sum of signed acks, not
// a claim about internal work.

import type { Json } from "./canonical.ts";
import type { Actor } from "./actor.ts";
import { verifyJson } from "./crypto.ts";

export type Direction = "upload" | "download";

export interface Ack {
  direction: Direction;
  cid: string;
  rangeStart: number; // inclusive
  rangeEnd: number; // exclusive
  incBytes: number;
  runningTotal: number; // cumulative bytes for THIS transfer
  ts: number;
}

export interface Receipt {
  ack: Ack;
  sender: string;
  receiver: string;
  senderSig: string;
  /** null models a walkaway: the receiver took the increment but never signed. */
  receiverSig: string | null;
}

function ackCore(a: Ack): Json {
  return {
    direction: a.direction,
    cid: a.cid,
    rangeStart: a.rangeStart,
    rangeEnd: a.rangeEnd,
    incBytes: a.incBytes,
    runningTotal: a.runningTotal,
    ts: a.ts,
  };
}

export interface TransferResult {
  receipts: Receipt[];
  total: number;
  /** Set when the receiver walked away on the final increment. */
  unsignedExposureBytes: number;
}

export interface TransferOptions {
  /** If true, the receiver takes the final increment without signing it. */
  walkawayOnLast?: boolean;
}

/**
 * Run one metered transfer. The receiver signs each increment; the sender
 * countersigns; both append the co-signed receipt to their ledgers. Returns the
 * receipts and the reconciled total.
 */
export function runTransfer(
  direction: Direction,
  cid: string,
  totalBytes: number,
  incrementSize: number,
  sender: Actor,
  receiver: Actor,
  ts: number,
  opts: TransferOptions = {},
): TransferResult {
  const receipts: Receipt[] = [];
  let moved = 0;
  let unsignedExposureBytes = 0;

  const nIncrements = Math.ceil(totalBytes / incrementSize);
  for (let i = 0; i < nIncrements; i++) {
    const start = i * incrementSize;
    const end = Math.min(start + incrementSize, totalBytes);
    const incBytes = end - start;
    moved += incBytes;
    const ack: Ack = {
      direction,
      cid,
      rangeStart: start,
      rangeEnd: end,
      incBytes,
      runningTotal: moved,
      ts,
    };

    const isLast = i === nIncrements - 1;
    const walkaway = Boolean(opts.walkawayOnLast) && isLast;

    // The receiver signs first (they acknowledge receipt); the sender
    // countersigns. On a walkaway the receiver never signs.
    const receiverSig = walkaway ? null : receiver.sign(ackCore(ack));
    const senderSig = sender.sign(ackCore(ack));
    const receipt: Receipt = {
      ack,
      sender: sender.id,
      receiver: receiver.id,
      senderSig,
      receiverSig,
    };
    receipts.push(receipt);

    if (walkaway) {
      // Exposure is exactly this one increment: the sender parted with the
      // bytes but holds no receiver acknowledgment for them. Record it as a
      // first-class reputation event, not an off-book loss.
      unsignedExposureBytes = incBytes;
      sender.ledger.append("reputation_event", ts, {
        kind: "unsigned_delivery",
        counterparty: receiver.id,
        cid,
        exposureBytes: incBytes,
        note: "receiver took increment without signing",
      });
      // The unsigned increment is NOT counted toward billable postage — you bill
      // what was agreed, and this was not agreed.
      moved -= incBytes;
    } else {
      const body = receiptBody(receipt);
      sender.ledger.append("receipt", ts, body);
      receiver.ledger.append("receipt", ts, body);
    }
  }

  return { receipts, total: moved, unsignedExposureBytes };
}

export function receiptBody(r: Receipt): Json {
  return {
    ack: ackCore(r.ack),
    sender: r.sender,
    receiver: r.receiver,
    senderSig: r.senderSig,
    receiverSig: r.receiverSig,
  };
}

/** Verify a receipt's two signatures against pinned peer keys held by `checker`. */
export function verifyReceipt(checker: Actor, r: Receipt): boolean {
  const core = ackCore(r.ack);
  const senderKey = checker.pinnedKeyFor(r.sender);
  const receiverKey = checker.pinnedKeyFor(r.receiver);
  if (!senderKey) return false;
  if (!verifyJson(senderKey, core, r.senderSig)) return false;
  if (r.receiverSig === null) return false; // unsigned increment does not verify
  if (!receiverKey) return false;
  return verifyJson(receiverKey, core, r.receiverSig);
}

export interface Reconciliation {
  ok: boolean;
  customerTotal: number;
  providerTotal: number;
  errors: string[];
}

/**
 * Reconcile the two ledgers' receipt rows. They must contain the same co-signed
 * acks, every embedded signature must verify, and the summed byte totals must be
 * identical. This is what exposes a party that altered a byte count in its own
 * copy: the counterparty signature no longer covers the altered ack.
 */
export function reconcile(customer: Actor, provider: Actor): Reconciliation {
  const errors: string[] = [];
  const cust = customer.ledger.entries().filter((e) => e.type === "receipt");
  const prov = provider.ledger.entries().filter((e) => e.type === "receipt");

  if (cust.length !== prov.length) {
    errors.push(`receipt count differs: customer ${cust.length} vs provider ${prov.length}`);
  }

  const sum = (rows: typeof cust): number =>
    rows.reduce((acc, e) => acc + Number((e.body as { ack: { incBytes: number } }).ack.incBytes), 0);

  // Every receipt on both sides must carry two valid signatures.
  const checkRows = (rows: typeof cust, checker: Actor, who: string): void => {
    for (const e of rows) {
      const b = e.body as unknown as Receipt;
      if (!verifyReceipt(checker, { ack: b.ack, sender: b.sender, receiver: b.receiver, senderSig: b.senderSig, receiverSig: b.receiverSig })) {
        errors.push(`${who} receipt seq ${e.seq} (${b.ack.cid} @${b.ack.rangeStart}) failed signature check`);
      }
    }
  };
  checkRows(cust, customer, "customer-side");
  checkRows(prov, provider, "provider-side");

  const customerTotal = sum(cust);
  const providerTotal = sum(prov);
  if (customerTotal !== providerTotal) {
    errors.push(`totals differ: customer ${customerTotal} vs provider ${providerTotal}`);
  }

  return { ok: errors.length === 0, customerTotal, providerTotal, errors };
}
