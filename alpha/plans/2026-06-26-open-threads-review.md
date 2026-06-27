# Open-threads review → OPEN-THREADS.md refactor (2026-06-26)

**Status:** EXECUTED 2026-06-26 (user-directed "process it and commit and push"). The structural
changes flagged APPLY below are in `beta/OPEN-THREADS.md`; the changes flagged SURFACE are registered as
tracked decisions in that file and listed here, not executed.

## Problem statement

The user voice-reviewed `beta/OPEN-THREADS.md` top-to-bottom and dictated follow-up notes (raw:
`beta/thinking/raw/open threads review Jun 26 at 8-17 PM.txt`, ~212 lines, preserved-verbatim). The
review is the author's own editorial pass on a **process artifact** (the beta staging queue), so its
notes are **authorized refinement direction**, not external opinion. It has two layers: (A) meta /
structural proposals about how the ledger itself should be shaped, and (B) per-thread notes on T1–T30.

## Approach

Classify every discrete note, de-dupe the circling-back, and split into **APPLY** (authorized,
low-risk, structural — the user clearly wants it) vs **SURFACE** (large reorg, net-new doc creation, or
a thread that needs the user's design call — register as a tracked decision, do not execute). No design
gate is resolved. Apply the structural changes, register provenance, record the trace, commit + push.

## Reasoning

OPEN-THREADS.md is the staging queue, not beta narrative — restructuring *it* (status vocabulary,
grouping, type tags) is process hygiene, not a content decision, so it is safe to apply. But (1) the
per-thread **directory split**, (2) the **"meat on the bone" expansion** of every thread, and (3) the
**new docs** the review asks to stand up (brand/voice, adoption-enablement, per-platform design files,
per-app PRDs) are either large reorgs or net-new authored content — those are surfaced as directives so
the user greenlights them, rather than executed unilaterally in a processing pass. This matches the
established discipline: register every borderline/deferred item as a tracked decision; surface, don't
decide.

Transcription read-throughs (infer from context): "dry Stone" = **Drystone**; "appear/a pier/up here" =
**a peer**; "mere/mirror/gear/geer pier set" = **meer PeerSet**; "Crips/in Crips" = **encrypts**;
"Violet" = **(unclear — likely "in play")**; "red DB" = **redb**; "woohoo forbid" = **(a hard-forbid
case)**; "clothes/close" = **closed**; "DOI" = **DOI**.

---

## A. Meta / structural proposals

| # | Note (de-duped) | Disposition |
|---|---|---|
| M1 | Promoted/closed items sitting in the open list are confusing; promotion should **move out** of the open list into a separate place (provenance retained) so the open queue is easy to scan. | **APPLY** — new `## Promoted & closed (provenance retained)` section; move T1, T26 (promoted) + T23 (closed) there. |
| M2 | The states are really **open / in-progress / promoted / closed** ("promoted = got integrated; closed = decided not to pursue it"). | **APPLY** — schema rewritten to this state model (open keeps surfaced/gated sub-state). |
| M3 | Each thread needs a **type**, not just a status — does it need *proving / experimentation / additional content / research / legal review / publish* — so threads can be grouped and "run out" together. | **APPLY** — added a `Type` field to the schema and tagged every open thread. |
| M4 | Each thread is too terse ("just need more meat on the bone"): wants a plain-language **problem statement / proposed directions / what's indeterminate** per thread. | **SURFACE + partial APPLY** — added the richer shape to the schema as the migration target; applied review notes only where the transcript supplied material (no invented design). |
| M5 | "Honestly, maybe each one of these open threads should be its own file in a directory" — an `open/` dir and a `closed/` dir, because they are getting bigger/more contextual. | **SURFACE** — large reorg; registered as a structural decision for the user to greenlight. Not executed. |
| M6 | Start a **brand / voice / messaging** working doc/folder (a theme), accreted over time (taglines, ideas, links). | **SURFACE** — maps to existing **T4**; registered as a "start now" directive. Not authored this pass. |
| M7 | **Adoption-enablement** and **branding/marketing** are **twin docs** being built up for thinking/reference, so nothing is forgotten and the "ammo" accretes. | **SURFACE** — maps to existing **T11**; registered as the twin-doc directive alongside M6. |
| M8 | Stand up **per-platform design files** (one each Linux / macOS / Android / iOS) and walk out the common-core-vs-platform-difference trades; "every platform not implemented yet needs a version of this thinking." | **SURFACE** — maps to **T6 / T14**; registered as a directive. Not authored this pass. |
| M9 | Stand up **per-app PRDs** (chat its own, games-pond its own), plus a new modest starter use case — a peer-to-peer **"thinking of you"** signal (touch a thing on your phone, reach out to another person) — each its own design doc. | **SURFACE** — maps to **T15** (+ a new "thinking of you" PRD candidate folded into T15). Registered; not authored. |
| M10 | Threads should be **groupable by type** so a batch can be run out together (the payoff of M3). | **APPLY** (as schema intent) — the Type field enables it; a grouped index view is a SURFACE follow-up. |

---

## B. Per-thread notes (T1–T30)

Status/type column shows the post-review state in the refactored file.

| Thread | Review note (de-duped) | Post-review disposition |
|---|---|---|
| T1 | Still shows as PROMOTED in the open list — should move out. | **MOVED** → Promoted section. |
| T2 — governance at scale | Liquid delegation for larger groups; needs the **test harness** to test scale (variety + quantity + peers); likely **more than one large-scale governance model** — (a) revocable/movable delegate-vote (liquid democracy), (b) elected-admin / Reddit-moderation "all-or-nothing" where peers still hold rights + vote with their feet, (c) **broadcast-only groups** (a different rights model). "Hypothesize then play it out." | Note appended; **Type: needs-experimentation**. Couples T5. |
| T3 — moderation under blind broker | The meer PeerSet has the **same rights as a full peer** but no local history + never sees content, so you **can't moderate on content** — moderation is **separate from the meer PeerSet** and is about abuse/side-channels. Two different delegation stacks. | Note appended; **Type: needs-content**. |
| (cross-cut) | Recurring: **confusion between rights / roles / capabilities / delegation** needs clearing up. Definitions given: **rights = inherent** (the combination that makes a valid peer; remove them and the system degrades); **capabilities = intrinsic** (radio, push token, cores/RAM — differ per peer, never confer governance dominance); **roles = delegated** (e.g. the Creator role, de-facto then reassignable by governance vote; can be **mutually exclusive**, one instance); **PeerSet = a named bundle of roles + capabilities with a functional expectation** (e.g. the meer/blind-broker), packaged so governance can reason about it. Mutually-exclusive roles → a **restricted combination** → don't silently forbid; **fail-noisy** (group locks / notifies / pauses, offer to fork excluding the offending voters) → human adjudication. The meer decrypt-vs-blind question should be settled in terms of **MLS capabilities**. | **NEW THREAD T31**; sharpens T21, informs T20 + T3. |
| T4 — brand/voice chapter | Wants to **start** the brand/messaging docs folder now (see M6). | Note appended (start-now directive). **Type: needs-content.** |
| T5 — protocol behavior at scale | Flows in with T2 — **build the test harness**, conformance + performance + quality test cases. | Note appended; **Type: needs-experimentation**. |
| T6 — per-platform trust-model doc | Still **deferred**; subsumed by the per-platform design-files directive (M8). | Note appended; **Type: needs-content**. Couples T14. |
| T7 — atproto private-data watch-item | Still **deferred forward** (external dependency). | **Type: needs-research** (external watch). |
| T8 — forward-only revocation | "Define the reversible-vs-committing decision tag; spec the permanent attribution record; reconcile with T1's append-only fold" — restated as the work. | **Type: needs-content**. |
| T9 — publication-readiness verification | Still need to get the raw material / primary-source pass. | **Type: needs-research**. |
| T10 — real-time media hardening | Still deferred for now. | **Type: needs-experimentation**. |
| T11 — adoption-chasm / institutional mandate | Twin doc to brand (see M7); accrete "ammo" over time. | Note appended (twin-doc directive). **Type: needs-content**. |
| T12 — consumer-pull inversion (M3) + M0–M4 | "I really just need you to **walk me through that** — I thought that was resolved." | Note appended (user requests a walk-through). **Type: needs-content** (explanation). |
| T13 — encrypt-then-content-address dedup | Same image → 15 per-user ciphertext copies is bad operationally. Possible directions to think through (not decided): treat a **group as a principal** and encrypt the asset once as a group blob → dedup at group scale; or a **key-envelope** scheme. Needs research — how do **Proton** (encrypted file store) and **peergos** do it. | Note appended (directions captured, undecided). **Type: needs-research**. |
| T14 — iOS opportunistic-only P2P | Don't know why it's gated; iOS app must play to iOS strengths (event-driven, system pushes, sync). Promote into the **per-platform design files** (M8) rather than a one-off thread. | Note appended; **Type: needs-content**. Couples T6. |
| T15 — P2P-games data layer | Game still called a **pond** (candidate). Wants a **games design doc/PRD** (chat gets its own too), plus the new **"thinking of you"** P2P use case as a modest starter PRD (see M9). | Note appended (PRD directive + new use case). **Type: needs-content**. |
| T16 — Matrix E2EE operational lessons | (No specific note beyond "more plain explanation / meat on the bone.") | **Type: needs-content**. |
| T17 — three-audiences settings | Considered **fairly settled** as a design principle that impacts the per-platform design specs; question is just "what to do with it." | Note appended (settled-stance). **Type: needs-content** (lands in design specs). |
| T18 — LTS-for-interfaces | Considered **pretty much settled** as a design ethos impacting all platform implementations; "more about what it looks like." | Note appended (settled-stance). **Type: needs-content**. |
| T19 — blind-peer encrypted search | Doesn't want to get into it; "seems like a losing game." Maybe **some experimentation** to see if there's utility, but low appetite. | Note appended (low appetite). **Type: needs-experimentation** (optional/deferred). |
| T20 — conflict-reason gaps C4/C7/C8/C9/C10 | Too terse — needs plain-language **problem statement + the solutions we already preferred** when we discussed it. | Note appended (needs expansion). **Type: needs-content**. |
| T21 — is `share` a right or a capability? | This is the **capability-vs-role confusion** again (see T31). Also: can we **define "share"**? The user is unsure "share" frames it right — peers have a right, but **you can't have a right to someone else's resources**. The real question is whether peers have a **right to communicate** with others and what the **boundary** is that still respects exit/fork. "There's something there; needs more thinking." | Note appended (reframe toward "right to communicate / boundary"). **Type: needs-content**. Couples T31. |
| T22 — survivor re-key vs `tenure` | **tenure** may be the user's version of the "right to share" — tenure = the ability to functionally be a peer to other peers. This needs **testing** — "we should just do that." | Note appended (tenure reframing + run the test). **Type: needs-proving**. |
| T23 — Beer/Ashby grounding quotes | CLOSED — confusing to have closed mixed in. | **MOVED** → Promoted & closed section. |
| T24 — what grounds peer authority | Need plain-language problem statement + the solutions already considered. Discussed in several places incl. Drystone spec Part 1: a peer's authority is **local by nature** (it has local state no one else has, corroborated by others). "What makes a right cost something to violate" = the **integrity of the whole system depends on each peer holding rights to preserve variety**, which preserves longevity/homeostasis — strip peers of rights and the broader social graph collapses. Maybe needs more research / "what are we missing." | Note appended (user's framing captured). **Type: needs-research**. |
| T25 — redb storage/projection layer | Explicitly a **third state** — *in progress*. | **Type: needs-proving**; state **in-progress** (already). |
| T26 — social-graph-as-substrate reframe | (PROMOTED already.) | **MOVED** → Promoted section. |
| T27 — "evidentiary, not operational" principle | (A curation decision; no new note.) | **Type: needs-content** (curation). |
| T28 — historical peer-rights home | Filed in alpha (pulled out of beta-01). But "the **history of peer rights and distributed social systems** is itself a topic" — maybe its **own theme**. | Note appended (leans toward its own theme). **Type: needs-content**. |
| T29 — MLS ↔ governance-log consistency | "A lot going on; unsure what we need — more experiments? Just nail the spec / data shape / typing?" | Note appended (unsure — needs scoping). **Type: needs-experimentation**. |
| T30 — mature Drystone spec to publication | Would rather **purge the meta framing** and make it concrete: T30 is **publish the final spec + get the DOI**. There are really **two open threads** here: one **legal review** (attorney review), one **publish**. | **SPLIT** — T30 stays the **publish** thread (**Type: publish**); new **T32** = the **legal-review** thread (**Type: legal-review**). |

