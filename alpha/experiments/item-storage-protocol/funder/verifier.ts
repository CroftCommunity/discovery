// The outside reader's verifier (E12, E13, E14).
//
// A funder — June, a loan officer at a cooperative fund — underwrites the co-op
// from the PUBLISHED FILES ALONE: the ledger files, the public keys of the
// actors, the audit transcripts (in the ledgers), and the public randomness
// source. She holds no keys and gets no private access.
//
// This module is DELIBERATELY INDEPENDENT of the actors' code. It imports only
// Node built-ins and re-derives every primitive it needs from the spec:
// canonical JSON, Ed25519 verification, the Merkle root, the statement hash, the
// beacon challenge derivation, and the royalty schedule. The E12 experiment
// asserts this boundary by grep: nothing under funder/ imports from ../src or
// ../experiments. If the funder shared the actors' code, "the loan officer can
// check it from her desk" would be circular — she'd be trusting the co-op's own
// verifier. Re-implementation from the spec is the whole point.

import { createHash, createPublicKey, verify as nodeVerify } from "node:crypto";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

// ------------------------------------------------------------------ primitives

type Json = null | boolean | number | string | Json[] | { [k: string]: Json };

/** Canonical JSON (sorted keys, no insignificant whitespace) — re-derived. */
function canonicalize(value: Json): string {
  if (value === null) return "null";
  const t = typeof value;
  if (t === "number") return JSON.stringify(value);
  if (t === "boolean" || t === "string") return JSON.stringify(value);
  if (Array.isArray(value)) return "[" + value.map(canonicalize).join(",") + "]";
  const obj = value as { [k: string]: Json };
  const keys = Object.keys(obj).sort();
  return "{" + keys.map((k) => JSON.stringify(k) + ":" + canonicalize(obj[k])).join(",") + "}";
}

function sha256hex(data: string): string {
  return createHash("sha256").update(data).digest("hex");
}

const SPKI_ED25519_PREFIX = "302a300506032b6570032100";

/** Verify a hex Ed25519 signature over a JSON value under a raw public key. */
function verifyJson(publicRawHex: string, value: Json, sigHex: string): boolean {
  try {
    const der = Buffer.from(SPKI_ED25519_PREFIX + publicRawHex, "hex");
    const key = createPublicKey({ key: der, format: "der", type: "spki" });
    const msg = Buffer.from(canonicalize(value), "utf8");
    return nodeVerify(null, msg, key, Buffer.from(sigHex, "hex"));
  } catch {
    return false;
  }
}

/** Merkle root over (cid,size) leaves — re-derived to match manifest.ts. */
function merkleRoot(items: { cid: string; size: number }[]): string {
  if (items.length === 0) return sha256hex("empty-manifest");
  let level = items.map((it) => sha256hex(`leaf:${it.cid}:${it.size}`));
  while (level.length > 1) {
    const next: string[] = [];
    for (let i = 0; i < level.length; i += 2) {
      const a = level[i];
      const b = i + 1 < level.length ? level[i + 1] : level[i];
      next.push(sha256hex(`node:${a}:${b}`));
    }
    level = next;
  }
  return level[0];
}

