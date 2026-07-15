# Social Layer Findings

Experiment group V — visibility regimes and propagation geometry.
All 9 invariants pass; findings below document data-model changes and
the structural-vs-runtime distinction for each.

---

## Data model changes

One file modified: `core/dag.ts` — the `genesis` payload union member gained
four optional fields:

```typescript
regime?: 'intimate' | 'public'
outward_propagation_depth?: number
inward_visibility?: 'full' | 'partial' | 'none'
openness_class?: 'closed' | 'open' | 'fully_open'
```

All four are optional so existing tests are unaffected.  Because they are
part of the genesis payload they are covered by the SHA-256 hash and are
therefore immutable once the genesis node is committed to the DAG — the same
guarantee that makes governance thresholds immutable.

Two new standalone types live in `core/visibility.ts` and are **not** DAG
nodes: `SocialContentItem` (content with a signed regime tag) and
`PropagationShare` / `RepublishAct` (social-layer objects with their own
computed IDs).  These ride on top of the DAG rather than inside it.

---

## Invariant-by-invariant findings

### V1 — Regime immutable  ✓ Fully structural
`attemptRegimeChange` is typed `never` — there is no code path that returns
normally.  The genesis payload is hash-linked; a node with a different regime
would have a different ID.  No `dial_change` op covers regime because regime is
not in `GovernanceDials`.  The change is unrepresentable at every layer.

### V2 — Regime in content  ✓ Fully structural
`computeContentId` hashes `{ groupId, regime, authorId, content, timestamp }`.
Flipping `regime` on an existing item produces a different ID.  The regime tag
is not client-side metadata; it is part of the signed object.

### V3 — No silent regime crossing  ⚠️ Structural for *automatic* crossing; not for deliberate disclosure
No payload type in the protocol simultaneously carries an existing item's
raw content and targets a different-regime group.  The `republish` act
carries only `sourceHash` (an opaque ID) plus `publicContent` written by the
author.

**Important limitation:** This prevents *automatic or silent* crossing.  A
human author can deliberately copy intimate text into their own `publicContent`
field.  That disclosure is a social/policy problem, not a data-model problem.
The mechanism does not and cannot prevent intentional quoting.  If this is a
threat in your model, the relevant control is at the application UX layer
(warn before composing a republish, redact paste from intimate threads), not
in the DAG.

### V4 — Republish is a distinct authored act  ✓ Fully structural
`createRepublishAct` validates `targetGroup.params.regime === 'public'` before
constructing the object.  Targeting an intimate group throws `VisibilityViolation`.
The `sourceHash` field is the ID of the intimate item; the intimate item's
`content` field is structurally absent from the republish object.  A public
observer who holds only the republish act cannot enumerate the intimate group's
members — the ID is opaque without access to the intimate DAG.

### V5 — Propagation depth enforced by verifier  ✓ Structurally enforced (verifier-side)
`verifyPropagationShare` throws for any `share.depth > group.outward_propagation_depth`.
The enforcement lives on the **receiving verifier**, not on the sender's client.
A hostile sender can craft a `PropagationShare` with any depth value; the
verifier rejects it identically regardless of the sender's behaviour.  This is
the correct trust model: enforcement that depends on the sender behaving
correctly fails precisely in the cases where enforcement matters.

### V6 — Openness caps depth  ✓ Fully structural
`validateGenesisParams` is called inside `createSocialGroup` before any DAG
operation.  An over-permissive genesis (e.g. `fully_open` with depth=1) cannot
be constructed — it throws before the genesis node is hashed or added to the
DAG.  The `MAX_DEPTH_FOR_CLASS` table is:

| Class | Max depth |
|-------|-----------|
| `closed` | 3 |
| `open` | 1 |
| `fully_open` | 0 (hard sink) |

These thresholds are illustrative.  Adjust `MAX_DEPTH_FOR_CLASS` in
`core/visibility.ts` when the product definition of "large" / "fully open"
is settled.

