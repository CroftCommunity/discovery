# Chapter 2 — Three people: the witness

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: hearsay, handled honestly.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: Carol joins. Carol hears from Bob what Alice said. Carol also sees Alice and Bob speak
> at the same moment — which came first? Neither.
> Real-world: gossip; "Bob told me you said…"; two people talking over each other and the group
> still agreeing on what was said, if not on the interleaving.

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation.*

## PRECISE STATEMENT

> Arc beat (mechanism): assertion vs corroboration; quantified trust (a claim is as strong as its
> independent attestations); first genuine concurrency, and why the notebooks still converge.

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

Every claim this chapter makes ends in something you can run:

- **Concurrent facts, any arrival order — the three notebooks still converge.** `convergence.rs`
  (order-independence) — evidence map rows §4.1–4.6 and §7.3.2,
  `beta/drystone-spec/EVIDENCE-MAP.md`.
- **Corroboration is counted, deterministically, from independent attestations.**
  `horizon_checkpoint.rs` (corroboration count identical across members and fold orders; a
  non-matching fold does not falsely corroborate — RUN-12 EXP-H2) — evidence map row
  §7.6.9 / §7.3.3.
- Run it: `cd alpha/experiments/croft-chat && cargo test -p croft-chat --test convergence --test horizon_checkpoint`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