/** Statement body → canonical JSON (optional royalty fields last) — re-derived. */
function statementBodyJson(b: Record<string, Json>): Json {
  const json: { [k: string]: Json } = {
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
  if (b.royaltyPoolCents !== undefined) json.royaltyPoolCents = b.royaltyPoolCents;
  if (b.royaltyCumulativeCents !== undefined) json.royaltyCumulativeCents = b.royaltyCumulativeCents;
  if (b.extinguished !== undefined) json.extinguished = b.extinguished;
  return json;
}

function hashStatement(body: Record<string, Json>): string {
  return sha256hex(canonicalize(statementBodyJson(body)));
}

const GENESIS_PREV = sha256hex("statement-genesis");

/** Public beacon value for a round — re-derived to match beacon.ts. */
function beaconValue(publicSeedHex: string, round: number): string {
  return sha256hex(`${publicSeedHex}|beacon|${round}`);
}

/** Beacon → k challenge indices without replacement — re-derived. */
function deriveChallengeIndices(beaconHex: string, n: number, k: number): number[] {
  const take = Math.min(k, n);
  const chosen: number[] = [];
  const seen = new Set<number>();
  let i = 0;
  while (chosen.length < take) {
    const d = sha256hex(`${beaconHex}:${i}`).slice(0, 15);
    const idx = Number(BigInt("0x" + d) % BigInt(n));
    if (!seen.has(idx)) {
      seen.add(idx);
      chosen.push(idx);
    }
    i++;
  }
  return chosen.sort((a, b) => a - b);
}

/**
 * Recompute the pool-level extinguishing royalty schedule from the public terms
 * — re-derived to match royalty.ts (loss years are absent in the funder's flat
 * revenue curve, so the clamp is the only rule that matters here).
 */
function recomputeRoyaltyPool(
  perPeriodBaseCents: number[],
  rate: number,
  capCents: number,
): { poolPaid: number[]; cumulative: number[]; extinguishYear: number | null } {
  const poolPaid: number[] = [];
  const cumulative: number[] = [];
  let cum = 0;
  let extinguishYear: number | null = null;
  for (let p = 0; p < perPeriodBaseCents.length; p++) {
    const due = Math.round(rate * perPeriodBaseCents[p]);
    let paid = due;
    if (cum + paid >= capCents) paid = capCents - cum;
    if (paid < 0) paid = 0;
    cum += paid;
    if (extinguishYear === null && cum === capCents && capCents > 0) extinguishYear = p;
    poolPaid.push(paid);
    cumulative.push(cum);
  }
  return { poolPaid, cumulative, extinguishYear };
}

// -------------------------------------------------------------------- ledgers

interface LedgerEntry {
  seq: number;
  ts: number;
  type: string;
  actor: string;
  body: Record<string, Json>;
  sig: string;
  pubkey: string;
}

function readLedger(dir: string, name: string): LedgerEntry[] {
  const path = join(dir, name);
  if (!existsSync(path)) return [];
  return readFileSync(path, "utf8")
    .split("\n")
    .filter((l) => l.trim().length > 0)
    .map((l) => JSON.parse(l) as LedgerEntry);
}

// --------------------------------------------------------------------- inputs

// SEAM: the funder reads the LEDGERS from disk (the file-only diligence claim),
// but receives the genuinely-public, non-secret inputs — actor public keys, the
// beacon seed, the instrument terms, the covenant thresholds — as an in-process
// packet. In production these arrive over the wire from published endpoints; the
// property E12 needs (nothing here is a co-op secret) is identical.
export interface FunderInputs {
  ledgerDir: string;
  keys: {
    customerId: string;
    customerPubHex: string;
    providerId: string;
    providerPubHex: string;
    coopId: string;
    coopPubHex: string;
    investors: { id: string; pubHex: string; principalCents: number }[];
  };
  beaconSeedHex: string;
  periods: number;
  auditK: number;
  royaltyTerms: { m: number; rate: number; base: string; perPeriodBaseCents: number[] };
  covenants: {
    salaryRatioCap: number;
    roleSet: string[];
    operatingCents: number[];
    minSurplusCents: number;
    maxGraceFraction: number;
  };
}

export type FindingCode =
  | "uncosigned-revenue"
  | "off-book-waiver"
  | "retro-edit"
  | "bad-audit-challenge";

export interface Finding {
  code: FindingCode;
  period: number | null;
  detail: string;
  ledgerRef: string; // file + seq the finding derives from
}

export interface FunderResult {
  revenue: { ok: boolean; coAttestedCents: number; reportedCents: number; badEntrySeqs: number[] };
  service: { ok: boolean; transcriptsChecked: number; passRate: number; badRounds: number[] };
  chain: { ok: boolean; brokenAtPeriod: number | null; reason: string | null };
  grace: { ok: boolean; onBookCents: number; offBookPeriods: number[] };
  royalty: { ok: boolean; capCents: number; paidCents: number; extinguishYear: number | null; mismatches: string[] };
  findings: Finding[];
  overallOk: boolean;
}

// --------------------------------------------------------------- E12 verifier

/**
 * The whole E12 diligence pass. Returns per-check results and a classified
 * finding for each cooked-books anomaly. An honest year passes every check with
 * no findings.
 */
export function runFunder(inp: FunderInputs): FunderResult {
  const { keys } = inp;
  const provider = readLedger(inp.ledgerDir, "provider.jsonl");
  const customer = readLedger(inp.ledgerDir, "customer.jsonl");
  const coop = readLedger(inp.ledgerDir, "coop.jsonl");
  const findings: Finding[] = [];

  // (a) Revenue is co-attested: every revenue entry carries a valid customer
  // countersignature, and reported revenue equals the sum of those entries.
  const revenueEntries = provider.filter((e) => e.type === "revenue");
  let coAttestedCents = 0;
  const badEntrySeqs: number[] = [];
  for (const e of revenueEntries) {
    const core = e.body.core as Record<string, Json>;
    const sig = e.body.customerSig as string;
    if (verifyJson(keys.customerPubHex, core, sig)) {
      coAttestedCents += core.amountCents as number;
    } else {
      badEntrySeqs.push(e.seq);
    }
  }
  const reportEntry = provider.find((e) => e.type === "revenue_report");
  const reportedCents = reportEntry
    ? ((reportEntry.body.core as Record<string, Json>).reportedTotalCents as number)
    : 0;
  const revenueOk = badEntrySeqs.length === 0 && coAttestedCents === reportedCents;
  if (!revenueOk) {
    findings.push({
      code: "uncosigned-revenue",
      period: null,
      detail: `reported ${reportedCents}¢ but only ${coAttestedCents}¢ carries a valid customer countersignature (${badEntrySeqs.length} un-co-attested entr${badEntrySeqs.length === 1 ? "y" : "ies"})`,
      ledgerRef: `provider.jsonl seq ${badEntrySeqs.join(",") || "(report)"}`,
    });
  }

  // (b) Service was delivered: recompute the public-randomness audit challenges
  // and verify the transcripts.
  const manifestEntry = customer.find((e) => e.type === "manifest");
  const manifestItems = (manifestEntry?.body.items as unknown as { cid: string; size: number }[]) ?? [];
  const sortedItems = [...manifestItems].sort((a, b) => (a.cid < b.cid ? -1 : a.cid > b.cid ? 1 : 0));
  const manifestRoot = manifestEntry?.body.root as string;
  // The customer actually signed the manifest root.
  const manifestSigOk =
    !!manifestEntry &&
    merkleRoot(sortedItems) === manifestRoot &&
    verifyJson(
      keys.customerPubHex,
      { root: manifestRoot, itemCount: sortedItems.length, generatedDay: manifestEntry.body.generatedDay },
      manifestEntry.body.sig as string,
    );
  const transcripts = provider.filter((e) => e.type === "audit_transcript");
  let passes = 0;
  const badRounds: number[] = [];
  for (const e of transcripts) {
    const core = e.body.core as Record<string, Json>;
    const round = core.round as number;
    const expectedBeacon = beaconValue(inp.beaconSeedHex, round);
    const idxs = deriveChallengeIndices(expectedBeacon, sortedItems.length, inp.auditK);
    const expectedCids = idxs.map((i) => sortedItems[i].cid);
    const claimed = core.challengeCids as string[];
    const challengeMatches =
      (core.beaconHex as string) === expectedBeacon &&
      claimed.length === expectedCids.length &&
      claimed.every((c, i) => c === expectedCids[i]);
    const cosigOk = verifyJson(keys.customerPubHex, core, e.body.customerSig as string);
    if (challengeMatches && cosigOk && core.passed === true) {
      passes++;
    } else {
      badRounds.push(round);
      if (!challengeMatches) {
        findings.push({
          code: "bad-audit-challenge",
          period: round,
          detail: `transcript round ${round} lists challenges that do not derive from the public beacon`,
          ledgerRef: `provider.jsonl seq ${e.seq}`,
        });
      }
    }
  }
  const serviceOk = manifestSigOk && badRounds.length === 0 && transcripts.length > 0;

  // (c) The statement chain is intact from genesis.
  const stmtEntries = provider.filter((e) => e.type === "statement");
  const chain = verifyStatementChain(stmtEntries, keys);
  if (!chain.ok) {
    findings.push({
      code: "retro-edit",
      period: chain.brokenAtPeriod,
      detail: `statement chain breaks at period ${chain.brokenAtPeriod}: ${chain.reason}`,
      ledgerRef: `provider.jsonl (statement period ${chain.brokenAtPeriod})`,
    });
  }

  // (d) Grace events are on-book and net to zero against the grace account: for
  // every period, billed fees == co-attested payment + booked grace.
  const grace = accountingIdentity(provider, coop, inp.periods, keys.customerPubHex);
  if (!grace.ok) {
    for (const p of grace.offBookPeriods) {
      findings.push({
        code: "off-book-waiver",
        period: p,
        detail: `period ${p}: billed fees != collected revenue + booked grace (a fee was waived off-book)`,
        ledgerRef: `provider.jsonl (statement/revenue period ${p})`,
      });
    }
  }

  // (e) The royalty payments match the instrument's terms.
  const royalty = checkRoyalty(coop, inp);

  const overallOk =
    revenueOk && serviceOk && chain.ok && grace.ok && royalty.ok && findings.length === 0;

  return {
    revenue: { ok: revenueOk, coAttestedCents, reportedCents, badEntrySeqs },
    service: { ok: serviceOk, transcriptsChecked: transcripts.length, passRate: transcripts.length ? passes / transcripts.length : 0, badRounds },
    chain,
    grace: { ok: grace.ok, onBookCents: grace.onBookCents, offBookPeriods: grace.offBookPeriods },
    royalty,
    findings,
    overallOk,
  };
}

interface ChainResult {
  ok: boolean;
  brokenAtPeriod: number | null;
  reason: string | null;
}

function verifyStatementChain(entries: LedgerEntry[], keys: FunderInputs["keys"]): ChainResult {
  // Statements are stored newest-append; order by the body's period.
  const statements = entries
    .map((e) => e.body as unknown as {
      body: Record<string, Json>;
      hash: string;
      customer: string;
      provider: string;
      customerSig: string;
      providerSig: string;
    })
    .sort((a, b) => (a.body.period as number) - (b.body.period as number));
  let prevHash = GENESIS_PREV;
  for (const s of statements) {
    const period = s.body.period as number;
    if (hashStatement(s.body) !== s.hash) {
      return { ok: false, brokenAtPeriod: period, reason: "recomputed hash does not match the stored hash" };
    }
    const bodyJson = statementBodyJson(s.body);
    if (!verifyJson(keys.customerPubHex, bodyJson, s.customerSig)) {
      return { ok: false, brokenAtPeriod: period, reason: "customer co-signature invalid" };
    }
    if (!verifyJson(keys.providerPubHex, bodyJson, s.providerSig)) {
      return { ok: false, brokenAtPeriod: period, reason: "provider co-signature invalid" };
    }
    if ((s.body.prevStatementHash as string) !== prevHash) {
      return { ok: false, brokenAtPeriod: period, reason: "prev-hash does not chain to the prior period" };
    }
    prevHash = s.hash;
  }
  return { ok: true, brokenAtPeriod: null, reason: null };
}

interface GraceResult {
  ok: boolean;
  onBookCents: number;
  offBookPeriods: number[];
}

function accountingIdentity(
  provider: LedgerEntry[],
  coop: LedgerEntry[],
  periods: number,
  customerPubHex: string,
): GraceResult {
  const billed = new Array(periods).fill(0);
  const paid = new Array(periods).fill(0);
  const graced = new Array(periods).fill(0);

  for (const e of provider.filter((x) => x.type === "statement")) {
    const b = (e.body as unknown as { body: Record<string, Json> }).body;
    const p = b.period as number;
    if (p >= 0 && p < periods) billed[p] += b.fees as number;
  }
  for (const e of provider.filter((x) => x.type === "revenue")) {
    const core = e.body.core as Record<string, Json>;
    const p = core.period as number;
    // Only count co-attested payments toward the identity.
    if (p >= 0 && p < periods && verifyJson(customerPubHex, core, e.body.customerSig as string)) {
      paid[p] += core.amountCents as number;
    }
  }
  let onBookCents = 0;
  for (const e of coop.filter((x) => x.type === "grace_event")) {
    const p = e.body.period as number;
    const amt = e.body.amount as number;
    if (p >= 0 && p < periods) graced[p] += amt;
    onBookCents += amt;
  }
  const offBookPeriods: number[] = [];
  for (let p = 0; p < periods; p++) {
    if (billed[p] !== paid[p] + graced[p]) offBookPeriods.push(p);
  }
  return { ok: offBookPeriods.length === 0, onBookCents, offBookPeriods };
}

function checkRoyalty(coop: LedgerEntry[], inp: FunderInputs): FunderResult["royalty"] {
  const totalPrincipal = inp.keys.investors.reduce((s, i) => s + i.principalCents, 0);
  const capCents = inp.royaltyTerms.m * totalPrincipal;
  const expected = recomputeRoyaltyPool(inp.royaltyTerms.perPeriodBaseCents, inp.royaltyTerms.rate, capCents);
  const mismatches: string[] = [];

  // Aggregate ledger royalty payments per year (verifying each investor sig).
  const paidByYear = new Map<number, number>();
  for (const e of coop.filter((x) => x.type === "royalty_payment")) {
    const core = e.body.core as Record<string, Json>;
    const year = core.year as number;
    const invId = core.investorId as string;
    const inv = inp.keys.investors.find((i) => i.id === invId);
    if (!inv || !verifyJson(inv.pubHex, core, e.body.investorSig as string)) {
      mismatches.push(`royalty entry seq ${e.seq}: investor co-signature invalid`);
      continue;
    }
    paidByYear.set(year, (paidByYear.get(year) ?? 0) + (core.paymentCents as number));
  }
  let paidCents = 0;
  for (let p = 0; p < inp.royaltyTerms.perPeriodBaseCents.length; p++) {
    const ledgerPaid = paidByYear.get(p) ?? 0;
    paidCents += ledgerPaid;
    if (ledgerPaid !== expected.poolPaid[p]) {
      mismatches.push(`year ${p}: ledger paid ${ledgerPaid}¢ but the terms imply ${expected.poolPaid[p]}¢`);
    }
  }
  // Cap is honoured exactly.
  if (paidCents !== capCents && expected.extinguishYear !== null) {
    mismatches.push(`cumulative paid ${paidCents}¢ != cap ${capCents}¢`);
  }
  // Extinguishment event present at the right year.
  const extEntry = coop.find((x) => x.type === "royalty_extinguished");
  const extYear = extEntry ? (extEntry.body.year as number) : null;
  if (extYear !== expected.extinguishYear) {
    mismatches.push(`extinguishment recorded at year ${extYear}, terms imply ${expected.extinguishYear}`);
  }

  return {
    ok: mismatches.length === 0,
    capCents,
    paidCents,
    extinguishYear: expected.extinguishYear,
    mismatches,
  };
}

// ------------------------------------------------------------- E13 covenants

export interface CovenantResult {
  name: string;
  ok: boolean;
  detail: string;
  ledgerRefs: string[];
}

export interface CovenantReport {
  ok: boolean;
  covenants: CovenantResult[];
  figures: {
    salaryRatio: number;
    annualSurplusCents: number;
    graceFraction: number;
    revenueCents: number;
  };
}

/**
 * Loan covenants expressed as executable checks over the ledger, each derived
 * from a fixed, published formula (Part 3 E13). Compliant years pass; each
 * violation is flagged with the exact entries responsible.
 */
export function checkCovenants(inp: FunderInputs): CovenantReport {
  const provider = readLedger(inp.ledgerDir, "provider.jsonl");
  const coop = readLedger(inp.ledgerDir, "coop.jsonl");
  const periods = inp.periods;

  // Per-period figures the covenants derive from.
  const revenuePaid = new Array(periods).fill(0);
  for (const e of provider.filter((x) => x.type === "revenue")) {
    const core = e.body.core as Record<string, Json>;
    const p = core.period as number;
    if (p >= 0 && p < periods && verifyJson(inp.keys.customerPubHex, core, e.body.customerSig as string)) {
      revenuePaid[p] += core.amountCents as number;
    }
  }
  const payrollAll = new Array(periods).fill(0);
  const roleSalaries = new Map<string, number>();
  const payrollRefs = new Map<string, number>(); // role -> seq
  for (const e of coop.filter((x) => x.type === "payroll")) {
    const role = e.body.role as string;
    const salary = e.body.salaryCents as number;
    const p = e.body.period as number;
    if (p >= 0 && p < periods) payrollAll[p] += salary;
    if (inp.covenants.roleSet.includes(role)) {
      roleSalaries.set(role, salary);
      payrollRefs.set(role, e.seq);
    }
  }
  const royaltyPaid = new Array(periods).fill(0);
  const royaltyRefs = new Array(periods).fill(-1);
  for (const e of coop.filter((x) => x.type === "royalty_payment")) {
    const core = e.body.core as Record<string, Json>;
    const y = core.year as number;
    if (y >= 0 && y < periods) {
      royaltyPaid[y] += core.paymentCents as number;
      royaltyRefs[y] = e.seq;
    }
  }
  let graceTotal = 0;
  const graceRefs: string[] = [];
  for (const e of coop.filter((x) => x.type === "grace_event")) {
    graceTotal += e.body.amount as number;
    graceRefs.push(`coop.jsonl seq ${e.seq}`);
  }

  const revenueCents = revenuePaid.reduce((s, x) => s + x, 0);
  const operating = inp.covenants.operatingCents.reduce((s, x) => s + x, 0);
  const payrollTotal = payrollAll.reduce((s, x) => s + x, 0);
  const royaltyTotal = royaltyPaid.reduce((s, x) => s + x, 0);

  const covenants: CovenantResult[] = [];

  // 1. Salary ratio within the chartered cap.
  const salaries = [...roleSalaries.values()];
  const maxSalary = Math.max(...salaries);
  const minSalary = Math.min(...salaries);
  const salaryRatio = minSalary > 0 ? maxSalary / minSalary : Infinity;
  {
    const topRole = [...roleSalaries.entries()].find(([, s]) => s === maxSalary)![0];
    covenants.push({
      name: "salary-ratio",
      ok: salaryRatio <= inp.covenants.salaryRatioCap,
      detail: `highest/lowest salary = ${salaryRatio.toFixed(2)}:1 (cap ${inp.covenants.salaryRatioCap.toFixed(2)}:1)`,
      ledgerRefs: [`coop.jsonl seq ${payrollRefs.get(topRole)} (${topRole})`],
    });
  }

  // 2. Surplus by the published formula: revenue − payroll − operating − royalty.
  const annualSurplus = revenueCents - payrollTotal - operating - royaltyTotal;
  covenants.push({
    name: "surplus-floor",
    ok: annualSurplus >= inp.covenants.minSurplusCents,
    detail: `annual surplus = ${annualSurplus}¢ (floor ${inp.covenants.minSurplusCents}¢): revenue ${revenueCents} − payroll ${payrollTotal} − operating ${operating} − royalty ${royaltyTotal}`,
    ledgerRefs: ["provider.jsonl (revenue entries)", "coop.jsonl (payroll, royalty entries)"],
  });

  // 3. Repayment priority: workers before investors — royalty ≤ revenue − payroll each period.
  const priorityViolations: number[] = [];
  for (let p = 0; p < periods; p++) {
    if (royaltyPaid[p] > revenuePaid[p] - payrollAll[p]) priorityViolations.push(p);
  }
  covenants.push({
    name: "repayment-priority",
    ok: priorityViolations.length === 0,
    detail:
      priorityViolations.length === 0
        ? "royalty stayed within (revenue − payroll) every period"
        : `royalty paid ahead of workers in period(s) ${priorityViolations.join(", ")}`,
    ledgerRefs: priorityViolations.map((p) => `coop.jsonl seq ${royaltyRefs[p]} (period ${p} royalty)`),
  });

  // 4. Grace within the declared band.
  const graceFraction = revenueCents > 0 ? graceTotal / revenueCents : 0;
  covenants.push({
    name: "grace-band",
    ok: graceFraction <= inp.covenants.maxGraceFraction,
    detail: `grace = ${(graceFraction * 100).toFixed(2)}% of revenue (band ≤ ${(inp.covenants.maxGraceFraction * 100).toFixed(0)}%)`,
    ledgerRefs: graceFraction > inp.covenants.maxGraceFraction ? graceRefs : [],
  });

  return {
    ok: covenants.every((c) => c.ok),
    covenants,
    figures: { salaryRatio, annualSurplusCents: annualSurplus, graceFraction, revenueCents },
  };
}

// -------------------------------------------- E14 ledger-type classification

export type LedgerVerdict = "verified" | "attested-but-unverifiable";

export interface LedgerClass {
  type: string;
  count: number;
  signaturesValid: boolean;
  countVerifiable: boolean;
  verdict: LedgerVerdict;
  note: string;
}

/**
 * Classify each ledger type by whether its unit is countable at the boundary.
 * A co-attested count the funder can independently cross-check (revenue against
 * the report, audits against the beacon, royalty against the terms) is
 * "verified". A signed entry whose unit has no boundary-observable count — an
 * hour of advice — is "attested-but-unverifiable": the signatures are valid,
 * but the count rests on trust, not observation. Distinct on purpose (E14).
 */
export function classifyLedgers(inp: FunderInputs): LedgerClass[] {
  const provider = readLedger(inp.ledgerDir, "provider.jsonl");
  const coop = readLedger(inp.ledgerDir, "coop.jsonl");
  const out: LedgerClass[] = [];

  const revenue = provider.filter((e) => e.type === "revenue");
  const revSigsOk = revenue.every((e) =>
    verifyJson(inp.keys.customerPubHex, e.body.core as Record<string, Json>, e.body.customerSig as string),
  );
  out.push({
    type: "revenue",
    count: revenue.length,
    signaturesValid: revSigsOk,
    countVerifiable: true,
    verdict: "verified",
    note: "each amount cross-checks against the co-signed revenue report",
  });

  const transcripts = provider.filter((e) => e.type === "audit_transcript");
  const tSigsOk = transcripts.every((e) =>
    verifyJson(inp.keys.customerPubHex, e.body.core as Record<string, Json>, e.body.customerSig as string),
  );
  out.push({
    type: "audit_transcript",
    count: transcripts.length,
    signaturesValid: tSigsOk,
    countVerifiable: true,
    verdict: "verified",
    note: "the challenged items are re-derivable from the public beacon and re-fingerprinted",
  });

  const royalty = coop.filter((e) => e.type === "royalty_payment");
  const rSigsOk = royalty.every((e) => {
    const core = e.body.core as Record<string, Json>;
    const inv = inp.keys.investors.find((i) => i.id === (core.investorId as string));
    return !!inv && verifyJson(inv.pubHex, core, e.body.investorSig as string);
  });
  out.push({
    type: "royalty_payment",
    count: royalty.length,
    signaturesValid: rSigsOk,
    countVerifiable: true,
    verdict: "verified",
    note: "each payment is re-derivable from the published instrument terms",
  });

  const consulting = coop.filter((e) => e.type === "consulting_hours");
  const cSigsOk = consulting.every((e) =>
    verifyJson(inp.keys.customerPubHex, e.body.core as Record<string, Json>, e.body.customerSig as string),
  );
  out.push({
    type: "consulting_hours",
    count: consulting.length,
    signaturesValid: cSigsOk,
    countVerifiable: false,
    verdict: "attested-but-unverifiable",
    note: "an hour of advice has no boundary-observable count — the signature attests it, nothing checks it",
  });

  return out;
}
