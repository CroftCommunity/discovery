// Append-only, hash-linked, signed ledgers — one per actor. Nothing is ever
// edited in place; a correction is a new entry. Each entry carries:
//   - seq / prevHash  : hash-links it to the entry before it (a tamper-evident chain)
//   - kind / body     : the payload (a receipt, a statement, a seal, a royalty, ...)
//   - hash            : SHA-256 over the canonical form of everything above
//   - sigs            : one or more signatures over that hash (bilateral entries
//                       carry both parties' signatures)
//
// A standalone verifier (verify.ts) re-reads the JSONL file, recomputes every
// hash, re-checks the chain linkage, and re-checks every signature against the
// pinned public keys. So "the books balance" is not something we assert on trust;
// it is recomputable by anyone holding the file.

import { hashCanonical, signMessage, verifyMessage, type Keypair } from "./crypto.ts";
import type { Json } from "./canonical.ts";

export const GENESIS_PREV = "0".repeat(64);

export type LedgerEntry = {
  seq: number;
  prevHash: string;
  ts: string;
  kind: string;
  body: Json;
  hash: string;
  /** actorId -> hex signature over `hash`. */
  sigs: Record<string, string>;
};

/** The payload we hash — everything except the hash and signatures themselves. */
function preimage(seq: number, prevHash: string, ts: string, kind: string, body: Json) {
  return { seq, prevHash, ts, kind, body };
}

export class Ledger {
  readonly ownerId: string;
  readonly entries: LedgerEntry[] = [];

  constructor(ownerId: string) {
    this.ownerId = ownerId;
  }

  private lastHash(): string {
    return this.entries.length === 0
      ? GENESIS_PREV
      : this.entries[this.entries.length - 1].hash;
  }

  /**
   * Append a signed entry. `signers` are the parties whose signatures the entry
   * carries — one for a unilateral note, two for a bilateral receipt/statement.
   * The identities used as signature keys come from each signer's actorId.
   */
  append(
    kind: string,
    ts: string,
    body: Json,
    signers: { actorId: string; keypair: Keypair }[],
  ): LedgerEntry {
    const seq = this.entries.length;
    const prevHash = this.lastHash();
    const hash = hashCanonical(preimage(seq, prevHash, ts, kind, body));
    const sigs: Record<string, string> = {};
    for (const s of signers) sigs[s.actorId] = signMessage(s.keypair, hash);
    const entry: LedgerEntry = { seq, prevHash, ts, kind, body, hash, sigs };
    this.entries.push(entry);
    return entry;
  }

  toJSONL(): string {
    return this.entries.map((e) => JSON.stringify(e)).join("\n") + "\n";
  }
}

export type VerifyIssue = {
  seq: number;
  problem: string;
};

/**
 * Standalone-style verification of a list of entries against a keyring
 * (actorId -> public key hex). Returns every problem found; an empty list means
 * the ledger is internally consistent, correctly chained, and every signature is
 * valid under a pinned key. This is exactly what verify.ts runs over the files.
 */
export function verifyEntries(
  entries: LedgerEntry[],
  keyring: Record<string, string>,
): VerifyIssue[] {
  const issues: VerifyIssue[] = [];
  let expectedPrev = GENESIS_PREV;
  entries.forEach((e, idx) => {
    if (e.seq !== idx) {
      issues.push({ seq: e.seq, problem: `seq out of order: expected ${idx}` });
    }
    if (e.prevHash !== expectedPrev) {
      issues.push({ seq: e.seq, problem: "prevHash breaks the chain link" });
    }
    const recomputed = hashCanonical(preimage(e.seq, e.prevHash, e.ts, e.kind, e.body));
    if (recomputed !== e.hash) {
      issues.push({ seq: e.seq, problem: "hash does not match body (entry edited)" });
    }
    const signerIds = Object.keys(e.sigs);
    if (signerIds.length === 0) {
      issues.push({ seq: e.seq, problem: "entry carries no signature" });
    }
    for (const signerId of signerIds) {
      const pub = keyring[signerId];
      if (!pub) {
        issues.push({ seq: e.seq, problem: `no pinned key for signer ${signerId}` });
        continue;
      }
      if (!verifyMessage(pub, e.hash, e.sigs[signerId])) {
        issues.push({ seq: e.seq, problem: `bad signature from ${signerId}` });
      }
    }
    expectedPrev = e.hash;
  });
  return issues;
}
