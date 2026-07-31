// A full co-op operating year, written to ledgers, for the outside-reader
// experiments (E12, E13, E14).
//
// Everything the co-op does across a set of periods lands in signed ledger
// files: the customer's signed manifest; co-signed balance-forward statements;
// co-attested revenue entries and a co-signed revenue report; on-book grace
// events; audit transcripts whose challenges derive from a PUBLIC randomness
// beacon; a royalty ledger (E11) paying investors to their cap; payroll entries;
// and a consulting-hours ledger whose unit is not boundary-countable (E14).
//
// The Funder (funder/verifier.ts) later reads ONLY these files plus a packet of
// genuinely-public inputs (keys, the beacon seed, the instrument terms, the
// covenant thresholds) and underwrites the co-op with no private access.
//
// A `defect` may be injected to model a cooked-books or covenant-violation
// scenario; the honest build passes every Funder check, and each defect trips
// exactly the check it is designed to trip.
//
// SEAM: a single customer (Ada) stands in for the co-op's whole member base, so
// her co-attested revenue entries represent the co-op's member revenue at co-op
// scale. In production these are many members' countersigned payments.

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { createWorld, type World } from "./world.ts";
import { Actor } from "./actor.ts";
import { Prng } from "./prng.ts";
import { makeItem, type NamedItem } from "./items.ts";
import { buildSignedManifest, type SignedManifest } from "./manifest.ts";
import { canonicalize, type Json } from "./canonical.ts";
import { beaconValue, deriveChallengeIndices } from "./beacon.ts";
import { computeCid } from "./store.ts";
import {
  coSignStatement,
  GENESIS_PREV,
  type Statement,
  type StatementBody,
} from "./statements.ts";
import { DAYS_PER_PERIOD } from "./time.ts";
import { computeSchedule, type Investor } from "./royalty.ts";
import { flatFuture } from "./futures.ts";

export type Defect =
  | "none"
  | "revenue" // E12: an inflated revenue entry lacking a valid customer countersignature
  | "waiver" // E12: an off-book fee waiver (payment short with no grace entry)
  | "retro" // E12: a retroactive edit to a closed statement
  | "audit" // E12: a fabricated audit transcript whose challenges don't derive from the beacon
  | "salary" // E13: salary ratio over the chartered cap
  | "surplus" // E13: annual surplus below the floor
  | "priority" // E13: royalty paid ahead of workers in some period
  | "grace-band"; // E13: grace over the declared band

export const PERIODS = 8;
export const AUDIT_K = 3;
export const BEACON_SEED_HEX = "b1a2c3d4e5f6";
const N_ITEMS = 10;
const ITEM_SIZE = 1024;

// Money (integer cents). One customer stands in for the co-op's member revenue.
const REVENUE_PER_PERIOD = 20_000_000; // $200,000 co-attested member revenue / period
const OPERATING_PER_PERIOD = 500_000; // $5,000 published operating cost / period
const ROLE_SALARIES: Record<string, number> = {
  coordinator: 800_000,
  steward: 500_000,
  apprentice: 300_000,
};
const ROLE_SET = Object.keys(ROLE_SALARIES);

// Royalty instrument (E11): three investors, cap m*P, revenue base.
const M = 3;
const ROYALTY_RATE = 0.05;

export interface CoopKeys {
  customerId: string;
  customerPubHex: string;
  providerId: string;
  providerPubHex: string;
  coopId: string;
  coopPubHex: string;
  investors: { id: string; pubHex: string; principalCents: number }[];
}

export interface CoopYear {
  dir: string;
  periods: number;
  auditK: number;
  beaconSeedHex: string;
  keys: CoopKeys;
  royaltyTerms: {
    m: number;
    rate: number;
    base: "revenue";
    perPeriodBaseCents: number[];
  };
  covenants: {
    salaryRatioCap: number;
    roleSet: string[];
    operatingCents: number[];
    minSurplusCents: number;
    maxGraceFraction: number;
  };
  manifestRoot: string;
}

/**
 * Append a co-signed body to an actor's ledger. The author signs the whole
 * entry (the ledger's own outer signature); the co-signer signs the canonical
 * `core` and that signature is stored under `sigField`, so the Funder can verify
 * the countersignature by canonicalizing `core` and checking `sigField`.
 */
function coSignedEntry(
  author: Actor,
  cosigner: Actor,
  sigField: string,
  type: string,
  ts: number,
  core: Json,
  extra: Record<string, Json>,
): void {
  const body: Json = { core, [sigField]: cosigner.sign(core), ...extra };
  author.ledger.append(type, ts, body);
}

