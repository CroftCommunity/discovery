# Feature & UX Comparison: Germ (Bluesky) and X Chat

source: research deliverable produced 2026-06-13; filed here as industry research. Raw dialogue
+ provenance in seeds/transcripts/raw/germ-xchat-design-dialogue.md. The new design thinking
this produced (interaction tiers, durable-product principles) lives in
thinking/interaction-tiers.md and crystallized/principles.md Tier 3.

scope: user-facing features and use cases of two adjacent encrypted messengers, mapped against
the Croft local-first encrypted messaging stack. Lens = the product surface a user touches.

---

## The three-things framing (don't conflate)

- **Germ DM** (Germ Network) — MLS-based E2EE messenger integrating atproto identity, launches
  from Bluesky profiles; iOS beta; **currently 1:1 text**; multi-identity ("cards"), no phone
  number; ~4-person team. The real comparable to our stack.

- **Bluesky native DMs** — Bluesky's built-in DMs, **not** fully E2EE.

- **Bluesky native group chats** — launched 2026-06-11, up to 50, **no media at launch**, not
  E2EE. Distinct from Germ.

- **X Chat** — X's rebuilt messaging + standalone iOS app; Juicebox-backed **server-held keys**,
  PIN recovery, **no forward secrecy (X's own admission)**; E2EE claims disputed by
  cryptographers (Green, Garrett, Trapp); seamless multi-device + rich media are the selling
  points.

## Fit ratings against our stack (Natural / Effortful / Hard)

Highlights (full matrix in the raw deliverable):

- **Natural (ship early):** 1:1 + small-group chat (MLS group is the base case); forward-only
  history on join (it's what MLS epoch keys do); avatars/group-icons/photos/voice/files via the
  encrypted blob path (and we can keep group name/icon *encrypted*, which X makes public);
  reactions/replies/mentions/edits/unread (CRDT edits or local derivations); profile-initiated
  chat + social-graph gating (DID resolves from the public atproto profile); local search
  (plaintext on-device is trivially indexable); multiple identities/personas (DID-per-persona —
  closer to free for us than Germ's paid power-feature).

- **Hard (design before committing):** multi-device + cross-device history sync (**the sharpest
  tension; the one place we're clearly weaker than X**, whose whole Juicebox design exists to
  solve it); account/key recovery after device loss; full history backfill on join (against MLS
  forward secrecy); presence/typing/read receipts (metadata-leak tension — resolved by the
  interaction-tiers model); push to a sometimes-offline P2P client; link-preview fetch (who
  fetches the URL — client-fetch leaks to target/SSRF, broker-fetch breaks blindness).

- **Cooperating-client (not cryptographic) guarantees:** delete-for-everyone, disappearing
  messages, screenshot-notify — honorable among well-behaved clients, unenforceable against a
  modified peer. X states the same caveat; say so plainly.

## The pattern (the strategic core)

Across many rows the same inversion recurs: **the privacy-preserving behaviour is the free one;
the convenience behaviour is the effortful one.** Forward-only history is free, backfill is
work; on-device keys are our default, multi-device sync is the expensive thing X spent its whole
design budget on; encrypted group metadata is natural, matching casual public-by-default
convenience is what costs us. X chose convenience and paid in encryption integrity (to the point
cryptographers dispute its E2EE claim); Germ chose privacy and pays in feature breadth
(iOS-only, 1:1, thin). Our architecture sits near Germ's end — so lean into the features where
privacy and ease coincide, and treat the convenience features users import from X/iMessage as
deliberate, scoped design problems, not assumed table stakes.

## Differentiators we can claim

True peer-to-peer when co-present (no server in path); genuinely forward-secret by default (X
conceded it is not); encrypted group metadata; multiple unlinkable identities as a base feature;
a clean public(atproto)/private(P2P) boundary defined deliberately rather than leaked at;
user-run, blind, self-hostable infrastructure.

## Primary open problem

Multi-device + total-device-loss recovery. Fork: trust-minimized key backup vs. device
delegation (likely delegation primary + optional trust-minimized backup). See COHESION.md #12.
