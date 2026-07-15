// E2.16 — tier degradation visibility (unblocked by the freshness-signal design).
//
// Three assertions:
//   E2.16a (INV-AVAILABILITY-NO-SUPERPEER): without any superpeer/peer, a peer still
//          applies its own ops locally — forward-send works, availability holds.
//   E2.16b (INV-NO-FALSE-CURRENT): behind-ness / staleness is SURFACED — a peer that
//          hears from no one past the horizon is "unverified", a peer that heard a
//          later tip is "behind", and only a peer that is caught up AND heard recently
//          is "current". Silence is never rendered as currency.
//   E2.16c (INV-TIER-DEGRADES-VISIBLY): the per-tier horizon changes WHEN the view
//          flips, but every tier yields an explicit, non-silent state — no tier ever
//          shows "current" while unable to prove it.

import { ScenarioRunner, ExperimentResult } from '../harness/runner';
import {
  newPeer,
  sendForward,
  receiveBeacon,
  viewState,
  tipHead,
  Tier,
  PeerView,
} from '../core/freshness';

const runner = new ScenarioRunner();

function runE216a(): ExperimentResult {
  return runner.run(
    'E2.16a: forward-send works with no superpeer — availability holds',
    'E2.16',
    () => {
      // A peer entirely alone: no beacon ever heard, no peer reachable.
      let p = newPeer(1);
      const startSeq = p.applied.seq;
      p = sendForward(p, 'note to self while offline');
      p = sendForward(p, 'another local op');
      const advanced = p.applied.seq === startSeq + 2;

      // Availability did not depend on hearing anyone.
      const appliedLocallyDespiteSilence = advanced && p.lastBeaconAt === Number.NEGATIVE_INFINITY;

      return {
        invariants: [
          {
            invariant: 'INV-AVAILABILITY-NO-SUPERPEER',
            passed: appliedLocallyDespiteSilence,
            details: [
              `Local ops applied with zero peers: seq ${startSeq}→${p.applied.seq} ✓`,
              `No beacon was needed to make local progress (lastBeaconAt = never) ✓`,
            ].join('; '),
          },
        ],
        provenanceTrace: [
          `Peer alone (no superpeer). sendForward x2 applied locally; seq advanced ${startSeq}→${p.applied.seq}.`,
          `Forward progress never blocks on connectivity — availability is local-first.`,
        ].join('\n'),
        socialInputsUsed: ['SCRIPTED: isolated peer applies local ops'],
        metrics: { appliedSeq: p.applied.seq },
      };
    },
  );
}

function runE216b(): ExperimentResult {
  return runner.run(
    'E2.16b: behind-ness/staleness surfaced — no-false-current',
    'E2.16',
    () => {
      const now = 1_000_000;
      const horizon = 15_000; // interactive

      // Case 1: heard a LATER tip → "behind".
      let behind = newPeer(1);
      behind = receiveBeacon(behind, { tip: { epoch: 1, head: tipHead(1, 5, 'ahead'), seq: 5 } }, now);
      const isBehind = viewState(behind, now + 1000, 'interactive') === 'behind';

      // Case 2: heard from NO ONE past the horizon → "unverified" (NOT current).
      const silent = newPeer(1);
      const isUnverified = viewState(silent, now, 'interactive') === 'unverified';

      // Case 3: caught up AND heard within horizon → "current".
      let fresh = newPeer(1);
      fresh = receiveBeacon(fresh, { tip: { ...fresh.applied } }, now); // beacon agreeing with our tip
      const isCurrent = viewState(fresh, now + 1000, 'interactive') === 'current';

      // Case 4 (the trap): a peer that applied forward locally but heard no one must
      // NOT claim current just because its own head moved.
      let progressedButSilent = newPeer(1);
      progressedButSilent = sendForward(progressedButSilent, 'local op');
      const trapNotCurrent =
        viewState(progressedButSilent, now + horizon + 1, 'interactive') !== 'current';

      const passed = isBehind && isUnverified && isCurrent && trapNotCurrent;

      return {
        invariants: [
          {
            invariant: 'INV-NO-FALSE-CURRENT',
            passed,
            details: [
              `Heard a later tip → "behind": ${isBehind} ✓`,
              `Heard from no one past horizon → "unverified" (not current): ${isUnverified} ✓`,
              `Caught up AND heard recently → "current": ${isCurrent} ✓`,
              `Local progress with no beacon does NOT claim current: ${trapNotCurrent} ✓`,
            ].join('; '),
          },
        ],
        provenanceTrace: [
          `"current" requires caught-up AND heard-within-horizon. Silence yields "unverified", never "current".`,
          `A peer advancing its own head while hearing no one stays "unverified" — it cannot prove global currency.`,
        ].join('\n'),
        socialInputsUsed: ['SCRIPTED: beacon-ahead / silence / agreeing-beacon / local-progress-only'],
        metrics: { cases: 4 },
      };
    },
  );
}

