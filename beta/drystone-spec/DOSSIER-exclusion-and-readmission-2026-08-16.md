# Dossier: exclusion and readmission — what MLS actually does, and what §11 has to become

date: 2026-08-16
status: **findings + rework scope. Not normative.** The spec is
`part-2-certifiable-design.md`; the revision surface is
`part-2-certifiable-design-WORKING-2026-08-16.md`.
evidence: `alpha/experiments/meer-queue/tests/s15_*.rs` … `s20_*.rs` (22 tests, Rung A) ·
`alpha/experiments/meer-queue/TEST-LOG.md`
provenance: `alpha/seeds/transcripts/raw/mls-exclusion-readmission-experiments-2026-08-12.md`
backlog: `alpha/ROADMAP_TODO.md` **E96, E105, E106, E107**
plan: `alpha/plans/2026-08-14-1-plan-readmission-and-groupinfo.md`

---

## The one-paragraph version

Six experiments (S15–S20) interrogated whether the delivery design's model of exclusion matches what
MLS does. **The base-layer model is confirmed:** a governance ban is a forced fork, and at N = 10 the
nine survivors share new key material while the banned member derives none of it. **But the spec uses
the identical mechanism for two opposite intentions** — migration to cold (§11.6) and a ban (§11.8)
are both removal commits, mechanically indistinguishable. The thing that was supposed to tell them
apart, §11.6's *"linked by the MLS resumption mechanism"*, **cannot be built** on the reference
library. So today **cold and banned are the same state**, the graceful path is unbuilt and the strict
path is unenforced, and both for one reason: nothing serves `GroupInfo` under a standing check.

---

## Part 1 — What was measured

All Rung A (real OpenMLS 0.8.1, real CISS where storage is involved). Fidelity notes are load-bearing
and are reproduced rather than summarized away.

### 1.1 The base-layer model is correct (S19, S20)

**An epoch roll is a genuine cryptographic exclusion.** Each commit mixes new path entropy with the
prior epoch's `init_secret`. A removed member holds neither the entropy nor a leaf on the re-keyed
path.

At **N = 10**, one governance removal commit:

- all ten members agreed on the derived queue name **before** the ban;
- the **nine survivors all derive the same new key material**;
- the **banned member derives none of it** and is stranded on the pre-ban key, unchanged;
- the roster goes to nine.

> This is not "roll and include everyone". Exactly one leaf was excluded from the re-keyed path, and
> it was the intended one.

**Measured at the strong grade.** A removed member is simply *behind*, so a naive test yields
`Message epoch differs from the group's epoch` — a **counter** check a merely-lagging member would
also hit. Advancing her own stale branch until both sides sit at the **same epoch number** converts
it into a **key** check: `An error occurred during AEAD decryption.` **That is the measurement the
model deserves; the weaker one proves nothing.**

### 1.2 There are three post-exclusion states, not two (S20)

| state | what she holds | how she got there |
|---|---|---|
| 1. never sees the removal | live object, stale key | she was offline |
| 2. **processes the removal** | **dead object — `UseAfterEviction`; no derivation at all** | she synced |
| 3. rebuilds from a `GroupInfo` | fresh object, current keys | external commit |

**Nothing carries from (1) or (2) into (3).** That is the mechanical reason the ban's key-layer work
cannot influence the re-entry path: the returner reuses nothing.

> **The well-behaved client gets the worst outcome.** A client that syncs and processes the commit
> has its group object marked inactive by OpenMLS. For a **ban** that is correct. For a **dormancy
> migration** it is a defect — the graceful path punishes exactly the clients that behave.

### 1.3 The exclusion does not prevent re-entry, because re-entry never derives (S19)

RFC 9420's external commit obtains the current epoch's `init_secret` by a KEM against the
**`external_pub`** key published *inside the `GroupInfo`*. **Prior membership contributes nothing**,
which is why destroying it prevents nothing.

Measured the sharpest way available: a returner on a **completely fresh provider** — no stored group
state, no prior epoch secrets, nothing carried over — joined a live group and the incumbent merged
it.

```
  DOOR 1 — derivation                    DOOR 2 — external join
  ───────────────────                    ──────────────────────
  epoch N+1 = prior init_secret          KEM against external_pub,
              + new path entropy           published IN the GroupInfo
  removed member has neither             → current epoch's init_secret
  → CLOSED by the roll                   → NEVER DERIVES, so the roll
                                            cannot close it
```

