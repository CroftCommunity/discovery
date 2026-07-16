# Chapter 3 — Seven people: the club

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: decisions that stick, with no boss.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: a book club. Dues change needs 3 of 7. The treasurer role. Someone is asked to leave —
> and can no longer read what's said after the door closes. Two motions pass simultaneously in
> opposite corners; the room stops and looks at both.
> Real-world: committee votes; minutes recording exactly what was moved; bait-and-switch on a
> motion's text failing because approvals name the text.

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation.*

## PRECISE STATEMENT

> Arc beat (mechanism): rules and roles as facts; k-of-n with the content-bound approval (you
> approve *these exact words*); the co-signed op; prior-rule-governs (you can't lower the gate
> with the act being gated); removal → ceiling → re-key (PCS); the contradiction banner: two
> honest quorums are a fact to show, never a race to win.

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

Every claim this chapter makes ends in something you can run:

- **A rule change needs its quorum, and each approval names the exact words.**
  `rulechange_quorum_via_api.rs` (end to end through the real Session API) — evidence map row
  §7.2 R7 (`Verified`, count), `beta/drystone-spec/EVIDENCE-MAP.md`.
- **Two honest quorums at once: both fold orders show the same banner; nobody's version wins.**
  `competing_quorums.rs` (contradiction byte-head identical across orders) — evidence map rows
  §7.2 (R7 residual) and §7.3.2.
- **The removed reader really can't read.** `l2a_sealed_frame.rs` assertion 4
  (`governed_removal_re_keys_the_departed_reader_out`, PCS on real openmls 0.8.1) — evidence map
  row §10.2 / §10.5 / §7.6.2 (croft-group L2a).
- Run it: `cd alpha/experiments/croft-chat && cargo test -p croft-chat --test rulechange_quorum_via_api --test competing_quorums`
  and `cd alpha/experiments/croft-group/crates/group-seal && cargo test --test l2a_sealed_frame`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
