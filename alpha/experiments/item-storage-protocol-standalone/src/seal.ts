// Sealing: cold storage where the plan is no movement and verification proves it.
// Three mechanisms compose:
//   - a SEAL DECLARATION: the customer pins a manifest root and signs it.
//   - a WRITE-PATH CEREMONY: the provider destroys the credential its write
//     function requires, so the normal write path fails closed. Immutability
//     becomes ENFORCED at the key ceremony, not merely promised.
//   - a ROTATION WATCH: any new signed root for the collection is an event;
//     the watch classifies it as customer-initiated (a legitimate unseal, proven
//     by the customer's signature) or an alarm (anything else).
//
// Detection and enforcement are distinct: audits against the pinned root catch a
// compromised path that mutates bytes directly (no new signature needed to be
// caught), while the watch catches root ROTATIONS. Together, every change is
// either customer-signed or alarmed.
//
// SEAM: "destroying" a credential here is deleting in-memory key material and
// making the guarded function fail closed. In production this is an HSM key
// ceremony / deletion of signing material, irreversible by construction.

import { signMessage, verifyMessage, type Keypair } from "./crypto.ts";
import { BlobStore, type Item } from "./item.ts";
import type { Manifest } from "./manifest.ts";

export type SealDeclaration = {
  collectionId: string;
  pinnedRoot: string;
  day: number;
  signerId: string;
  signature: string;
};

export function signSeal(
  collectionId: string,
  pinnedRoot: string,
  day: number,
  customer: { id: string; keypair: Keypair },
): SealDeclaration {
  const signature = signMessage(customer.keypair, `seal:${collectionId}:${pinnedRoot}`);
  return { collectionId, pinnedRoot, day, signerId: customer.id, signature };
}

/** The provider's write path. Requires a credential; fails closed without it. */
export class CollectionWriter {
  private credential: Keypair | null;

  constructor(credential: Keypair) {
    this.credential = credential;
  }

  write(store: BlobStore, item: Item): { ok: boolean; reason?: string } {
    if (!this.credential) {
      return { ok: false, reason: "write path is sealed: no write credential" };
    }
    store.put(item);
    return { ok: true };
  }

  /** The seal ceremony: destroy the write credential. Irreversible here. */
  destroyCredential(): void {
    this.credential = null;
  }

  get hasCredential(): boolean {
    return this.credential !== null;
  }
}

/** The customer's rotation (unseal) capability. Destroyed for the tombstone tier. */
export class UnsealAuthority {
  private key: Keypair | null;
  readonly ownerId: string;

  constructor(owner: { id: string; keypair: Keypair }) {
    this.key = owner.keypair;
    this.ownerId = owner.id;
  }

  /** Sign a rotation to a new root. Fails closed once the capability is destroyed. */
  rotate(collectionId: string, newRoot: string, day: number): RootAnnouncement | null {
    if (!this.key) return null;
    const signature = signMessage(this.key, `rotate:${collectionId}:${newRoot}`);
    return { collectionId, newRoot, day, signerId: this.ownerId, signature };
  }

  destroy(): void {
    this.key = null;
  }

  get isDestroyed(): boolean {
    return this.key === null;
  }
}

export type RootAnnouncement = {
  collectionId: string;
  newRoot: string;
  day: number;
  signerId: string;
  signature: string;
};

export type WatchEvent =
  | { type: "customer-initiated"; root: string; day: number }
  | { type: "alarm"; root: string; day: number; reason: string };

/** Monitors root announcements for a sealed collection. */
export class RotationWatch {
  readonly collectionId: string;
  readonly customerId: string;
  readonly customerPublicKeyHex: string;
  readonly events: WatchEvent[] = [];

  constructor(collectionId: string, customerId: string, customerPublicKeyHex: string) {
    this.collectionId = collectionId;
    this.customerId = customerId;
    this.customerPublicKeyHex = customerPublicKeyHex;
  }

  /** Classify an announced root change by its signature. */
  observe(a: RootAnnouncement): WatchEvent {
    const validCustomerSig =
      a.signerId === this.customerId &&
      verifyMessage(this.customerPublicKeyHex, `rotate:${a.collectionId}:${a.newRoot}`, a.signature);
    const ev: WatchEvent = validCustomerSig
      ? { type: "customer-initiated", root: a.newRoot, day: a.day }
      : { type: "alarm", root: a.newRoot, day: a.day, reason: "root change not signed by the customer" };
    this.events.push(ev);
    return ev;
  }
}

export type SealState = {
  collectionId: string;
  pinnedRoot: string;
  pinnedManifest: Manifest;
  writer: CollectionWriter;
  watch: RotationWatch;
  unsealAuthority: UnsealAuthority;
};