**Exclusion is passive-reading exclusion. Re-entry is an active protocol operation gated on a
published key.** Reading the first as implying the second is reading a guarantee the protocol does
not make — which is precisely why §11.8 is right to put ban enforcement in an application-layer gate.

### 1.4 Re-entry is self-admission, not a request — and the window is the lagging member (S20)

The banned member **asks nobody for anything.** She takes a `GroupInfo` from a member who has not yet
synced the ban, and unilaterally constructs a commit seating herself.

> **There is no request, so there is nothing for a member to deny.** The gate cannot be a permission
> prompt.

Measured: her re-entry commit **was** admitted by the member who had not synced the ban, and was
**not applicable** at a member already past it — that member has superseded the epoch her commit was
built on, and refuses it without any policy being consulted.

> **The exposure window is not "anyone can let her back in". It is precisely: the set of members
> whose view predates the ban.**

**So the gate is not at the ban and not at the re-entry — it is at the moment a `GroupInfo` is
served.** The ban's *enactment* is instantaneous and total for key material; its *enforcement*
against re-entry is only as fast as the ban reaches whoever will hand out a `GroupInfo`.

### 1.5 There is no "safe" `GroupInfo` (S19)

`export_group_info` offers exactly **one** option, `with_ratchet_tree`. Measured across both
settings: with the tree a stranger gets in; without it the refusal is specifically the missing
**tree**, never a missing `external_pub`. `export_group_info_with_additional_extensions` documents
that it **errors** if a `RatchetTreeExtension` or `ExternalPubExtension` is supplied directly.

> **Every `GroupInfo` a member can produce carries the external-join key.** There is no way to prove
> current group state without also admitting the holder.

**The ratchet tree is therefore the only dial** (S18): the same member, same removal, same epoch is
refused on a tree-less `GroupInfo` and admitted the moment the tree is bundled. The export flag is
**per call**, independent of the group's `use_ratchet_tree_extension`, so this is enforced at the
**serving node** and no group-wide setting can defeat it.

**Bandwidth agrees with governance:** S8 measured the tree roughly doubling `Welcome` (330 vs 152
bytes/member) and crossing the 2 MiB cap first, at N ≈ 6,350.

### 1.6 Refusal holds, at two independent layers (S18)

A member who declines to merge a re-entry is protected without anyone's cooperation:

- **Keys** — a message the refuser seals afterwards fails for the returner with a real AEAD
  decryption error (measured at the strong grade, same epoch number, different branches).
- **Addressing** — the refuser's queue name is not one she can derive. His mail sits at an address
  she cannot even ask for; her drain returns empty.

> **Who you key for is your group, and nobody can force you to key for someone.** Confirmed
> mechanically.

### 1.7 But a fork is invisible in the epoch counter (S18)

After one member accepts and another declines and each advances once, both hold the **same epoch
number** and **different secrets** — measured by divergent derived queue names.

> **A client cannot detect a fork by comparing epochs.** The only symptom is that peers silently stop
> being able to read each other.

The returner did not split the group. **The disagreement about the returner split the group.** Which
is why the readmission rule must be a group-wide policy decided in advance: **a dialog box asking
each member "allow her back?" is a partition generator that hides its own damage.**

### 1.8 §11.7's two-part credential is not implementable as written (S16)

- **Standing half — no protocol mechanism.** A party who was never a member, never invited, holding
  no group secret, joined on a `GroupInfo` alone and the incumbent merged it. **This confirms §11.8
  rather than correcting it** (see Part 2).
- **Key half — unreachable API.** A resumption PSK **cannot be attached to an external commit** on
  openmls 0.8.1. Resumption PSKs resolve from the group's own `ResumptionPskStore`
  (`schedule/psk.rs:530-537`); a group built by external commit initialises that store **empty**
  (`commit_builder/external_commits.rs:290`); and its `add` is `pub(crate)`.

**What does work, measured end to end:** a **governance-issued external PSK**. It resolves from
provider storage, attaches to the external commit, is visible to the incumbent as a countable
`psk_proposals()` entry **before merging**, and the merge seats the returner. **It carries both
halves at once** — possessing it proves the governance issued it (standing), and it binds into the
key schedule so it cannot be claimed without being held (keys). Its one honest difference from a
resumption PSK: it proves **the governance vouched for you**, not **that you were there**.

