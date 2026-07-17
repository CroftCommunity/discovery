# The access-gated large-group tier — design brief (D11)

`Corpus deliverable. Design material, stays in discovery; excluded from the
appview-infra extraction (D15). Written 2026-07-17 for the RUN-15 hosting-kit
run. Asks three owner decisions at the end; makes none.`

This brief states the design stance for large groups served from the AppView,
analyses the one write-path fork that is an owner decision, and stages a
spec-facing reconciliation note. It does not build the fork — D12 builds only
the fork-agnostic serving behind a storage trait.

## 1. The tier: private, roster-gated, honestly not confidential

Below a scale boundary, groups stay **MLS-sealed** (the croft-group crates) with
the AppView as a content-blind distributor — the §H hybrid topology, whose serve
half RUN-14 EXP-B demonstrated (offer ciphertext only to a verified roster
member; the server holds no key). Confidentiality there rests on encryption.

Past that boundary the design takes a different, deliberately honest stance.
**At large scale, cryptographic group confidentiality is a mirage.** The
member-leak equivalence argument: in a group of thousands, confidentiality
against outsiders requires that *no* member ever leaks plaintext — but any one
member can screenshot, copy, or re-publish, and the probability that some member
does approaches one as membership grows. End-to-end encryption at that scale
protects against the network operator while giving members a false belief in
confidentiality *from each other's onward sharing*, which it never provided.

So large groups serve **private but not E2EE**: the AppView reads content and
gates *offering* it by verified roster membership. The posture is stated plainly
— **roster-gated, private, trusted-gatekeeper, NOT cryptographically
confidential**. The AppView operator is a trusted gatekeeper for this tier, and
saying so is the feature: search, helpers, and moderation become native and
honest instead of bolted onto a confidentiality claim the scale already broke.

### The boundary is a parameter, not this run's decision

The crossover is a manifest/policy parameter **`group_scale_boundary`**, working
number **5000** members. It is **deferred to the owner** — 5000 is a placeholder,
not a finding. Below it: MLS-sealed, content-blind AppView. At or above it:
roster-gated AppView serving per this brief. A group can be born large or cross
the boundary; the migration story is part of the fork analysis below.

## 2. The write-path fork (OWNER DECISION — guardrail 5)

The one thing the owner must decide: **where large-group content canonically
lives.** Both variants serve the same way (roster-gated reads); they differ in
who holds the canonical bytes. D12 builds behind a `GroupStore` trait so the
serving path is identical either way.

### Variant A — repo-canonical ciphertext, AppView decrypts to serve

Authors publish ciphertext records to **their own repos**, sealed to a
server-held **group scope key**. The AppView holds that scope key + the roster,
decrypts to build its index, and serves roster-gated.

