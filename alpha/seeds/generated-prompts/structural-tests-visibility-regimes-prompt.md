# Add structural tests: visibility regimes and propagation geometry

source: generated 2026-06-14 during the design dialogue; preserved verbatim as a seed

status: not yet run — this is a prompt to paste into the Claude Code experiment session

intent: add structural test coverage (V1–V9) for the social layer's two-regime model and propagation geometry, before any UI is built on them

---

## Context

I'm extending the lineage-groups experiment suite (see experiments/lineage-groups/THESIS.md, MULTI_DEVICE.md, and SOCIAL_LAYER.md). The protocol already models groups as a lineage DAG with a governance tree (signed forward-only op log, immutable genesis thresholds) bound to an MLS epoch chain, plus consensual backfill for history reconciliation.

I'm now adding a social layer that rides on top, and it introduces two new structural concepts that need test coverage before I build UI on them, because if they're not enforceable in the data model they're not safe. Read SOCIAL_LAYER.md first for the full framing. The two concepts:

Visibility regimes. A group is born into one of two regimes, fixed at genesis and immutable: intimate (default-private, quiet membership, visibility is opt-in exposure) or public (visible-to-co-members, affiliation is the point). Content is born into the regime of the group it's authored in and cannot silently change regime. Crossing from intimate to public is never a forward/move — it must be a distinct, newly-authored republish act in the public regime that may reference but never carry the private original.

Propagation geometry. Two separate, independent genesis parameters, not one:

- outward_propagation_depth — how many hops a member's affiliation with this group can propagate through them to others. This is the deanonymization control. It must scale inversely with group openness: intimate groups may allow a few hops, public groups collapse toward one, and a fully-open/large public group is a hard 0 — a visibility sink, connections terminate at it and do not flow through any member.

- inward_visibility — how much of the group's own member/structure a member can see looking in. For a public board this can be permissive (seeing co-members is desirable) even while outward propagation is zero. These are different risks and must be controllable separately.

The core safety principle: these limits must be structurally unrepresentable to violate, not runtime-warned. A share exceeding declared depth should be invalid and rejected by every verifying client the same way an under-threshold governance op is rejected — enforced by everyone's verification, never by the sharer's client behaving well. The safety case is exactly the one where the sharer's client is hostile or sloppy.

## What to build

Add a test module (sim-only, no transport — consistent with Phases 0–2) covering the invariants below. Where the current data model can't yet express a concept, add the minimal genesis-parameter fields needed, keep them immutable post-genesis (reuse the I1 genesis-immutability machinery), and note any model change in a short SOCIAL_LAYER_FINDINGS.md.

## Invariants to assert

V1 — Regime is born-in and immutable. A group's regime is set at genesis; any governance op attempting to change it is rejected by all honest clients (reuse I1).

V2 — Content carries its origin regime. Every authored item is tagged with the regime of its authoring group, and that tag is part of the signed data, not client-side metadata.

V3 — No silent regime crossing. There is no operation that moves or forwards intimate-origin content into a public group. Assert this is structurally absent — the operation cannot be constructed — not merely blocked at runtime.

V4 — Republish is a distinct authored act. Crossing the boundary is only possible as a new public-regime item that references the original by handle/hash and does not embed the original's plaintext or carry its membership/edge data. Assert a republish exposes only what the author explicitly included, and that the reference alone does not let a public-side reader enumerate the intimate group.

V5 — Outward propagation depth is enforced by verification. A share/affiliation-propagation that exceeds the group's outward_propagation_depth is invalid and rejected by a verifying client that did not author it. Assert with an adversarial author whose client emits an over-depth share anyway.

V6 — Openness caps depth. Assert the constraint that outward_propagation_depth cannot be set, at genesis, above the maximum permitted for the group's declared openness/size class, and that a fully-open class forces depth 0 (hard sink). An over-permissive genesis is itself invalid.

V7 — Inward and outward are independent. Construct a public group with permissive inward_visibility and outward_propagation_depth = 0; assert a member can enumerate co-members (inward) while no member's other edges propagate outward through the group (outward). Then the inverse case. Assert neither parameter's setting forces the other.

V8 — Public membership leaks only membership. A member joining a large public group exposes the fact of that affiliation and nothing else about their graph. Assert that from the public group's vantage, no member's intimate-group edges, labels, or other affiliations are derivable (combines V5/V6/V7 into the end-to-end deanonymization-resistance check).

V9 — Freeze-by-default holds across regimes. Assert that nothing from a public group enters a user's intimate view without an explicit pull, and that joining/seeing a public board never auto-adds edges to the user's own graph (reuse / extend S1).

## Adversarial emphasis

V5 and V8 are the ones that matter most — they're the deanonymization defenses. Write them with a deliberately hostile author/client that tries to over-share, and assert the receiving verifier rejects it, since that's the only enforcement that holds when the sharer is the adversary. A green V5/V8 under a hostile sender is the gate for this batch.

## Output

- The test module, named to match (e.g. v1_regime_immutable … v9_freeze_across_regimes).

- Any minimal genesis-field additions, kept immutable, noted in SOCIAL_LAYER_FINDINGS.md with: which invariants hold, which forced a data-model change, and anything that turned out structurally hard to make unrepresentable-rather-than-warned (that's the highest-value finding — if V3 or V5 can only be runtime-blocked and not structurally prevented, say so loudly).

Keep it sim-only and deterministic, consistent with the existing harness. Don't wire in transport. Don't build UI.

---

## Open parameter (deferred from the dialogue)

The openness/size classes in V6 are deliberately undefined — where "large" starts, how many classes there are, the depth cap per class. Either define these before running, or let the session propose a scheme to react to.
