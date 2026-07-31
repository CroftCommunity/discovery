// Balance-forward statements. Each period closes into a co-signed statement:
// opening root, closing root, rent (byte-days integrated over the period),
// postage (sum of the period's receipts), audit cost, grace, fees. Statement N+1
// references statement N by hash, forming a chain from genesis. Because last
// month was agreed and hash-linked, this month only has to explain the change,
// and any edit to a historical figure breaks the chain at exactly that link.

import { hashCanonical } from "./crypto.ts";
import type { Json } from "./canonical.ts";

export const GENESIS_STATEMENT = "0".repeat(64);

export type StatementBody = {
  period: number;
  periodStartDay: number;
  periodEndDay: number;
  openingRoot: string;
  closingRoot: string;
  byteDays: number;
  rentCents: number;
  postageBytes: number;
  postageCents: number;
  auditCount: number;
  auditBytes: number;
  auditCents: number;
  auditTier: string;
  graceCents: number;
  feesCents: number;
  totalCents: number;
  prevStatementHash: string;
};

export type Statement = StatementBody & { hash: string };

export function buildStatement(body: StatementBody): Statement {
  const hash = hashCanonical(body as unknown as Json);
  return { ...body, hash };
}

export type ChainResult = { ok: true } | { ok: false; failedAt: number; reason: string };

/** Verify the statement chain from genesis; report the exact link that fails. */
export function verifyChain(statements: Statement[]): ChainResult {
  let expectedPrev = GENESIS_STATEMENT;
  for (let i = 0; i < statements.length; i++) {
    const s = statements[i];
    const { hash, ...body } = s;
    if (s.prevStatementHash !== expectedPrev) {
      return { ok: false, failedAt: i, reason: "prevStatementHash breaks the chain" };
    }
    if (hashCanonical(body as unknown as Json) !== hash) {
      return { ok: false, failedAt: i, reason: "statement body edited (hash mismatch)" };
    }
    if (s.period !== i) {
      return { ok: false, failedAt: i, reason: "period number out of sequence" };
    }
    expectedPrev = hash;
  }
  return { ok: true };
}
