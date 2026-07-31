// Append-only, signed ledger — one JSON Lines file per actor under ./ledgers/.
//
// Ground rules (Part 2): every entry is a signed object; nothing is ever edited
// in place; corrections are new entries. That is enforced structurally here —
// the only mutation is `append`, which opens the file in append mode.
//
// Each entry is self-describing enough that a standalone verifier (see
// scripts/verify-ledgers.ts) can check every signature with no other state: the
// signer's raw public key travels with the entry, and the identifier is a pure
// function of that key (crypto.ts), so the verifier can confirm the entry was
// signed by the actor it claims to be from.

import { appendFileSync, existsSync, readFileSync } from "node:fs";
import type { Json } from "./canonical.ts";
import {
  identifierFromRawHex,
  signJson,
  verifyJson,
  type Keypair,
} from "./crypto.ts";

/** The portion of an entry that is covered by the signature. */
export interface SignedCore {
  seq: number;
  ts: number;
  type: string;
  actor: string;
  body: Json;
}

export interface LedgerEntry extends SignedCore {
  sig: string;
  pubkey: string; // raw ed25519 public key, hex
}

export class Ledger {
  readonly path: string;
  readonly actorId: string;
  private readonly keypair: Keypair;
  private seq: number;

  constructor(path: string, actorId: string, keypair: Keypair) {
    this.path = path;
    this.actorId = actorId;
    this.keypair = keypair;
    // Resume the sequence counter if the file already exists (append-only).
    this.seq = existsSync(path) ? Ledger.read(path).length : 0;
  }

  /** Append a signed entry authored by this ledger's owner. */
  append(type: string, ts: number, body: Json): LedgerEntry {
    const core: SignedCore = {
      seq: this.seq++,
      ts,
      type,
      actor: this.actorId,
      body,
    };
    const sig = signJson(this.keypair.privateKey, core as unknown as Json);
    const entry: LedgerEntry = {
      ...core,
      sig,
      pubkey: this.keypair.publicRawHex,
    };
    appendFileSync(this.path, JSON.stringify(entry) + "\n");
    return entry;
  }

  /** Read all entries from a ledger file. */
  static read(path: string): LedgerEntry[] {
    if (!existsSync(path)) return [];
    return readFileSync(path, "utf8")
      .split("\n")
      .filter((l) => l.trim().length > 0)
      .map((l) => JSON.parse(l) as LedgerEntry);
  }

  entries(): LedgerEntry[] {
    return Ledger.read(this.path);
  }
}

export interface VerifyResult {
  path: string;
  count: number;
  ok: boolean;
  errors: string[];
}

/**
 * Verify every signature in a ledger file with no external state. For each
 * entry: the signature must verify over the signed core under the entry's own
 * public key, the actor id must be the derived identifier of that key, and the
 * seq column must be a dense 0..n-1 run (no silent gaps or reorders).
 */
export function verifyLedgerFile(path: string): VerifyResult {
  const errors: string[] = [];
  const entries = Ledger.read(path);
  entries.forEach((e, i) => {
    const core: SignedCore = {
      seq: e.seq,
      ts: e.ts,
      type: e.type,
      actor: e.actor,
      body: e.body,
    };
    if (!verifyJson(e.pubkey, core as unknown as Json, e.sig)) {
      errors.push(`entry ${i} (seq ${e.seq}, ${e.type}): bad signature`);
    }
    const derived = identifierFromRawHex(e.pubkey);
    if (derived !== e.actor) {
      errors.push(
        `entry ${i} (seq ${e.seq}): actor ${e.actor} does not match key-derived id ${derived}`,
      );
    }
    if (e.seq !== i) {
      errors.push(`entry ${i}: seq is ${e.seq}, expected ${i} (append-only order broken)`);
    }
  });
  return { path, count: entries.length, ok: errors.length === 0, errors };
}
