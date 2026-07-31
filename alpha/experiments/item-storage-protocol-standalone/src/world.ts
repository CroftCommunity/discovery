// The shared world the experiments run against, in order. One customer (Ada),
// one provider (the co-op), a simulated clock, a content-addressed store, and the
// two append-only ledgers. Later experiments attach their outputs here (the
// signed manifest, the statement chain, the seal state, the royalty pool) so the
// narrative is one continuous story rather than twelve disconnected demos.

import { SimClock } from "./clock.ts";
import { Rng } from "./rng.ts";
import { Actor } from "./actor.ts";
import { BlobStore, type Item } from "./item.ts";
import { buildManifest, expectedBytes, type Manifest, type ManifestLeaf } from "./manifest.ts";
import { buildStatement, GENESIS_STATEMENT, type Statement, type StatementBody } from "./statement.ts";
import type { ExperimentResult } from "./types.ts";
import type { Json } from "./canonical.ts";

/** A change in bytes-at-rest at a given day — the timeline rent integrates over. */
export type TimelinePoint = { day: number; totalBytes: number };

/** A single signed transfer increment, as recorded by both ledgers (E3). */
export type ReceiptRecord = {
  day: number;
  direction: "upload" | "download";
  cid: string;
  byteStart: number;
  byteEnd: number; // exclusive
  bytes: number;
  runningTotal: number;
};

/** An audit event booked in a period (E5/E6). */
export type AuditRecord = {
  day: number;
  k: number;
  bytesRead: number;
  passed: boolean;
};

export const PERIOD_DAYS = 30;

export class World {
  readonly masterSeed: string;
  readonly clock = new SimClock();

  readonly customer: Actor; // Ada
  readonly provider: Actor; // the co-op
  readonly investors: Actor[] = []; // Bram et al. (E11)

  readonly store = new BlobStore();
  readonly keyring: Record<string, string> = {};

  items: Item[] = [];
  manifest: Manifest | null = null;
  // Set by E7 (seal ceremony); read by E8 (tombstone). Kept as `unknown`-friendly
  // any to avoid a type import cycle; the concrete shape is seal.ts's SealState.
  seal: import("./seal.ts").SealState | null = null;

  /** Bytes-at-rest timeline, seeded once the first manifest exists. */
  readonly timeline: TimelinePoint[] = [];
  readonly receipts: ReceiptRecord[] = [];
  readonly audits: AuditRecord[] = [];
  readonly statements: Statement[] = [];

  /** The co-op's grace account balance (cents it has absorbed on members' behalf). */
  graceAccountCents = 0;

  readonly results: ExperimentResult[] = [];

  constructor(masterSeed = "croft-item-storage-protocol-v1") {
    this.masterSeed = masterSeed;
    this.customer = new Actor("Ada", masterSeed, "customer/ada");
    this.provider = new Actor("Co-op", masterSeed, "provider/coop");
    this.pin(this.customer);
    this.pin(this.provider);
  }

  /** Pin an actor's public key so its signatures are verifiable everywhere. */
  pin(actor: Actor): void {
    this.keyring[actor.id] = actor.publicKeyHex;
  }

  /** A deterministic RNG for a named use (seeded from the master seed). */
  rng(label: string): Rng {
    return new Rng(`${this.masterSeed}::rng::${label}`);
  }

  /** Record that bytes-at-rest changed to `totalBytes` as of the current day. */
  setBytesAtRest(totalBytes: number): void {
    const day = this.clock.now();
    const last = this.timeline[this.timeline.length - 1];
    if (last && last.day === day) {
      last.totalBytes = totalBytes;
    } else {
      this.timeline.push({ day, totalBytes });
    }
  }

  /** Bytes-at-rest at a given day, from the timeline (step function). */
  bytesAtRestOn(day: number): number {
    let current = 0;
    for (const p of this.timeline) {
      if (p.day <= day) current = p.totalBytes;
      else break;
    }
    return current;
  }

  /** Integrate byte-days over [startDay, endDay). The rent base for a period. */
  byteDays(startDay: number, endDay: number): number {
    let total = 0;
    for (let day = startDay; day < endDay; day++) {
      total += this.bytesAtRestOn(day);
    }
    return total;
  }

  /**
   * Re-author the manifest from a new item set: rebuild + re-sign (the customer
   * holds the list), update bytes-at-rest as of now, and record the new signed
   * root in the customer's ledger. Every manifest change thus produces a fresh
   * customer signature — the fact the rotation watch in E7 depends on.
   */
  updateManifest(items: Item[], reason: string): Manifest {
    this.items = items;
    const leaves: ManifestLeaf[] = items.map((i) => ({ cid: i.cid, size: i.size }));
    const manifest = buildManifest(leaves, this.customer.id, this.customer.keypair);
    this.manifest = manifest;
    this.setBytesAtRest(expectedBytes(manifest));
    this.customer.ledger.append("manifest-update", this.clock.iso(), {
      root: manifest.root, totalBytes: manifest.totalBytes,
      itemCount: manifest.leaves.length, reason,
    }, [this.customer.signer()]);
    return manifest;
  }

  /**
   * Close a period into a co-signed, hash-chained statement, appended to both
   * ledgers. Callers supply the period figures; the period number and the link to
   * the prior statement are filled in here so the chain stays contiguous.
   */
  commitStatement(fields: Omit<StatementBody, "period" | "prevStatementHash">): Statement {
    const period = this.statements.length;
    const prev = this.statements[period - 1];
    const stmt = buildStatement({
      ...fields,
      period,
      prevStatementHash: prev ? prev.hash : GENESIS_STATEMENT,
    });
    this.statements.push(stmt);
    const ts = this.clock.iso();
    this.customer.ledger.append("statement", ts, stmt as unknown as Json,
      [this.customer.signer(), this.provider.signer()]);
    this.provider.ledger.append("statement", ts, stmt as unknown as Json,
      [this.provider.signer(), this.customer.signer()]);
    return stmt;
  }
}
