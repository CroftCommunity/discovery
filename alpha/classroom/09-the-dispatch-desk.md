# Chapter 9 — The for-profit and the dispatch desk

`Classroom tier — chapter skeleton (scaffold landed RUN-13; beats from alpha/classroom/00-arc.md).
Prose bodies: DRAFT-PENDING (written in conversation, not by runs).`

## NEED

> Arc beat: paid guarantees. A county emergency-services department runs its dispatch channels on
> the protocol and pays a vendor a premium for 24/7 operations, redundant relays, audits, and
> response times.

DRAFT-PENDING (written in conversation, not by runs).

## STORY

> Arc beat: the vendor sells uptime and pager duty. What the vendor *cannot* sell — at any price —
> is a supervisor knob: who is on dispatch tonight is the department's own k-of-n, and when a
> responder's access is revoked, their device fails closed even if the vendor's server is behind.
> The department, being prudent, ALSO peers with the co-op's relays: two providers, one group, and
> the group's history can't tell which carried which frame.
> Real-world: county EMS/fire dispatch, its vendor contracts and audits; badge revocation that
> actually sticks.

DRAFT-PENDING (written in conversation, not by runs).

## DIAGRAM

*Placeholder — drawn when this chapter's prose is drafted in conversation. Production note (arc):
institutional chapters diagram the planes (group plane vs operator plane; the absent arrow drawn
struck-through — the absence is the figure).*

## PRECISE STATEMENT

> Arc beat (mechanism): an SLA is a contract about delivery and availability, never about content
> or membership; the corroboration dials as *operational* settings (dispatch dials freshness k
> tight; delay-over-breach reads as "a stale badge doesn't open the door"); multi-provider
> redundancy as a consequence of plane separation; price discrimination lives entirely on the
> utility plane.

DRAFT-PENDING (written in conversation, not by runs).

## PROVE-IT

Every claim this chapter makes ends in something you can run:

- **A stale badge doesn't open the door: enforcement stalls below corroboration; reads continue.**
  `completeness_ahead.rs` — `stall_at_threshold_no_breach_reads_unaffected` (EXP-C1, RUN-07) —
  evidence map row §7.3.3 / §7.4.1, `beta/drystone-spec/EVIDENCE-MAP.md`.
- **Revocation actually sticks: the removed reader is re-keyed out (PCS), vendor server or no
  vendor server.** `l2a_sealed_frame.rs` assertion 4
  (`governed_removal_re_keys_the_departed_reader_out`) — evidence map row §10.2 / §10.5 / §7.6.2
  (croft-group L2a).
- **"Majority of on-shift supervisors" folds identically everywhere: k can be a formula over
  folded state.** `completeness_ahead.rs` — `formula_valued_k_identical_across_orders` (EXP-C1) —
  same evidence map row.
- Run it: `cd alpha/experiments/croft-chat && cargo test -p croft-chat --test completeness_ahead`
  and `cd alpha/experiments/croft-group/crates/group-seal && cargo test --test l2a_sealed_frame`

## REFRAIN

*"And underneath, nothing changed: it is still two people keeping their own memory of what was
said, signing it, and pointing at each other's words."*
