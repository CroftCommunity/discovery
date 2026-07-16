# Chapter 8 — The nonprofit

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: groups that can't pay; neutrality; and someone to hold a recovery share.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: a foundation runs free relays for shelters, schools, a union local. It also serves as
> one of the *independent trust domains* holding a share of a member's sealed recovery payload —
> the library that keeps a copy of your spare key without the power to use it alone.
> Real-world: public libraries, Signal-style foundations, community bail funds.

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation. Production note (arc):
institutional chapters diagram the planes (group plane vs operator plane; the absent arrow drawn
struck-through — the absence is the figure).*

## PRECISE STATEMENT

> Arc beat (mechanism): same operator plane, different funding — and the protocol *cannot see the
> difference*: funding model is a utility-layer fact. Recovery's two tiers: the lock (threshold
> shares across independent domains, built) vs the trust predicate (who may trigger release — the
> open human question, taught as open).

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

Every claim this chapter makes ends in something you can run — and the field's largest open call,
taught by name:

- **The lock is built and it holds.** `alpha/experiments/bip39-recovery-roundtrip` (RUN-08, 11
  tests: recoveryKey ⇄ 24-word mnemonic round-trips bit-exact; a corrupted word, a transposed
  pair, a wrong count are each rejected, never silently accepted; the wrapped masterKey unwraps
  only under the right key) — evidence map row §7.3.9 (Tier-1 lock, `Verified`,
  experiment-grade), `beta/drystone-spec/EVIDENCE-MAP.md`.
- **The trust predicate is open, and the classroom says so.** Who may trigger release is the I9
  trust tier — the open human question this design refuses to paper over. Taught as open.
- Run it: `cd alpha/experiments/bip39-recovery-roundtrip && cargo test`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
