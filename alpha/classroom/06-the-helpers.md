# Chapter 6 — The helpers

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: somebody runs the relays; somebody stores your encrypted history while your phone is
> dead; somebody answers a page at 3 a.m.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: enumerate the helper cast — relay operators, blob/storage providers, app vendors,
> recovery share-holders — and give the group *several* of each, interchangeably.
> Real-world: the phone company carries the call without joining the marriage; your email host
> doesn't get a vote in your family.

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation. Production note (arc):
diagram the planes — group plane vs operator plane; the arrows that exist and the arrows that
provably don't, with the absent arrow drawn struck-through. The absence is the figure.*

## PRECISE STATEMENT

> Arc beat (mechanism): the operator plane and plane separation. Trust is a gradient with a
> direction: "trust *for what*?" You trust the relay to carry, never to read; the store to hold
> ciphertext, never to open it; and NO helper holds a path into the group's social authority —
> operator and member-authority are mutually exclusive roles by construction. Interchangeability
> follows: because helpers live entirely on the provenance plane's outside, swapping them changes
> nothing the group can see in its own history.

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

This chapter's proofs are mostly **proofs of absence** — said plainly, because *absence proofs
are proofs*:

- **The store can't read what it holds.** §5.11 read-scoped content keys (how a read Role is
  cryptographically enforced) — the sealed frames the L2a suite round-trips are ciphertext to
  every non-member, including the operator (`l2a_sealed_frame.rs` assertion 1).
- **No helper holds a path into the group's authority.** `l2a_sealed_frame.rs` assertion 6
  (`firewall_guard_no_authority_or_projection_api`: the public API exposes no authority or
  projection knob) — evidence map row §10.2 / §10.5 / §7.6.2 (croft-group L2a),
  `beta/drystone-spec/EVIDENCE-MAP.md`.
- **A helper-fed lie is attributable, not silent.** §7.2 R6 (attributable acceptance: acceptance
  records the synced governance frontier, so a later-synced revocation makes a stale acceptance
  detectable and attributable).
- Run it: `cd alpha/experiments/croft-group/crates/group-seal && cargo test --test l2a_sealed_frame`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