export function buildCoopYear(seed: number, id: string, defect: Defect = "none"): CoopYear {
  const w: World = createWorld(id, seed);
  const { customer, provider, coop, store } = w;
  const prng = new Prng(seed);

  // Investors (E11), each with their own ledger; the co-op co-signs royalties.
  const investorActors = [
    new Actor(seed, "investor-bram", join(w.dir, "investor-bram.jsonl")),
    new Actor(seed, "investor-cleo", join(w.dir, "investor-cleo.jsonl")),
    new Actor(seed, "investor-dai", join(w.dir, "investor-dai.jsonl")),
  ];
  for (const inv of investorActors) {
    coop.pin(inv);
    inv.pin(coop);
  }
  const investors: Investor[] = [
    { id: investorActors[0].id, principalCents: 1_000_000 },
    { id: investorActors[1].id, principalCents: 600_000 },
    { id: investorActors[2].id, principalCents: 400_000 },
  ];

  // --- The customer's signed manifest (the list of what's owed). ---
  const items: NamedItem[] = [];
  for (let i = 0; i < N_ITEMS; i++) {
    const it = makeItem(prng, `coop-item-${i}`, ITEM_SIZE);
    store.put(it.bytes);
    items.push(it);
  }
  const manifest: SignedManifest = buildSignedManifest(
    customer,
    items.map((it) => ({ cid: it.cid, size: it.size })),
    0,
  );
  customer.ledger.append("manifest", 0, {
    items: manifest.items,
    root: manifest.root,
    generatedDay: manifest.generatedDay,
    sig: manifest.sig,
  });
  const bytesAtRest = items.reduce((s, it) => s + it.size, 0);

  // --- The royalty schedule over the year (revenue base). ---
  const perPeriodBaseCents = Array.from({ length: PERIODS }, () => REVENUE_PER_PERIOD);
  const royaltyFuture = flatFuture(REVENUE_PER_PERIOD, REVENUE_PER_PERIOD, PERIODS);
  const schedule = computeSchedule(royaltyFuture, ROYALTY_RATE, "revenue", investors, M);

  // Per-period accounting we will also feed to statements.
  const operatingCents = Array.from({ length: PERIODS }, () => OPERATING_PER_PERIOD);

  // --- Drive each period, emitting all ledgers. ---
  const statements: Statement[] = [];
  let prevHash = GENESIS_PREV;
  let reportedRevenueCents = 0;

  for (let p = 0; p < PERIODS; p++) {
    const closeDay = p * DAYS_PER_PERIOD + DAYS_PER_PERIOD - 1;
    const billed = REVENUE_PER_PERIOD;

    // Grace: a single fee waiver in period 2 (on-book, nets against the shortfall).
    let graced = 0;
    if (p === 2) graced = 1_000_000;
    // grace-band defect: waive a large slice every period (still on-book).
    if (defect === "grace-band") graced = 5_000_000;

    // Off-book-waiver defect: pay short in period 3 with NO grace entry.
    let paid = billed - graced;
    if (defect === "waiver" && p === 3) paid = billed - 1_000_000; // shortfall, no grace booked

    reportedRevenueCents += paid;

    // Co-attested revenue entry (customer countersigns the amount they paid).
    coSignedEntry(provider, customer, "customerSig", "revenue", closeDay, { period: p, amountCents: paid }, {});

    // On-book grace event (skipped for the off-book-waiver defect).
    if (graced > 0) {
      const graceBody: Json = {
        kind: p === 2 ? "fee_waiver" : "throttle_hold",
        reasonCode: "hardship",
        amount: graced,
        period: p,
        benefitTo: customer.id,
        chargedTo: "coop-grace",
      };
      coop.ledger.append("grace_event", closeDay, graceBody);
      customer.ledger.append("grace_event", closeDay, graceBody);
    }

    // Payroll (E13): one entry per role per period.
    const salaries = { ...ROLE_SALARIES };
    if (defect === "salary") salaries.coordinator = 1_600_000; // ratio 1.6M/0.3M = 5.33 > cap
    for (const role of ROLE_SET) {
      coop.ledger.append("payroll", closeDay, { role, salaryCents: salaries[role], period: p });
    }
    // priority defect: an extraordinary payroll spike in one period so workers
    // outrun revenue that period — royalty should not have been paid ahead of them.
    if (defect === "priority" && p === 4) {
      coop.ledger.append("payroll", closeDay, { role: "extraordinary", salaryCents: 19_000_000, period: p });
    }

    // Audit transcript: challenges derive from the public beacon over the manifest.
    const bh = beaconValue(BEACON_SEED_HEX, p);
    let challengeIdx = deriveChallengeIndices(bh, manifest.items.length, AUDIT_K);
    // audit defect: in period 5 the transcript lists challenges that DON'T
    // derive from the beacon (first-k by manifest order), co-signed anyway.
    if (defect === "audit" && p === 5) {
      challengeIdx = Array.from({ length: AUDIT_K }, (_, i) => i);
    }
    const challengeCids = challengeIdx.map((i) => manifest.items[i].cid);
    // Verify (honest path) that the challenged bytes fingerprint correctly.
    const passed = challengeCids.every((cid) => {
      const raw = store.getRaw(cid);
      return raw !== undefined && computeCid(raw) === cid;
    });
    coSignedEntry(
      provider,
      customer,
      "customerSig",
      "audit_transcript",
      closeDay,
      { round: p, beaconHex: bh, k: AUDIT_K, manifestRoot: manifest.root, challengeCids, passed },
      {},
    );

    // Royalty payments (E11): one co-signed entry per investor, until extinguished.
    const yr = schedule.years[p];
    for (const pay of yr.perInvestor) {
      if (pay.paymentCents === 0 && yr.poolPaidCents === 0) continue; // nothing paid this period
      const invActor = investorActors.find((a) => a.id === pay.investorId)!;
      coSignedEntry(
        coop,
        invActor,
        "investorSig",
        "royalty_payment",
        closeDay,
        {
          year: p,
          investorId: pay.investorId,
          paymentCents: pay.paymentCents,
          cumulativeCents: pay.cumulativeCents,
          poolPaidCents: yr.poolPaidCents,
          base: "revenue",
          rate: ROYALTY_RATE,
        },
        { payer: coop.id, payee: pay.investorId },
      );
    }
    if (schedule.extinguishYear === p) {
      coop.ledger.append("royalty_extinguished", closeDay, {
        year: p,
        cumulativeCents: yr.cumulativePoolCents,
        capCents: schedule.capCents,
      });
    }

    // The balance-forward statement commits the period's figures into the chain.
    const body: StatementBody = {
      period: p,
      openingRoot: manifest.root,
      closingRoot: manifest.root,
      rentByteDays: bytesAtRest * DAYS_PER_PERIOD,
      postageBytes: 0,
      auditCount: 1,
      auditBytes: AUDIT_K * ITEM_SIZE,
      fees: billed,
      graceNet: graced,
      prevStatementHash: prevHash,
      closeDay,
      royaltyPoolCents: yr.poolPaidCents,
      royaltyCumulativeCents: yr.cumulativePoolCents,
      extinguished: yr.extinguished,
    };
    const stmt = coSignStatement(customer, provider, body, closeDay);
    statements.push(stmt);
    prevHash = stmt.hash;
  }

  // Co-signed revenue report: the co-op's claimed total member revenue.
  let reported = reportedRevenueCents;
  if (defect === "revenue") {
    // Inflate the reported total AND drop in a revenue entry whose customer
    // countersignature is invalid (signed by the provider, not the customer).
    const bogusAmount = 9_999_999;
    const core: Json = { period: 99, amountCents: bogusAmount };
    provider.ledger.append("revenue", PERIODS * DAYS_PER_PERIOD, {
      core,
      customerSig: provider.sign(core), // WRONG key: not the customer's countersignature
    });
    reported += bogusAmount;
  }
  coSignedEntry(provider, customer, "customerSig", "revenue_report", PERIODS * DAYS_PER_PERIOD, { reportedTotalCents: reported }, {});

  // --- Consulting-hours ledger (E14): both parties sign every entry, but an
  // "hour of advice" has no boundary-observable count. ---
  const advisors = ["ext-counsel", "ext-counsel", "growth-advisor"];
  const hours = [12, 8, 20];
  for (let i = 0; i < advisors.length; i++) {
    coSignedEntry(
      coop,
      customer,
      "customerSig",
      "consulting_hours",
      i * DAYS_PER_PERIOD + 5,
      { advisor: advisors[i], hours: hours[i], rateCents: 25_000, period: i, note: "strategy session" },
      {},
    );
  }

  // --- retro defect: after everything is closed and co-signed, retroactively
  // edit a figure inside a historical statement in the provider's ledger file. ---
  if (defect === "retro") {
    retroEditStatement(join(w.dir, "provider.jsonl"), 4);
  }

  const keys: CoopKeys = {
    customerId: customer.id,
    customerPubHex: customer.publicRawHex,
    providerId: provider.id,
    providerPubHex: provider.publicRawHex,
    coopId: coop.id,
    coopPubHex: coop.publicRawHex,
    investors: investorActors.map((a, i) => ({ id: a.id, pubHex: a.publicRawHex, principalCents: investors[i].principalCents })),
  };

  return {
    dir: w.dir,
    periods: PERIODS,
    auditK: AUDIT_K,
    beaconSeedHex: BEACON_SEED_HEX,
    keys,
    royaltyTerms: { m: M, rate: ROYALTY_RATE, base: "revenue", perPeriodBaseCents },
    covenants: {
      salaryRatioCap: 3.0,
      roleSet: ROLE_SET,
      operatingCents: defect === "surplus" ? operatingCents.map(() => 25_000_000) : operatingCents,
      minSurplusCents: 0,
      maxGraceFraction: 0.2,
    },
    manifestRoot: manifest.root,
  };
}

/**
 * Model a retroactive edit to a closed period: rewrite one statement line in a
 * ledger file, bumping a figure while leaving the co-signed hash and signatures
 * stale. The Funder recomputes the statement hash and co-signatures and locates
 * the break. (This is exactly the append-only rule being violated by hand.)
 */
function retroEditStatement(path: string, period: number): void {
  const lines = readFileSync(path, "utf8").split("\n").filter((l) => l.trim().length > 0);
  const edited = lines.map((line) => {
    const e = JSON.parse(line);
    if (e.type === "statement" && e.body?.body?.period === period) {
      e.body.body.rentByteDays += 1; // the retroactive edit
    }
    return JSON.stringify(e);
  });
  writeFileSync(path, edited.join("\n") + "\n");
}
