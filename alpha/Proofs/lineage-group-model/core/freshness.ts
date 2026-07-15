// E2.16 — tier-degradation visibility, built on the freshness signal designed in
// discovery/thinking/freshness-signal.md.
//
// The model: a peer holds its own applied tip and the best tip it has heard via a
// (content-free, signed) beacon, plus the local time it last heard ANY beacon.
// Three properties this enables:
//   (a) availability: a peer with NO superpeer/peers still applies its own ops
//       locally (forward-send works);
//   (b) honest staleness: behind-ness / uncertainty is surfaced — a peer that
//       hears from no one past the horizon is "unverified", never silently "current"
//       (the no-false-current invariant);
//   (c) tiers degrade VISIBLY: the per-tier freshness horizon changes WHEN the view
//       flips, but every tier produces an explicit, non-silent state.

import { createHash } from 'crypto';

export type ViewState = 'current' | 'behind' | 'unverified';
export type Tier = 'interactive' | 'quiet-large' | 'broadcast';

// Per-tier freshness horizon (ms). Interactive: tight (presence matters). Quiet-large:
// hours (eventual contract). Broadcast: no time-based liveness promise (best-effort) —
// position is judged against the latest announcement seen, not against a clock.
export const FRESHNESS_HORIZON_MS: Record<Tier, number> = {
  interactive: 15_000,
  'quiet-large': 6 * 3_600_000,
  broadcast: Number.POSITIVE_INFINITY,
};

export interface Tip {
  epoch: number;
  head: string;
  seq: number;
}

export function tipHead(epoch: number, seq: number, payload: string): string {
  return createHash('sha256').update(`${epoch}|${seq}|${payload}`).digest('hex');
}

export interface PeerView {
  applied: Tip; // what this peer has applied locally
  bestSeen: Tip; // furthest-ahead tip heard via beacon (starts == applied)
  lastBeaconAt: number; // monotonic ms of the last beacon heard; -Infinity if never
}

export function newPeer(epoch: number): PeerView {
  const genesis: Tip = { epoch, head: tipHead(epoch, 0, 'genesis'), seq: 0 };
  return { applied: { ...genesis }, bestSeen: { ...genesis }, lastBeaconAt: Number.NEGATIVE_INFINITY };
}

/// (a) Forward-send: apply a local op. Always succeeds regardless of connectivity —
/// availability does not depend on a superpeer or any peer being reachable.
export function sendForward(view: PeerView, payload: string): PeerView {
  const seq = view.applied.seq + 1;
  return {
    ...view,
    applied: { epoch: view.applied.epoch, seq, head: tipHead(view.applied.epoch, seq, payload) },
  };
}

/// A signed, content-free tip beacon (modelled; signature elided — authorship is the
/// faithful-path concern). Receiving one updates best-seen + the liveness clock.
export interface TipBeacon {
  tip: Tip;
}

export function receiveBeacon(view: PeerView, beacon: TipBeacon, now: number): PeerView {
  const ahead =
    beacon.tip.epoch > view.bestSeen.epoch ||
    (beacon.tip.epoch === view.bestSeen.epoch && beacon.tip.seq > view.bestSeen.seq);
  return {
    ...view,
    bestSeen: ahead ? { ...beacon.tip } : view.bestSeen,
    lastBeaconAt: now,
  };
}

/// The no-false-current rule. "current" requires BOTH (a) applied is at/after the
/// best-seen tip AND (b) a beacon was heard within the tier's freshness horizon.
/// Otherwise: "behind" if we know of a later tip, else "unverified" (we cannot prove
/// currency — silence is never rendered as current).
export function viewState(view: PeerView, now: number, tier: Tier): ViewState {
  const horizon = FRESHNESS_HORIZON_MS[tier];
  const heardRecently = now - view.lastBeaconAt <= horizon;
  const caughtUp =
    view.applied.epoch > view.bestSeen.epoch ||
    (view.applied.epoch === view.bestSeen.epoch && view.applied.seq >= view.bestSeen.seq);

  if (heardRecently && caughtUp) return 'current';
  if (!caughtUp) return 'behind';
  return 'unverified';
}
