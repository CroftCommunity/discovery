// The suite: the twelve experiments in dependency order, run against one shared
// world so the printed output tells the Part 4 story end to end (Ada brings her
// items; the two parties become keys; the list is signed; bytes move under
// receipts; months close into a chain; Ada picks her paranoia at cost; she seals,
// then entombs; the co-op waives a fee it can afford; the erasure-coded door is
// shown but not walked through; and Bram's royalty pool extinguishes on the same
// ledger machinery).

import { World } from "./world.ts";
import type { ExperimentResult } from "./types.ts";
import { run as e0 } from "./exp/e0_identity.ts";
import { run as e1 } from "./exp/e1_items.ts";
import { run as e2 } from "./exp/e2_manifest.ts";
import { run as e3 } from "./exp/e3_receipts.ts";
import { run as e4 } from "./exp/e4_statements.ts";
import { run as e5 } from "./exp/e5_audits.ts";
import { run as e6 } from "./exp/e6_dial.ts";
import { run as e7 } from "./exp/e7_seal.ts";
import { run as e8 } from "./exp/e8_tombstone.ts";
import { run as e9 } from "./exp/e9_grace.ts";
import { run as e10 } from "./exp/e10_erasure.ts";
import { run as e11 } from "./exp/e11_financing.ts";

export const EXPERIMENTS: ((w: World) => ExperimentResult)[] = [
  e0, e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11,
];

/** Run every experiment in order against a fresh (or supplied) world. */
export function runSuite(world = new World()): World {
  for (const run of EXPERIMENTS) {
    world.results.push(run(world));
  }
  return world;
}

/** Total assertions and failures across the run. */
export function tally(world: World): { total: number; failed: number } {
  let total = 0, failed = 0;
  for (const r of world.results) {
    for (const a of r.assertions) {
      total++;
      if (!a.ok) failed++;
    }
  }
  return { total, failed };
}
