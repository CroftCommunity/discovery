# Revocation & membership authority — the threshold dial and its order of operations

date: 2026-06-16 · updated 2026-06-17
status: **decided** (order-of-operations) — see "Decision" below; freshness coupling stays design input for Job 4

## Decision (2026-06-17) — CO-SIGNED OP is the canonical mechanism; PROPOSAL+VOTES is an optional deliberative mode

For threshold (k>1) membership authority, **pattern (A) the co-signed op is the default and the only
mechanism v0 must implement; pattern (B) proposal+votes is deferred as an opt-in, per-group
"deliberative mode," not built until a use case needs in-flight vote visibility.**

**Why A is canonical:**
1. **It is already green-real over the wire.** C-faithful-revoke (2026-06-17) carries a real
   `gov::SignedOp` k-of-n bundle over live iroh-gossip and the receiver validates it with
   `meets_threshold_by_lineage` (AUTHORIZED accept / UNDER-THRESHOLD reject). Pattern A *is* that bundle.
   Pattern B's vote-accumulation state (expiry, retraction, stale-vote rejection) is explicitly unmodeled
   (Open edges). Decide for the proven thing; defer the unmodeled thing.
2. **Self-certifying matches the project's spine.** A removal op carries its own proof (k signatures over
   a pre-image that names the current epoch); every member validates it locally from signed data alone —
   the same "authority from signed data, never from assertion or observed side-state" rule the faithful
   path (I3) rests on. B requires observing accumulated vote state, which is side-state and
   partition-sensitive.
3. **One broadcast, no per-group tally state → scales and is partition-tolerant.** The bundle either meets
   the threshold against the validator's current epoch or it does not; there is no cross-member vote
   ledger to keep consistent under churn.
4. **B does not solve the hard case.** The genuine residue — two partitions each gathering a local quorum
   — is identical for A and B and is resolved by the existing green-real **hard-stop on the
   `RemovedThenIncluded` contradiction** + **freshness gating** (don't act on a removal authorized against
   a stale epoch). B's transparency buys nothing there.
5. **Auditability is not lost.** The co-signs and the resulting op live in the replicated log, so the
   decision is auditable *after the fact* under A. B's only extra is *real-time* visibility of an
   in-progress vote — valuable for deliberative/contentious decisions (e.g. admitting a disputed member),
   not for the common revoke. So B is the right tool *only* when a group opts into deliberation, and it
   layers on top without changing A.

**Mechanics of A (v0):** the proposer requests co-signs from k authorized signers (each validates the
proposed op against its **own current-epoch** policy and returns a signature over the epoch-naming
pre-image), then broadcasts `Remove{sigs[k]}`. A co-sign whose pre-image names a stale epoch is rejected
on validation — so freshness gates A naturally (cleaner than B, whose accumulated votes may span epochs).
The MLS Commit that advances the epoch and re-keys without the target is the *membership* half; the
signature bundle is the *authorization* half. Both are validated independently by each member.

**What this does NOT decide:** vote-accumulation semantics *if and when* B is ever built. (The
membership-op freshness *threshold* and the admin-floor rule — listed open here originally — are now
**decided 2026-06-17**: see the ADMIN FLOOR block immediately below, and `freshness-signal.md`
MEMBERSHIP-FRESH.)

---

## Decision (2026-06-17) — ADMIN FLOOR = threshold-satisfiability + a never-irrevocable role ladder

The admin floor is **not a separate dial**. It is *derived from the policy*: a group must always
retain enough authority to satisfy its own current threshold, and a threshold may never be set beyond
what the group can meet.

**THRESHOLD-SATISFIABILITY (the spine rule).** A threshold `k_op` **MUST** be ≤ the eligible signers
counted by **distinct lineage** (never device leaves) at the epoch it takes effect.
- *Genesis:* `k_op ≤ founding-roster size`. A solo genesis ⇒ every `k_op = 1`. A group **MAY** be born
  with a large roster and a high bar ("create with 10, need 5") — born matured.
- *Raising later:* an ordinary governance op, valid only if `n ≥ new_k` at the effective epoch.
  Raising a threshold above the current headcount self-bricks the capability and is **rejected**.
- This dissolves the bootstrap paradox: a solo founder cannot create a "5-to-add" group and then add
  under a 1-vote ramp, because solo ⇒ k=1. To get a real 5-to-add group you either declare ≥5 founders
  at genesis (born matured) **or** raise k once you actually have 5 members. **No provisional phase, no
  ramp, no maturation lock** — those edge cases are designed out, because requirements must be meetable
  at the time they are established.

