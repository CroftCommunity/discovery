# Freshness signal — absence-of-news is not evidence of currency

date: 2026-06-16
status: thinking (design; unblocks E2.16). Problem / Approach / Reasoning.

## Problem

Every "stale is visible" result we have is **comparative**: it depends on a peer *hearing from
someone who is ahead*.

- **AR-2** (malicious blind sequencer): a dropped op leaves the victim with a head that is visibly
  behind the others' — *once the victim compares against another peer's head*.
- **Multi-device / fold**: a device that missed updates "looks behind" — *relative to the sibling
  devices it syncs with*.

There is no mechanism for the case that actually bites in a partition: **a peer that hears from no
one cannot tell it is behind.** Silence is indistinguishable from "nothing has happened." A device
in a tunnel, a member whose only superpeer went offline, a phone that has not synced since this
morning — each may render a confident, *current-looking* view that is in fact hours stale, and the
protocol gives it no way to know. The interaction tiers (`interaction-tiers.md`) *promise* a failure
signal ("it'll arrive or you'll be told it didn't") but we have not specified the thing that
produces the signal when there is no counter-party to compare against. That mechanism is the
freshness signal, and its absence is gap #1 from the proof narratives.

## Approach

**A periodic signed "tip beacon," plus a local freshness clock, plus a hard no-false-current rule.**

### 1. The tip beacon

Each peer (and any always-on broker/meer) periodically emits a small, signed beacon over the group's
gossip topic:

```
TipBeacon {
  group_id,
  epoch,            // MLS/membership epoch the sender is on
  head,             // hash of the sender's latest known head (per lineage/branch tip set)
  seq_high,         // highest op seq the sender has applied
  emitted_at,       // sender's clock (advisory; see "no trusted time" below)
  sig               // signed by the emitting device's key (authorship, as in the faithful path)
}
```

The beacon carries **head/epoch/routing metadata only — never content** (tie to AR-4 and the
blind-broker E3.4 result). A broker can relay or originate beacons without decrypting anything: a
beacon is exactly the order-preserving metadata (head hash, epoch, seq) a blind mirror already sees,
plus a signature. So beacons are safe to gossip through an untrusted relay/broker.

### 2. The local freshness clock — "I am behind" / "I have not heard in N"

A peer maintains, per group:

- `last_beacon_at` — when it last received *any* valid beacon from *any* other member.
- `best_seen_head/epoch/seq` — the furthest-ahead tip any beacon has advertised.

From these it computes two distinct states (they are not the same and must not be conflated):

- **BEHIND** — `best_seen_head` is ahead of my applied head (I have heard, and I know I am behind).
  This is the comparative case AR-2/multi-device already cover; the beacon just makes it work without
  exchanging full state.
- **UNCERTAIN / STALE-RISK** — `now - last_beacon_at > freshness_threshold`. I have heard from *no
  one* for longer than the threshold, so I **cannot prove** my view is current. This is the new,
  load-bearing state — the partition/tunnel case.

`freshness_threshold` is per-tier (below). It is a *staleness horizon*, deliberately generous —
crossing it does not mean "you are wrong," it means "you can no longer claim you are right."

### 3. No trusted wall-clock — use heard-from-ness, not timestamps

`emitted_at` is advisory only; we do not trust peers' clocks (and the blind broker must not need a
synchronized clock). Freshness is measured in **the receiver's own monotonic time since it last
heard a valid beacon**, not in the difference between two wall clocks. "I have not heard in N
minutes" is a local, unspoofable measurement; "this beacon was emitted at T" is a claim. Epoch and
seq are the *ordering* truth (monotone, signed); time-since-heard is the *liveness* truth.

### 4. Surfacing — fail-early / fail-clearly, never silent

The UI must reflect freshness honestly:

- **Current** — heard within threshold AND applied head == best-seen head. Only here may the UI
  present the view as up to date.
- **Behind (catching up)** — known behind; show a catching-up affordance, not a current view.
- **Unverified / possibly stale** — past the freshness horizon; the UI must *visibly mark the view as
  unverified* ("last synced 3h ago — may be out of date"), never render it as if current.

This is the **fail-early/fail-clearly** principle made concrete for liveness: staleness is a
first-class, visible state, never a silent default.

