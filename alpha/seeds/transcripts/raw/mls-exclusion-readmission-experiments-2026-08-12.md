# MLS exclusion, readmission, and the epoch-roll model — experiment session

`Source: Claude Code live session (CroftC workspace), 2026-08-12 → 2026-08-16.`
`Status: **preserved-condensed** — user turns preserved VERBATIM (voice-dictation artifacts kept`
`as-is); assistant turns preserved content-faithful, NOT byte-pristine (verdicts, tables and`
`corrections retained, connective prose tightened); tool invocations summarized as bracketed notes.`
`Per PLAYBOOK §4. No credentials in session.`
`Row: RAW-ARTIFACTS-MANIFEST.md. Distilled → beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md`

**Tags:** Drystone-delivery · Drystone-spec §11 · meer-queue · MLS

> **This file is a provenance record and is NOT the findings document.** The findings, with their
> fidelity rungs and their consequences for the spec, live in the dossier. The authoritative
> evidence is the code: `alpha/experiments/meer-queue/tests/s15_*.rs` … `s20_*.rs`, and the verdicts
> printed by those tests. **Where this narrative and the code disagree, the code is right.**

---

## What the session was

A single continuing session that began as "run the three remaining experiments" from
`alpha/experiments/meer-queue/STATE-AND-NEXT.md` and turned into an interrogation of whether the
delivery design's model of exclusion matches what MLS actually does.

It produced **six new scenario files (S15–S20, 22 tests)**, one new production module
(`src/outer_seal.rs`), one production seam (`Meer::sweep_with_retention`), and **five corrections —
three to our own prior conclusions, one to a normative claim in the spec, and one to a test that was
measuring nothing.**

Test count over the session: **53 → 75**. `cargo clippy --all-targets` clean throughout.

---

## The arc, in order

### 1. The three parked experiments (S15, S16, S17)

`STATE-AND-NEXT.md` listed three experiments "that would still teach something". All three ran.

- **S15 — the limbo state, walked.** Previously asserted as a policy comparison between two
  constants. Walked at Rung A it is a reachable state with three simultaneous properties. **And it
  corrected S14:** limbo is *escapable* — the library does not distinguish "cold" from "stranded".
- **S16 — the governance attestation.** Both halves of §11.7's two-part credential fail. One
  assumption in the test itself was **refuted mid-write** (see §3 below).
- **S17 — nested sealing (E96).** Built and measured. 28 flat bytes; nothing else affected.

### 2. The owner's question that redirected the session

> **Owner:** *"Hold on, so does 'walk back in' just mean that they can still send an encrypted
> message from a prior lineage to these people or does it mean that they can encrypt at the current
> epoch. I'm not even past number one yet."*
>
> *"and I want you to go look through the latest drystone spec so we can ground our next reasoning
> on where the spec needs to change"*

This was the turn that changed the session's character. Measured answer: **the strong reading.** A
re-seated member is a full member at the current epoch — she sealed a message a member read in
cleartext, and read a member's message sealed after her return. She recovers **no** history.

Reading the spec then **reversed part of the session's own framing** (see §3).

### 3. The owner's methodological challenge, which found a real weakness

> **Owner:** *"I'm wondering when an epoch roll happens — how is the excluded user getting the group
> shared key materials? In our 'ban' case wouldn't they be EXCLUDED because it's basically an MLS
> group fork where they are not included in the next distribution of group key material, I worry our
> testing right now is 'roll epoch and equally include all existing users including the ban prospect'
> which would def fail the test, we need a way to say 10 ppl, 1 is banned by legit group governance,
> epoch roll, only include non-banned folks in new group"*

The concern was correct to raise. S19 had used a **two-person** group, which cannot distinguish "the
removal excluded her" from "the group is now one person and trivially disagrees with her". **S20 was
written to answer it at N = 10** and confirmed the exclusion is genuine: nine survivors agree, the
banned member derives none of the new key material.

S20 also surfaced a third post-ban state nobody had asked about (`UseAfterEviction`).

### 4. The owner's framing that was confirmed mechanically

> **Owner:** *"being in the group is a multi-tiered constraint and I think who you key your responses
> for is the truest sense of your group and you can't be forced to do it, but aligning that to the UX
> is the key part"*

Measured true at two independent layers (keys and addressing), neither needing the meer's
cooperation nor the returner's. **And the UX half is the harder half**, because S18 measured that
disagreement forks the group *invisibly* — two branches agree on the epoch number.

### 5. The owner's social framing, recorded as a design requirement

> **Owner:** *"think in terms of real social groups, like they fork, they splinter, they regroup,
> they separate… So someone being cut off or excluded, it may just be they passed and people don't
> want the reminder. It may be a divorce, like plenty of social situations where we have to account
> for that honestly."*