**The policy hook is complete and pre-merge** (S16): AAD survives byte-exact, the sender is
distinguishable as `NewMemberCommit`, and the joiner's credential is readable — all before
`merge_staged_commit`. Declining left the group unchanged.

**Two limits.** The AAD is signed by the joiner's own new leaf key, so it authenticates the *carrier*
and never the *claim* — the attestation must be a governance-issued token verified out of band. And
refusal is not consensus (§1.7).

### 1.9 What re-entry costs, bounded both ways (S18)

- **Forward:** a re-seated member is a **full member at the current epoch** — she sealed a message a
  member read in cleartext, and read a member's message sealed after her return. **Not a
  stale-lineage ghost.**
- **Backward:** she recovers **no** history. *Stated precisely:* this surfaces as an epoch/ordering
  rejection and **cannot be made into a decryption test**, because every pre-re-entry message is by
  construction at an older epoch. The structural reason she could not decrypt regardless is that an
  external commit derives the new epoch and nothing earlier.

> **Exposure is strictly forward-looking: everything from admission onward, nothing before it.**

### 1.10 Nested sealing works and costs 28 flat bytes (S17, E96)

`group_id` is verbatim in a bare MLS envelope (S7 reproduced by grepping bytes) and **absent** under
an outer seal, which no longer parses as MLS. **28 bytes overhead** (12-byte nonce + 16-byte tag),
**measured flat** at 64 KiB. Routing, dedup, M2 byte-identity and the catch-up walk all survive; a
non-member is refused with a real `AeadDecryptionError`.

**One new discipline — the wrapping rule:** wrap with the key of the epoch whose **queue** carries the
object, so the commit that *closes* an epoch is wrapped at the epoch it closes. **Verified from the
failing side:** backwards, the walk deadlocks silently and looks like a corrupt object. OpenMLS
exports the current epoch only, so the API cannot prevent this mistake.

### 1.11 Limbo, and retention as a Group value (S15)

A member absent past retention but inside the liveness window is **simultaneously** seated in the hot
Group, holding a watermark of lost mail, and able to name **exactly one** queue — the stale one.

**Correction to S14:** limbo is *escapable* (§1.3). **But the escape needs a `GroupInfo`** — see Part
2.

Constructively: with retention set to the Group's liveness window the same absence costs nothing.
Landed as `Meer::sweep_with_retention(days)`, because §11.6's windows are per-Group (90 days at
250–1k down to 14 at 7–10k) and **a service constant can only ever suit the most aggressive band.**

---

## Part 2 — What this means for the spec

**Three of the six experiments largely confirmed §11.** The delivery work had been treating §11 as
out of scope and independently re-derived several of its conclusions — useful corroboration, but it
produced a false sense of missing capability. **Reading the spec removed more work than the
experiments added.**

| finding | spec position | effect |
|---|---|---|
| MLS admits a stranger on a `GroupInfo` alone | §11.8 **already says this** | **confirms** at Rung A; a ***Design*** claim becomes measured |
| A removed member re-seats the same way | implied, never stated | confirms; names the window |
| The re-seated member is **current** | **§11.8 says the opposite** | **normative correction** |
| No history recovered through MLS | §11.7 supplies it via the DAG + archival key | the DAG mechanism is **load-bearing, not optional** |
| Refusal holds at two layers | not stated | **new**; supports the §11.8 gate |
| Fork invisible in the epoch counter | §11.11 item 3 knows forks can happen | **sharpens** — no cheap local signal |
| Resumption PSK unattachable | §11.11 item 6 flags the pattern as unproven | **sharpens** unproven → **blocked in the reference impl** |
| Admission surface is the ratchet tree | §11.11 item 6 notes the HCS conveys the tree | **promotes** plumbing to a governance control point |
| Nothing serves `GroupInfo` | §11.6/§11.11 name the **history-convergence node** | **retracted** — an unwired seam in the delivery doc, not a spec gap |

### 2.1 The one normative error

§11.8 (and **Appendix E, L6**) argue eventual-consistency ban propagation is safe because *"the
removal re-keys and the banned lineage is cut from new entropy, having gained only prior entitlement
(**zero marginal exposure, since a returned lineage briefly re-holds only keys it already had**)."*

**Measured false** (§1.9). She receives **new entropy** and reads traffic sent after her return.

Proposed replacement:

> A returning lineage admitted before a ban propagates re-keys to the **current** epoch and receives
> new entropy. Its exposure is therefore **everything sent between admission and re-exclusion**, not
> zero. The window is bounded by ban-propagation latency and hot-Group commit liveness — the floor
> §11.8 already owns — and the safety argument rests on that window being short, not on the exposure
> being absent.

**The surrounding argument survives.** "Dormancy cannot evade a ban" holds; "no consensus on
membership required" holds. What changes: **exclusion latency becomes a confidentiality parameter,
not only a liveness one** — which *strengthens* §11.8's own conclusion that commit liveness is the
thing to monitor.

### 2.2 The structural problem: one mechanism, two opposite intentions

**Migration to cold (§11.6) is a removal commit. A ban (§11.8) is a removal commit.** S20 measured
that exactly one thing happens either way, with identical durability and the same three post-states.

```
   migration to cold ─┐
   (unintentional)    │  removal commit → excluded forward,
                      ├─►  same durability, same three states,
   ban ───────────────┘     same re-entry path

   ONLY distinguisher:      standing at head (§11.8 governance chain)
   ONLY place to apply it:  who is served a GroupInfo (S20 §1.4)
```

**The key layer cannot tell them apart**, so the graceful path cannot be made easier there and the
strict path cannot be made harder there.

**And the thing that was supposed to distinguish them does not work.** §11.6 says the two Groups are
*"linked by the MLS resumption mechanism (§11.7, §11.8): a client's prior membership in one is
provable when re-entering the other."* That linkage is the cold Group's entire reason for existing —
a parking place you can walk back out of, as distinct from expulsion. **S16 measured that it cannot
be built.**

> **Today, mechanically, cold and banned are the same state.** The distinction exists in the prose
> and nowhere in the code.

### 2.3 The constraint that closes one design option

The obvious repair — *don't evict dormant members, just mark them dormant* — is **not available.**

> **Owner, 2026-08-16:** *"hot tree size is exactly the thing being managed here for the most part in
> terms of hot<>cold so that's not great."*

§11.4's scaling claim is that cost scales on the live set. Keeping dormant leaves in the hot tree
spends exactly what the split buys. **So the graceful path must be built on top of eviction, not by
avoiding it.**

*(Also settled: there is no "warm" tier. §11.6 defines **hot and cold** only.)*

---

## Part 3 — Walking the mechanisms we have against what §11 needs

Nothing below is new machinery. Each row is something already measured working, matched to a need
§11 states.

| §11 needs | mechanism we have | measured | gap |
|---|---|---|---|
| Cold members cannot read hot traffic | removal commit re-keys the path | **S20** — 9 agree, 1 excluded, AEAD-grade | none |
| Cold ≠ banned (a distinction that means something) | **standing at head** on the governance chain (§11.8) | resolution order is ***Design*** in §7.3.1 | **not wired to any admission point** |
| A returner re-establishes current keys "at its own cost" | external commit | **S15, S18, S19** — works, incl. from a fresh provider | needs a `GroupInfo` |
| Something serves the returner a `GroupInfo` | **history-convergence node** (§11.6, §11.11 item 6) | exists in spec; **unwired** in the delivery design | must resolve standing before serving |
| Prove prior membership on return | resumption PSK | **S16 — unreachable API** | **replace** with governance-issued external PSK |
| Prove present standing on return | governance attestation | **S16** — AAD carries it, pre-merge, byte-exact | self-asserted; needs a governance-issued token |
| A member can refuse a returner | decline to merge | **S18** — holds at keys *and* addressing | refusal forks (§1.7) |
| The group agrees in advance so refusal doesn't fork | **group-context extension** | not built | the readmission policy's home |
| Returner recovers the dormancy gap | history DAG + archival key (§11.7) | MLS supplies nothing (**S18**) | **load-bearing, not optional** |
| Narrow who can re-enter | **withhold the ratchet tree** | **S18, S19** — refused without it, admitted with it | policy at the serving node |
| Carrier cannot bucket by conversation | outer seal (E96) | **S17** — 28 flat bytes, nothing else affected | adoption decision |
| Retention ≥ liveness window | `sweep_with_retention` | **S15** | per-Group governance value |

### 3.1 The shape this suggests

**Same cryptography everywhere; the entire difference is a policy applied at one point.**

