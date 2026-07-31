// An actor is a keypair plus a ledger plus the set of peer keys it has pinned.
//
// E0: "both parties exist as keys, nothing more." An actor can sign, can verify
// a claimed peer's signature against the key it pinned for that peer, and owns
// exactly one append-only ledger.

import type { Json } from "./canonical.ts";
import {
  deriveSeedHex,
  identifierFromRawHex,
  keypairFromSeedHex,
  signJson,
  verifyJson,
  type Keypair,
} from "./crypto.ts";
import { Ledger } from "./ledger.ts";

export class Actor {
  readonly label: string;
  readonly id: string;
  readonly keypair: Keypair;
  readonly ledger: Ledger;
  /** peer id -> pinned raw public key (hex). */
  private readonly pinned = new Map<string, string>();

  constructor(masterSeed: number, label: string, ledgerPath: string) {
    this.label = label;
    this.keypair = keypairFromSeedHex(deriveSeedHex(masterSeed, label));
    this.id = identifierFromRawHex(this.keypair.publicRawHex);
    this.ledger = new Ledger(ledgerPath, this.id, this.keypair);
  }

  get publicRawHex(): string {
    return this.keypair.publicRawHex;
  }

  /** Pin a peer's public key so future messages from them can be verified. */
  pin(peer: Actor): void {
    this.pinned.set(peer.id, peer.publicRawHex);
  }

  pinnedKeyFor(peerId: string): string | undefined {
    return this.pinned.get(peerId);
  }

  /** Sign a JSON value with this actor's private key. */
  sign(value: Json): string {
    return signJson(this.keypair.privateKey, value);
  }

  /**
   * Verify a signature claimed to be from `peerId` using the key WE pinned for
   * them. Returns false if we never pinned that peer — recognition requires a
   * prior key exchange.
   */
  verifyFrom(peerId: string, value: Json, sigHex: string): boolean {
    const key = this.pinned.get(peerId);
    if (!key) return false;
    return verifyJson(key, value, sigHex);
  }
}