- **State taxonomy (§1.2).** Server-canonical = **scope key + roster only**
  (small, in `state.db`, backed up by Litestream). Content stays
  **repo-canonical** (the authors' PDSes). The AppView index is **disposable**
  — rebuildable by re-reading repos + re-decrypting.
- **Backup cost.** Minimal. Canonical state is a key and a member list; kilobytes.
- **Restore-drill shape.** Restore `state.db` (key + roster), then rebuild the
  disposable index from repos. Content is never at risk on the AppView because
  the AppView never held the canonical copy.
- **Moderation / helpers.** The AppView decrypts, so search/moderation/helpers
  are native — same as Variant B. Removal from the roster stops future offering;
  already-distributed plaintext cannot be recalled (honest, same as B).
- **Migration.** To move to Variant B, ingest repo content into server-canonical
  storage and stop relying on repos. Reversible-ish while repos retain history.
- **§H posture.** Closest to §H: content-blindness is *given up on purpose* for
  this tier (the AppView decrypts), but canonical custody still rests with
  authors. The trusted-gatekeeper acceptance is scoped to *serving*, not custody.

### Variant B — members write to the AppView API, server-canonical content

Members write content **directly to the AppView**; the server holds the
canonical bytes (`blobs/` + `state.db`).

- **State taxonomy (§1.2).** Server-canonical = roster + grants + **content**
  (`state.db` + `blobs/`, all Litestream/rclone-backed). Index stays disposable.
- **Backup cost.** Heaviest — every message and blob is canonical server state
  that must be backed up. This is the tier that stresses the R2 free tier first.
- **Restore-drill shape.** The full D13/P2-6 drill matters most here: canonical
  content lives only on the box + R2, so a restore must bring back `state.db`
  **and** the blob mirror, then rebuild the disposable index.
- **Moderation / helpers.** Native and simplest — content is right there, no
  repo round-trip, no decrypt step.
- **Migration.** To move to Variant A, authors would have to re-home content into
  their repos — the **weakest portability story**; content born server-canonical
  has no author-side copy by construction.
- **§H posture.** Furthest from §H: the AppView is the canonical home. Simplest
  immediate build; the honest cost is backup weight and portability.

### Scoring summary

| Dimension            | Variant A (repo-canonical)     | Variant B (server-canonical)   |
|----------------------|--------------------------------|--------------------------------|
| Canonical on server  | key + roster (tiny)            | roster + content (heavy)       |
| Backup cost          | minimal                        | heaviest (blobs + content)     |
| Restore-drill stakes | low (content safe in repos)    | high (only copy is box + R2)   |
| Moderation/helpers   | native (after decrypt)         | native (simplest)              |
| Migration path       | A→B straightforward            | B→A weak (no author copy)      |
| Portability          | strong (repo-canonical)        | weak                           |
| Immediate build      | more moving parts (scope key)  | simplest                       |
| §H alignment         | closer (custody stays authors) | further (server is home)       |

**No recommendation is forced here** — it is a values call (portability &
custody vs immediacy & simplicity). The kit is fork-agnostic; D12 builds the
shared serving so either can be chosen without rework.

## 3. Spec-status: taking the trusted-gatekeeper arm, this tier only

RUN-14 left social-mapping's **AppView-provisioned scope key** item open
(`Design; open` — how the audience scope key is provisioned/granted/rotated;
stop rule 5b). This brief deliberately takes the **trusted-gatekeeper arm** of
that open item — **for the large tier only**: past `group_scale_boundary`, the
AppView is a trusted gatekeeper that reads and serves, and the scope key (Variant
A) or direct custody (Variant B) is provisioned to that end. Below the boundary,
the content-blind §H stance is unchanged.

This is a **proposed reconciliation note, staged**, not a spec edit. It is
appended to `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`
(RUN-15 section) with a row in
`beta/impl/experiments/drystone-reviews-and-experiments-log.md`, in the same
commit. The reviewed spec files (`part-*`, conventions) are **untouched**
(guardrail 2).

## 4. What D12 builds (fork-agnostic)

Behind a `GroupStore` trait — `roster(group)` and `content(group, cursor)` —
both variants can implement. The stub implements a fixture store. Roster and
grants are classed **canonical** in the manifests (D3 asserts; D5 enforces a
backup target exists); the search/moderation index is **disposable**. The
offering-vs-reading distinction (RUN-14 EXP-B) does **not** apply to this tier:
the server reads by design, and the honest posture is the feature.

## 5. Decision request (the owner answers; the run does not)

1. **Which write-path variant** for the large-group tier — **A** (repo-canonical
   ciphertext, AppView decrypts; strong portability, minimal backup) or **B**
   (server-canonical content; simplest build, heaviest backup, weak portability)?
2. **The `group_scale_boundary` number** — confirm 5000, or set another. It is a
   parameter, not a mechanism; the kit reads it from policy.
3. **Launch order** — does **croft-groups** launch **before** or **with**
   stellin-appview? Both are `gated_groups` tenants sharing this mechanism.
