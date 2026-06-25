# thinking/drystone-spec — Drystone protocol working spec drafts

date: 2026-06-25

These are **DRAFT** sections of the Drystone P2P protocol spec, distilled as working artifacts from
design dialogues. They are not yet normative and are not yet folded into
`crystallized/CROFT-PROTOCOL.md` (the existing wire spec). Treat them as the governance-and-peer-model
layer in flight; promotion/reconciliation into CROFT-PROTOCOL is a later step.

| Section | File | Status | Realizes |
|---|---|---|---|
| §2 Peers, Rights, Capabilities, PeerSets | `section-2-peers-rights-capabilities.md` | DRAFT | P-Local-Truth, P-Peer-Equality, P-Knowable-Truth, P-Durable-Enablement |
| §X Concurrent & Conflicting Governance Changes | `section-x-governance-conflicts.md` | DRAFT / ENABLING | P-Peer-Equality, P-Local-Truth, P-Knowable-Truth |

## Provenance

Both distilled 2026-06-25 from
`../../seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md` (a
claude.ai design dialogue, cleaned-paste). The dialogue's deep Matrix architecture-and-UX contrast is
the reasoning behind these drafts.

## What is settled vs open

**Settled vocabulary (this batch):** one kind of peer; rights (universal, never delegated) vs
capabilities (additive, delegated, revocable); the three capability-plane layers
**capability / role / PeerSet**; **PeerSet** as the term (renamed from "Set"); the **exitability**
backstop and the **asymmetry-of-expressible-range** framing; revocation as an epoch-rotating
expulsion-shaped governance fact; the **meer** reframed as a PeerSet (`floor + requires{availability}
+ forbids{read}`), satisfying read-your-own-history vacuously.

**Open / deferred (tracked in `../../ROADMAP_TODO.md`):**

- The `P-*` principles these sections `Realizes` and the §1 that defines them are **not yet written** (E30).
- Capability mechanism: **Track A (Meadowcap-shaped) vs Track B (Keyhive-shaped)** — the user's/Phase-2 call (A11).
- Key-custody default: blind-relay vs revocable trusted delegate (A12).
- The "geer" gating-peer name (the user dislikes it) (A13).
- The `ENABLING` wire-format items (canonical fact encoding, frontier-closure, frontier-commitment) — gate the prior-art DOI.

**Cited facts not yet in the FACTCHECK SoT:** the Matrix/Willow/Meadowcap/Keyhive claims were
web-verified *in the source dialogue* only. Confirm the load-bearing ones (CVE-2025-49090, room v12 /
MSC4289, Meadowcap "no native revocation", Willow's unenforceable timestamp, Seshat-class
encrypted-search limits) before they harden into beta.
