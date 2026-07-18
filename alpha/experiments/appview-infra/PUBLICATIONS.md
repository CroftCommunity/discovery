# The publications positioning — what the substrate proves, what the model adds

`Corpus deliverable. Design material, stays in discovery; excluded from the
appview-infra extraction (D15), same class as GROUPS.md. Landed 2026-07-18
(RUN-18) as the standalone positioning doc the RUN-18 v2 brief specified
(§0.2); the brief's PUBLICATIONS-DESIGN.md payload did not accompany the
dispatched brief, so this text is authored from that specification directly —
recorded, not glossed, in RUN-18-SUMMARY.md. It records the vanilla-atproto
comparison for the publications (newsletter) shape: what the substrate proves
natively, the degeneration principle that binds our open/single scopes to it,
the single-agent limit and the one atom the model adds, the delta table, and
the subscriber reframe. §6 anchors every claim into the landed
[GROUPS.md](GROUPS.md) model text and the RUN-17/18 tier-proof evidence.`

## 1. What vanilla atproto proves natively

Three proofs come free with the substrate, and the design leans on all three
rather than rebuilding them:

- **Authorship.** Every record lives in its author's repo, under the author's
  repo signature chain; the signing key chains to the DID document. A record is
  a speech act of its account, provably — nobody else can produce it.

- **Integrity.** Records are content-addressed; the commit signs the MST root,
  so a record's bytes cannot be altered in place without breaking the chain a
  verifier walks. What you fetched is what was signed.

- **Current-state completeness.** A repo commit commits to the *entire current
  state* of the repo through the MST root. A full checkout at commit C is
  verifiably complete *as of C*: no record present in C can be hidden from a
  reader holding C. This is a completeness proof over the PRESENT — and only
  the present (see §4).

## 2. The degeneration principle

Stated as binding, not aspiration: **where a scope's policy pair asks nothing
the substrate does not already prove, the machinery MUST degenerate to
bare atproto records plus chaining, and nothing else.** Our `open`-membership
scopes — the open forum (open/open) and the newsletter (open/single) — ride
bare records: the genesis, the self-registration, and every issue are ordinary
signed records in their authors' repos; the only addition is the envelope's
antecedents field, i.e. chaining (GROUPS.md A.5), which the substrate does not
supply and reception completeness (GROUPS.md A.2) requires. No parallel
delivery authority, no server-side membership store, no second identity system:
if vanilla proves it, vanilla carries it. The gated and sealed tiers switch
machinery ON precisely where their policy pairs ask for proofs vanilla cannot
give (two-sided membership, sealed deliberation); the open/single publication
is the degenerate case, and the degeneration is the design.

## 3. The single-agent limit, and the one added atom

The substrate's grain is the single account: every record is a speech act of
one author. There is no collective noun in vanilla atproto — no record whose
author is "us", no institution that can speak as itself. The closest native
shape, the list, is a **curator record**: it lives in the curator's repo and
names other accounts, who sign nothing — a consent-free enumeration. That is
exactly right for curation (mutes, feeds, starter packs) and exactly wrong for
membership: a roster nobody consented onto proves nothing about its members.

The model therefore adds ONE atom on top of the substrate: the **provable
multi-party fact** — the two-sided membership fact of GROUPS.md A.3, a claim
record in the joiner's repo answered by co-signed grant records in the
stewards' repos, folded by causal position. Everything institutional in the
model (rosters, stewardship, revocation, the catalogue) is built from that
single atom plus the fold; nothing else multi-party is ever asserted without
it. Where the scope is open, even that atom degenerates (§2): the
self-registration is claim, consent, and roster entry in one record, and the
consent contrast with curator lists still holds — nobody appears on a roster
without a record they authored (structural consent, GROUPS.md A.4).

## 4. The delta table

