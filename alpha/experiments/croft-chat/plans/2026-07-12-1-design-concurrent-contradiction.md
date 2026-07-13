# Design note: the §7.6 reconcile hard-stop for concurrent contradiction

`Status: proposal. Motivated by the mutual-expulsion refutation
(mutual_expulsion.rs). Scopes the fix, names the design choices, and flags the one
risk that makes it delicate. No reconcile-semantics code has changed yet; the
concurrency primitive it needs is built and tested (governance::are_concurrent).`

## The finding this addresses

A expels B while B expels A at equal standing (the §2.5 canonical residue). The fold
today:

- does **not** flag it (fork_status stays `clean`), and
- resolves it to a **first-fold-wins survivor**, so the winner is **order-dependent**
  — order `[A⊗B, B⊗A]` → `{O, A}`, order `[B⊗A, A⊗B]` → `{O, B}`.

Two honest peers with different arrival orders reach different membership with no
signal: a silent divergence (I5) on precisely the case §7.6.1 says must hard-stop and
§2.5 says has no coordination-free resolution.

## Root cause (two independent bugs)

1. **Collision detection is genesis-only.** Non-genesis governance facts are placed at
   `gov_seq = current-count`, so two *concurrent* facts get *sequential* slots and
   never collide. The detector keys on same-slot collisions, which only genesis
   (always slot 0) produces. So P20's genesis case is caught and this is not.
2. **Authorization-at-position is order-sensitive for mutual acts.** Whichever remove
   folds first expels the other author, whose counter-remove is then unauthorized and
   dropped. The "resolution" is an artifact of fold order, not a decision.

Both must be addressed: (1) is why it isn't *detected*; (2) is why the undetected
outcome *diverges*.

## The fix, in three layers

### Layer 1 — concurrency (built, low-risk)

`governance::is_ancestor` / `are_concurrent` over the antecedent DAG. Two facts are
concurrent iff distinct and neither causally precedes the other. This is the
reachability primitive; it is pure, tested, and cannot itself change fold behaviour.

### Layer 2 — the conflict predicate (the delicate part)

Concurrency is **necessary but not sufficient**. Two admins concurrently removing
*different* members commute and must stay benign — flagging them would false-trip the
escalation channel, the one thing §7.5.2/§7.6 says must not erode. So the hard-stop
fires only on `are_concurrent(F, G)` **and** a conflict predicate. The §7.6.1 shapes,
made precise:

- **Mutual expulsion.** F removes G's author and G removes F's author (or, more
  generally, F removes a principal whose act G is, and vice-versa). Symmetric, and the
  cleanest to state exactly.
- **Removed-then-included / role thrash.** F removes (or demotes) subject S; G adds (or
  promotes) the same S; F and G concurrent. Here the residue is "is S in or out?" with
  no causal order to decide.

Recommendation: implement **mutual expulsion first** (exact, symmetric, unambiguous,
and it is the §2.5 canonical case), and add removed-then-included as a second,
separately-tested predicate. Keep each predicate narrow and name what it does *not*
cover, so the escalation channel is never widened past a case we can defend.

### Layer 3 — representation and resolution

- **Status.** Add `ForkStatus::Contradiction` (distinct from `ForkedFrom`, which means
  a genesis/​slot collision, and from `UnderDetermined`, the "too few" shape). All three
  are hard-stops; keeping them distinct keeps the surfaced picture legible (§7.6.1).
  Alternatively generalize `ForkedFrom` into a richer `Escalation { kind, … }`; the
  three-variant enum is the smaller change.
- **Resolution = none.** On detecting a conflicting concurrent pair, the fold records
  the hard-stop and does **not** apply the second fact as a silent winner. It surfaces
  both claims (the legible picture) for the humans → fork. No auto-resolve, by §2.5.
- **Hook point.** Same place under-determination is computed — after `apply_governance`
  produces the next state, before it is written (`fold_derived::ingest`, the Step 5.5 /
  fork block). Detecting the conflict needs the incoming fact plus a scan of concurrent
  already-admitted governance facts for the group (bounded by the governance log).

## Interaction with the antecedent guard (already shipped)

