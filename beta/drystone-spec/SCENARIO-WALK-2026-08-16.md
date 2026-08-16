# Scenario walk: Appendix E's L-arc against the measurements, and the full matrix

date: 2026-08-16
status: **walk complete for L1–L6; matrix in Part 2. Not normative.**
evidence: `alpha/experiments/meer-queue/tests/` S15–S22 (32 tests, Rung A) · `mls-replant` (M1) ·
`TEST-LOG.md`
companion: `DOSSIER-exclusion-and-readmission-2026-08-16.md` (findings) ·
`part-2-certifiable-design-WORKING-2026-08-16.md` (revision surface)

**Layer key**, used throughout. The question for every claim is not only *is it true* but *what makes
it true*, because that decides who can change it.

| tag | layer | what it means |
|---|---|---|
| **MLS** | RFC 9420 / openmls | the messaging protocol. We do not get to choose; we can only use or refuse |
| **DRY** | Drystone | architecture and grounding protocol. Ours to specify, constrained by MLS |
| **CRO** | Croft | product dials and governance in practice. Ours to choose within Drystone |
| **∅** | nothing yet | claimed but not grounded anywhere — the interesting column |

---

## Part 1 — Walking L1 → L6

### L1. The steady state *(Ada, two clients, one persona)*

> *"Ada is a persona in good standing running two clients… folded to one persona with one unit of
> weight. She is a live member of the hot Group, the live-membership boundary whose per-commit cost
> scales on the live set."*

**Verdict: HOLDS. One grounding note worth adding.**

| claim | layer | status |
|---|---|---|
| per-commit cost scales on the live set | **MLS** | measured — `mls-replant` M1, O(N) floor ↔ O(log N) ceiling |
| two clients fold to one persona, one unit of weight | **DRY** | **MLS provides no help here** |

**The folding is entirely Drystone's.** MLS sees **clients (leaves)**, never personae or lineages —
§11.8 says so, and S16/S18/S20 measured the consequence from the other end (a lineage-scoped ban
cannot be enforced by leaf logic). L1 reads as though folding is ambient; it is a Drystone
construction sitting on top of a protocol that is actively unaware of it.

**Suggested extension:** state the layer explicitly at L1, because every later beat that reasons
about *lineages* (L5, L6) depends on a folding MLS does not know about.

---

### L2. Boreas goes dormant *(migration to cold)*

> *"his client migrates from the hot Group to the linked cold Group… The cold Group rolls epochs
> freely in his absence and keeps no frozen-epoch snapshot for him, **because the resumption secret,
> not a held key, is what will carry his continuity across those rolls.**"*

**Verdict: REWORK. The stated continuity mechanism does not exist, and the beat omits what migration
costs the migrated client.**

| claim | layer | status |
|---|---|---|
| migration to cold is a batched removal | **MLS** | measured — S20, at N = 10 |
| the hot set shrinks to live members | **MLS** | follows |
| the resumption secret carries continuity across rolls | **MLS** | ✗ **S16: cannot be attached to an external commit on openmls 0.8.1** |
| a linked *cold Group* exists as a second MLS group | **∅** | **never tested; and its stated linkage is the mechanism above** |

Two problems, and the second is the one nobody had noticed:

1. **The continuity token doesn't work.** Resumption PSKs resolve from the group's own
   `ResumptionPskStore`; an external-commit group initialises it empty; `add` is `pub(crate)`.
2. **The beat is silent on what migration does to Boreas's client** (S20). He lands in one of three
   states, and **which one depends on whether he was awake**: never saw the commit (stale key), *saw
   it* (**dead object — `UseAfterEviction`**), or rebuilt from a `GroupInfo`. The well-behaved client
   — the one that syncs — gets the worst outcome. For a **ban** that is correct; for a **dormancy
   migration** it is a defect the narrative should own.

