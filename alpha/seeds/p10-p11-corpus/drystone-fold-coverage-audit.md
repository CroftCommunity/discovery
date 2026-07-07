# Drystone fold-coverage audit

`Status: working audit. Concept walk of the folded companion docs against Parts 1 and 2, to test whether the specs are self-contained.`

The reviewer flagged the companion docs as referenced but not folded. This walks each companion doc concept
by concept and checks whether the spec carries the concept and its reasoning, not just the filename. The
check is concept-level, not phrase-match. A `PRESENT` verdict is shape-level with a section pointer and was
confirmed by grep plus a targeted read; a byte layout may still be deferred under it. A `GAP` means the
concept and its reasoning appear absent from the spec and it is a candidate to fold. Byte layouts may stay
deferred (`[gates-release]`), but the concept and its reasoning have to be in the spec for it to stand alone.
Where a verdict needs a closer read to be sure, it says so.

## Verdict legend

- `FOLDED`: concept and reasoning are in the spec at the cited section, confirmed.

- `PRESENT`: concept is in the spec in shape at the cited section, shape-level check.

- `GAP`: concept and its reasoning appear absent, candidate to fold.

- `DEFERRED`: byte-level encoding intentionally open (`[gates-release]`); the concept itself should still be
  present, and this notes when it is the encoding, not the concept, that is out.

- `RATIONALE/EXAMPLE`: a design-choice justification or a worked trace, not a mechanism; a candidate for the
  "not needed in the spec" bucket.

## 1. governance-finality: folded

`FOLDED` throughout. Its Part A (merge-ready, normative) maps onto Part 2 section by section: A0 sign-the-state
to §7.3.4, A1 fold-folds-quorums to §7.3.1 (the standing-at-vote work), A2 recognizing-a-threshold to §7.3.1,
A3 ceiling to §7.3.5, A4 decision-vs-enactment to §7.3.6, A5 dial and A6 two-phase to §7.3.6, A7 the now to
§7.3.7, A8 fail-closed to §7.3.8, A10 read/enforce to §7.3.3, A11 registers to §7.6.5, A12 layered fold to
§7.3.2, A13 within-tier tiebreak to §7.3.1, A14 who-lands-where to §7.6.6, A15 hold and audience split to
§7.6.7, A16 being-in-both to §7.6.8, A17 posture and dials to §7.6.9. (A2, non-exclusive recognition, folds
into §7.3.6 rather than §7.3.1, corrected on the grounded pass.) Re-verified concept by concept against Part
2's text: Part A is confirmed self-contained with no gap. Part B is open questions tracked in
`open-threads.md`, with B2 already folded as A10 into §7.3.3, and B1's detection-threshold residual and B7's
status discipline folded into the threads on this pass.

## 2. fact-and-chain-representation: fully covered (the two concept gaps now folded at §4.6)

- §1 two chains linked (history chain plus the now, hash-linked both ways): `PRESENT`, §7.3 and §7.3.7.

- §2 the fact as one signed delta addressed by hash (operation, author, observed head, per-author counter,
  signature; content-addressed; deltas not snapshots): `PRESENT` in shape, §7.3.1 and §4.2; canonical byte
  encoding `DEFERRED` (§7.3.1, §4.2).

- §3 canonical dag-cbor and why it is not JSON: `FOLDED`, §4.6. The choice of dag-cbor, the determinism
  argument (map-key ordering, number model, the silent cross-node FactId divergence a non-canonical form
  causes), and the canonical-form-versus-presentation-lens split are now stated; the exact bytes stay
  `DEFERRED` (`[gates-release]`, §4.2).

- §4 signatures in three roles (fact authorship; a decision as k signed facts, not a separate signature; and
  the third role): `PRESENT`, §4.4 (integrity-and-ordering vs authorship-and-standing) and §7.3.4
  (sign-the-state). Worth confirming the third role is named.

- §5 one committed head on the wire, full DAG in local storage, concurrency as derived: `FOLDED`. The
  mechanism (a fact references the observed frontier, transitive closure is happens-before) is in §7.3.1 and
  §7.5.2; on a concept-by-concept re-check the reference-form decision was thin, so §4.6 now carries the
  single-committed-reference-versus-explicit-frontier choice, concurrency as derived, the forbidden
  bare-parent-pointer, and the local DAG index. (The earlier `§7.5` pointer here was wrong; §7.5 is
  attributable acceptance, and the loose reference in §4.6 was corrected.)

- §6 the now's derivation (genesis-derivability floor, checkpoints as local acceleration, corroborated
  checkpoint optional): `FOLDED`, §7.3.7.

- §7 reconciliation format versus local storage, decoupled (the wire is fixed, local storage and derived
  views are free): `FOLDED`, §4.6. The wire-fixed/local-free decoupling, the pruning-behind-a-checkpoint and
  rebuildable-views freedoms, and the three-independent-dials framing are stated, with the shared-state-chain
  contrast.

- §8 the through-line (authenticate on the wire, optimize locally): `RATIONALE`, and now carried directly as
  the framing sentence opening §4.6. No separate spec text needed; §3, §5, §6, §7 are all present.

## 3. history-durability: folded and verified concept by concept