```
   returner presents itself
        │
        ▼
   ┌─────────────────────────────────────────────────────┐
   │  GroupInfo server (the history-convergence node)     │
   │  resolves standing AT HEAD on the governance chain   │
   └─────────────────────────────────────────────────────┘
        │                              │
   standing intact                standing revoked
   (cold / dormant)               (banned)
        │                              │
        ▼                              ▼
   serve GroupInfo               refuse; re-entry requires
   + ratchet tree                quorum action to restore
        │                          standing first
        ▼
   self-service external commit,
   immediate, costs the group nothing
        │
        ▼
   ┌─────────────────────────────────────────────────────┐
   │  merge-time policy at current members — BACKSTOP    │
   │  group-context extension, agreed in advance:        │
   │    sender == NewMemberCommit ?                      │
   │    psk_proposals() carries a known governance token?│
   │    AAD attestation verifies against the credential? │
   │  → merge, or drop (same answer at every member)     │
   └─────────────────────────────────────────────────────┘
```

**Why the serving node is the primary gate and merge-time is the backstop:** S20 measured that a
member already past the ban cannot apply the returner's commit at all — it is built on a superseded
epoch and is refused with no policy consulted. **The only members who *can* admit her are those whose
view predates the ban**, and those are exactly the members who would also serve a stale `GroupInfo`.
Gating the serve closes the window at its source; gating the merge closes it at members who mostly
cannot be reached by it anyway.

### 3.2 This is a dial, and it should be specified as one

Drystone already treats several of these as **dials for implementation — both product and user
choice**. Readmission is one, and framing it as a rework of §11.6–§11.8 understates it. The spec's
job is to **define the positions, say what is enforceable at each, and say what is impossible at
every position**. Croft's job is a *separate* decision: which position it ships by default, and which
it exposes to users.

**The dial: who is served a `GroupInfo`, and on what showing.** Every position below uses identical
cryptography (§2.2). They differ only in policy at the serving node, plus an optional merge-time
backstop.

| # | position | serve `GroupInfo` to | tree bundled | merge-time check | grounded? |
|---|---|---|---|---|---|
| **0** | **Open return** — *an ungated server* | anyone who asks | always | none | **fully measured** — S16, S18, S19, S20 |
| **1** | **Standing-checked** | standing intact at head | after standing check | optional backstop | **window measured** (S20); the check itself **untested** — no server exists |
| **2** | **Vouched** | holders of a governance-issued token | after token check | require known PSK + attestation | **mechanism measured** (S16); **not end-to-end as an admission decision** |
| **3** | **Closed** — *no server; where bare MLS sits* | nobody; return requires a member's `Welcome` | n/a | n/a | **`Welcome` path measured** (S12, S21); **cost shifts to an active member**, breaking §11.7's self-service claim |

> **Correction to an earlier framing in this document (owner challenge, 2026-08-16).** Position 0
> was described as "what you get if you build nothing". **That is wrong, and it made the dial look
> disconnected from S20's result.** Building nothing means **nobody serves a `GroupInfo` at all** —
> which is *position 3*, and it is where a bare MLS deployment naturally sits. The re-key already
> produces a fresh `GroupInfo`, and if it only ever circulates among the included set, exclusion is
> **de facto durable**.
>
> **Position 0 is what you get once you build the `GroupInfo` server that §11.7's self-service return
> requires, and then do not gate it.** The dial exists *because* the graceful path needs a door that
> the strict path then has to control. **That is the whole tension**, and S20's nine-persist result
> and position 0 are not in conflict: the ban's *enactment* excluded her totally, and position 0
> describes what happens afterwards if anything will hand her a current `GroupInfo`.

### 3.2.1 Invites and external joins are different problems with different gates (S21)

The owner's model — *"C can invite D, but A and B still need to accept and agree to key for D"* — is
**correct about the decision and wrong about the mechanism**, and the difference decides where a gate
can live.

**There is one shared secret per epoch, not per-member keying** (S21). A, B and C derive the
*identical* epoch secret; there is no operation that encrypts to A and B but not C. So "agreeing to
key for D" is **not a per-member act** — the only decision available is whether to be in an epoch
that contains D. Measured: after B merged the commit seating D, a message B sealed *for the group*
was read by **D** in cleartext. **To exclude D after merging requires a new commit removing D. There
is no lesser move.**

**But the decision is real, and MLS already has the phase for it.** C's Add **proposal** for D left
the roster unchanged at every recipient — a proposal seats nobody, rolls no epoch, and grants no
keys. Only when a member **commits** it does D get anything, and at that moment A, B, C and D all
hold one secret.