| Dimension | Vanilla atproto proves | The model adds | Where |
|---|---|---|---|
| Who may speak | **authorship** — this account signed this record | **authorization-at-position** — the author was on the roster and admitted by the write policy *at the record's causal position* | [GROUPS.md A.2](GROUPS.md#a2-two-policy-axes-not-one), [A.8](GROUPS.md#a8-transports-the-split-the-swarm-and-one-planned-crossing); evidence B1 (RUN-18), P3/P4 (RUN-17) |
| History | **tamper-free current state** — the present is complete and unforgeable, but a deleted record is simply absent, indistinguishable from one that never existed | **tamper-evident history** — chaining makes absence classifiable three ways: **never-existed** (no chain references it), **retracted** (referenced, deletion verifiable at source), **withheld** (referenced, no source offers it, deletion not shown) | [GROUPS.md A.2](GROUPS.md#a2-two-policy-axes-not-one) (reception paragraph); evidence B2/B3/B5 (RUN-18) |
| The collective | **account** — a single agent's repo; lists are consent-free curator records | **institution** — a lineage with governed roster, stewards, and policy axes, built from the multi-party fact | [GROUPS.md A.1](GROUPS.md#a1-principals-and-names), [A.3](GROUPS.md#a3-membership-facts-gated-tier-the-open-and-sealed-tiers-are-its-degenerate-and-augmented-forms), [A.6](GROUPS.md#a6-governance-the-small-real-mls-group-and-the-true-ceiling) |
| Reach | a count the server asserts | an **auditable** count — any second fold over the same records re-derives it | [GROUPS.md A.4](GROUPS.md#a4-the-open-tier-one-signature-zero-canonical-bytes); evidence B6 (RUN-18) |

The history row is the publication-facing consequence of chaining, and the
three-way distinction is its load-bearing cell: vanilla's current-state proof
is **tamper-free** but memoryless (retraction and never-existence collapse into
the same absence), while the chained stream is **tamper-evident** — history's
shape survives even where its content is gone, and a reader can say *which*
kind of gone. Retraction remains possible and honest (the author deletes the
content record; the substrate proves the deletion); what retraction can no
longer be is silent.

## 5. The subscriber reframe

- **Two rosters, one lineage.** A publication is one lineage (one genesis, one
  catalogue entry) carrying two roster facts: the *writer* roster fixed by the
  write policy (`single`, or `named-set` for a masthead) and the *reader*
  roster grown by open enrollment. Vanilla conflates these into "follows"; the
  model keeps them as two policy values on one scope (GROUPS.md A.2).

- **The subscriber is the guarantee beneficiary.** Reception completeness
  (GROUPS.md A.2) is a subscriber-side guarantee: any reader can verify their
  held stream complete up to the newest held issue, detect any gap as a known
  omission, and repair it from any role or peer. The guarantee runs on public
  data; open enrollment never weakens it.

- **Auditable reach and churn.** The subscriber count is not the operator's
  assertion: anyone folding the same records re-derives the roster, so reach is
  auditable and churn is visible as interval opens and closes (GROUPS.md A.3).
  A count with no folded records behind it is detectable as unsupported
  (evidence B6, RUN-18).

- **Structural consent.** Every subscriber holds their own self-registration
  record; every unsubscribe is an authenticated deletion. Nobody can be
  enrolled, or kept, by someone else's record — the exact consent contrast
  with the curator list (§3).

- **The paid tier is a policy value.** A paid publication is the same lineage
  with a `gated` reader roster (grant-on-payment as an admission rule,
  GROUPS.md A.6) — a policy value on the scope, not new machinery. Named here
  as positioning only; nothing is built and no pricing posture is taken.

- **The honest scope of "managing".** What an operator of a publication
  actually manages: they run replaceable delivery roles over public facts —
  serving, indexing, search. They do not own the roster (it re-derives from
  records subscribers author), cannot invent or expel subscribers in a way that
  survives audit, cannot reorder or silently truncate history (chaining), and
  hold no canonical bytes for the open/single shape (degeneration, §2).
  "Managing a publication" is operating conveniences, never custody of the
  publication's facts — that is the honest scope, and saying so is the
  positioning.

## 6. Where this lives in the model, and what proves it

- The two policy axes and the reception-completeness paragraph:
  [GROUPS.md A.2](GROUPS.md#a2-two-policy-axes-not-one) — proven by B1–B4
  (chaining validation, gap detection and repair, the honest tail, interval
  interaction), RUN-18.
- The membership fact and intervals:
  [GROUPS.md A.3](GROUPS.md#a3-membership-facts-gated-tier-the-open-and-sealed-tiers-are-its-degenerate-and-augmented-forms)
  — proven by P4 (RUN-17); churn visibility rides the same intervals.
- The open tier's one-signature enrollment and audit-surviving roster:
  [GROUPS.md A.4](GROUPS.md#a4-the-open-tier-one-signature-zero-canonical-bytes)
  — proven by P2 (RUN-17) and B6 (RUN-18, auditable reach).
- Envelope identity and chaining's carrier (antecedents):
  [GROUPS.md A.5](GROUPS.md#a5-the-assertion-layer-what-survives-of-mls-at-each-tier)
  — proven by P1 (RUN-17).
- The delivery plane the operator actually runs (the "managing" scope):
  [GROUPS.md A.7](GROUPS.md#a7-the-delivery-plane-roles-realized-as-separate-processes-none-of-them-authorities)
  — proven by P8 (RUN-17).
- The transport split and the swarm path that turns a withheld tail from
  silent to detected:
  [GROUPS.md A.8](GROUPS.md#a8-transports-the-split-the-swarm-and-one-planned-crossing)
  — proven by B3 (RUN-18).
- The tier table, history-access row:
  [GROUPS.md A.9](GROUPS.md#a9-the-tier-table) — the three-way retraction
  distinction proven by B5 (RUN-18).
