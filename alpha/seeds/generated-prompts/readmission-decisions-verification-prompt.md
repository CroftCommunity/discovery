# Handoff prompt: adversarially verify the readmission decision talk-through against the original specs

`Written 2026-08-17, at the close of the decision talk-through (decisions 1–4 ratified; merge
pending). Copy everything below the line into a fresh session.`

---

You are the **independent reviewer** for a decision conversation that ran 2026-08-16/17. Another
session talked through five readmission/governance decisions and recorded its reasoning into a
WORKING spec copy, backlog rows, a build plan, and a state file. Your job is to **double-check that
work against the two original Drystone specs** — verify every claim's grounding, catch every
overclaim, miscitation, or contradiction with the canonical text, and report. **You change nothing
except your own report file.** The owner decides what gets fixed.

## Orient (read in this order)

1. `discovery/beta/drystone-spec/part-1-reasoning-underpinnings.md` — canonical Part 1, in full.
2. `discovery/beta/drystone-spec/part-2-certifiable-design.md` — canonical Part 2. Read closely:
   §4.2, §5.7–§5.9, §6.2–§6.8, §7.2–§7.6, §7.9, §8, §10.2, §11 entire, Appendices B, D, E.
3. `discovery/beta/drystone-spec/part-2-certifiable-design-WORKING-2026-08-16.md` — the revision
   surface under review. The banner indexes two passes; `grep 'REV 2026-08-1'` (catches both
   dates) is the authoritative diff. Pass-2 blocks (the talk-through's output) are the review
   subject; pass-1 blocks were reviewed in the prior arc but citations you happen to check are
   in scope.
4. `discovery/beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md` and
   `SCENARIO-WALK-2026-08-16.md` — the measured findings the decisions rest on.
5. `discovery/alpha/ROADMAP_TODO.md` rows **E94, E96, E105, E106, E107, E108, E109, E110, E111**.
6. `discovery/alpha/plans/2026-08-16-1-plan-token-reentry-proveout.md` — the S23–S26 build plan.
7. `discovery/alpha/experiments/meer-queue/STATE-AND-NEXT.md` — the handoff state.

**Ground truth ordering:** test code (`discovery/alpha/experiments/meer-queue/tests/s*.rs`,
`discovery/alpha/experiments/croft-chat/croft-chat/tests/fold_ordering_keys.rs`) beats TEST-LOG.md
beats the dossier/walk beats the WORKING copy's REV prose beats the ROADMAP rows. Where any two
disagree, the more-upstream one is right and the disagreement is a finding. RFC 9420 / RFC 9750
claims may be checked against the RFCs themselves where you can; otherwise flag as
`[unverifiable here]` rather than assuming either way.

## Standing rules (do not relitigate, DO verify citation)

The measured findings of S15–S22 and G1 are settled: do not re-run or dispute the measurements.
But the talk-through's **readings of** those measurements are fully in scope — if a REV cites S19
for a claim S19 does not support, that is a finding. Withdrawn claims (S14's "neither mechanism
applies"; S22's "small enumerable serving tier"; G1's "key 1 fails open") must not be resurrected
by any artifact under review; if one is, that is a finding. The owner's ratified decisions are
decisions — you do not get to reverse them — but you may and should flag where a decision's
*recorded rationale* misstates the spec or the evidence.

## The claims to verify, enumerated

Work through these one at a time. For each: locate the claim in the artifact, locate its claimed
ground (spec section, test, RFC), and judge: **CONSISTENT / OVERCLAIM / MISCITATION /
CONTRADICTS-SPEC / UNSUPPORTED-INFERENCE**, with a one-line proposed correction where not
consistent.

### A. E106 ratification (WORKING §11.7 REV; ROADMAP E106)

1. **Scoping claim:** resumption PSKs remain valid on member-side paths — verify §7.6.2/§7.6.3
   (re-plant/ReInit linkage), §11.9.3.3 (parallel-group healing), §6.8.5 (G-hist MAY) actually
   use resumption PSKs on paths where the store is populated, and that S16's blocker is
   specifically the external-commit path (`s16_governance_attestation.rs`).
2. **"I was there" homed on the chain:** verify §7.6.2 ("membership history composes over gaps…
   two spans") and §7.6.4 (ban-vs-voluntary artifacts) support the claim that membership
   intervals are chain-discernible; verify the caveat (routine PCS rotations roll epochs with no
   chain fact — §7.3.7) is accurately stated.
3. **Three-way cross-check binding:** the claim that redemption requires chain issuance fact +
   leaf credential resolving to the lineage + attached PSK. Check the *mechanics* half: does
   RFC 9420's PreSharedKey proposal processing actually require every processing incumbent to
   resolve the PSK from its own storage (the "token ledger" constraint), and does S16's test
   setup corroborate (did the incumbent resolve the PSK from its own provider store)? This
   constraint drives S23; if it is wrong, say so loudly.

### B. Cold-is-a-state (WORKING §11.6 REV)

4. **Two-phase dormancy:** phase 1 (absent-but-seated → ordinary forward catch-up) — verify
   against S10/S15 and the MLS commit-chain model; phase 2 boundary at eviction; retention ≥
   liveness window as S15's limbo fix (`sweep_with_retention`).
5. **Universal rebuild path:** the claim openmls does not distinguish evicted from stranded
   (S15) and rebuild-on-`UseAfterEviction` as normal path (S20 / pass-1 REV).
6. **What was dropped vs kept:** the REV keeps eviction and the liveness window, drops
   cold-as-second-Group. Verify no pass-2 text accidentally contradicts §11.4's scaling claim
   or the owner's hot-tree constraint.

### C. Walk-in mechanics and artifact properties (WORKING §11.7 DECISION-2 addenda)

7. **External-commit mechanics as narrated:** KEM against `external_pub` → bridge/init secret;
   joiner supplies fresh path entropy; new epoch = f(both); joiner reads nothing at E or
   earlier. Check against S19 and, where possible, RFC 9420 §12.1.6.
8. **"Every `GroupInfo` carries `external_pub`, unstrippable"** — S19 (§1.5 of the dossier).
9. **`external_pub` is re-derived per epoch** — this was asserted from the key-schedule
   derivation; verify independently (RFC 9420 key schedule → external_secret). If wrong, the
   perishability claim weakens to context-binding only — still real (S20's refusal), but the
   REV's "two independent ways" sentence would need correction.
10. **Tree contents claim:** leaf HPKE/signature keys + credentials + internal-node public keys,
    no decryption capability, not inferable from the social roster. Verify nothing in the tree
    as served leaks more than claimed (note RFC 9420 §16.4.3 roster leak IS claimed).
11. **Epoch-perishability:** an external commit on the epoch-E pair is refused past E (S20);
    the self-closing-serve claim (a successful join is itself the expiring commit).
12. **Pull/push split and the advertising rule** (`GroupInfo` freely / tree never / combo only
    through the door): check the directory-grade framing against §7.4.2 (GroupInfo as claim
    corroborated against the chain) — specifically the added requirement that a directory entry
    pair with a chain anchor.

### D. Serve/merge layering and catch-up rules (WORKING §11.7 DECISION-2 addenda; plan S24–S26)

13. **"Serve protects the roster; merge protects the membership"** and its corollary ("under
    strict merge, the worst outcome of any bad serve is roster disclosure, never admission").
    Adversarially probe: is there ANY path by which a leaked `GroupInfo`+tree yields more than
    roster knowledge when every member applies the merge rule? (Consider: partition scenarios,
    members who auto-apply before policy evaluation in real libraries, the joiner reading
    traffic on their own one-person branch.) If a hole exists, name it.
14. **Admission-at-position:** the claim that §7.3.1's authorization-at-causal-position rule
    legitimately extends to admission evaluation, and that §7.4.3's locator-not-authorization
    discipline covers the AAD-carried position. Check the extension is faithful, not a stretch.
15. **Live-edge refusal vs catch-up correction:** check coherence against §7.6.12 (two-phase
    revocation), §7.6.2 ("an epoch roll carries no inherent social meaning"), and §11.8's
    re-fire. Adversarial question: does "process structurally, correct forward" ever require a
    member to *cryptographically key* for an invalid member in a way §5.7/S18's
    who-you-key-for-is-your-group would forbid? If the tension is real, flag it as a design
    question the merge prose must address.
16. **§7.3.8 applied to the merge:** is "merging a `NewMemberCommit` is an irreversible
    authority-enforcing action" a faithful application of the finality gate's own definition,
    or an extension needing its own normative sentence at merge time? Either answer is fine —
    say which, because it decides whether the merge prose can cite §7.3.8 or must amend it.

### E. E108 ratification (WORKING §7.3.2 REV)

17. The recorded grounds: (a) manufactures a mini-verdict; (b) untyped absence; (c) closes the
    divergence by construction and repairs the "hard-stopped + member" incoherence. Verify
    against G1 §3.1a (scenario-walk Part 3) and §7.6's presentation language. Check the claimed
    scope ("only subjects named by an open contradiction pair") is well-defined against the
    contradiction artifact G1 actually produces (`contradiction:{byte-head}` — does it name the
    subject pair in a machine-readable way the projection can key on? If not, that is a real
    implementation gap to flag for the schema change).

### F. E96 ratification and the style principle (WORKING §6.4 REV)

18. **The leak profile correction:** verify the S7/S17 figures (28 flat bytes; `group_id`
    absent; non-parse; dedup/byte-identity/catch-up survive; `AeadDecryptionError` refusal)
    against `s7_carol_carries.rs` / `s17_nested_sealing.rs` and the E96 ROADMAP row.
19. **The wrapping rule** as stated (wrap at the queue's epoch; closing commit at the epoch it
    closes; silent deadlock reversed) — against S17's failing-side verification.
20. **The attribute-conditioned-MUST form:** check the defaults (ON for confidential-on-fabric;
    OFF for all-member-carrier and public-regime) for coherence with §6.11 Mode 2 and §11.9.3,
    and the `carrier-visible` declared attribute against E94's metadata-transparency guard.
    Check the interaction with §6.6.2's byte-identical rule (the meer must store the
    outer-sealed object unchanged — confirm S17 exercised this).
21. **The general style principle** (attribute-conditioned MUST over SHOULD): scan canonical
    Part 2 for SHOULDs that carry conformance weight and would now be candidates for conversion
    — produce a short list (section + the SHOULD + whether a declared attribute could replace
    it). This is input to the merge, not a demand to convert them all.

### G. E109 and the regime-transition reading

22. Verify the quoted §8 posture ("regime and visibility are born at genesis and immutable…
    no silent regime crossing… a republish is a distinct authored act") against canonical §8's
    actual sentence, and that the re-plant-shaped transition sketch does not contradict §7.6.2's
    arities or §7.6.6's placement rules.

### H. The conformance sweep and the build plan

23. Re-derive the decision-2 conformance sweep independently (met / changed / cannot-meet) for:
    §11.7 self-service + progressive return; §11.6 batching; §11.8 interlock; Part 1 §2.4
    quiescence ("eviction and issuance ride one commit, so no new liveness assumption" —
    adversarially check this); §7.3.6. Flag any expectation the recorded sweep missed.
24. **Coverage audit of S23–S26:** for every claim the pass-2 REVs mark as testable or
    preliminary, name the experiment arm that discharges it — or report the claim as
    **uncovered**. Particular attention: the lineage binding (A/3 above), token revocation
    semantics, the token-ledger transfer to post-issuance joiners, `CONTESTED`'s arrival-order
    pinning (which lives in croft-chat, outside the plan — confirm that's stated, not lost).

### I. Internal consistency of the session's own artifacts

25. Banner vs blocks (does every pass-2 banner entry have its block, and vice versa); WORKING
    REVs vs ROADMAP rows vs STATE-AND-NEXT vs the plan — no two artifacts asserting different
    statuses for the same item. Verify canonical `part-2-certifiable-design.md` and Part 1 are
    **untouched** (`git -C discovery status` / `git -C discovery diff --stat` — only the
    WORKING copy, ROADMAP, plan, prompts, and STATE-AND-NEXT should have changed).

## Output

Write your report to
`discovery/beta/drystone-spec/REVIEW-decision-talkthrough-2026-08-17.md`:

- **Verdict summary first** (plain English, a paragraph: is the work sound, and what are the
  worst three findings).
- Then one row or short block per numbered item above: verdict
  (CONSISTENT / OVERCLAIM / MISCITATION / CONTRADICTS-SPEC / UNSUPPORTED-INFERENCE /
  UNVERIFIABLE-HERE), the evidence you checked, and the proposed correction where needed.
- A closing section: **anything load-bearing the talk-through missed entirely** — an
  expectation neither the sweep nor this checklist covers.

**Edit nothing else.** Do not fix findings in place, do not touch the WORKING copy or canonical
specs, do not commit or push. Plain-English-first throughout; every claim you make carries its
evidence pointer; anything you could not verify is labeled so, never silently assumed.
