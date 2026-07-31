// An actor is a keypair and nothing more. Its identifier is derived from its
// public key, so "who you are" and "how we recognize your signature" are the same
// fact. Each actor owns exactly one append-only ledger.
//
// SEAM: a real deployment would carry a did:key / did:plc identity resolvable over
// the network. Here the identifier is a short hash of the raw public key and the
// two actors are in-process objects, not networked services.

import { deriveKeypair, sha256hex, type Keypair } from "./crypto.ts";
import { Ledger } from "./ledger.ts";

export class Actor {
  readonly name: string;
  readonly keypair: Keypair;
  readonly id: string;
  readonly ledger: Ledger;

  constructor(name: string, masterSeed: string, label: string) {
    this.name = name;
    this.keypair = deriveKeypair(masterSeed, label);
    this.id = Actor.deriveId(this.keypair.publicKeyHex);
    this.ledger = new Ledger(this.id);
  }

  /** Stable identifier: a short, deterministic function of the public key alone. */
  static deriveId(publicKeyHex: string): string {
    return "id:" + sha256hex(Buffer.from(publicKeyHex, "hex")).slice(0, 16);
  }

  get publicKeyHex(): string {
    return this.keypair.publicKeyHex;
  }

  signer() {
    return { actorId: this.id, keypair: this.keypair };
  }
}