**Suggested rework:** drop "the resumption secret… carries his continuity" and say what actually
carries it (a governance-issued external PSK, §11.7 REV); add a sentence on the three states and the
`SHOULD` that rebuild-on-eviction is a normal path, not an error path.

**Open, and larger than this beat:** whether the cold Group survives as *a Group at all* once its
linkage mechanism is gone, or becomes **a standing state with a delivery consequence**. See the
dossier §3.3.

---

### L3. Boreas returns, and pays his own way

> *"A stored KeyPackage cannot bring him in… so he re-enters by his own external commit against a
> fetched GroupInfo… He presents one bound two-part credential… The GroupInfo is treated as a claim
> corroborated against the governance chain he already holds, never as authority."*

**Verdict: the SPINE HOLDS and is measured; the CREDENTIAL needs rework; one actor is missing.**

| claim | layer | status |
|---|---|---|
| a stored KeyPackage cannot bring him in | **MLS** | ✓ correct — he cannot produce a Welcome under current-epoch secrets |
| self-service external commit, cost on the returner | **MLS** | ✓ **measured** — S15/S18/S19, and S19 measured it **from a completely fresh provider** |
| the two-part credential (attestation + resumption PSK) | **MLS+DRY** | ✗ **S16: neither half implementable as written** |
| the GroupInfo is a claim, corroborated, never authority | **DRY** | ✓ sound — **and S19 adds a second reason it matters** |
| **who serves him the GroupInfo** | **∅** | **the beat does not say, and the answer is load-bearing** |

**On corroboration, sharpened.** L3 treats "GroupInfo as claim, not authority" as an *integrity*
property (a stale-but-signed GroupInfo could defeat PCS). **S19 adds a confidentiality/admission
dimension the beat does not mention: every `GroupInfo` carries `external_pub`, so handing one over
does not merely make a claim — it admits its holder.** Corroborating it protects the *receiver*; it
does nothing about what the *sender* just granted. **Withholding the ratchet tree is what separates
"prove current state" from "admit the holder"** (S18/S19), and that distinction belongs in L3.

**On the missing actor.** "a fetched GroupInfo" — fetched from whom? **Part 1 §2.4 answers it and the
answer is uncomfortable: from any member**, because a Group MUST NOT depend on any single persona to
act and every member can export one. So there is no serving tier, and **S22 measured what that costs:
a negative standing check fails open at the least-synced peer.**

**Suggested rework:** replace the credential with the governance-issued external PSK; name the server
as "any member, under a group-context serving policy"; add the tree-withholding distinction.

---

### L4. What Boreas recovers, honestly bounded

> *"On admission Boreas re-keys to the current hot epoch and participates at once, with the
> dormancy-window history streaming in behind him from the history DAG… Reading that archived gap
> relies on a re-grantable archival key — a deliberate and stated bounded relaxation of forward
> secrecy."*

**Verdict: HOLDS, and the measurements make it STRONGER than it claims for itself.**

| claim | layer | status |
|---|---|---|
| re-keys to the current epoch, participates at once | **MLS** | ✓ **measured** — S18: he sealed a message a member read, and read a member's message sealed after his return |
| history is decoupled from key continuity | **DRY** | ✓ — and **necessarily so** |
| archival key = bounded FS relaxation, stated | **DRY** | ✓ honest, and now demonstrably **load-bearing** |

**S18 measured that a returner recovers *nothing* through MLS** — an external commit derives the new
epoch and nothing earlier. So the history-DAG-plus-archival-key is **not an optimization for a nicer
return experience; it is the only thing standing between "return" and "you begin at the moment you
returned".** L4 presents it as a considered trade. It is that, *and* it is the mechanism without which
L4's own promise is empty.

**Suggested extension:** one clause saying the DAG path is required rather than additive — it
strengthens the honesty of the FS-relaxation disclosure rather than weakening it.

---

### L5. Cyrus is banned

