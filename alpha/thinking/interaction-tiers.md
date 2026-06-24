# Interaction Tiers, Presence, and the Group-Size Question

date: 2026-06-15

status: design thinking distilled from the Germ/X Chat dialogue (raw:
seeds/transcripts/raw/germ-xchat-design-dialogue.md; research: research/germ-xchat-features.md).
Extends the lineage-groups model (thesis, multi-device) with the scale/real-time dimension.

---

## The reframe

Group size is not one axis with a power knob — it is **three different products that share a
send button**, and the architecture changes character at each break. Rich real-time signals
(presence, typing) are an **opt-in power tier carried over iroh gossip between already-
connected peers**, so they never touch the blind broker, and they degrade automatically as a
room's behaviour changes.

Two enabling principles:

- **The metadata-leak objection dissolves for gossip between live peers.** Presence/typing
  only reach the broker if routed through it. When peers already hold a direct iroh
  connection, the ephemeral signal rides that channel and the broker sees nothing; when no
  direct path exists, it silently degrades to nothing (the correct behaviour anyway).

- **The real cost is connection-keepalive and wake frequency, not payload.** A typing burst
  is tiny; what drains battery is keeping connections warm and waking the radio. That cost
  scales with live-connection count, which is why tiering by room size/behaviour maps to the
  real cost curve. ([UNVERIFIED] exact keepalive/battery figure — ship a conservative
  default, measure, tune.)

## The three tiers

1. **Interactive (~2 to a few dozen).** Live iroh connections; gossip presence + typing;
   low-latency delivery; strong convergence (everyone connected). Presence/typing default
   **off**; turning them on is the user accepting a *visible battery cost*, never a hidden
   metadata cost. The size ceiling is "how much drain you'll tolerate."

2. **Quiet-but-large (the room with everyone you've met).** The room's behaviour rescues the
   architecture: if almost nobody talks, there's nothing to fan out, so size's battery cost
   largely disappears. Presence/typing switch off above a threshold and nobody misses them
   (no real-time expectation to violate). Lean into eventual consistency. Guarantee kept:
   "it will get there, or you'll know it didn't." Guarantee dropped: "when."

3. **Broadcast (very large, or any one-to-many room).** Above ~1000, or whenever rate +
   membership imply it, a "chat" was always broadcast pretending to be conversation. Drop the
   whole real-time apparatus: no presence, no typing, no per-message delivery anxiety. It
   becomes an append-only **rolling-forward log** of announcements, gossip-replicated,
   best-effort fan-out. Deliberately **Scuttlebutt-shaped, without SSB's fatal flaw**: keep
   the rolling forward log, drop the immutable-infinite-history requirement (bounded/rolling
   window). Announcements can link out to public surfaces (atproto/Bluesky) for anything that
   doesn't need to pretend to be real-time. Broadcast is the **pressure-release valve** that
   lets strong guarantees survive where they matter — not a sad side-feature.

## Type at creation, not switchable mode

Interactive and broadcast are **different object types chosen at creation**, not one object
with a mode toggle. An interactive room can grow and shed its rich signals automatically
(typing off, presence off) without changing what it fundamentally is, because it never
promised broadcast semantics. A broadcast channel is a rolling log from day one. **Live
in-place conversion is not supported** — it would swap mechanisms underneath people and break
MLS membership coherence. A room that genuinely needs to "become" broadcast is a
create-new-and-redirect operation, not a mutation. So mode is never negotiated; type is
chosen once, guarantees slide automatically with behaviour, and the UI communicates the tier
by what it does/doesn't promise (the absence of a typing indicator *is* the communication).

## Guarantees by tier (state delivery promises explicitly)

- **Interactive:** strong-ish — prompt delivery + a real failure signal when a peer is
  unreachable.

- **Quiet-large:** eventual — it'll arrive or you'll be told it didn't, but not "when."

- **Broadcast:** weakest — probably delivered, eventually; no per-recipient accounting.

(The "you'll know it didn't get there" guarantee is real work: distinguishing failed from
not-yet-heard-back needs acknowledgment structure, itself best-effort under pure eventual
consistency. Promise the strong version only in interactive.)

## Read receipts are a separate decision

Presence/typing are ephemeral (if they don't arrive, nothing is lost) → gossip/best-effort
bucket. A read receipt is **durable state** ("did they see it"). If it must be reliable, it
wants to be a small CRDT update on the normal sync path — which means the normal metadata
profile, not the zero-broker gossip profile. Decide deliberately: receipts are either
best-effort+opt-in (gossip bucket) or durable-on-the-normal-path (CRDT bucket). Don't let it
default silently.

## The product framing

This turns the privacy posture from a missing-feature apology into a user-legible choice:
"Presence and typing are off by default because they require staying connected, which uses
battery, and we never send them through our servers." Same inversion as the rest of Croft:
the privacy-preserving default is the free one; the convenience is an opt-in that costs
something *visible* (battery), not something *hidden* (metadata).
