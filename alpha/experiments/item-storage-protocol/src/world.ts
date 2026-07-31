// A "world" is one experiment's stage: the two actors (and, where needed, the
// co-op's own grace account), their fresh ledgers, a content store, and a clock.
//
// Each experiment gets its own world in its own ledger subdirectory with keys
// derived from its own seed, so experiments are independently runnable and never
// race on a shared ledger file. The narrative thread across experiments is in the
// printed output, not in shared mutable state.

import { mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { Actor } from "./actor.ts";
import { Store } from "./store.ts";
import { SimClock } from "./time.ts";

export interface World {
  seed: number;
  dir: string;
  customer: Actor; // Ada
  provider: Actor; // the co-op's storage provider
  coop: Actor; // the co-op's own grace account (E9)
  store: Store;
  clock: SimClock;
}

export function ledgerRoot(): string {
  return join(process.cwd(), "ledgers");
}

/**
 * Build a fresh world for an experiment. Wipes and recreates the experiment's
 * ledger subdirectory so a run always starts clean and deterministic.
 */
export function createWorld(id: string, seed: number): World {
  const dir = join(ledgerRoot(), id);
  rmSync(dir, { recursive: true, force: true });
  mkdirSync(dir, { recursive: true });

  const customer = new Actor(seed, "customer", join(dir, "customer.jsonl"));
  const provider = new Actor(seed, "provider", join(dir, "provider.jsonl"));
  const coop = new Actor(seed, "coop", join(dir, "coop.jsonl"));

  // Mutual key exchange and pinning (E0). Everyone pins everyone they transact
  // with — including themselves, since verifying a co-signed receipt requires
  // both parties' keys and an actor always knows its own.
  for (const self of [customer, provider, coop]) {
    for (const peer of [customer, provider, coop]) self.pin(peer);
  }

  return { seed, dir, customer, provider, coop, store: new Store(), clock: new SimClock() };
}