### 5. Interaction with the tiers

- **Interactive (~2–dozens):** short threshold (seconds–minutes). The beacon *is* the "real failure
  signal when a peer is unreachable" the tier promises — crossing the horizon flips presence to
  "unreachable" and the view to unverified, promptly.
- **Quiet-large:** long threshold (hours). This tier's promise is "it'll arrive or you'll be told it
  didn't" — the beacon is *how you're told*. Low beacon rate (it is a quiet room); a single broker/
  meer beacon is enough to keep the room "fresh enough."
- **Broadcast:** weakest. The rolling-forward log advertises its own tip in the stream; a reader
  knows its position relative to the latest announcement it has seen. No per-recipient freshness
  accounting — consistent with "probably delivered, eventually."

Threshold scales with the tier's real-time expectation: tight where presence matters, loose where
eventual is the contract.

### 6. Interaction with the blind broker

The beacon is the ideal blind-broker payload: head + epoch + seq + signature + routing, no content.
A blind broker (Tier 0 meer, E3.4) can **originate** beacons on behalf of the group's tip it holds
(it knows the tips for range reconciliation) and **relay** members' beacons, giving offline-heavy
groups a freshness source without any peer being online — and without the broker learning anything
it did not already see. This makes the always-on meer a *freshness provider*, not just a store: the
answer to "is anyone keeping this group fresh while we sleep?" is the meer emitting tip beacons.

### 7. The no-false-current guarantee

**A peer never displays "current" when it cannot prove freshness.** Concretely: the "current" UI
state requires *both* (a) applied head == best-seen head *and* (b) time-since-last-beacon <
threshold. If either fails, the view degrades to "behind" or "unverified." There is no path where
silence is rendered as currency. This is the invariant the whole mechanism exists to guarantee, and
it is what E2.16 will test: without a superpeer, a peer (a) still sends forward locally, (b) has its
behind-ness/staleness surfaced, (c) degrades visibly across tiers, never silently.

## Reasoning

- **Why a beacon and not full-state exchange:** comparing full heads is expensive and content-bearing;
  a beacon is O(1), content-free, and blind-broker-safe. It gives the liveness signal without the
  reconciliation cost — reconciliation happens only once a peer *knows* it is behind.
- **Why time-since-heard, not timestamps:** trusting peer clocks would import a time-authority and a
  spoofing surface, and would break the blind broker (which has no business holding synchronized
  time). Liveness is inherently a local measurement; ordering is the signed/monotone part.
- **Why a generous, per-tier horizon:** a tight global threshold would cry stale constantly on mobile/
  CGNAT (Croft's real population); too loose and the guarantee is hollow. The horizon must match each
  tier's real-time contract — and it is honest precisely because crossing it claims only "unverified,"
  not "wrong."
- **Why no-false-current is the core:** the danger is not being behind — being behind is recoverable.
  The danger is *believing you are current when you are not*, because that is the state in which a
  user acts on stale data (sends to a removed member, misses a revocation, trusts an old membership).
  Making "current" provable-or-not-shown is the whole point.
- **Coupling to authority/freshness (revocation-authority.md):** a membership change authorized
  against a stale epoch is dangerous; the freshness signal is the gate that says "do not act on this
  removal/auth decision — you cannot prove your group view is current." Freshness gates authority.

## Open edges

- **Beacon rate vs battery/metadata** — too frequent leaks more timing metadata (AR-4) and drains
  battery; too sparse widens the blind window. The rate is a per-tier dial to calibrate.
- **Beacon authenticity at scale** — beacons are signed, but a flood of valid-but-stale beacons (or
  an adversary withholding beacons) is a liveness attack; relates to the failed-op/immune-signal
  spike (`failed-op-response.md`) and to AR-2's withholding sequencer.
- **Threshold calibration** is unmodelled — the numbers (seconds/hours per tier) need measurement on
  the real fabric.
- **Whose beacon counts** — in a partition, a clique can keep each other "fresh" while collectively
  behind the true tip (a fresh-but-wrong partition). Freshness proves *liveness*, not *global
  currency*; distinguishing them needs the reconcile hard-stop on reconnect. Worth stating explicitly
  so freshness is not over-claimed.
