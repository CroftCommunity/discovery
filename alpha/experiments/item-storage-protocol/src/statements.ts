// Balance-forward statements: each month stands on the last, so a dispute is
// bounded to a single period.
//
// A statement records the period's opening and closing manifest roots, the rent
// (byte-days), the postage (summed receipts), the audit cost, any fees, and any
// net grace. It is co-signed by customer and provider, and it references the
// previous statement by hash — the statements form a chain from genesis. Editing
// a historical figure breaks the chain at exactly that link; a fabricated extra
// period fails to attach.

import { canonicalize, type Json } from "./canonical.ts";
import { sha256hex } from "./crypto.ts";
import type { Actor } from "./actor.ts";

export const GENESIS_PREV = sha256hex("statement-genesis");

export interface StatementBody {
  period: number;
  openingRoot: string;
  closingRoot: string;
  rentByteDays: number;
  postageBytes: number;
  auditCount: number;
  auditBytes: number;
  fees: number;
  graceNet: number;
  prevStatementHash: string;
  closeDay: number;
}

export interface Statement {
  body: StatementBody;
  hash: string;
  customer: string;
  provider: string;
  customerSig: string;
  providerSig: string;
}

function bodyAsJson(b: StatementBody): Json {
  return {
    period: b.period,
    openingRoot: b.openingRoot,
    closingRoot: b.closingRoot,
    rentByteDays: b.rentByteDays,
    postageBytes: b.postageBytes,
    auditCount: b.auditCount,
    auditBytes: b.auditBytes,
    fees: b.fees,
    graceNet: b.graceNet,
    prevStatementHash: b.prevStatementHash,
    closeDay: b.closeDay,
  };
}

export function hashStatement(b: StatementBody): string {
  return sha256hex(canonicalize(bodyAsJson(b)));
}

/**
 * Both parties co-sign the statement and append it to their ledgers. The
 * statement hash covers the whole body including the link to the prior period.
 */
export function coSignStatement(
  customer: Actor,
  provider: Actor,
  body: StatementBody,
  ts: number,
): Statement {
  const json = bodyAsJson(body);
  const hash = hashStatement(body);
  const customerSig = customer.sign(json);
  const providerSig = provider.sign(json);
  const stmt: Statement = {
    body,
    hash,
    customer: customer.id,
    provider: provider.id,
    customerSig,
    providerSig,
  };
  const entry: Json = {
    body: json,
    hash,
    customer: customer.id,
    provider: provider.id,
    customerSig,
    providerSig,
  };
  customer.ledger.append("statement", ts, entry);
  provider.ledger.append("statement", ts, entry);
  return stmt;
}

/** Verify one statement: recomputed hash matches, both co-signatures verify. */
export function verifyStatement(checker: Actor, s: Statement): boolean {
  if (hashStatement(s.body) !== s.hash) return false;
  const json = bodyAsJson(s.body);
  const cKey = checker.pinnedKeyFor(s.customer);
  const pKey = checker.pinnedKeyFor(s.provider);
  if (!cKey || !pKey) return false;
  return (
    checker.verifyFrom(s.customer, json, s.customerSig) &&
    checker.verifyFrom(s.provider, json, s.providerSig)
  );
}

export interface ChainResult {
  ok: boolean;
  brokenAt: number | null; // period index of the first broken link, or null
  reason: string | null;
}

/**
 * Verify the whole chain from genesis. Each statement must co-sign-verify, its
 * hash must recompute, and its prevStatementHash must equal the prior
 * statement's hash (genesis sentinel for the first). Returns the exact link
 * where verification first fails.
 */
export function verifyChain(checker: Actor, statements: Statement[]): ChainResult {
  let prevHash = GENESIS_PREV;
  for (let i = 0; i < statements.length; i++) {
    const s = statements[i];
    if (!verifyStatement(checker, s)) {
      return { ok: false, brokenAt: s.body.period, reason: `statement ${s.body.period}: signature or hash invalid` };
    }
    if (s.body.prevStatementHash !== prevHash) {
      return {
        ok: false,
        brokenAt: s.body.period,
        reason: `statement ${s.body.period}: prev-hash ${s.body.prevStatementHash.slice(0, 12)} != expected ${prevHash.slice(0, 12)}`,
      };
    }
    prevHash = s.hash;
  }
  return { ok: true, brokenAt: null, reason: null };
}

/** Reconstruct Statement objects from a ledger's statement rows. */
export function readStatements(actor: Actor): Statement[] {
  return actor.ledger
    .entries()
    .filter((e) => e.type === "statement")
    .map((e) => {
      const b = e.body as unknown as {
        body: StatementBody;
        hash: string;
        customer: string;
        provider: string;
        customerSig: string;
        providerSig: string;
      };
      return {
        body: b.body,
        hash: b.hash,
        customer: b.customer,
        provider: b.provider,
        customerSig: b.customerSig,
        providerSig: b.providerSig,
      };
    });
}
