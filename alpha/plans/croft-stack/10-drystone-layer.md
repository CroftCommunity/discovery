# Phase 10 — Remaining drystone-layer (tracked, later)

← [09-stellin-index.md](09-stellin-index.md) · [roadmap](README.md)

**Status:** SCAFFOLD (tracked, later) · **Depends-on:** product need (croft-groups) / drystone's
fold+MLS becoming real (convergence server) · **Gate-out:** each lands as an independently-deployable,
governed mini-stack with its serverless floor intact.

---

## Problem

Two drystone-layer components round out the estate but are not near-term: **croft-groups** (roster-gated
large-group serving) and the **MLS history-convergence server** (content-blind meer). Both follow the
same optional / serverless-floor ethos; both are gated — one on product need, one on a dependency.

## croft-groups — an AppView variant (factoring open)

Roster-gated serving **is an AppView**: the cache/index engine with a different membership/write policy
(the open/gated/sealed tiers are already framed as *policy on one engine, not separate servers* —
RUN-16/17/18/19). It deploys as its own **isolated mini-stack instance** (own user/data/ports/blast
radius) regardless; the **code factoring is open** (Open decision 13) — a further `--mode`, a shared-lib
sibling binary, or a separate build. *(lean: mode-or-shared-libs, given the tier-as-policy model.)*
**Decide the factoring when we scope the group product concretely — not before.**

- Deployment axis (settled): own isolated mini-stack instance.
- Code axis (open): mode / shared-lib sibling / separate build → Open decision 13.
- Trigger: product need for large-group serving (nothing live needs it yet).

## MLS history-convergence server — the content-blind meer

A content-blind "meer"/blind-mirror that helps MLS-group peers converge their append-only history
without reading content (relay lab E8/E9 shape). **Gated on drystone's fold/MLS becoming real** — there
is nothing to converge until a real MLS group produces real history (the convergence briefs are still
reference-model: "no production fold exists in croftc/upstream-repo"). Last by dependency, not choice.
Serverless floor: P2P convergence over iroh.

## TODO (decide on arrival)
- [ ] Open decision 13: croft-groups code factoring — resolve at concrete group-product scoping.
- [ ] Convergence server: unblock only when drystone's fold/MLS is real; then design the blind-mirror
      confidentiality tier (relay lab E9).
- [ ] Whether croft-groups shares the auth helper / own-data API addendum.

## Risks & cautions
- Do not pre-build the convergence server against a reference-model fold — it would encode
  assumptions the real MLS/fold may break.
- croft-groups' "separate mini-stack instance" (deployment) must not be confused with "separate build"
  (code) — keep the two axes distinct (Open decision 13).

## Validation
Each: independently deployable + removable, governed, serverless floor intact (group P2P / open-tier
on-ramp works without it).

## References
`alpha/experiments/appview-infra/GROUPS.md` A.10 (croft-groups); `beta/impl/drystone-design/` +
`beta/impl/experiments/drystone-convergence-experiment-brief-v3.md` (fold/convergence); relay lab
E8/E9 (relay-vs-meer, blind-mirror); RUN-16/17/18/19 (tier-as-policy).