- §C two reconciliation questions, live delivery versus history convergence (set difference, not a push):
  `PRESENT`, §6.8.1.

- §D two tiers on one primitive (members who read; history stores that read nothing): `FOLDED`, §6.8.5 (the
  two tiers stated, both helpers with capability not standing).

- §D the rejected third tier (a structure-aware replica on Willow Confidential Sync, rejected to avoid a
  second crypto primitive and a second threat boundary): `FOLDED`, §6.8.5 ("why not a structure-aware
  replica"), stated as the one-line rationale for the mirror-group choice.

- §E the mirror group G-hist and nested sealing: `FOLDED`, §6.8.5. The store is a member of a mirror group
  G-hist for transport and never of G, content is double-sealed (G-hist transport layer around an
  asset-key-sealed blob), the reason it is mandatory (MLS has no member-who-cannot-decrypt) is stated, the
  envelope rides in the encrypted payload not the AAD, and the optional resumption-PSK binding carries a
  `[confirm]`.

- §F the store's Group Role Set (carriage-and-durability only, mutual-exclusion of every content and
  governance role): `FOLDED`, §6.8.5, with the exclusion now stated and Role-Set-plus-key-withholding named
  as complementary.

- §G the reconciliation envelope and its fields (subspace_id hashed, predecessor_digest, entry_digest,
  counter, size_hint; explicit exclusions of path, wall-clock, capabilities): `FOLDED`, §6.8.5, with the
  fields and the leak profile in prose and the byte layout `DEFERRED` (`[gates-release]`).

- §H device-subspaces and the logical counter (one device per subspace, Willow timestamp as a logical
  counter, lineage-based pool): `FOLDED`. The per-subspace single-writer counter is in §6.8.5 and §7.4.3,
  the one-device-per-subspace rule is explicit there (single-writer per device-subspace), and the
  device-pool-as-one-key-lineage is §5.2.

- §I and §J member-to-member and history-store convergence: `FOLDED`. Member-to-member convergence is
  §6.8.1, and the store-side content-blind chain-level reconciliation over the envelope is §6.8.5.

- §K ordering is content, not trust: `PRESENT`, the timestamp-free, content-addressed ordering of §7.3.1.

- §L pruning and checkpoints: `PRESENT`, §7.7.1 and §7.3.7.

- §M Willow and MLS conformance: `PRESENT`, §7.7 and the Willow and MLS treatment.

## 4. asset-keying: folded and verified concept by concept

- §F the two-anchor content header: `FOLDED`, §7.4.3 (the governance-generation stamp).

- §H the governance/epoch seam and the re-key trigger, retained-copy floor: `FOLDED`. The two-phase and the
  epoch/governance decoupling are §7.3.6, §7.3.7, and §7.9.1; the re-key trigger rule, the
  epoch-not-authority-signal, and the R5-causal-not-temporal partition safety are now in §5.11 (folded on the
  grounded pass).

- §C the layer split, three planes and one identity: `PRESENT`, §6 and §7.

- §D fold-gated key provisioning (a new member is provisioned the content key only after the fold admits it,
  so key access follows governance rather than leading it): `FOLDED`, §5.11 (add-wraps, remove-mints-fresh,
  provisioning-follows-the-fold, quiescence predicate).

- the per-scope asset key itself (the content key each scope is sealed under): `FOLDED`, §5.11 (confirmed a
  genuine gap on read: Part 2 had the read Role as authorization but not the key that enforces it; now the
  read-scoped keying plane, the per-scope asset key wrapped to Role-holders, and siblings-not-hierarchy are
  in §5.11).

- §E one substrate, many convergence models: `PRESENT`, §7.7 (the two dataplane modes and the merge guard).

- §G the concurrency trace: `RATIONALE/EXAMPLE`. A worked trace; not needed as spec text.

- §I why no continuous group key agreement is required (MLS's key schedule suffices; no separate continuous
  key-agreement protocol): `FOLDED`, §5.11.

## 5. The real gaps, gathered (for the missing-versus-not-needed conversation)

Folded so far: the read-scoped content-key model, per-scope asset keys, fold-gated provisioning, and
no-continuous-key are in `§5.11`; the content-blind history store, its mirror group and nested sealing, its
Role Set, and the reconciliation envelope are in `§6.8.5`; canonical dag-cbor with the why-not-JSON argument
and the reconciliation-format-versus-local-storage decoupling are in `§4.6`. The fold Chase requested is
complete: every genuine miss identified in this walk is now in the two specs, with byte layouts still
`[gates-release]` where noted.

Nothing remains to fold. What is left is by design not spec text:

- The **concurrency trace** (asset-keying §G) and the **through-line synthesis** (fact-and-chain §8, now
  carried as the framing of §4.6). Worked reasoning and synthesis, not normative text. The rejected-third-tier
  rationale is folded as one line in §6.8.5.

Shape-level confirms worth a later pass, none blocking:

- The **third signature role** named at §4.4/§7.3.4, the **store Role Set exclusion** (now stated at §6.8.5),
  and the **one-device-per-subspace rule**.

Confirm before acting:

- Resolved: the per-scope asset key was a genuine gap and is folded (§5.11). Three shape-level items remain
  worth a confirm: the third signature role, the store Role Set exclusion, and the one-device-per-subspace
  rule.