function runE216c(): ExperimentResult {
  return runner.run(
    'E2.16c: tier guarantees degrade VISIBLY, never silently',
    'E2.16',
    () => {
      // Same scenario at every tier: a peer that has heard nothing for 1 hour.
      const now = 10_000_000;
      const oneHour = 3_600_000;
      const heardAt = now - oneHour;

      const states: Record<Tier, string> = {} as Record<Tier, string>;
      const tiers: Tier[] = ['interactive', 'quiet-large', 'broadcast'];
      for (const tier of tiers) {
        const p: PeerView = { ...newPeer(1), lastBeaconAt: heardAt };
        states[tier] = viewState(p, now, tier);
      }

      // Interactive (15s horizon): 1h of silence → unverified (flips promptly).
      const interactiveFlips = states['interactive'] === 'unverified';
      // Quiet-large (6h horizon): 1h silence is still within horizon → current allowed
      // (it heard within the tier's contract; the tier promises eventual, not instant).
      const quietHoldsWithinHorizon = states['quiet-large'] === 'current';
      // Broadcast (no time horizon): judged by position vs latest seen; caught up here
      // → current, but it never *claimed* time-freshness (best-effort, honestly labeled).
      const broadcastHonest = states['broadcast'] === 'current';

      // The cross-tier property: each tier returns an EXPLICIT state and NONE silently
      // shows current when it cannot prove it. Crank silence to 1 week:
      const week = 7 * 24 * oneHour;
      let anySilentFalseCurrent = false;
      for (const tier of tiers) {
        const stale: PeerView = {
          // applied is BEHIND a later seen tip, and last heard a week ago
          ...newPeer(1),
          bestSeen: { epoch: 1, head: tipHead(1, 9, 'far-ahead'), seq: 9 },
          lastBeaconAt: now - week,
        };
        const s = viewState(stale, now, tier);
        if (s === 'current') anySilentFalseCurrent = true; // must NEVER happen
      }

      const passed =
        interactiveFlips && quietHoldsWithinHorizon && broadcastHonest && !anySilentFalseCurrent;

      return {
        invariants: [
          {
            invariant: 'INV-TIER-DEGRADES-VISIBLY',
            passed,
            details: [
              `Interactive (15s): 1h silence → "${states['interactive']}" (flips promptly) ✓`,
              `Quiet-large (6h): 1h silence → "${states['quiet-large']}" (within the eventual contract) ✓`,
              `Broadcast (no clock): caught-up → "${states['broadcast']}" (best-effort, honestly labeled) ✓`,
              `No tier shows "current" while behind + a week stale: ${!anySilentFalseCurrent} ✓`,
            ].join('; '),
          },
        ],
        provenanceTrace: [
          `Same silence, three tiers: the horizon sets WHEN the view flips, but the state is always explicit.`,
          `Interactive flips to unverified fast; quiet-large tolerates within its hours-long horizon; broadcast`,
          `judges by position, never by a freshness clock. A behind+stale peer is "behind", never silently current.`,
        ].join('\n'),
        socialInputsUsed: ['SCRIPTED: 1h and 1-week silence evaluated across all three tiers'],
        metrics: {
          interactive: states['interactive'],
          quietLarge: states['quiet-large'],
          broadcast: states['broadcast'],
        },
      };
    },
  );
}

export function run(): ExperimentResult[] {
  return [runE216a(), runE216b(), runE216c()];
}
