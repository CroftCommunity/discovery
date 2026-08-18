# Independent review: the readmission decision talk-through, verified against the canonical specs

`Written 2026-08-17 by the independent-review session commissioned in`
`alpha/seeds/generated-prompts/readmission-decisions-verification-prompt.md. Review method: the`
`reviewer read canonical Part 1 and Part 2 in full, then verified the 25 enumerated claims against`
`the canonical specs, the dossier/scenario-walk, the s-series test code, TEST-LOG.md, and (where`
`reachable) RFC 9420/9750 primaries. Verdict vocabulary per the commission: CONSISTENT / OVERCLAIM /`
`MISCITATION / CONTRADICTS-SPEC / UNSUPPORTED-INFERENCE / UNVERIFIABLE-HERE. This file is the only`
`artifact this review created or modified.`

---

## Verdict summary

**The work is sound.** Across all 25 items, no decision's recorded rationale contradicts the
canonical specs, no measurement is misreported, no withdrawn claim (S14's "neither mechanism
applies", S22's "small enumerable serving tier", G1's "key 1 fails open") is resurrected anywhere,
and the epistemic discipline is generally excellent — the artifacts hedge where the evidence hedges,
and the RFC-facing mechanics of the walk-in narration checked out cleanly against RFC 9420 itself
(in one place the talk-through's wording is *more* RFC-accurate than the dossier prose above it in
the ground-truth order). The ratifications of E106, E108, E96, and cold-is-a-state are faithful to
what was measured and decided. The findings below are mostly scope and citation corrections for the
merge pass, plus a handful of genuine composition gaps the build plan or the merge prose must
address before decision-2 text graduates.

**The worst three findings:**

1. **A banned ex-member exits holding the entire token ledger, and nothing reconciles this with
   "possession proves issuance" or door B's "fails CLOSED at a stale peer"** (item 3's missed
   issue). The ledger obligation (RFC-verified, real) makes every member's token group state held
   by every incumbent — so possession is universal among past members, the PSK's standing
   contribution collapses against exactly the adversary bans target, and a banned lineage's
   pre-ban token *is* in every stale peer's ledger while its revocation is precisely the chain fact
   the stale peer has not synced. S22's measured "fails CLOSED" used a tokenless stranger; for the
   banned-ex-member population the flat claim is unproven and likely false at the serve gate,
   leaving the strict merge as the only backstop. No artifact states the corollary: post-ban
   ledger hygiene (does a ban force re-issuance of every member's token?).

2. **The absolute "under strict merge, the worst outcome of any bad serve is roster disclosure,
   never admission" is stronger than §7.3.8 claims for itself** (item 13). Canonical §7.3.8's
   freshness "corroborates but does not prove" completeness (the Appendix B beam), so an eclipsed
   member whose corroboration set falsely attests freshness merges — a bad serve composed with an
   eclipsed corroboration set *is* admission. The corollary needs the qualifier "up to §7.3.8's
   corroborated-completeness residual," and "only the roster" undersells the non-admission
   outcomes (refused-fork processing load / partition-bait, credential correlation).

3. **E108's ratified scope — "only subjects named by an open contradiction pair" — is not
   implementable on the contradiction artifact G1 actually produces** (item 17). The artifact is
   `contradiction:{hash}` where the hash is the lexicographic *min* of the two conflicting
   envelope hashes: no subject principal, no pair, and `ForkStatus` is a single group-level slot
   that cannot represent two simultaneously open contradictions on different subjects. The REV
   owns "a schema change" but names only the member-view third state; the prerequisite change —
   the contradiction artifact itself must carry the pair as data, in a *set*, plus a
   resolution-fact type that does not exist anywhere — is real work the ratification presupposes.

Close behind: the family-group dial paragraph in the cold-is-a-state block is a genuine overclaim
(item 6) — "effectively infinite liveness window → every return is cheap phase-1 catch-up, no meer
required" stitches a phase-1 claim to a phase-2 citation (S22 measured GroupInfo serving, not
commit-stream retention), and §11.5's mandatory strict-PCS heals below 250 members make missed
epochs grow with absence duration, voiding S10's own boundedness argument for the infinite-window
case. And two canonical §11.8 paragraphs (the ticket/attestation era-consistency MUST; the
cross-chain-ordering subsection) are orphaned by the ratified decisions with no REV marking them —
a merge that folds only marked blocks ships dead normative text (item 23).

**One meta-finding about this review's own commission:** checklist item 23's quoted claim —
"eviction and issuance ride one commit, so no new liveness assumption" — **appears nowhere in the
artifacts under review**. The record honestly says the near-opposite (token delivery to an absent
member is named as the open question; at-join issuance is the §2.4-clean answer). The quote was
apparently synthesized when the verification prompt was written. It made the artifacts look
weaker than they are; other quotes in handoff prompts should be spot-checked against their
sources rather than trusted.

---

## Per-item findings

### A. E106 ratification (items 1–3)

