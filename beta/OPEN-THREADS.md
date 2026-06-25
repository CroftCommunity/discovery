# discovery / beta — open threads (the staging queue at the beta gate)

date: 2026-06-25

## What this is

A holding ledger for threads that are **being pulled toward beta but are not yet settled enough to
become resolved beta narrative**. It exists so a live need is never lost, while the eight resolved
theme docs (`01`–`08`) stay a clean, settled synthesis. A thread lives here — referenced, not asserted
— until its gates clear; only then does it graduate into a theme doc (and earn a row in
`../alpha/BETA-ROLLUP.md`).

It is a **process artifact**, peer to `../alpha/BETA-ROLLUP.md` and `../alpha/COHESION.md` — **not** a
theme doc and **not** part of beta's forward narrative. Unlike a theme doc, it may point down into
`alpha/` freely (that is its job). It deliberately holds DRAFT / decision-gated / fact-unconfirmed
material **out** of the settled themes.

## Why it lives here (and not in alpha)

The settling happens **at the beta gate**, so the queue lives at the beta layer. But alpha must not
lose the need, so the alpha indexes reference it (`ROADMAP_TODO.md`, `COHESION.md`, `BETA-ROLLUP.md`
point here). Division of labor:

- `../alpha/ROADMAP_TODO.md` — the alpha **backlog** (everything up for consideration, any stage).
- `../alpha/BETA-ROLLUP.md` — the trace of what **landed** in beta (treatment → section), plus a
  coverage view of alpha sources **not yet** pulled up.
- **this file** — the subset of not-yet-pulled-up material that is **actively queued for beta and
  blocked on specific settling work**, with that work named per thread. A thread here is a `deferred`
  rollup item with its gates made explicit and a promotion target attached.

## Entry schema

Each thread carries:

- **Status** — `surfaced` (logged, gates not yet worked) · `gated` (blocked on named decisions/work) ·
  `ready-to-promote` (gates clear; next pass folds it into a theme).
- **What it is** — one or two lines.
- **Promotion target** — which beta theme(s)/section it would land in (or a proposed new theme).
- **Gates — must settle before it becomes resolved beta narrative** — the explicit blockers
  (decisions, `ENABLING` spec work, fact-confirmation).
- **Alpha provenance** — where the material lives now.

## Promotion rule

A thread leaves this file and enters a theme doc **only when its gates are clear**. On promotion:
write the settled synthesis into the theme doc (quotes whole, verification flags inline), add the
`../alpha/BETA-ROLLUP.md` trace row, and strike the thread here with a one-line "promoted → NN §X
(date)" note. Until then, beta theme docs may **not** assert the thread's content as resolved.

---

## Open threads

### T1 — Drystone governance & peer model (§2 peers/rights/capabilities + §X governance conflicts)

- **Status:** `gated`.
- **What it is:** the governance-layer design distilled 2026-06-24 — one kind of peer; **rights**
  (universal, never delegated) vs **capabilities** (additive, delegated, revocable); the
  **capability / role / PeerSet** layers (the meer recast as a PeerSet, satisfying read-your-own-history
  vacuously); the **exitability** backstop + **asymmetry-of-expressible-range** framing; revocation as an
  epoch-rotating expulsion-shaped fact; and the §X conflict-resolution model (**append-only fold →
  no-state-reset**, a **timestamp-free causal order**, an unconflictable capped root, the R1–R6
  capability interface, attributable-acceptance, and the regress-breaking termination construction). Its
  spine is a deep **Matrix close-cousin contrast** (where Matrix's choices cost a CVE + a multi-week
  outage; Croft's eventual-consistency bet dodges that class).
- **Promotion target:** primarily **04 (the protocol we proved)** and **06 (safety without
  surveillance)**; possibly a dedicated governance theme if it grows. The new principles
  (`P-Local-Truth`, `P-Peer-Equality`, `P-Knowable-Truth`, **`P-Durable-Enablement`**, the
  **peer-capability-floor**) would land in **01 / the principle set** once written.
- **Gates — must settle before resolved-beta:**
  1. **Status is DRAFT / ENABLING.** The wire formats are unspecified; the hardest piece
     (**frontier-closure before sort**, §X.8.5) is open. Beta 04 is about what we *proved*; this is
     design we have *not built*.
  2. **Two core mechanisms undecided** — capability mechanism **Track A (Meadowcap) vs Track B
     (Keyhive)** (`ROADMAP_TODO` **A11**); key-custody default **blind-relay vs trusted-delegate**
     (**A12**, incl. the "does Option-B-as-default rebuild a readable homeserver?" question). The "geer"
     name is also open (**A13**).
  3. **Cited facts not yet SoT-confirmed.** The Matrix / Willow / Meadowcap / Keyhive claims were
     web-verified *in the source dialogue only* — confirm (CVE-2025-49090; room v12 / MSC4289;
     Megolm/UTD; Seshat desktop-only search; Meadowcap "no native revocation"; Willow unenforceable
     timestamp + `is_authorised_write`; matrix.org Postgres postmortem; Karlsruhe SACMAT 2020; Element X
     vs Classic) against a source of truth before any of it hardens into beta. **Do not** re-introduce
     the dialogue's self-corrected false claim ("Matrix E2EE is bilaterally disable-able" — it is a
     one-way latch).
  4. **The `P-*` principles do not yet exist** in `../alpha/crystallized/principles.md` by these names;
     §2/§X `Realizes` them but they are unwritten.
- **Alpha provenance:** `../alpha/thinking/drystone-spec/section-2-peers-rights-capabilities.md`,
  `…/section-x-governance-conflicts.md`, README; raw dialogue
  `../alpha/seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md`.
  Backlog: `../alpha/ROADMAP_TODO.md` **E30** (+ A11/A12/A13); seam: `../alpha/COHESION.md` **§37**.

---

## How to use this file

When a beta theme doc is tempted to assert something that is actually still in flight, park it here
instead with its gates named, and reference it. When the gates clear, promote it per the rule above and
record the trace in `../alpha/BETA-ROLLUP.md`. Keep this list short — a thread that has been
`ready-to-promote` for a while should be promoted, not parked.