Orthogonal, and complementary. The guard holds a fact whose antecedents are *absent*
(incompleteness). This is about facts that are all *present* but *concurrent and
conflicting*. In fact the guard *helps*: because a later fact declaring one side as its
antecedent is held until that side arrives, the conflicting pair tends to meet on one
node, which is exactly where Layer 2 can see and flag it.

## Test plan (mirrors the battery discipline)

- `mutual_expulsion.rs` flips from refutation to verification: both orders must now
  reach the **same** outcome — a `Contradiction` hard-stop — so the order-dependent
  divergence assertion is replaced by "both orders hard-stop, identically."
- New `benign_concurrent_removes.rs`: two admins concurrently remove *different*
  members → must stay `clean` and converge (the false-trip guard — the most important
  negative test).
- `removed_then_included.rs` when Layer 2's second predicate lands.
- Property test: random concurrent governance schedules → no order-dependent divergence
  (every schedule of the same fact set reaches one head or one hard-stop).

## Status (2026-07-12) — Layers 1–3 for mutual expulsion are IMPLEMENTED

- **Layer 1 (concurrency):** `governance::is_ancestor` / `are_concurrent`, tested. Done.
- **Layer 2 (conflict predicates):** BOTH §7.6.1 shapes are now implemented.
  - `detect_mutual_expulsion` — a `MembershipRemove` whose author was removed by a
    *concurrent* remove that it in turn removes (A⊗B). Fires on the unauthorized path.
  - `detect_removed_then_included` — a concurrent add/remove race on one subject (both
    actors survive, so it fires on the *authorized* path).
  - `detect_role_thrash` — a concurrent role race on one subject (authorized-path),
    keyed on the two acts' *resulting role* differing, so it covers grant-vs-revoke and
    grant-vs-grant-to-different-roles alike while same-resulting-role acts stay benign.
    Resolution reverts the subject to its pre-thrash role.
  All require `governance::are_concurrent`, so a causally-*later* act is normal, not a
  contradiction — which is what keeps benign concurrency from false-tripping.
  - **Soundness condition (learned in implementation):** concurrency must be
    *positively established*, which needs a causal claim. A fact with **no antecedents**
    makes none, and `are_concurrent` treats all antecedent-free facts as mutually
    concurrent — so an un-guarded detector false-fires on a bare sequential
    add-then-remove of one subject (it regressed a legacy `test_remove_member_outcome`).
    Both detectors therefore ignore antecedent-free facts: without a causal claim a fact
    is treated as sequential, not a contradiction. In a real deployment governance facts
    always carry antecedents; the empty case is a bare/legacy fact. This is the
    conservative side of the false-trip line: never hard-stop on unprovable concurrency.
- **Layer 3 (status + resolution):** `ForkStatus::Contradiction(hash)` (wire tag 0x03,
  distinct app banner). On detection the fold calls the shared `resolve_contradiction`,
  which recomputes membership by replaying the log in canonical (`merge_cmp`) order
  **excluding the conflicting remove(s)** — contested parties retained, no verdict — and
  flags Contradiction with the *canonical* (lexicographically smaller) pair hash so both
  arrival orders surface the same status. Mutual expulsion retains both authors;
  removed-then-included retains the subject (inclusive default — do not drop a member on
  a contested basis; humans decide via fork).
- **Verified:** `mutual_expulsion.rs` and `removed_then_included.rs` both flipped to
  verification (both orders → contested party retained + identical contradiction status);
  `benign_concurrent_removes.rs` stays green (the false-trip guard holds). Substrate lib
  suite green; croft-chat green.

**Honest residual limit.** Membership and the surfaced status are now order-independent —
the refutation's core (silent, order-dependent *membership* divergence) is closed. But the
full `GroupState.computed_at_gov_head` still records the *triggering* fact's hash, which
differs by arrival order, so the byte-level head/fingerprint of a contradicted group is not
yet identical across nodes. That is a pre-existing property of incremental folding under
concurrency (the head names the last-applied fact), not specific to this fix, and it does
not reintroduce the membership divergence. Canonicalizing the contradicted head fully is a
follow-up.

**Still deferred:** the full §7.6 legible-picture surfacing (who claimed what) beyond a
status flag; byte-identical contradicted-head convergence. The concurrent-conflict lattice
for the current op set (membership add/remove, role grant/revoke) is now complete.