**1. Scoping claim (resumption PSKs remain valid on member-side paths) — CONSISTENT, one unearned
sub-claim flagged.** Canonical §7.6.3 carries the ReInit/branch resumption-PSK linkage (RFC 9420
§11.2/§11.3/§8.6, verified: the reinit Welcome "MUST specify a PreSharedKeyID of type resumption
with usage reinit", delivered member-side to holders); §6.8.5's G-hist binding is accurately
characterized as a MAY; S16's blocker is measured as external-commit-specific
(`s16_governance_attestation.rs:170–253`, cause pinned to `external_commits.rs:290`). **Flag:** for
the two *cross-group* list members (§11.9.3.3 healing, §6.8.5 G-hist), "whose PSK store is
populated" is an untested assumption that S16's own resolver detail cuts against — openmls resolves
`Psk::Resumption` from the group's **own** `ResumptionPskStore`, which would not hold another
group's PSK, and `add` is `pub(crate)`. Canonical carries `[confirm]` / "not yet examined" flags on
exactly these two; the REV drops them. *Correction: scope "store is populated" explicitly to
ReInit/branch; mark §11.9.3.3 and §6.8.5 as RFC-valid but implementation-unexamined.*

**2. "I was there" homed on the chain — CONSISTENT.** §7.6.2's "membership history composes over
gaps… two spans" and §7.6.4's ban-vs-voluntary artifact distinction support the claim exactly as
stated; the PCS-rotation caveat is accurately grounded in §7.6.2's "an epoch roll carries no
inherent social meaning" (verbatim at canonical line 1282). One navigational note: the checklist
pointed at §7.3.7 for the caveat, but canonical §7.3.7 ("the now") states the *converse* decoupling
(governance-without-epoch); the caveat's true home is §7.6.2 with §7.3.6. The REV cites no section
for it, so there is no miscitation in the artifact — but merge prose should cite §7.6.2, not §7.3.7.

**3. Three-way cross-check / token-ledger mechanics — CONSISTENT; the S23-driving constraint is
real and RFC-verified.** RFC 9420 verified affirmatively at four points: §12.4.2 (a processing
member derives `psk_secret` from the actual PSK values — structural, not policy), §12.4.3.1 (a new
member lacking a referenced PSK must error), §16.5 ("the application needs some external
arrangement that ensures that the legitimate members of the group have the required PSKs" — the
ledger obligation nearly verbatim), §12.4.3.2 (continuing members need the PSK). S16 test 3
corroborates (the incumbent's provider store is seeded before processing); S16 has no negative arm
and the WORKING says exactly that ("visible in S16's setup… Untested"), with S23 correctly
RED-first. The three-way binding is labeled "design, untested" with §11.11 item 4 left standing —
no overclaim. **Missed load-bearing issue: the ledger/possession/ban composition (verdict summary
item 1).** *Correction: reconcile "possession proves issuance" with universal-possession-under-
ledger; scope the door-B "fails CLOSED" claim by population (stranger vs banned ex-member); state
the post-ban ledger-hygiene question (re-issue on ban?) — candidates: an S23 arm or an explicit
S25 note.*

### B. Cold-is-a-state (items 4–6)

**4. Two-phase dormancy — CONSISTENT, two citation-precision notes.** Phase 1 matches S10
(forward in-order replay measured; skip-ahead refused) and S15 test 3 (adequate retention → drain
and current, "no re-entry, no GroupInfo, no cost to anyone else"); `sweep_with_retention` exists
(`src/meer.rs:261`) and "a per-Group governance value, never a service constant" is a faithful
quote of the landed finding. Notes: (a) the most on-point measurement for "queues unnameable after
eviction" is S14 §2 (TEST-LOG:999–1006), which the REV avoids citing (S14 carries the superseded
clause) — the S9/S13 citation supports the claim only by composition; (b) "S15's limbo fix"
(singular) compresses S15's own log ("not only retention ≥ liveness… also: something must serve
GroupInfo") — the block carries the second half in the next paragraph, so the block is complete
but the label alone is not. *Correction: cite S14 §2 (or S20) for phase-2 unnameability at merge;
phrase retention as "half of S15's limbo fix."*

**5. Universal rebuild path — CONSISTENT, one evidentiary gap.** S15 test 2 measured the
seated-side non-distinction verbatim; the evicted side is measured in S14 §3 / S18 / S20. The
pass-1 REV's SHOULD-not-MUST split (measured state vs inferred handling) is exactly right and
licenses the flatter phrasing. Gap: **no test asserts the `UseAfterEviction` variant** — it
appears only in a doc comment, a println, and TEST-LOG; the s20 helper discards the error via
`.ok()`, so what is measured is "export_secret fails after processing own removal" for one
operation. *Correction (experiment side): one `assert!(matches!(err, …))` closes it.*

**6. Dropped vs kept — structure CONSISTENT; family-dial paragraph OVERCLAIM.** Nothing in pass-2
contradicts §11.4's scaling or the hot-tree constraint; the drop matches dossier Part 3.3 and the
open question correctly points at dossier Part 5 item 2; no withdrawn claim resurrected. The
individual citations in the family-dial paragraph are accurate (S10 measured
governance-events-not-messages; S22 measured every-member-serves). **The composition outruns
them:** (i) S10's "cheap" rests on N being "bounded twice over," and the second bound is the
retention cap — an effectively infinite window (with the block's own retention ≥ liveness rule)
voids it, while canonical §11.5 **mandates** periodic strict-PCS Updates below 250 members, so
missed-epoch count grows linearly with absence duration — multi-year family absence is not "a
handful of hops"; (ii) S22 measured members serving **GroupInfo** (the phase-2 rebuild artifact),
not the retained commit stream phase-1 needs — in a no-meer group no measured component stores or
serves that stream, so "every return is phase-1 catch-up … with no meer required (S22)" stitches a
phase-1 claim to a phase-2 citation; (iii) at window = ∞, "the tree holds only live clients" is
vacuously true and the tree holds the roster — harmless at N=6, but say so. *Correction: qualify
at merge — per-hop cost measured; N under an infinite window grows with §11.5's heal cadence
(unmeasured at that horizon); "no meer required" is measured for the serve/rebuild path, and
no-meer phase-1 catch-up needs a member-side commit-stream retention/serving mechanism no S-test
has measured (or state that long family absences exit via rebuild + DAG backfill, which makes the
token load-bearing, not "pure backstop").*

### C. Walk-in mechanics and artifact properties (items 7–12)

**7. External-commit mechanics — CONSISTENT, one loose sentence.** KEM → bridge/init secret, fresh
path entropy (external commits MUST contain a path, RFC §12.4.3.2), E+1 = f(both) (Fig. 22), joiner
reads nothing at E or earlier (S19 + s18 lines 320–333) — all check out against RFC 9420 §8.3
directly. The WORKING's "bridge secret" framing is *more* RFC-accurate than the dossier §1.3 / S19
test prose, which say the KEM "obtains the current epoch's init_secret" (per §8.3 the joiner sends
a *new* init_secret; it never obtains E's). Loose sentence: "the commit is a **proposal** for E+1
that does nothing until members process it" — (a) S21/TEST-LOG make "commit, never a proposal"
load-bearing in the opposite direction; (b) "does nothing" is per-member true but group-wide
misleading: one merge suffices (S22: "she needed exactly one yes"), and "forked themselves into a
branch of one" holds only when *no* member merges. *Correction: "a commit that takes effect at
each member only when that member processes it," plus the one-yes-suffices asymmetry. Also fix the
dossier/test "obtains the current epoch's init_secret" wording — ground-truth ordering puts those
above the REV.*

**8. "Every GroupInfo carries `external_pub`, unstrippable" — CONSISTENT at implementation scope;
OVERCLAIM at protocol scope.** The dossier's headline is scoped ("every GroupInfo **a member can
produce**", i.e. openmls 0.8.1); the WORKING drops the qualifier. At the RFC level the ExternalPub
extension is optional (§8: "can be published"; §12.4.3.2 requires it only where external commits
are wanted). Secondary: the REV (c) headline "no GroupInfo proves group state without admitting
its holder" preserves the S19 framing that S22 §3 corrected (bare GroupInfo proves state, admits
nobody). *Correction: "every GroupInfo openmls 0.8.1 can produce; protocol-optional,
implementation-unstrippable," and align the headline with S22 §3.*

**9. `external_pub` re-derived per epoch — CONSISTENT, independently RFC-verified.** RFC 9420 §8
Table 4: `external_secret` is an epoch-derived secret; `external_priv/pub =
KEM.DeriveKeyPair(external_secret)`; epoch_secret fresh each epoch; §12.4.3.2 confirms GroupInfo
is epoch-specific. The "two independent ways" framing stands; no weakening needed. (Footnote: a
path-less proposal-only commit does not mutate leaves, but every commit rolls the epoch and hence
`external_pub` — perishability unaffected.)

**10. Tree contents — OVERCLAIM (mild); conclusion survives.** "No decryption capability" holds
(all public keys). But the tree as served also carries per-member `capabilities`, leaf
`extensions`, KeyPackage lifetimes, and structural metadata (blank nodes, `unmerged_leaves`) —
"leaks exactly the roster" undersells it, and RFC §16.4.3 treats the leak as
credential-dependent. "Every value is generated randomness" is false as quantified — credentials
*are* identity data. The operative conclusion (the tree can only be handed over, never inferred —
S18 measured withholding works) survives via the key material. *Correction: "the key material is
generated randomness"; "leaks the roster plus per-client capability/lifetime metadata and
membership-history structure."*

**11. Epoch-perishability + self-closing serve — perishability CONSISTENT (measured, S20);
self-closing serve is Verified-RFC-grounded inference, currently untagged.** The self-closing
claim is near-verbatim RFC §12.4.3.2 ("each GroupInfo object can be used for one external join"),
so it should be tagged ***Verified-RFC*** + inference — the talk-through paragraphs carry no
evidence-grade tags at all (see the missed issue below). Overstatement inside it: "**immediately**
expires every outstanding copy" — expiry propagates with the join commit; a member still at E
accepts a different external commit on the same pair, so the residual window is served-but-unused
*plus* propagation lag (the same lagging-member window S22 measured). *Correction: "expires every
outstanding copy at every member that processes the join commit; laggards remain the window, per
S22."*

**12. Pull/push split and the advertising rule — split CONSISTENT; the §7.4.2 cite is an
extension presented as "the same rule" (mild MISCITATION).** Canonical §7.4.2's setting is a
*rejoining member* corroborating against the chain **it already holds**; the WORKING applies the
posture to an *outsider* doing directory-grade verification and adds anchor-pairing. The
claim-not-authority posture carries; the rule is new, and an outsider has no fold — how they trust
the anchor is an open problem §7.4.2 does not solve. *Correction: "extends §7.4.2's
claim-corroborated posture to the outsider case (new requirement; outsider anchor-trust open)."*

**Missed issue (group C): the walk-in/talk-through addenda (WORKING ~2262–2267) are the only REV
blocks in the §11.7 area carrying no evidence-grade tags**, while mixing Measured, Verified-RFC,
and Design sentence-by-sentence. Given the DECISION-2 header's own "no prose may claim otherwise"
discipline, these untagged paragraphs are where a merge will most easily promote an inference to a
measurement. *Correction: tag per-sentence at merge.*

### D. Serve/merge layering and catch-up rules (items 13–16)

**13. "Serve protects the roster; merge protects the membership" — OVERCLAIM (narrow; the layering
rule is sound and honestly labeled preliminary).** The pre-merge hook is structural in openmls
0.8.1 (process → StagedCommit → explicit merge; declining measured real in S16/S18), but nothing
forces policy evaluation — "every member applies the merge rule" is an application-discipline
premise, correctly made a group-context MUST elsewhere. Confidentiality on a refused joiner's
one-person branch holds under the all-refuse premise (S18/S21). Two holes: (a) **"never admission"
rests on §7.3.8's own "corroborates but does not prove"** — an eclipsed member whose corroboration
set falsely attests freshness merges; the claim is stronger than the gate claims for itself; (b)
"can only leak the roster" understates: unbounded refused-fork processing load (challenge
rate-limits key on rotatable `EndpointId`s), forks that share epoch numbers with the real chain
(partition-bait, per S18 §3), and leaf credential/key correlation across groups. S25 arm 4 exists
and matches — but it can measure only the refusal half; "roster knowledge only" is structural, not
harness-measurable. *Correction: qualify the corollary with the §7.3.8 residual and the
processing-load outcome; read S25 arm 4's graduation scope accordingly.*

**14. Admission-at-position — CONSISTENT as the labeled extension it claims to be.** §7.3.1's
at-position precondition and its votes-are-not-retracted sentence are directly analogous, and
membership additions are already a fold tier resolved at-position. The genuine stretch, named:
§7.3.1 gates the *author's* standing to contribute a fact; door-B admission evaluates the
*subject's* standing/token, and an external commit's author is the joiner, who holds no standing.
Gate-on-author → gate-on-subject needs its own normative sentence (S26's rule statement is that
text — a bare §7.3.1 cite without it would be a miscitation). §7.4.3's locator-never-authorization
discipline transfers exactly. *Correction: none to the claim; at merge, pair the §7.3.1 cite with
the new normative sentence and reconcile explicitly with the standing-at-head rule (see missed
issue 1 below).*

**15. Live-edge refusal vs catch-up correction — citations CONSISTENT; the adversarial tension is
REAL and unaddressed.** §7.6.2's sentence exists verbatim; §7.6.12 and §11.8's re-fire are cited
faithfully; S26 arm 3 encodes the posture. The gap: during catch-up-then-live-edge, member M —
possibly with a governance fold already at head showing the ban — is instructed to process the
extended chain structurally, and until the corrective removal enacts, everything M seals at the
live edge is readable by the invalid member. §7.3.6's tolerated-window rationale ("content it
would have seen anyway") does not transfer to a never-entitled joiner, and the who-you-key-for
finding (S18) is in tension with instructing M to key for a known-invalid member. Canonical
machinery offers mitigations the prose does not name (§7.3.6 any-member fallback enactment;
§7.6.12 force-roll). *Correction: one sentence resolving M's send-posture during the window —
enact/await the corrective removal before authoring, or own the §11.8-REV exposure window as
covering this case.*

**16. §7.3.8 applied to the merge — CONSISTENT; faithful application of a property-defined gate.**
Admission passes both prongs (irreversibility measured in S21 — "no lesser move" than a new
removal commit; enforce-a-revoked-authority shape fits a revoked standing/token). §7.3.8 names
only §7.3.5/§7.3.6 as invokers, so the WORKING's "(proposed normative)" new-clause form is exactly
right rather than a bare cite. Bonus coherence: §7.3.8's "MUST NOT be extended to reads" confirms
tree-*serving* is correctly a dial outside the mandatory gate. *Correction (editorial): extend
§7.3.8's invoker parenthetical to name the admission merge at the merge pass.*

**Missed issues (group D):** (1) **contradiction-in-placement** — "admission is evaluated at the
commit's causal position, never at the evaluator's head" (WORKING ~2265) sits ~60 lines above
"standing is resolved over the full chain to head, never over the returner-asserted range"
(canonical §11.8 / WORKING ~2329); reconcilable (redemption-at-live-edge vs replay-evaluation) but
the reconciling sentence must be written before both ship verbatim. (2) **The serve-time
challenge-response is a new signature surface with no review item** — the lineage root key (which
also signs governance facts and MLS credentials) signs a server-influenced value; domain tag
present, cross-protocol key-reuse unassessed, response-sealing deferred. (3) S25 arm 4 measures
only half its claim (see item 13). (4) **S23's negative arm is load-bearing for item 13's
premise**: a member missing ledger PSK bytes has an unknown failure mode (clean error vs silent
drop); a silent drop under uneven ledger distribution would reproduce the S18(d) partial-merge
divergence *through the strict-merge path itself*.

### E. E108 ratification (item 17)

**17. Recorded grounds CONSISTENT; §7.6 cite is a location MISCITATION; the claimed scope is an
UNSUPPORTED-INFERENCE against the current artifact.** Grounds (a) and (b) are sound design
reasoning congruent with the walk's own record and canonical §7.3.2's absence semantics; the
divergence-closure and incoherence-repair claims match what G1 measured (divergence real at
`fold_ordering_keys.rs:582–600`; the remove-vs-re-add MEMBER-while-hard-stopped shape recorded
twice, with "socially unreachable" correctly the owner's label). Findings:

- **Location miscitation:** "an unambiguous, grounded statement of the two conflicting facts"
  exists verbatim in canonical — but at §7.3.2 (line 1050), where it is *described as* the §7.6
  posture; §7.6 proper never contains the sentence. Defensible shorthand; strictly a wrong
  pointer. *Correction: cite as "§7.3.2's statement of the §7.6 posture," or move the sentence
  into §7.6 at the merge.*
- **Scope gap (real implementation gap, partially owned):** the artifact is
  `ForkStatus::Contradiction(TypesHash)` carrying the lexicographic **min of the two conflicting
  envelope hashes** — one hash, not the pair, no subject principal; the corpus term "byte-head" is
  loose (it is a full 64-hex-char hash of one pair member). A projection cannot key "subjects
  named by an open contradiction pair" on it. The REV owns "a schema change" but names only the
  member-view third state, not the prerequisite that the contradiction artifact carry the
  pair/subject as data.
- **Missed load-bearing issue:** `ForkStatus` is a **single group-level slot** — the REV's plural
  scope ("the subject of *any* open contradiction") cannot be represented when two contradictions
  on different subjects are open simultaneously; the schema change needs a *set* of open
  contradiction pairs. Also: the REV's "on resolution the projection returns to member/not-member
  per the resolving facts" references a resolving-fact type that exists nowhere —
  `resolve_contradiction` is the hard-stop replay, not human resolution — so the return path is
  unspecified work outside the owned cost sentence. Minor: the `fold_derived.rs` doc comments
  ("the hash is the *other* conflicting fact"; replay "fixes the divergence") both overclaim
  against the code and G1 — test beats source comment; fix when the schema change lands.

### F. E96 ratification and the style principle (items 18–21)

**18. Leak-profile correction — CONSISTENT.** All figures check out against the tests as
assertions, not prints: `group_id` present in inner / absent in wrapped, wrapped fails MLS parse,
28 = 12+16 flat at 64 KiB, dedup/byte-identity/catch-up survive (s17:153–216, 306–421), stranger
refused (structurally the AEAD-open path; the *variant* is print-level plus code structure, not an
asserted variant — evidentiary nuance, not a finding). S7's cleartext triple and the
any-swarm-participant generalization are faithful to the E96 row, with measurement and inference
correctly separated.

**19. Wrapping rule — CONSISTENT.** The failing-side arm exists exactly as claimed
(`wrapping_a_commit_at_the_epoch_it_opens_deadlocks_the_walk`, s17:260–297): reversed wrap →
member at N `expect_err`s; the API-cannot-catch-it claim is documented rationale
(`src/outer_seal.rs:26–28`) demonstrated by consequence. The "walk deadlocks" phrasing is the
correct induction from the measured first-hop refusal, stated as such.

**20. Attribute-conditioned-MUST defaults — one MISCITATION; rest CONSISTENT.** Mode-2 OFF default
coherent with §6.11.2 (conditioned on the every-carrier-is-a-member property, correctly).
`carrier-visible` vs E94's metadata-transparency guard: faithful. §6.6.2 byte-identity: S17
exercised dedup/byte-identity on the **sealed** object (drained == wrapped; dedup over wrapped
bytes). **The miscitation:** "(§11.9.3 — nothing on the envelope is more sensitive than the public
content)" — §11.9.3 concedes *content* confidentiality but explicitly leaves governance visibility
a separate decision that "MUST be explicit"; the envelope's `epoch` field is precisely a
commit/membership signal (the REV's own S7 half says so). A public-content/private-governance
group's envelopes carry a governance signal the public content does not. *Correction: condition
the public-regime OFF default on the group's explicit public-governance acceptance per §11.9.3.*
Merge-consistency note: canonical §6.6.2's literal "identical `PrivateMessage` bytes" becomes
false-as-worded under sealing (the stored object deliberately does not parse as MLS) — reword to
"the outermost sealed bytes."

**21. SHOULD-conversion candidates in canonical Part 2 (produced list, input to the merge):**
strongest candidates — the three reunion-view SHOULDs (§7.6.10 lines ~1410–1414: exclude-banned,
more-restrictive threshold, more-restrictive slot; the text itself calls the loosened alternative
"a downgrade attack in a merge's clothing"); §6.5.2/§6.9.1 relay "SHOULD meter and isolate per
tenant" (→ a declared relay operational-profile attribute); §6.12 "SHOULD use a blind forwarding
helper" past a size (→ call-topology attribute per size band); §7.4 tip-beacon SHOULD (→ beacon
cadence as a profile dial; the surrounding MUSTs already enforce the consequence); §7.6.11
last-resort KeyPackage ("SHOULD NOT reuse, except last-resort" — already the pattern's exact
shape: MUST NOT + declared enumerated exception); §8 failed-op "SHOULD require k-observer
corroboration" (→ rides the existing loud/silent/blackhole dial declaration). Correct *non*-
candidates (keep SHOULD, genuine engineering advice): §7.3.6 single-enactor SHOULDs (text already
says correctness MUST NOT depend on it), §7.4.3 delta-encoding, §7.6.9 product-default guidance
(E111 territory), §7.6.12 force-roll default (already `[confirm]`/dial).

**Missed issue (group F): random-nonce wrapping silently breaks cross-path dedup unless
"wrap once at origin" is stated.** `outer_seal::wrap_with` draws a fresh random nonce per call, so
the same inner message wrapped twice yields different outer bytes; §6.6/§6.6.4's racing rests on
byte-identical dedup. S17 tested exactly and only the wrap-once case. A client that re-wraps per
path breaks dedup with no error anywhere. *Correction: add a second clause at merge — "an object
is wrapped once, at origin; the wrapped bytes are the identity all paths carry." Secondary: state
the residual that the per-epoch queue name remains a within-epoch conversation selector (the §6.4
irreducible routing floor); "the closable leak is closed" is accurate as scoped but should say
so.*

### G. E109 and the regime-transition reading (item 22)

**22. Split verdict.** (a) The §8 posture paraphrase — CONSISTENT; canonical §8 line 1627 is
quoted faithfully (born-at-genesis, immutable, of *a Group's* regime and visibility class, no
silent crossing, republish as distinct authored act; byte-identical in the WORKING, so the session
did not alter its own ground). (b) "Re-plant-shaped" — CONSISTENT for the WORKING's hedged
wording, OVERCLAIM for ROADMAP E109's "governed re-plant": all three §7.6.2 arities stay inside
one Group lineage rooted at one genesis, while a regime-change successor necessarily carries a
**new genesis** (regime is genesis-immutable per §8) — a fourth, re-plant-*analogous* shape, not
an existing arity. (c) "Members land by their own choice per §7.6.6" — OVERCLAIM (flattening):
§7.6.6 prescribes role-based placement (voters self-place by their vote; bystanders default to
**both** lineages; a contested-removal subject MUST NOT get a "both" option), and its trigger is a
same-facts disagreement, not a governed transition. The bystander both-lineages default raises an
unaddressed question for confidential→public (staying in the sealed predecessor while joining the
public successor — is the predecessor frozen?). (d) "Sealed history stays sealed" — CONSISTENT
(§8 republish clause; §7.6.2/§5.11; §8.1). (e) ROADMAP-only "the reverse is the same move
mirrored and cheaper" — UNSUPPORTED-INFERENCE: no canonical sentence addresses public→confidential,
and §8's threat model names retroactive confidentiality a non-guarantee, so the mirror is
structurally lossy one way (prior public content stays public forever). (f) The leaning claims:
"MLS-shaped for provenance, not confidentiality" is a fair gloss of §11.9.3's
attestation-retained posture; **"confidentiality past a certain group size is an illusion" appears
nowhere in canonical Part 2** — it is the owner's conversational gloss presented inside a §11.9.3
parenthetical (MISCITATION as placed; same defect for ROADMAP's "rides the atproto normal");
"size prices the attribute, never picks it" is the owner's *extension* of canonical, not a reading
of it — canonical bands the public-by-default option by size (above ~7k it opens; below, public
exists only as the §11.9.2 opt-in projection), and the addendum presents the
all-four-quadrants-legitimate posture without marking the delta.

**Missed issue (group G): canonical carries an unflagged internal tension E109 resolves but never
names.** §11.9.3/§11.10 repeatedly describe an in-place-sounding move ("enter the public-by-default
regime") with no mechanism, while §8 forbids regime crossing; the two sections never cross-reference
on this, and the reconciliation additionally rests on identifying §8's *visibility*-regime sentence
(`Modeled`) with §11's *confidentiality* regime — an identification made nowhere in canonical. When
E109 lands, §11.10's and §11.9.3's "enter the regime" wordings need rewriting to the
successor-Group form (or a cross-reference to the E109 operation), else the spec will contain an
immutability rule and an "enter the regime" instruction with no bridge.

### H. The conformance sweep and the build plan (items 23–24)

**23. Conformance sweep re-derived — the recorded sweep matches on four of five expectations; two
canonical paragraphs were missed; and one item in this checklist itself is unattested.**

- **§11.7 self-service + progressive return — CHANGED (self-service) / MET (progressive return);
  sweep MATCHES, two omissions.** Self-service becomes charter-conditional: door B interposes a
  serve check an active member must answer and fails CLOSED at a governance-stale peer (S25 arm 3
  prices this); door C surrenders self-service outright — all recorded. Progressive return is
  mechanically unchanged. Omissions: (i) canonical §11.7's own normative sentences ("cost falls on
  the returner"; the fresh-signature MUST; the progressive-return MUST) carry no REV marker saying
  they become door-conditional — the reconditioning lives only in adjacent blocks; (ii) the
  progressive-return subsection is untouched, unjudged, and "backfill streaming" is in no
  experiment arm (acceptable — canonical Design, not a decision-2 claim — but the sweep never says
  so).
- **§11.6 batching — MET; cold-as-second-Group CANNOT-MEET; sweep MATCHES.** Batched migration
  survives verbatim; the two-linked-Groups structure is unbuildable (S16) and replaced by
  cold-as-state, recorded. The helper-not-authority clause vs the new serving-peer role is
  answered by serve-protects-roster under strict merge (S25 arm 4 measures it). Consistent.
- **§11.8 interlock — CHANGED (gate location: merge time → serve time, with merge upgraded to the
  §7.3.8 floor) / MET (re-fire); sweep MATCHES — but two canonical paragraphs are orphaned and
  unmarked (MISMATCH):** (1) "Positioning artifacts against the single chain" (WORKING ~2367–2369)
  — its MUST ("the gate must confirm the ticket's epoch and the attestation's position resolve to
  a consistent era") references the two-part credential's ticket/attestation pair, which the
  ratified one-artifact external PSK dissolves; the MUST is vacuous as written and no REV marks
  it. (2) "Cross-chain ordering: eliminated, not bridged" (~2371–2375) — premised on the hot/cold
  two-Group boundary and the resumption PSK, both dead under cold-as-state; moot and unmarked.
  **A step-5 merge that folds only marked blocks would ship dead normative text.** Minor: the
  fresh-signature MUST is declared "satisfied structurally by their being one artifact" — true for
  the *binding* function, but the *freshness/present-control* function is actually redistributed
  to the serve challenge-response nonce and the leaf-credential check, and the REV does not say
  so.
- **Part 1 §2.4 quiescence — the checklist's quoted claim ("eviction and issuance ride one commit,
  so no new liveness assumption") DOES NOT EXIST in any artifact.** No REV block, plan, or E-row
  contains it; the record honestly says the near-opposite: the §11.6 REV names token delivery to
  an absent member as *the* open question, and decision 2 resolves it by making at-join canonical
  ("no meer, no CISS, no delivery problem — the Part 1 §2.4-clean default") while deferring
  at-need-with-deposit on the CISS blocker. Adversarially checked anyway: the migration-minted
  variant *can* mint on the eviction commit but cannot deliver (the member is absent by
  definition), so it does reintroduce an infrastructure dependency — owned in the artifacts as
  "optional equipment, never required." Door B's serve step meets §2.4 via any-member-serves
  (S22) and at-join-in-own-store. **Verdict: MET on the canonical path; the mismatch is between
  this checklist's quote and the record** — whatever source produced that quote should be treated
  as unreliable for other quotes too.
- **§7.3.6 — MET; sweep MATCHES.** Decision 2 touches it obliquely and consistently: the
  propose/govern/commit invite path is §7.3.6's decide-then-enact split, structurally unavailable
  for external joins (the joiner performs both halves — REV (a) says so); "recognition is the
  merge" plus the strict floor re-creates the seam at each incumbent; §7.3.6's own text correctly
  carries no REV. Nuance carried implicitly: §7.6.7's hold-suspends-enactment cannot act
  pre-construction for an external commit — the group-wide merge rule is the functional
  replacement, and S25 arms 1 vs 2 measure exactly the failure when it is absent.

**24. Coverage audit of S23–S26.** Covered (arm named): the ledger negative arm (S23-1), **ledger
transfer to post-issuance joiners (S23-2)** — checklist particular (iii); position 2 end-to-end
(S24 graceful); the refusal triad (S24 a/b/c); perishability and artifact isolation (S24);
challenge-response nonce single-use and wrong-lineage-at-serve (S24 s-i/s-ii); strict-merge floor
(S25-2/3); serve-protects-roster (S25-4, with item 13's caveat that only the refusal half is
harness-measurable); stale-peer divergence (S25-1); admission-at-position + the mutation-killing
characterization arm (S26-1/2); catch-up posture (S26-3, stretch). Deferred **with a stated
pointer** (fine): response-sealing ("may stay untested at this rung"), refusal-verbosity dial
("not tested here"), at-need-with-deposit (CISS blocker), regime transitions (E109).
**`CONTESTED`'s arrival-order pinning test — checklist particular (iv) — is confirmed stated as
outside this plan in all three artifacts** (WORKING §7.3.2 REV, ROADMAP E108, STATE-AND-NEXT):
not lost.

**Coverage gaps — PARTIAL or UNCOVERED, nowhere stated as deferred:**

1. **The positive chain-fact leg of the three-way binding — checklist particular (i) —
   UNCOVERED.** The lineage-key leg (S24-a, s-ii) and PSK leg (S24 graceful) have arms; the
   chain-fact leg is tested only in the *revocation* direction (S23-3). No arm severs
   fact-from-bytes: PSK bytes present in incumbent storage with **no issuance fact at all** (a
   forged or out-of-band ledger entry). Scoping decision 3's "ratifying the credential does not
   claim the binding machinery exists" partially covers, but the plan claims S24's merge rule
   checks "credential resolves to the issued-to lineage" without this arm.
2. **Token-revocation interaction — checklist particular (ii) — PARTIAL.** Revoked-token (S23-3)
   and revoked-lineage (S24-c) are each covered once, separately; the interaction (does a lineage
   ban revoke the token as a distinct fact? re-issuance after revocation?) has no arm and no
   deferral statement.
3. **Position 1 / door A end-to-end — UNCOVERED.** Named unearned in the WORKING's own §11.11
   item-4 REV ("no serving node resolves standing today"), yet no S23–S26 arm exercises a
   standing-check serve; S22 measured only its fail-open property, and S25's "strict serve" is
   the §7.3.8 stall, not a standing-check serve. The plan's "they cover exactly what decision 2
   asserts" quietly drops it.
4. **Rate-limit-by-`EndpointId`-only — UNCOVERED** (restated in the plan as a "standing rule,"
   no arm, no deferral sentence).
5. **Invite-lifecycle unification** (never-active leaf expired by the liveness machinery) and the
   **token-lifetime lapse variant** — UNCOVERED, no deferral statement (E110/E111 defer the
   *writing*, not a test).
6. **Add-commit as mint point** and **readmission-rule-as-group-context-extension** — PARTIAL
   (the motivating fork is measured, S25-1; the mechanisms themselves have no arm).
7. **`GroupInfo`-with-chain-anchor for outsiders — UNCOVERED** (design note, no arm; composes
   with item 12's open outsider-trust question).

**Missed load-bearing issues (group H):** (1) the S23–S25 vs S23–S26 gate inconsistency
(independently found; see item 25) — plus the plan is internally inconsistent ("Four experiments"
in Approach, "these three" in Reasoning); (2) the two orphaned §11.8 paragraphs above; (3) **the
ledger's scale and leaked-ledger threat are never priced** — at-join issuance makes token-PSK
bytes O(N) secrets × N holders with whole-ledger transfer on every join (S23-2 runs at N=5 only),
no §11.10/§11.11 row prices this at the 1k–10k tiers, and one compromised member leaks *all*
tokens, after which the binding rests entirely on the untested lineage-key leg — the REVs discuss
only a member-leaked *tree*; (4) **the last-serving-member corner** — door B redemption needs at
least one live member to serve and incumbents to merge; the evict-then-vanish shape (the last
live member batch-evicts the rest, then is lost) leaves every token holder unredeemable, and
neither the sweep, the dial's achievable-set discussion, nor the plan names it (it is the
D-self-floor analog for serving).

### I. Internal consistency of the session's own artifacts (item 25)

**25. Mostly clean; three findings.** Canonical Part 1 and Part 2 are **untouched** — `git -C
discovery status` shows modifications only to the WORKING copy, ROADMAP_TODO.md, and
STATE-AND-NEXT.md, with the plan and the verification prompt as new untracked files. Every pass-2
banner entry has its block and vice versa (the §11.11 item-6 RATIFIED update rides the E106 entry).
Cross-artifact statuses for E106 (ratified + four conditions), E108 (rule (c), visible-by-default,
pinning test outstanding), E96 (attribute-conditioned MUST + style principle), and decision-2
(PRELIMINARY) agree across WORKING, ROADMAP, STATE-AND-NEXT, and the plan. Findings:

1. **The banner's authoritative-diff instruction is stale.** Lines 43 and 62 of the WORKING copy
   say `grep 'REV 2026-08-16'` "remains the authoritative diff" / "for the diff" — but the §6.4
   E96 block (line 736) is tagged `[REV 2026-08-17]`, so that grep misses it (16 hits vs 1). The
   verification prompt itself already works around this with `grep 'REV 2026-08-1'`. *Correction:
   update both banner lines (or retag the §6.4 block).*
2. **The artifacts disagree on whether S26 gates the merge.** The DECISION-2 block header
   (WORKING line 2253: "until the S23–S25 build proves it") and ROADMAP rows E105, E107, E110,
   and E111 say **S23–S25**; the pass-2 banner ("gated on the S23–S26 build"), E108's row,
   STATE-AND-NEXT ("Nothing merges until S23–S26 run green"), and the plan's own title and exit
   criteria ("All S23–S26 arms green") say **S23–S26**. The plan is internally inconsistent too
   ("Four experiments" in Approach; "why these **three** and not more" in Reasoning). Whether S26
   (catch-up replay determinism — which tests the admission-at-position rule, item 14) gates
   decision-2's graduation is exactly the kind of status a hostile reader checks. *Correction:
   pick one (the plan's exit criteria suggest S23–S26) and align all six artifacts.*
3. **Cosmetic:** the pass-2 banner paragraph is inserted mid-way through the pass-1 ten-block list
   (the banner itself flags the pass-1 count as stale), and "REVISION PASS 2 — 2026-08-16"
   carries decisions dated 2026-08-17. Tidy at merge.

Also noted under items 4–6 (group B): the unmodified canonical framing at WORKING lines
2159–2161 ("carried as two linked Groups… linked by the MLS resumption mechanism") still stands
above the REV that drops cold-as-a-second-Group, and §11.7's base prose still speaks of the cold
Group — expected for a working copy, but the merge must rewrite the base prose, not only append
blocks.

---

## What the talk-through missed entirely (load-bearing, beyond the checklist)

Collected from the per-group verifications; each is stated in its home section above.

1. **Post-ban token-ledger hygiene** (A): a banned ex-member holds every member's token; nothing
   states whether a ban forces ledger re-issuance, and door B's "fails CLOSED" claim is
   population-dependent. The sharpest single gap in decision 2.
2. **The §11.5 × infinite-window interaction** (B): mandatory strict-PCS heals below 250 members
   make the family-dial's "cheap phase-1 catch-up forever" composition fail; no-meer phase-1
   catch-up has no measured commit-stream home.
3. **"Never admission" inherits the Appendix B completeness beam** (D): the corollary must carry
   §7.3.8's corroborated-not-proven qualifier.
4. **The catch-up keying window** (D): a ban-aware member's send-posture while processing an
   extended chain containing an invalid member is unstated, and the tolerated-window rationale
   from §7.3.6 does not transfer.
5. **At-position vs at-head contradiction-in-placement** (D): the reconciling sentence between
   the admission-at-position rule and §11.8's standing-at-head rule must be written.
6. **The serve-time challenge-response as a new signature surface** (D): lineage-root key reuse
   as a challenge-response oracle, unassessed.
7. **Wrap-once-at-origin** (F): random-nonce sealing breaks §6.6.4 cross-path dedup unless the
   rule is stated; S17 tested only the wrap-once case.
8. **`CONTESTED` needs a pair-carrying, set-valued contradiction artifact and a resolution-fact
   type** (E): the ratified scope is unimplementable on the current single-slot min-hash artifact.
9. **The §8 ↔ §11.9.3/§11.10 regime bridge** (G): canonical's "enter the regime" wording
   contradicts genesis-immutability until the E109 successor-Group operation is written in and
   cross-referenced.
10. **Untagged evidence grades in the talk-through addenda** (C): the only REV prose without
    Measured/Verified-RFC/Design tags is exactly the prose most likely to be promoted wholesale at
    merge.
11. **Two orphaned canonical §11.8 paragraphs** (H): the era-consistency MUST and the
    cross-chain-ordering subsection are invalidated by the ratified credential and cold-as-state,
    and no REV marks either — the step-5 merge must rewrite base prose, not only fold marked
    blocks (corroborates the group-B note on §11.6/§11.7 base prose).
12. **Ledger scale and the leaked-ledger threat are never priced** (H): O(N) token secrets × N
    holders, whole-ledger transfer on every join, untested past N=5; one compromised member leaks
    every token, after which the binding rests entirely on the untested lineage-key leg.
13. **The last-serving-member corner** (H): the evict-then-vanish shape leaves every token holder
    unredeemable — no live member can export a `GroupInfo`. The D-self-floor analog for serving,
    named nowhere.
14. **Six plan-coverage gaps with no deferral statement** (H, item 24): the positive chain-fact
    leg of the three-way binding (forged-ledger-entry arm), the token-revocation interaction,
    position-1/door-A end-to-end, rate-limit-by-EndpointId-only, invite-lifecycle unification +
    token-lifetime lapse variant, and outsider chain-anchor verification.

---

*Review complete: all 25 items verified (per-item blocks above), plus the missed-issues register.*
*Nothing outside this file was created or modified by this review. No fixes were applied; the*
*owner decides what gets fixed.*
