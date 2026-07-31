// Seal and tombstone mechanics (E7, E8).
//
// A sealed collection's plan is "no movement." Two capabilities gate movement:
//   - the provider's WRITE credential (needed to mutate stored bytes the honest
//     way), and
//   - the customer's ROTATION/UNSEAL capability (needed to legitimately publish
//     a new signed root).
//
// Sealing (E7, revocable) destroys the provider's write credential: the write
// path fails closed, but the customer can still unseal by signing a rotation
// with their offline key. Tombstoning (E8, permanent) additionally destroys the
// customer's rotation capability: no party can move anything, ever.
//
// SEAM: destroyed key material is modeled by deleting the in-process capability
// (setting it null) and making the guarded function fail closed without it. In
// production this is a key-ceremony step — shredding an HSM-held key or the
// offline recovery share — not a nulled field.

import type { Json } from "./canonical.ts";
import type { Store } from "./store.ts";
import type { Actor } from "./actor.ts";
import { verifyJson } from "./crypto.ts";

/** Guards the honest write path with a destroyable credential. */
export class WriteGate {
  private writeKey: string | null;

  constructor(writeKey: string) {
    this.writeKey = writeKey;
  }

  hasWriteCredential(): boolean {
    return this.writeKey !== null;
  }

  /** Honest write. Fails closed if the credential has been destroyed. */
  write(store: Store, bytes: Buffer): string {
    if (this.writeKey === null) {
      throw new Error("write path failed closed: write credential destroyed by seal ceremony");
    }
    return store.put(bytes);
  }

  /** The seal ceremony: destroy the provider-held write credential. */
  destroyWriteCredential(): void {
    this.writeKey = null;
  }
}

/** Guards the customer's legitimate unseal/rotation path. */
export class RotationCapability {
  private capability: string | null;

  constructor(capability: string) {
    this.capability = capability;
  }

  hasCapability(): boolean {
    return this.capability !== null;
  }

  /**
   * Customer publishes a new signed root (a legitimate unseal/rotation). Fails
   * closed if the capability has been destroyed (tombstone).
   */
  rotate(customer: Actor, newRoot: string, day: number): RootChangeEvent {
    if (this.capability === null) {
      throw new Error("unseal path failed closed: rotation capability destroyed by tombstone ceremony");
    }
    const body = rotationCore(newRoot, day);
    const sig = customer.sign(body);
    return { root: newRoot, day, claimedSigner: customer.id, sig };
  }

  /** The tombstone ceremony: destroy the customer's rotation capability. */
  destroyCapability(): void {
    this.capability = null;
  }
}

export interface RootChangeEvent {
  root: string;
  day: number;
  claimedSigner: string;
  sig: string;
}

export function rotationCore(root: string, day: number): Json {
  return { root, day };
}

export type RotationClass = "customer-initiated" | "alarm";

/**
 * The rotation watch. Any new signed root for a sealed collection is an event.
 * It is legitimate ONLY if it verifies under the customer's pinned offline key;
 * anything else — including an unsigned or attacker-signed root — is alarmed.
 *
 * `customerPinnedKey` is the raw ed25519 public key the monitor pinned for the
 * customer at seal time.
 */
export function classifyRotation(
  customerId: string,
  customerPinnedKey: string,
  ev: RootChangeEvent,
): RotationClass {
  const verifies = verifyJson(customerPinnedKey, rotationCore(ev.root, ev.day), ev.sig);
  if (verifies && ev.claimedSigner === customerId) return "customer-initiated";
  return "alarm";
}