> *"a fork, not a deletion… Because a ban is lineage-scoped where MLS sees only clients, enforcement
> lives in an application-layer admission gate that resolves a returner's lineage against the ceiling
> at head, never over the range the returner chooses to assert."*

**Verdict: HOLDS — and this beat is the one the measurements CONFIRM most strongly. It needs
extension, not correction.**

| claim | layer | status |
|---|---|---|
| the ban excludes from forward key material | **MLS** | ✓ **measured at N = 10** — S20: nine survivors agree, the banned member derives none of it, at AEAD grade |
| MLS sees clients, not lineages | **MLS** | ✓ **measured** — S16: a party who was never a member joined on a `GroupInfo` alone |
| enforcement must be an application-layer admission gate | **DRY** | ✓ **confirmed at Rung A**, not merely asserted |
| resolved at head, never over the asserted range | **DRY** | ✓ sound; **implemented in S22's stub** |
| a ban is a fork, not a deletion | **DRY** | ✓ and **S18 measured its shape**: refusal holds at keys *and* addressing |
| **where the gate runs** | **∅ → now DRY** | **S22: at every peer, when asked to serve** |

**The extension L5 needs is one sentence about the gate's *location*, and it is not a detail.** The
beat says enforcement "lives in an application-layer admission gate" without saying who runs it.
S19/S20 located it — **the moment a `GroupInfo` is served, not the moment of re-entry, because
re-entry is self-admission and there is nothing to deny** — and Part 1 §2.4 then forces the
uncomfortable half: **every member is that gate.**

**And L5 should carry the invite/external-join split (S21)**, because it changes what "admission
gate" even means. For a member *inviting* someone, MLS's own **proposal phase** is a real gate:
a proposal seats nobody until a member commits it. For an outsider *seating herself*, **no proposal
phase exists**. One beat, two mechanisms, and only the second needs the dial.

---

### L6. A lagging node admits Cyrus, and the key layer heals it

> *"the removal re-keys and the banned lineage is cut from new entropy with **zero marginal exposure,
> since he briefly re-held only keys he already had**… The residual floor: exclusion latency is
> bounded by hot-Group commit liveness."*

**Verdict: the SCENARIO is exactly right and now measured; the SAFETY CLAUSE is measured FALSE.**

| claim | layer | status |
|---|---|---|
| a lagging node admits him | **DRY** | ✓ **measured** — S22: nine peers refuse, one stale peer serves, he needs one yes |
| ban propagation is eventually consistent | **DRY** | ✓ |
| **zero marginal exposure; he re-holds only keys he already had** | **MLS** | ✗✗ **MEASURED FALSE** — S18: he re-keys to the **current** epoch and receives **new entropy** |
| re-exclusion is guaranteed by re-keying | **MLS** | ✓ |
| exclusion latency bounded by commit liveness | **DRY** | ✓ **and incomplete** |

**The false clause is struck in the working copy** (§11.8 and here). The replacement: his exposure is
**everything sent between admission and re-exclusion**, bounded by ban-propagation latency and commit
liveness. The surrounding argument survives — dormancy still cannot evade a ban, membership consensus
is still not required — but **exclusion latency becomes a confidentiality parameter, not only a
liveness one**, which strengthens L6's own closing advice.

**And the residual floor is wider than L6 states.** It names *commit liveness* (how fast someone
enacts the removal). S22 adds a second term: **standing-chain propagation to every member**, because
every member can serve. **Both must be short, and only one is named.**

**The deeper consequence, and the reason L6 should not be patched in place:** a *negative* check —
"refuse if I know he is banned" — **fails open** on a stale peer. A *positive* check — "serve only
against a governance-issued token" — **fails closed** (S22, same staleness, every peer refused).
**L6's self-healing story is sound for the determinate case it describes; the design it implies
(everyone learns the ban eventually) is the weaker of the two available postures.**

---

## Part 1 summary