### V7 — Inward and outward are independent  ✓ Fully structural
`inward_visibility` and `outward_propagation_depth` are separate fields with
no coupling in the validation or verification logic.  A fully-open public
board (`inward=full, outward=0`) enumerates all members and propagates nothing.
A closed private group (`inward=none, outward=3`) propagates freely and
enumerates nothing.  The parameters are independently settable at genesis.

### V8 — Public membership bounded  ✓ Fully structural
The key guarantee is DAG isolation: each group's governance history lives in
its own DAG instance.  The public group's DAG contains no reference to the
intimate group's genesis ID or any of its content IDs.  This is structural
because no cross-DAG write path exists — the `add_member` op that records L1
joining the public group is added only to `publicGroup.dag`, never to
`intimateGroup.dag`.  Additionally, `fully_open` groups are depth-0 sinks, so
no propagation share from the public group can reach the intimate group.

### V9 — Freeze-by-default  ✓ Fully structural
Joining a public group (`add_member` in the public DAG) has no side effect on
the member's intimate DAG.  There is no publish/subscribe mechanism that would
auto-inject nodes across DAG instances.  The intimate graph size is unchanged
after the join; no public-group reference appears in the intimate DAG.

### S2 — Scoped visibility, not opaque structure  ✓ Structural (the headline social-layer result)
`core/scoped_visibility.ts`, experiments `S2a`/`S2b` (`green-model`).

**S2a — structure-only sharing is re-identifying, and therefore unrepresentable
(`INV-STRUCTURE-LEAKS-IDENTITY`).** The tempting product feature is "show the
useful structure but hide the names." It cannot be offered, because graph
topology *is* a near-fingerprint. Modelling the canonical attack — a town of
4,000 where 3,994 are generic, 5 touch the Henderson family group, and exactly
one person additionally touches the Oak-St school-parents group — the anonymity
set for the target's connection shape collapses to **1**. Withholding every name
does not help: the shape alone re-identifies. So `attemptStructureOnlyShare()` is
a `never`-returning constructor (the same "unrepresentable" idiom as V1's regime
change) — there is no code path that produces a topology-with-hidden-identities
share for a viewer to receive.

**S2b — the only safe, constructible share is consented-distance/resolution-scoped
(`INV-SCOPED-CONSENT`).** A `ScopedShare` is a per-distance consent map and carries
no topology field. A viewer at distance *d* sees exactly what the owner consented
to expose at distance *d* — not nearer distances' content, and over-reach beyond
the consented horizon yields `[]` (silence, not "structure minus names"). This is
the honest version of the Kevin-Bacon-distance idea: distance gates *consented
content*, never the graph shape.

**Scope note:** S3 (quiet membership) and S4 (multi-identity) are NOT modelled here
— they require design gate G5 (`discovery/thinking/social-layer.md` §75–77) before a
test. S2 is the one done in this pass, and it is the highest-value social-layer
result: it rules out an entire class of "anonymized graph" features as unsafe-by-
construction.

---

## Summary: what is structurally unrepresentable vs. runtime-warned

| Invariant | Enforcement | Notes |
|-----------|-------------|-------|
| V1 Regime immutable | Structural (type + hash) | |
| V2 Regime in content | Structural (signed hash) | |
| V3 No silent crossing | Structural for automatic; **not** for deliberate | See V3 note above |
| V4 Republish is distinct | Structural (no content field in republish) | |
| V5 Depth enforced | Structural at **verifier** | Sender cannot bypass |
| V6 Openness caps depth | Structural (genesis validation) | |
| V7 Parameters independent | Structural (separate fields) | |
| V8 Public bounded | Structural (DAG isolation + sink) | |
| V9 Freeze-by-default | Structural (no cross-DAG write path) | |
| S2a Structure leaks identity | Structural (unrepresentable constructor) | Topology = fingerprint; anonymity set →1 |
| S2b Scoped consent | Structural (per-distance consent, no topology field) | Distance gates content, never shape |

**The highest-value finding is V3's limitation.** Every other invariant is
fully structural.  V3 prevents the protocol from silently moving intimate
content to a public group, but it cannot prevent a human from typing intimate
content into a republish act.  This should be the first thing addressed in
the UX layer before shipping the republish feature.