**THE FLOOR (`n ≥ k`), post-set.** Once `k_op` is in force, the group **MUST** retain ≥ `k_op` eligible
lineages to keep that capability. A membership op whose post-state drops below the floor is
**structurally invalid** — rejected by every verifier from replicated policy state alone, regardless of
valid signatures (deterministic, same spine as the §7 hard-stop). Removing a floor-critical member is
valid only as an **atomic replace** (remove+add) preserving `n ≥ k`. `k` is bounded by `n` at
*set-time* going up, and held ≤ `n` by the floor going down — it **MUST NOT auto-track `n` downward**
(auto-tracking would be a threshold-downgrade attack: shrink to lower the bar, then admit a slate).

**Direction asymmetry (safe by construction).** *Tighten is easy* — raising `k` is authorized under the
current (lower) bar; tightening your own group is not an attack. *Loosen is hard* — lowering `k` needs
the current (higher) bar; you cannot unilaterally weaken a matured threshold.

**Per-operation-class thresholds.** `k_add`, `k_remove`, and `k_policy` (who may change the rules) are
separate dials — a design aesthetic and a need. "Who can adjust the vote count" is `k_policy`.

**The floor is ANTI-BRICK ONLY — capture is not structurally blocked.** A legitimate quorum acting
within policy (A+B remove C under 2-of-3; or A+B raise then strip down to a captured 1-of-1) is
**accepted** — that is what threshold authority *means*. The recourse for an out-voted minority is the
§7 **re-formation fork** (vote with your feet), never a structural veto. Capture ≠ brick: even a
deliberate capture bottoms out at a *governable* (not frozen) group.

**Attrition is out of scope.** The floor governs *ops*, not *attrition*. If members simply lose devices
below `k` (the T12 shadow), the capability is liveness-stuck; recourse is recovery (T12) +
materially-reversible re-host (§6) + the fork (§7), surfaced via freshness (§9) — not prevented here.
Running at exactly `k` is therefore fragile — keep headroom.

**Roles are delegated authority, never irrevocable — including the creator's.** The creator holds **no**
structural superuser right. At creation they are granted a **bootstrap admin role** purely so a
one-member group can function (while solo, `k=1`, they act alone); that role is a normal **revocable
delegation**, special in no other way and strippable by the ladder below. A group **MAY** also
designate the creator (or any peer) a longer-lived revocable admin delegation — "more management
expectations," scoped enumerated rights, **disclosed to members on join** — for a parent/kids group, a
customer-service room, whatever the group chooses. It is never a power that cannot be removed.

**Anti-entrenchment ladder (always-true).** No delegated role can entrench against a united group, even
if its own grant tried to make itself irrevocable:
1. **Routine** — revoke under `k_policy`.
2. **Backstop (always available)** — **unanimity of the non-holders** (all members except the
   role-holder, by lineage) revokes the role, *regardless of how the grant was configured*. This is the
   **ceiling** on revocation difficulty: a group MAY set an easier bar, never a harder one. It is the
   in-group instantiation of §6 "delegation MUST be materially reversible," and it breaks any
   "you need me to change the rules and I won't let you remove my power" lock.
3. **Fork (ultimate)** — §7 re-formation: vote with your feet. Always available if even the backstop is
   gridlocked (non-holders cannot reach unanimity under partition/attrition).

The ladder is **general over every delegated role** — creator, admin, and the content/infrastructure
roles **meer** (§8.1) and **geer** (§6.1) alike. If all other members vote to strip a meer or geer, it
is stripped. In the case of a co-op or external authority *operating* a geer/meer, the group then
**detaches** from that operator and becomes a differently-shaped group — an outcome that could not have
been prevented anyway (the operator can always leave); the protocol's contribution is narrow and
specific: it **preserves history and provenance** up to the detachment and legitimizes/erases nothing
retroactively. This is the in-group expression of §6's "delegation MUST be materially reversible"
(re-host / stand-up-and-elect a different holder), with the fork as the backstop.

In a 2-person group the non-creator alone *is* "all but the creator," so a single vote strips the role
— they continue as equals or the group ends. Guiding principle: **sane management must be possible,
transparent, and rooted in delegated-but-never-irrevocable roles, to the benefit of the group as a
whole; users can always vote with their feet.** Tests: experiment suite **group I** (specified, not
yet run).

---

