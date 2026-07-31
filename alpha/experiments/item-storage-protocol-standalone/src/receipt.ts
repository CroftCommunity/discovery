// Transfer receipts: bilaterally signed acknowledgments of bytes crossing the
// boundary between the two actors. Postage is charged by weight (bytes), not by
// trips, and each increment is signed on BOTH ends so billing is by agreement,
// not by one party's assertion.
//
// A receipt is a self-contained signed object — its signatures are taken over the
// receipt's own content, NOT over a ledger position — so the identical receipt can
// be embedded in both parties' ledgers and cross-checked. If either party later
// alters a byte count in its own copy, the embedded signatures no longer verify.

import { hashCanonical, signMessage, verifyMessage, type Keypair } from "./crypto.ts";
import type { Json } from "./canonical.ts";

export type ReceiptCore = {
  direction: "upload" | "download";
  cid: string;
  byteStart: number;
  byteEnd: number; // exclusive
  bytes: number;
  runningTotal: number;
  day: number;
  receiverId: string;
  senderId: string;
};

export type Receipt = ReceiptCore & {
  contentHash: string;
  /** Present iff the receiver acknowledged. Absent on a walkaway. */
  receiverSig?: string;
  /** Present iff the sender countersigned. */
  senderSig?: string;
};

export function receiptCoreHash(core: ReceiptCore): string {
  return hashCanonical(core as unknown as Json);
}

/**
 * Build a receipt. The receiver signs first (acknowledging receipt); the sender
 * countersigns. A walkaway is modeled by omitting the receiver's signature.
 */
export function makeReceipt(
  core: ReceiptCore,
  receiverKey: Keypair | null,
  senderKey: Keypair,
): Receipt {
  const contentHash = receiptCoreHash(core);
  const receiverSig = receiverKey ? signMessage(receiverKey, contentHash) : undefined;
  // The sender only countersigns an acknowledged transfer.
  const senderSig = receiverSig ? signMessage(senderKey, contentHash) : undefined;
  return { ...core, contentHash, receiverSig, senderSig };
}

/** A receipt is fully valid only if both parties signed its content. */
export function verifyReceipt(r: Receipt, keyring: Record<string, string>): boolean {
  const core: ReceiptCore = {
    direction: r.direction, cid: r.cid, byteStart: r.byteStart, byteEnd: r.byteEnd,
    bytes: r.bytes, runningTotal: r.runningTotal, day: r.day,
    receiverId: r.receiverId, senderId: r.senderId,
  };
  if (receiptCoreHash(core) !== r.contentHash) return false;
  const rPub = keyring[r.receiverId];
  const sPub = keyring[r.senderId];
  if (!rPub || !sPub) return false;
  if (!r.receiverSig || !r.senderSig) return false;
  return (
    verifyMessage(rPub, r.contentHash, r.receiverSig) &&
    verifyMessage(sPub, r.contentHash, r.senderSig)
  );
}

/** True when the receiver acknowledged (used to distinguish a walkaway). */
export function isAcknowledged(r: Receipt): boolean {
  return !!r.receiverSig && !!r.senderSig;
}
