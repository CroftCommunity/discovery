# Chapter 7 — The co-op

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: the group wants its infrastructure to answer to it.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: the club and its neighbors charter a co-op to run their relays and storage. Members
> own the operator. And here the arc folds onto itself: the co-op's own bylaws — dues, board
> quorum, changing the change-rule — are the *same machinery from Chapter 3*, because a co-op is
> just another group.
> Real-world: rural electric co-ops, food co-ops, community mesh networks.

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation. Production note (arc):
institutional chapters diagram the planes (group plane vs operator plane; the absent arrow drawn
struck-through — the absence is the figure).*

## PRECISE STATEMENT

> Arc beat (mechanism): nothing new — that is the lesson. Governance of the infrastructure reuses
> R7 verbatim; the cooperative-ownership companion argument grounds why this pairing is natural.

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

The proof here is the **reuse itself** — the co-op's rule changes are RuleChange facts through
exactly the tests Chapter 3 already ran:

- **Same machinery, verbatim.** `rulechange_quorum_via_api.rs` (k-of-n with content-bound
  approval, end to end) and `competing_quorums.rs` (two honest quorums → the same banner, both
  orders) — evidence map rows §7.2 R7 and §7.3.2, `beta/drystone-spec/EVIDENCE-MAP.md`. No
  co-op-specific test exists because no co-op-specific mechanism exists.
- Run it: `cd alpha/experiments/croft-chat && cargo test -p croft-chat --test rulechange_quorum_via_api --test competing_quorums`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