> **Owner:** *"a ban is basically a forced fork where the banned is not included in a new group
> object, that's the goal at a base layer, then we can talk about how to gracefully handle
> unintentional exclusions like missing for longer than is functional or other mishaps, which could
> be a set of rules for 'easy' re-entry which the 'cold' and 'warm' groups in the spec were meant to
> speak to, but feels like needs real rework and grounding"*

**Recorded as the reason the rework exists.** (Correction made in-session: the spec has **hot and
cold only** — there is no "warm" tier. The owner acknowledged this as a miscall.)

> **Owner (on retention):** *"resource constraints are real so maybe we do 30 days but there's gotta
> be a limit. Right."*

> **Owner (on not evicting to preserve easy re-entry):** *"on 1 hot tree size is exactly the thing
> being managed here for the most part in terms of hot<>cold so that's not great"*

**This closes one of the two open decisions the plan had raised** — not evicting is not an available
answer, because hot-tree size is precisely what the hot/cold split exists to manage.

### 6. The owner's mechanism idea, which resolved a filed gap

> **Owner:** *"I'm kinda thinking on like whether the meer is just a function on top of the history
> convergence server which already runs on the CISS storage server… because really it's a CISS client
> that presents a view of cryptographically related data… you could kind of pay for your own
> longer-lived queue if we just sort of marry the two concepts first class… if it's the same person
> who runs your mirror and your history convergence server, like there's a natural continuity there
> that we should design for on purpose."*

Reading the spec confirmed this matches §11.6/§11.11's own framing, and it **retired the session's
own "nothing serves `GroupInfo`" finding** from *missing capability* to *unwired seam*.

---

## Corrections made during the session

Recorded because the corpus's methodology treats a refuted assumption as a result, not an
embarrassment.

| # | What was wrong | How it surfaced | Where it landed |
|---|---|---|---|
| 1 | **S14's "neither mechanism applies"** — that a stranded-but-live member has no recovery path | S15 measured external commit working for her | S14's log entry marked SUPERSEDED IN PART; S14's own printed verdict corrected in code |
| 2 | **S16's PSK arm assumed "works but optional"** | The test failed with `KeyNotFound`; source inspection showed the store is initialised empty and `add` is `pub(crate)` | Rewritten to measure the blocker; a fourth test added to *measure* the external-PSK workaround rather than assert it |
| 3 | **The assistant told the owner "the spec needs to move" on standing** | Reading §11.8 showed the spec **already says** MLS cannot enforce standing | Corrected to the owner in-session; S16 reclassified from *correction* to *confirmation at Rung A* |
| 4 | **E105 "nothing serves `GroupInfo`"** | §11.11 item 6 names the history-convergence node | Row narrowed to "unwired seam, not missing capability" |
| 5 | **S18's ratchet-tree test measured nothing** | It turned the extension off in the *group config* then exported the `GroupInfo` with `with_ratchet_tree: true`, handing the joiner the tree anyway | Rewritten with both arms; the flag is per-call and independent of group config — which became a finding in its own right |

**Two measurement-grade upgrades** were also made, both for the same reason: a refusal that surfaces
as an *epoch/ordering* check proves less than one that surfaces as a *decryption* failure (the S7
lesson). Where possible the excluded party's own branch was advanced first so both sides sat at the
**same epoch number**, converting a counter check into a key check. Where that is impossible by
construction (pre-re-entry history), it is **said so explicitly** rather than overclaimed.

---

## What the session produced

**Code** (`alpha/experiments/meer-queue/`):

```
src/outer_seal.rs                      E96's nested seal — real ciphersuite AEAD, real exporter key
src/meer.rs                            + sweep_with_retention() — retention as a Group value
tests/s15_limbo_walked.rs              3 tests
tests/s16_governance_attestation.rs    4 tests
tests/s17_nested_sealing.rs            5 tests
tests/s18_removal_durability.rs        4 tests
tests/s19_what_an_epoch_roll_does.rs   3 tests
tests/s20_governance_removal_at_scale.rs  3 tests
```

**Documents:**

- `TEST-LOG.md` — S15–S20 entries with rungs; S14 marked superseded-in-part
- `alpha/plans/2026-08-14-1-plan-readmission-and-groupinfo.md`
- `alpha/ROADMAP_TODO.md` — **E105, E106, E107** added; **E96** updated to built-and-measured
- `alpha/thinking/meer-two-target-delivery.md` — amended and corrected
- `beta/drystone-spec/part-2-certifiable-design-WORKING-2026-08-16.md` — working fork
- `beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md` — **the findings document**

**Not done, deliberately:** the canonical `part-2-certifiable-design.md` was **not edited**.
Normative changes were surfaced for the owner rather than applied as a side effect.

---

## Open at session end

- The two spec corrections (§11.8's exposure claim, Appendix E L6) are **identified and drafted, not
  applied.**
- The hot/cold rework is **scoped, not written** — with one of its two open decisions now closed by
  the owner (evicting must stay).
- `[UNVERIFIED]` and untested: whether a **CISS assertion-plane payload size limit** exists (searched,
  none found) — bears on the inbox-plane decision, which remains open.