| beat | verdict | layer of the problem |
|---|---|---|
| **L1** steady state | **holds** | add a DRY grounding note (folding is ours, MLS is unaware) |
| **L2** goes dormant | **REWORK** | **MLS** — the continuity mechanism does not exist; plus an unowned client cost |
| **L3** returns | **spine holds, credential reworks** | **MLS+DRY** — replace the credential; name the server; add the tree distinction |
| **L4** what he recovers | **holds, strengthen** | **DRY** — say the DAG path is required, not additive |
| **L5** banned | **holds, confirmed** | **DRY** — add the gate's location and the invite/external split |
| **L6** lagging node | **scenario right, clause FALSE** | **MLS** — strike; widen the residual; note negative-vs-positive |

**Two beats need real work (L2, L3), one needs a strike (L6), two need extensions (L1, L5), and one
gets stronger for free (L4).** No beat is wrong about *what happens* — the arc's narrative judgement
holds throughout. **Every defect is in a stated mechanism or an unstated actor.**

---

## Part 2 — The scenario matrix

Every scenario from group formation to a 50-member community, each tied to **where the functionality
grounds** and **whether it is measured**. `∅` in the layer column is the interesting result: claimed
somewhere, grounded nowhere.

**Status key:** ✓ measured Rung A · ~ partially measured · ✗ measured *not* to work · **∅** ungrounded
· — not applicable.

### 2.1 Formation and the small group

| # | scenario | MLS | DRY | CRO | status | evidence / gap |
|---|---|---|---|---|---|---|
| 1 | One creator founds a group | creates the tree | names it, seeds governance | UX of "make a group" | ✓ | every test founds one |
| 2 | Creator adds a second member | Add + Welcome | — | — | ✓ | S12 handshake end to end |
| 3 | Two members exchange messages | one epoch secret | queue naming | — | ✓ | M1, S21 |
| 4 | A second device for one member | leaves | **lineage folding** | device UX | ~ | folding is **DRY and MLS is unaware** (L1); E92 device-group arm **∅** |
| 5 | A member goes briefly offline | — | meer store-and-forward | retention dial | ✓ | M1 — the central claim |
| 6 | Two devices, one persona, both draining | — | have/want per device | — | ✓ | S4 (starvation dissolved by the fabric model) |

### 2.2 Growth, invitation, and first contact

| # | scenario | MLS | DRY | CRO | status | evidence / gap |
|---|---|---|---|---|---|---|
| 7 | A member invites a known person | **proposal → commit** | governance in the proposal phase | who may propose | ✓ | **S21 — the gate exists in-protocol** |
| 8 | The group vetoes an invite before it lands | proposal is inert until committed | the veto rule | moderator UX | ~ | mechanism ✓ (S21); **the rule itself ∅** |
| 9 | A stranger contacts someone with no prior relationship | `Welcome` only | **personal inbox**, DID-resolvable | discovery UX | ~ | S12 measured; **write blocked, HTTP 403** |
| 10 | A stranger seats you in a group you never asked for | permits it | cannot prevent | can bound + attribute | ✓ | S11 — **not cryptographically preventable** |
| 11 | KeyPackage distribution | public key material | owner's namespace | — | ✓ | S12 — published, fetched, `validate()`d |
| 12 | Burning someone's published KeyPackages | single-use is real | — | ceiling + rate limit | ✓ | S11 — **the bound lands on the owner; rejected** |

### 2.3 Ban, exclusion, and readmission