> **propose → (governance decides) → commit.** This is the protocol-level form of the spec's
> decide-then-enact split (§7.3.6), available today, and it is where the invite gate belongs.

**And this is exactly what an external join lacks.** Measured: it arrives as a `StagedCommit` from
`NewMemberCommit` — a **commit, never a proposal**. The joiner performed both halves itself, so
**there is no pending-proposal phase to gate.**

| path | gate available | where |
|---|---|---|
| **C invites D** | **yes — the proposal phase** | in-protocol, before anything changes |
| **outsider seats herself** | **no proposal phase exists** | only: who is served a `GroupInfo`, + a merge-time policy every member evaluates identically |

**Conflating these two is what made the readmission discussion confusing.** "Members must agree" is
straightforwardly available for invites and structurally unavailable, *as a request*, for external
joins. **So all-members-can-invite is a feature to manage, not a hole to prevent** — the owner's
reading — and it is managed in the proposal phase. The external-join path is the one that needs the
dial.

#### What is enforceable at each position

| position | enforceable | by what | residual |
|---|---|---|---|
| 0 | nothing | — | anyone with a `GroupInfo` is a member |
| 1 | exclusion of a **banned** lineage | standing resolved at head before serving | **propagation lag** — the window is exactly the set of members whose view predates the ban (S20 §1.4) |
| 2 | exclusion **+ attribution** of who vouched | external PSK binds into the key schedule; countable pre-merge | token distribution and revocation become real work |
| 3 | full admission control | an active member must act | **self-service return is lost**; cost falls on the group, not the returner |

#### What is NOT enforceable at ANY position — measured, and it belongs in the spec

These are the honest floor. No dial setting reaches them:

- **You cannot serve a `GroupInfo` that does not admit its holder** (S19 §1.5). Every one carries
  `external_pub`; the API refuses to hand-manage the extension. The only lever is **withholding the
  ratchet tree** and **choosing who is served**.
- **You cannot stop a member leaking one.** A single member in good standing can hand a `GroupInfo`
  and tree to anyone. Position 1 and 2 constrain *the server*, never *a member*.
- **You cannot force a member to key for someone** (S18 §1.6) — nor should you; this is the property
  the owner named, and it holds at keys and addressing both.
- **You cannot make per-member refusal safe** (S18 §1.7). It forks, and the fork is invisible in the
  epoch counter. This is why the merge-time check must be a **group-context policy agreed in
  advance**, never a prompt.
- **You cannot prevent a merged returner reading forward** (S18 §1.9). Exposure runs from admission
  to re-exclusion; only the *window* is controllable.
- **You cannot recover the dormancy gap through MLS** (S18 §1.9). §11.7's history-DAG-plus-archival-key
  is load-bearing, not an optimization.

> **The spec should state these as the achievable set.** A reader who assumes the epoch roll closes
> re-entry (§1.3) will over-trust every position on the dial.

#### What needs testing before the dial can be specified

Grounded in what exists today, three gaps:

1. **Position 1 end to end** — a serving node that resolves standing at head and refuses. Nothing in
   the spike resolves standing at all; only the *window* is measured.
2. **Position 2 as an admission decision** — S16 measured the external PSK attaches, binds, and is
   visible pre-merge. It has **not** been measured as "the group refuses a commit lacking a known
   token", which is the thing the position actually claims.
3. **The propagation window, quantified.** S20 measured *that* a lagging member admits her. It did
   not measure how long that lasts under realistic sync, which is the number that decides whether
   position 1 is adequate on its own or needs position 2's backstop.

### 3.3 What this makes of hot/cold

Cold stops being *a place* and becomes *a standing state with a delivery consequence*:

- **hot** — in the tree, keys current, mail flows.
- **cold** — evicted from the tree (so §11.4's scaling is preserved), **standing intact**, therefore
  entitled to a `GroupInfo` on request. Re-entry is self-service and immediate.
- **banned** — evicted from the tree, **standing revoked at head**, therefore not entitled to a
  `GroupInfo`. Re-entry requires restoring standing first.

The cryptographic treatment is identical in all three. **The linked-Groups framing — cold as a second
MLS Group joined by resumption PSK — is what S16 refutes, and it is what the rework should drop.**
Whether a literal second Group is still wanted for any *other* reason (cold-side messaging,
governance participation while dormant) is a live question this dossier does not answer.

---

## Part 4 — What Croft does within the dial (a separate decision)

**The spec says what is possible and enforceable. Croft decides what it ships.** Keeping these apart
matters: a Drystone implementation that chose position 0 would still be conformant, and Croft's
choice should be defensible on its own terms rather than smuggled into the protocol text.

**Nothing in this section is decided.** It is the shape of the decision, with the evidence each
option would rest on.

### 4.1 The default position

The argument for **position 1 (standing-checked) as Croft's default**:

- It is the cheapest position that makes "we removed them" mean anything (§3.2).
- It preserves §11.7's self-service return for the **common** case — dormancy — which is the case
  that actually happens, at the frequency §11.6's windows imply.
- Its residual is a **latency window**, which §11.8 already owns and already says to monitor.

The argument against, and it is real: **position 1 is untested** (§3.2), and its enforcement depends
on a component that does not exist. Shipping it as a default before the serving node resolves
standing means shipping **position 0 wearing position 1's label**.

> **So the honest sequencing is: build the standing check, measure it, then claim the default.** Not
> the reverse.

### 4.2 What Croft exposes to a Group

Candidate user-facing controls, each mapping to something measured:

| control | what it actually sets | grounded in |
|---|---|---|
| "Anyone who was here can come back" | position 1 — serve on standing intact | S20 (window), needs the check |
| "Returns need a moderator's say-so" | position 2 — serve on governance token | S16 (external PSK works) |
| "Only an invite gets you back" | position 3 — no server; `Welcome` only | S12 (handshake measured) |
| retention window | `sweep_with_retention` | S15 |
| — *not* a control: per-return prompts | would fork the group | **S18 §1.7 — do not offer this** |

**That last row is the one worth defending.** The obvious product instinct is a notification: *"Carol
is trying to rejoin — Allow / Deny."* S18 measured that as a partition generator whose damage is
invisible in the epoch counter. **Croft should not ship it**, and the spec should say why so that no
implementation reinvents it.

### 4.3 The defaults that are not really choices

Two things Croft should do regardless of dial position, because the failure mode is silent:

- **Serve `GroupInfo` without the ratchet tree by default** (§1.5). Costs nothing, narrows the
  surface, and bandwidth agrees (S8).
- **Set `read_class: owner` at inbox creation** (S12). Unset is world-readable; this is provisioning,
  not hardening.

### 4.4 The product question the measurements raise but do not answer

**A dormant member who syncs gets a dead group object** (§1.2). For a ban that is right. For dormancy
it means the well-behaved client is the one that has to rebuild.

Croft can hide this — detect `UseAfterEviction`, resolve the `GroupInfo`, rebuild silently, and the
user experiences "a slow refresh" rather than "you were removed". **Whether it *should* is a product
decision with an honesty dimension**, since the same silent repair would hide a ban as well as a
dormancy migration. Recorded as an open question, not resolved.

---

## Part 5 — Open, and owned

1. **The two spec corrections** (§11.8, Appendix E L6) — drafted (§2.1), **not applied**. The
   canonical file is untouched by design.
2. **Does the cold Group survive as a Group at all**, or become a standing state? §2.2 removes its
   stated mechanism; it does not by itself decide the question.
3. **Who runs the `GroupInfo` server, and the trust asymmetry.** The owner's meer-as-a-view-of-the-HCS
   framing gives a natural home. Recorded deliberately: **a meer holds no keys; an HCS in your pool
   holds group secrets**, so one operator running both is a design choice, not a convenience.
4. **A fork signal that is not the epoch counter** (§1.7). Cheapest candidate: peers compare a
   derived value they already hold — the group queue name is exactly such a value.
5. **Adopt nested sealing, or not** (E96). Cost measured; decision open.
6. **`[UNVERIFIED]`:** whether the CISS assertion plane has a payload size limit. Searched, none
   found. Bears on the still-open inbox-plane decision.
7. **Three dial positions need grounding before the spec can specify them** (§3.2): position 1 end to
   end, position 2 as an admission *decision* rather than a mechanism, and the propagation window
   **quantified** rather than merely demonstrated.
8. **Croft's default is sequenced behind the standing check** (§4.1) — claiming position 1 before
   building it ships position 0 under the wrong name.
9. **Not addressed here:** §11.11's **gap-completeness** beam. Everything in §3.1's merge-time
   backstop is sound *given* gap-completeness and unproven without it. That beam is Part 2 Appendix
   B's and is not discharged by any of this.
