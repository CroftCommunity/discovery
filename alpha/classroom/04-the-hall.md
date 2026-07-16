# Chapter 4 — Twenty-one: the hall

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: you can't hear everyone anymore.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: a town-hall-sized group. Side conversations carry the news; a latecomer catches up
> from whoever's nearest; the group keeps periodic minutes so catching up never means re-reading
> a year.
> Real-world: how news actually moves through a large room; meeting minutes; "what did I miss?"

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation.*

## PRECISE STATEMENT

> Arc beat (mechanism): swarm fan-out (epidemic delivery), catch-up on connect, steady-state
> repair (a dropped remark gets refilled without anyone reconnecting), horizons and checkpoints
> (minutes = corroborated "we all folded to the same page here").

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

Every claim this chapter makes ends in something you can run:

- **Fan-out cost is measured, not guessed: live_sent = 2N+1 per commit, exactly, 150/150 runs.**
  `alpha/experiments/croft-chat/FANOUT-M1.md` (RUN-01 EXP-1 + the RUN-09 K=5 repeated-run
  addendum) — evidence map row §11.11 #1 / §11.4–11.5, `beta/drystone-spec/EVIDENCE-MAP.md`.
- **A frame lost between connected peers is detected and repaired without a reconnect.**
  `steady_state_anti_entropy.rs` (RUN-09) and `partitioned_anti_entropy.rs` (RUN-12, diff-only
  repair in O(log)-ish rounds) — evidence map rows §6.8.1.
- **The minutes are deterministic: the horizon manifest is byte-identical across members and
  orders.** `horizon_manifest.rs` (RUN-07 EXP-H1) — evidence map row §7.6.9.
- Run it: `cd alpha/experiments/croft-chat && cargo test -p croft-chat --test steady_state_anti_entropy --test partitioned_anti_entropy --test horizon_manifest`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