| # | scenario | MLS | DRY | CRO | status | evidence / gap |
|---|---|---|---|---|---|---|
| 13 | Governance bans one of ten | removal commit re-keys | ceiling on the governance chain | ban UX | ✓ | **S20 — nine agree, one excluded, AEAD grade** |
| 14 | The banned member reads forward | ✗ cannot | — | — | ✓ | S19/S20 — key check, not counter check |
| 15 | The banned member processes her own ban | **object dies** (`UseAfterEviction`) | — | hide or surface? | ✓ | S20 — **and the well-behaved client is punished** |
| 16 | She re-seats herself from a `GroupInfo` | permits it | must gate at serve time | dial position | ✓ | S18 — **removal is as durable as `GroupInfo` distribution** |
| 17 | A peer refuses to merge her re-entry | — | refusal holds at keys **and** addressing | — | ✓ | S18 — **and it forks, invisibly** |
| 18 | Two members disagree about readmitting | — | **group-context rule required** | never a prompt | ~ | fork measured (S18); **the rule ∅** |
| 19 | A stale peer serves her a `GroupInfo` | — | negative check **fails open** | — | ✓ | **S22 — nine refuse, one serves, she needs one yes** |
| 20 | Governance issues a re-entry token | external PSK binds to key schedule | token as standing+keys | issuance/revocation UX | ~ | mechanism ✓ (S16); **issuance & revocation ∅** |
| 21 | Detecting a fork after a disagreement | ✗ **epoch counter is useless** | needs a signal | — | ✗ | **S18 — same epoch number, different secrets** |
| 22 | Re-admitting someone the group forgave | external commit + token | standing restored at head | reinstatement UX | ~ | S16/S22; **§11.11's tier mapping for `reinstate` ∅** |

### 2.4 Dormancy, hot/cold, and return at scale

| # | scenario | MLS | DRY | CRO | status | evidence / gap |
|---|---|---|---|---|---|---|
| 23 | Silent reader stays live | — | liveness = processing | — | ✓ | S14 — **the queue name *is* the liveness indicator** |
| 24 | Member absent past the liveness window → cold | batched removal | window schedule | window dial | ~ | removal ✓ (S20); **batching at scale ∅** |
| 25 | Cold ≠ banned, mechanically | ✗ **identical operation** | standing chain must distinguish | — | ✗ | **S16 — the stated linkage cannot be built** |
| 26 | Dormant member returns in good standing | external commit | serve on standing intact | "welcome back" UX | ✓ | **S22 — served and returned immediately** |
| 27 | Member absent past retention, inside liveness (limbo) | — | retention ≥ liveness | retention dial | ✓ | S15 — walked; escapable, needs a `GroupInfo` |
| 28 | Catch-up walk after absence | serial epoch chain | queue-per-epoch | — | ✓ | S10 — 124 ms / 10 epochs; N = **governance events** |
| 29 | A hop expires mid-walk | — | watermark | gap UX | ✓ | S13 — **loss is total from the break forward** |
| 30 | Returner recovers the dormancy gap | ✗ **nothing** | history DAG + archival key | — | ~ | S18 — **DAG is required, not additive**; DAG itself **∅ here** |
| 31 | Group grows 1 → 50 | linear commit cost | — | when to shard | ~ | S8 sized objects; **50-member commit timing ∅** |
| 32 | Hot tree stays small under churn | batched removals | dynamic window | comfort ceiling | **∅** | §11.11's **first unearned measurement** |

### 2.5 Delivery, carriage, and privacy

| # | scenario | MLS | DRY | CRO | status | evidence / gap |
|---|---|---|---|---|---|---|
| 33 | A meer carries mail for an absent member | opaque bytes | blind store-and-forward | who runs it | ✓ | **M1/M2 — the central claim** |
| 34 | The meer is absent entirely | — | **optional by principle** | — | ~ | Part 1 §2.4; **no-meer path not exercised in the spike** |
| 35 | A carrier buckets traffic by conversation | ✗ `group_id` cleartext | outer seal closes it | adopt or not | ✓ | S7 + **S17 — 28 flat bytes** |
| 36 | Nested sealing across a catch-up walk | — | wrapping rule | — | ✓ | S17 — **wrap at the queue's epoch, verified from the failing side** |
| 37 | Retention expiry actually deletes | — | — | retention policy | ✗ | S5 — **CISS has no object `DELETE`** (E95) |
| 38 | A stranger deposits into your inbox | — | custodial write | ceiling + verified DID | ✗ | S12 — **HTTP 403**, the standing blocker |

