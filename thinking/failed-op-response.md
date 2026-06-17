# Failed-operation response — leak, immune signal, and the silence/blackhole dial

date: 2026-06-16
status: thinking (design input for the wire spec + the metadata-leak spike; extends AR-4)

## Problem

When an adversarial operation **fails** — a forged-history branch, a non-member join, a tampered
message — the system has caught it (deterministically; see the faithful path / `backfill_import`).
But "caught it" is not the end of the story. A failed op is an **observable**: it can leak metadata
to passers-by, or it can be deliberately recorded and shared as a defensive signal. We owe the
protocol an explicit answer to *what happens after detection*, because the naive default (reject and
move on) silently picks one point on a dial that has real privacy, security, and DoS consequences.

## Approach

**Detection is fixed; response is a dial.** Detection is deterministic — every member runs the same
`backfill_import` over the same recorded group state and reaches the same ACCEPT/REJECT verdict
(this determinism is a *prerequisite*, satisfied by the faithful path). What a group *does* after a
deterministic rejection is governance:

```
  detected failure (deterministic verdict)
        │
        ├── LOUD       emit a SIGNED rejection-event ──► group "immune memory"
        │              (reputation, rate-limit, alert, input to a removal vote)
        │              max awareness │ max leak + amplification │ tells attacker "caught"
        │
        ├── SILENT     reject locally, no signal, no absorb
        │              attacker cannot distinguish reject from packet loss / offline
        │              no shared memory │ privacy-preserving │ denies attacker feedback
        │
        ├── BLACKHOLE  engage-then-void (tarpit): appear to accept, drop into nothing
        │              attacker cannot confirm reject vs success │ burns attacker effort
        │              │ costs us resources │ may confuse a misconfigured honest peer
        │
        └── COMBINED   silent-to-attacker + loud-to-members
                       (record internally, give the attacker no feedback)
```

**The immune-signal path, done safely.** A rejection-event is
`{observer, target_did, reason, epoch, op-hash}`, **signed by the observer**. To drive a serious
response (auto-rate-limit, a removal vote) it should require **k corroborating observers** — the same
threshold dial as revocation authority (`revocation-authority.md`). One member's claim is a flag, not
a verdict; otherwise a malicious member fabricates "alice forged history" to frame an honest member —
the immune system turned into an injection vector.

## Reasoning

- **Same observable, two valences.** The metadata an attacker leaks by failing is the same metadata a
  group can harvest defensively. Whether it is a *leak* or an *immune response* is the
  loud/silent/blackhole choice — they are one mechanism, not two.
- **Determinism is what makes auto-response safe.** If members disagreed on whether an op failed, an
  automated immune response would attack the group itself (members diverge on who is "infected"). The
  faithful path's identical-verdict property is the foundation; without it, only manual review is safe.
- **Silence is an application-layer property, not a transport one.** Members can decline to propagate
  an immune event, but the **relay still sees the connection attempt** — EndpointId pair, timing,
  volume (the AR-4 bound). Silent-reject removes the *deliberate, group-wide, attributable* leak; it
  does **not** remove the *passive transport* leak. Blackhole manipulates only what the *attacker*
  infers from the application response, not what the relay sees. Be honest about which layer each
  setting addresses.
- **Silent vs blackhole differ in attacker inference.** Silent = no response (ambiguous: looks like
  loss/offline). Blackhole = response-then-void (deceptive: looks like success, nothing happens; a
  tarpit that can waste attacker effort). Blackhole is the stronger anti-enumeration play at the cost
  of engagement resources.
- **Loud signaling is a DoS amplifier if naive.** One gossiped event per failed op lets an attacker
  flood failures to trigger an event storm. Mitigation: aggregate/rate-limit immune events ("N
  rejections from X in window"), never echo per-op.
- **It is per-group governance, and couples to other dials.** Privacy-max / blind-broker groups lean
  silent; security-max co-ops lean loud + corroborated. Immune *memory* must also respect freshness
  (`freshness-signal.md`) — do not act on stale immune memory from a partition you cannot prove is
  current — and maps onto the confidentiality tiers (loud security vs quiet privacy).

## Open edges

- **Event schema + retention:** exact fields, signing, dedupe, and how long immune memory persists
  (and whether it is itself foldable/expirable like history).
- **Corroboration window:** how k observers' independent reports are correlated without a coordinator,
  and how that interacts with partition (the reconcile contradiction shape again).
- **Blackhole cost model:** resource cost of tarpitting vs the enumeration-denial benefit; abuse by an
  attacker who *wants* you to spend resources engaging.
- **Honest-peer false positives:** a misconfigured or stale honest peer can trip the immune system;
  the response dial must not auto-escalate a benign desync into a removal. (Ties to freshness +
  the manual-review floor.)
- **Quantify the residual transport leak** under each setting (the metadata-leak spike, task #10).