---

## New threads created

- **T31 — Disentangle rights / roles / capabilities / delegation (+ PeerSet, mutually-exclusive roles,
  fail-noisy restricted combinations).** Captures the recurring conceptual cleanup with the user's
  definitions as the proposed resolution; promotion target `drystone-spec` Part 2 §5; sharpens T21,
  informs T20 + T3. Type: needs-content.
- **T32 — Attorney legal review of the Drystone defensive-publication (patent-non-assertion ¶).** Split
  out of T30 so the legal-review work is its own typed thread. References (does not duplicate) the
  README cooperative legal-review gate. Type: legal-review.

## Surfaced structural decisions (registered in OPEN-THREADS.md, not executed)

S1 (M5) per-thread directory split · S2 (M4) per-thread Problem/Proposed/Indeterminate expansion ·
S3 (M6+M7) brand/voice + adoption-enablement twin docs, start now · S4 (M8) per-platform design files ·
S5 (M9) per-app PRDs incl. the new "thinking of you" use case · S6 (M10) a by-type grouped index view.

## What this pass did NOT do (and why)

- Did not author the brand/adoption/per-platform/per-app docs (net-new content — a separate session;
  registered as directives S3–S5).
- Did not split the ledger into a directory (large reorg awaiting the user's greenlight — S1).
- Did not resolve any decision gate (Track A/B, key-custody, geer/Noria names, MPL/AGPL, recovery
  anchor, cooperative legal review, CroftC IP, genome-vs-strategy).
- Did not edit settled beta narrative or the spec (the T31 definitions are captured as the *direction*
  for a later spec-side promotion, not folded into the spec here).