### 2.6 Croft governance intersection

| # | scenario | MLS | DRY | CRO | status | evidence / gap |
|---|---|---|---|---|---|---|
| 39 | k-of-n quorum bans a lineage | — | §7.3.1 fold | quorum UX | ~ | **CORRECTED: the fold IS implemented** (`local_storage_projection::fold_derived`, RUN-01/03). G1 measures it against its own keys |
| 40 | Two nodes resolve standing identically | — | order-independent fold | — | ✓ / ✗ | **G1: holds over a COMPLETE set (6 permutations); diverges over an incomplete one** — §11.11's beam, now demonstrated |
| 41 | Moderator role grants ban authority | — | Group Role Set | role UX | **∅** | not touched by any experiment |
| 42 | A member exits voluntarily | — | fork primitive | export UX | **∅** | §7.6.4 — same primitive as a ban, untested |
| 43 | Group forks and both sides continue | — | fork, no verdict | — | ~ | S18 measured an *accidental* fork; **a deliberate one ∅** |
| 44 | Two lineages re-compose after a split | ✗ **no MLS merge** | re-plant a fresh Group | — | **∅** | E10 — declared, never exercised |

### 2.7 What the matrix says

**Three clusters of `∅`, and they are not equally worrying.**

1. **Croft governance (§2.6) is thinly grounded — and this bullet was WRONG when written.** It said
   the §7.3.1 fold "exists in prose and in a stub". **It does not: it is implemented** in
   `local_storage_projection::fold_derived` (3,276 lines) and exercised by `competing_quorums.rs`
   (RUN-01/03). **G1 corrects that and measures the real thing** — see Part 3. Four of six rows here
   remain `∅` (roles, voluntary exit, deliberate fork, re-composition).
2. **Scale is unmeasured (§2.4, rows 31–32).** The corpus has object *sizes* at scale (S8) and commit
   *cost* shape (M1), but **no timing at 50, let alone at §11.6's bands.** §11.11's first unearned
   measurement is exactly this and it is still unearned.
3. **The no-meer path (row 34) is unexercised.** Part 1 §2.4 makes it mandatory; the spike always has
   a meer. **A principle that is never tested is a principle that can quietly stop being true.**

**And two rows are measured *not to work*, which is more useful than `∅`:**

- **Row 25 — cold ≠ banned.** The stated distinguishing mechanism cannot be built (S16). Until the
  credential is replaced, the hot/cold split is a performance structure wearing a governance label.
- **Row 21 — fork detection.** The epoch counter is useless for it (S18). Nothing else is proposed.

**The single highest-value next experiment**, on this matrix: **row 39/40 — implement enough of
§7.3.1's fold that two peers provably resolve the same standing.** It is the dependency under S22's
gate, under L5's admission gate, and under every Croft governance row. Everything in §2.3 is sound
*given* it and unproven without it.


---

## Part 3 — G1: the §7.3.1 fold against its own three ordering keys

**Written after the matrix, and it corrects the matrix.** Row 39 claimed the fold was unimplemented.
**It is implemented** — `local_storage_projection::fold_derived`, exercised by `competing_quorums.rs`
(RUN-01/03). So the useful experiment was never "build the fold" but "check it against the three keys
§7.3.1 specifies."

**Fidelity: Modeled** — a real fold over a real store, with this experiment's envelope type rather
than a wire-format-final Drystone encoding. **Code:** `croft-chat/tests/fold_ordering_keys.rs`.
**This file does not test gap-completeness and is not evidence about it.**

The fold resolves by **sequential replay in `merge_cmp` order** — `lamport → author_device →
envelope_hash`.

### 3.1 Key 1 — "subtractions before additions" is NOT implemented