date: 2026-06-16
status: thinking (design input for the wire spec's governance section and for Job 4 freshness)

## Problem

MD-G5 demonstrated revocation *mechanics* over the wire (a marker propagates over the NAT path,
survivors enforce it, pre-revoke history is retained). But the marker's **authority** is a
sha-256 MAC keyed to the group genesis — anyone who knows the `group_id` can mint it. That proves
the revoke is well-formed and group-scoped; it does **not** prove the issuer is *allowed* to revoke.
"Who can kick whom" was left asserted, not verified. We need a normative model for revocation (and
add) authority that scales and survives partition.

## Approach

**Authority is a threshold-of-authorized-signers policy — the same primitive for add and remove.**
A membership op (add or remove) is authorized iff it carries signatures meeting the group's current
policy. The policy is a dial the cooperative sets per use case:

- **1-of-any** (default): any member may remove/add any member.
- **k-of-any**: any two/three members in agreement.
- **k-of-n role-restricted**: only designated admins, k of them.

This is the green-real **T3 threshold-signed checkpoint** shape applied to a membership op: we have
already proven threshold-signed governance ops verify. Revocation authority = "a removal op bearing
signatures that satisfy the current threshold policy."

**The policy lives in replicated, versioned group state (the admin chain / MLS epoch state).** Every
member already holds it, so a removal is validated **locally against the validator's current epoch**
— no synchronous "pull the admin chain before sending" round-trip in the common case. Policy changes
are themselves governance ops, authorized under the *current* policy; the chain extends itself.

**Order of operations** — two patterns, chosen by the threshold:

```
 k = 1 (any member removes)              k > 1 (threshold agreement)
 ─────────────────────────              ───────────────────────────────────────────
 proposer ──Remove(sig)──▶ all          A) CO-SIGNED OP (self-certifying)
 each member validates sig vs              proposer gathers k co-signs off-band,
 current-epoch policy, applies,            then ──Remove{sigs[k]}──▶ all; everyone
 re-keys WITHOUT the target                validates the bundle locally.
 (target can't read new epoch)             One broadcast, no vote-tracking state.

                                        B) PROPOSAL + VOTES (eventual, visible)
                                           Propose(remove X) ──▶ all-but-X; members
                                           ──Vote──▶ ; when k accumulate (observable
                                           to all) the Commit lands. Slower, auditable.
```

The intuition "send the kick to everyone NOT that person for validation" is the **MLS re-key step**:
the Commit advances the epoch and re-keys without the target, so the target is cryptographically
excluded going forward — that is the *membership* half. The *authorization* half is the signature
bundle (A) or accumulated votes (B), validated independently by each member. **(A) scales better**
(single broadcast, self-certifying); (B) is more transparent but needs vote-accumulation state and is
eventual.

## Reasoning

- **Why policy-in-state, not pre-pull:** a synchronous admin-chain fetch before every message does
  not scale and adds a partition-sensitive round-trip. Replicating the policy makes the common case a
  local check; only the *op* (the co-signed removal) crosses the wire.
- **Why threshold over ad-hoc:** "true peers are equals" means no built-in superuser; equality is
  expressed by making authority a *quorum* of equals, dialable from 1 to k. The same machinery
  expresses "anyone can add" and "two must agree to add."
- **The hard residue is partition.** Two partitions can each gather a local quorum and remove the
  other's members (concurrent membership change). This is exactly the green-real
  **`RemovedThenIncluded` reconcile contradiction**, resolved by the **hard-stop** (no silent merge;
  surface it). It is *why revocation couples to freshness* (Job 4): a member must not authorize a
  membership change evaluated against a group view it cannot prove is current. **Freshness gates
  authority** — don't act on a removal authorized against a stale epoch.
- **Relation to the faithful path (Task 7):** Task 7 proves the *per-message* authorship+standing
  gate over the wire (valid signature but no standing → rejected). Threshold *removal authority* is
  the next layer — a governance op composing the T3 threshold primitive — and is specified as its own
  normative section in the wire spec, not folded into Task 7.

## Open edges

- **Vote-accumulation state (pattern B)** under churn/partition is unmodeled — when do votes expire,
  can a vote be retracted, how is a stale vote rejected?
- ~~**Removing an admin** (last admin / quorum-breaking removals) needs a floor rule.~~ **DECIDED
  2026-06-17** — ADMIN FLOOR above (threshold-satisfiability + `n ≥ k` floor + never-irrevocable role
  ladder; anti-brick only, capture → fork). Tests: suite group I.
- **Policy-change races** (two concurrent policy edits) reduce to the same reconcile contradiction;
  confirm the hard-stop covers them. *(still doable — open-edges §3.)*
- ~~**Freshness threshold for membership ops** specifically.~~ **DECIDED 2026-06-17** — MEMBERSHIP-FRESH
  (`freshness-signal.md`; CROFT §9): originate/co-sign requires strict CURRENT + corroboration; content
  ungated; apply gated by epoch-chain + §7. Tests: suite group H.