§7.3.1 key 1 requires a **layered** fold with membership removals in a strictly higher tier than
additions, *"biasing every intermediate state toward the more restrictive reading (the fail-safe
direction)"*.

**Measured:** a genuinely concurrent `RemoveMember(m)` and `AddMember(m)` — two authorized devices,
equal lamport, same observed frontier — resolves to **MEMBER in both arrival orders**.

> The fold **converged** (which is the property that must never break) but resolved **permissively**.
> The addition won. Resolution is a flat replay, so **the later-sorted fact wins whatever its type.**

**This touches the ban work directly.** Key 1 *is* the fail-safe direction, so its absence **fails
open** in exactly the case readmission cares about. **It is the governance-layer instance of the shape
S22 found at the delivery layer: a restrictive rule that is not actually applied fails open.**

### 3.2 Key 3 — the concurrent tiebreak is not party-neutral

§7.3.1 key 3: the default key is the **content address**, *"party-neutral, ungameable"*, and *"any
party-privileging key is opt-in and itself under k-of-n governance"*.

**Measured:** `merge_cmp` consults **`author_device` before the content address**, so among genuine
concurrents the author's device decides and the hash never runs.

**A divergence to adjudicate, not a bug.** The ordering is still deterministic and identical on every
node, so **convergence is not at risk and nothing here shows divergence.** What is at risk is the
*reason* §7.3.1 gives: a party-derived key lets a participant who chooses device identifiers bias
every concurrent tie they are party to. **Two honest readings:** the spec should say the tiebreak is
`(device, hash)`, or the code should drop to the hash. **The spec and the code currently disagree.**

### 3.3 Order independence HOLDS — over a complete set

**Measured:** six arrival permutations of three genuinely concurrent facts (two removals, one
addition, three distinct devices, one shared frontier) resolved to the **identical** membership state.

**The fold's central property holds.** And the qualifier is the whole point: this is order
independence over a **complete** set.

### 3.4 And over an incomplete set it diverges — which is the beam, and it explains S22

**Measured:** peer A (complete set) resolves the subject as **not a member**; peer B, which never
received the ban, resolves it as **a member**. **Neither peer is wrong over the set it holds.**

> **This is the governance-layer source of the delivery-layer behaviour S22 measured.** The stale peer
> that served a banned lineage a `GroupInfo` was not malfunctioning — it was folding an incomplete set
> correctly.

**The two experiments meet here, and the conclusion is stronger than either alone.** S22 measured that
a **negative** standing check fails open at the least-synced peer; G1 shows *why* that peer answers as
it does. So **"keep everyone synced" is not a mitigation at either layer — it is a restatement of the
beam.** The available mitigation is the one S22 measured: **a positive credential, which a peer
verifies from what it already holds and which therefore does not depend on its set being complete.**

**This is now the strongest argument in the corpus for dial position 2.**

### 3.5 Two harness artifacts, recorded because they nearly became findings

Both would have read as "the fold is order-dependent", and both were mine:

1. **Same-device concurrency is not concurrency.** A first draft authored both conflicting facts from
   one device at one lamport; the fold **rejected** the second (`lamport violation… expected > 2, got
   2`) because a device's own clock must be monotonic. Different facts were accepted in different
   orders — an apparent divergence that was pure harness error. **Genuine concurrency requires two
   devices.**
2. **Causally-dependent facts cannot be permuted.** A second draft permuted a removal against the
   addition it referenced; arriving first it was rejected (`missing antecedents: have 0 of 1`) and
   **not retried on this path**, silently losing a removal in three of six permutations.
   `DerivedFold::ingest` is the raw seam and the buffering the module documents lives above it — a
   **harness boundary, not a fold defect**, but only visible because ingest outcomes were surfaced
   rather than swallowed.

**The methodological point:** the first version of this file swallowed ingest results with `let _ =`.
Both artifacts were invisible until that changed. **An experiment that discards error returns cannot
tell "the system resolved differently" from "the system never saw the fact."**
