# RUN-14: Stellin AppView spikes, operator brief for Claude Code (unattended)

`Status: runnable brief, 2026-07-17. Written to be executed by a fresh Claude Code session in the
discovery repo. Scoped to what runs in THIS environment (HTTPS via proxy works, live Jetstream and
PDS reachable, Internet UDP blocked, no browser for interactive OAuth) so it can run unattended.
Everything here is sequenced by leverage and carries an explicit PASS/FALSIFY condition, a stop
rule, and failing-tests-first order.`

> **Numbering.** This brief assumes RUN-13 has landed (EXPERIMENT-BACKLOG shows the classroom
> scaffold landed RUN-13). Confirm against `alpha/experiments/MASTER-INDEX.md` before starting; if
> RUN-14 is taken, renumber this run and its summary consistently and note the renumber in the
> summary header. Do not reuse a taken number.

You are continuing the Croft/Drystone experiment program, this run in service of **Stellin**, the
professional-networking app being designed on Drystone machinery. Orientation lives in
`discovery/AGENTS.md` and the registers in `alpha/experiments/` (`MASTER-INDEX.md`,
`EXPERIMENT-BACKLOG.md`, `SPEC-DIVERGENCE-REGISTER.md`). Read those first; do not re-derive facts
they establish.

---

## 1. Why this run exists (context, read once)

Stellin needs a privately run, small AppView. The design conversation (2026-07-17) mapped its
requirements against the experiment corpus and found the pipeline side fully proven
(`appview-validation`: ingest, index, serve, write loop, backfill-tail handoff, crash recovery,
~64x single-node headroom over the full firehose) while every Stellin differentiator reduces to
**one unproven capability: the AppView has never known who its caller is.** Viewer-gated fields,
sealed-record offer-gating, telemetry, and recruiter-only visibility are the same missing piece
wearing four hats.

The governing design text is `beta/impl/drystone-design/social-mapping.md`:

- **§H invariant:** encryption is the confidentiality boundary in every topology; an AppView gates
  *offering* (who is handed ciphertext), never *reading*. A cleartext-serving AppView rebuilds the
  visibility flag. Hybrid (MLS seal, AppView serve) is the recommended default.

- **Helper roles:** a *distribution* helper serves only ciphertext; a *content* helper admitted to
  a scope by grant may hold cleartext for search/index/aggregation, revocably, and never gains
  authority.

- **Open item:** the AppView-provisioned scope key. This run does NOT resolve it and must not
  attempt to (stop rule 5b). This run earns the parts that do not depend on it.

**Evidence targets.** A green run moves, via the staging doc (never Part 2 directly):

- §H hybrid topology, serve half: `Design` toward experiment-earned (offer-gating demonstrated
  against a real verified viewer identity).

- social-mapping helper delegation: `Design` toward experiment-earned (content helper indexes by
  grant; revocation makes it forward-blind).

- A named record that interactive OAuth/DPoP (the PWA client-login leg) remains unproven, so the
  gap register stays honest.

---

## 2. Guardrails (read before touching anything)

1. **Branch discipline.** Do not commit on `main` or any branch under human review. Start fresh:
   `git fetch origin && git checkout -B claude/experiments-run-14 origin/main`. Push with
   `git push -u origin claude/experiments-run-14`. One commit per numbered step below. Do not open
   a PR unless asked.

2. **Never edit the reviewed spec.** `beta/drystone-spec/part-*` and `conventions-and-decisions.md`
   are review-frozen. When an experiment earns a spec-status change, append to
   `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md` and add a row to
   `beta/impl/experiments/drystone-reviews-and-experiments-log.md` in the same commit.

3. **The honesty contract.** Any green that rests on a stand-in (fixture, weakened assertion,
   environment adaptation) MUST carry a `SPEC-DELTA[<id> | <kind>]` tag at the site and a row in
   `SPEC-DIVERGENCE-REGISTER.md` in the same commit. Distinguish modeled vs real. When in doubt,
   weaken the claim. This run deliberately uses two declared stand-ins (see EXP-A step 4 and EXP-B
   fixture roster); register them, do not hide them.

4. **TDD is mandatory and evidenced (standing directive, 2026-07-15).** For every experiment part:
   write the acceptance criteria as failing tests FIRST, commit them red (message prefix
   `test(run-14): red ...`), then implement to green (`feat(run-14): green ...`). Fixtures before
   features. The run summary MUST show the red-to-green order per part (commit hashes in a table).
   A part whose tests were written after its implementation is a FALSIFY of the run's process
   regardless of code quality.

5. **Prediction-first (hypothesis-validation house style).** Where a part touches the live network,
   write the predicted wire shape into the test file as constants or doc-comments BEFORE the first
   live call, then report predicted-vs-actual in the part's report section. The gap is a primary
   deliverable, not an embarrassment.

6. **Environment preflight.** Record in the summary: `rustc --version` (expect 1.94+), reachability
   of `wss://jetstream2.us-east.bsky.network` and `https://bsky.social` via the proxy, and presence
   of test-account credentials in env (`ATP_TEST_HANDLE`, `ATP_TEST_PASSWORD`, same convention as
   appview-validation phase 2). Credentials never appear in code, fixtures, logs, or commits. If
   the live network is unreachable, EXP-A parts 1 to 3 still run (local fixtures); mark live parts
   BLOCKED and continue.

7. **Stop rules.** (a) Time-box: any build/run stuck ~30 min wall time, checkpoint, mark PARTIAL,
   move on. (b) Design decisions are not yours: if a part seems to require deciding the
   AppView-provisioned scope key, recruiter-admission governance, lobby semantics, or any
   key-management policy, write the options into `EXPERIMENT-BACKLOG.md` and skip. (c) Finish the
   queue, stop, report; do not invent scope.

8. **Per-part deliverable.** Tests (red commit then green commit), a report section
   (method, predicted vs actual, PASS/FALSIFY verdict, SPEC-DELTA if any), backlog row updated,
   registers updated as applicable. The run ends with `alpha/experiments/RUN-14-SUMMARY.md` in the
   established summary format, including the red-to-green evidence table.

---

## 3. Where the code lands

- **EXP-A and EXP-B:** new phase binaries inside the existing crate
  `alpha/experiments/appview-validation/` (`src/bin/authserve.rs`, `src/bin/sealed.rs`), reusing
  the shared core (`record_source`, `index`, `server` modules). That crate's phase-per-binary
  pattern is the house style; extend it, do not fork it. Update its README phase table.

- **EXP-C:** new directory `alpha/experiments/helper-seam/`, its own crate. It may depend on the
  croft-group crates (path deps) and may copy (not import) the `NormalizedEvent`/`LocalPath`
  boundary from `public-roundtrip/src/bridge.rs` with a header comment naming the source; the
  experiments convention is self-containment, and a copied 100-line boundary with provenance beats
  a cross-experiment build coupling. If croft-group's workspace layout makes path deps painful
  (firewall-guard API, MLS deps kept out of pure workspaces), copying the minimal seal/unseal
  surface with provenance is acceptable; register it as a stand-in if the copy diverges.

---

## 4. The queue (do in order; each part self-contained)

### EXP-A: viewer-aware serving via atproto service auth  · runnable now

**Stellin need under test:** every read that depends on who is asking (recruiter-only fields,
telemetry, helper-gated views). **Mechanism under test:** atproto inter-service auth JWTs, the
mechanism by which real AppViews learn caller identity: the client authenticates to its own PDS,
requests a short-lived token via `com.atproto.server.getServiceAuth` with `aud` set to the target
service DID, and presents it; the service verifies the JWT signature against the signing key in the
issuer's DID document. This requires no browser and no OAuth, so it is unattended-runnable, unlike
the PWA's eventual client-login leg (see part 6).

**Predictions to write down first (constants in the test file, before any live call):**

- P-A1: `getServiceAuth` returns `{ "token": <compact JWT> }`.

- P-A2: JWT claims include `iss` (the user's DID), `aud` (the DID we requested), `exp` (short,
  order of a minute), and `lxm` when a lexicon method was specified in the request.

- P-A3: the signature verifies against the `#atproto` verification method in the issuer's DID
  document (secp256k1 or p256 depending on the account), resolved via the PLC directory over HTTPS.

Report each prediction as CONFIRMED or DIVERGED with the observed shape.

**Steps (each: red commit, then green commit):**

1. **Fixture tokens and the verifier.** Failing tests first: a `verify_service_jwt(token, expected_aud, now)`
   function must accept a well-formed fixture token and reject, with distinct error variants,
   (a) bad signature, (b) expired, (c) wrong `aud`, (d) wrong `lxm` for the route, (e) unresolvable
   issuer DID. Fixture keys generated in-test; DID-doc resolution behind a trait so tests use a
   stub resolver. Then implement.

2. **The gated route.** Failing tests first against the axum server: a new XRPC route
   `app.stellin.getProfileView` (name is experiment-local, not a published lexicon; do NOT publish
   any `app.stellin.*` lexicon schema records in this run, the namespace decision is pending
   domain purchase) serving a profile view where the field `openToWork` appears ONLY when the
   verified viewer DID is present in a `recruiters` table AND the viewer's `affiliation` differs
   from the subject's `employer`. Tests enumerate: non-recruiter sees no field; recruiter at a
   different employer sees it; recruiter at the subject's employer does not; anonymous request
   gets the public view with no field; malformed token gets 401 not a degraded view. The
   `recruiters` table is a SQLite fixture: **declared stand-in** (admission governance is R7
   territory, out of scope), register it. Then implement.

3. **Telemetry as a side effect of verified reads.** Failing test first: a verified read of the
   profile route inserts a `(viewer_did, subject_did, ts)` row; the subject, calling a
   `getProfileViews` route as themselves, sees the count; anonymous reads insert nothing. Assert
   the log table is part of the disposable index (dropped and rebuilt empty, not recovered), which
   keeps canonical-state discipline: telemetry is observed state, losable by design. Then
   implement.

4. **Live confirmation.** With env creds: create a session (password auth, the phase-2 pattern),
   call `getServiceAuth` for our service's DID (the service DID may be a `did:web` stub or the test
   account's own DID as audience; if a real service DID cannot be provisioned in-environment, use
   the stub and tag `SPEC-DELTA[run14-A4 | stand-in]`, register it), present the real token to the
   real server, and pass the same test matrix as step 2. Report predicted-vs-actual for P-A1..A3.

5. **PASS/FALSIFY.** PASS: all step-1..3 tests green with red-first evidence, and step 4 green
   against a live-issued token OR cleanly BLOCKED with the network unreachable (fixtures alone do
   not earn the live claim; say so in the summary). FALSIFY: signature verification cannot be made
   to work against real DID-doc keys, or the served view cannot be made to differ by viewer without
   leaking the field through another route (check `getRecord`-style generic routes do not bypass
   the gate; add a test for that).

6. **Named non-goal.** Interactive atproto OAuth + DPoP (the PWA login leg) is explicitly NOT
   attempted: it requires a browser hop this environment does not have. Record it in the backlog as
   the standing gap with one sentence on why it is attended-run territory.

### EXP-B: sealed offer-gating (the §H hybrid serve half)  · after EXP-A step 1

**Design claim under test:** social-mapping §H: the AppView gates offering, never reading; the
private-AppView topology is safe only when records stay sealed and the AppView never holds the
scope key.

**Steps:**

1. **Content-blind store and gate.** Failing tests first: the server stores opaque ciphertext blobs
   keyed by `(group_id, seq)` plus a roster table `(group_id, member_did)`. A
   `getSealedRecords(group_id, since)` route returns ciphertext ONLY when the verified viewer DID
   (EXP-A verifier) is in the roster; non-members and anonymous callers get 403 with no
   length/existence leak beyond the error itself. Then implement.

2. **The blindness assertion, mechanical not rhetorical.** Failing test first: the server binary's
   dependency graph contains no decryption capability for the sealed payloads; concretely, the
   crate feature that compiles the seal/unseal code is not enabled for the server target, and a
   test asserts the served bytes round-trip decrypt ONLY in the client-side test harness (which
   holds the group key) and that no configuration path feeds key material to the server. If the
   assertion can only be made by convention rather than compilation boundary, weaken the claim in
   the report and say exactly what enforces it. Then implement.

3. **Roster is offer-policy, not confidentiality.** Failing test first: removing a member from the
   roster stops future offering (403 on next call) but the test states, in the report, that
   already-fetched ciphertext plus a retained key still decrypts, i.e. the roster gates offering
   while encryption alone gates reading, which is the §H sentence made executable. Then implement.

4. **PASS/FALSIFY.** PASS: member served, non-member refused, blindness assertion holds at a
   compilation or dependency boundary, and the offering-vs-reading distinction is demonstrated.
   FALSIFY: gating cannot be enforced without the server touching key material.

### EXP-C: the helper seam (content helper indexes by grant, goes blind on revocation)  · independent of EXP-B

**Design claim under test:** social-mapping helper delegation: a content helper admitted to the
scope holds cleartext by the same grant any member holds keys by, and it is revocable; plus the
`Verified` source-agnostic boundary extended to its designed private side.

**Steps:**

1. **Helper as member, feeding the index.** Failing tests first: a helper process that is a
   croft-group member receives sealed group messages, decrypts them as any member would, normalizes
   them through the copied `NormalizedEvent`/`LocalPath` boundary, and writes them into the same
   SQLite index schema the appview core uses; a served search route then returns a group-content
   hit. Assert the identical index/serve code path handles a public-source row and a helper-fed row
   (the source-agnostic claim, now live on the private side). Then implement.

2. **Revocation makes the helper forward-blind.** Failing tests first: remove the helper from the
   group (membership remove, epoch roll, whatever the croft-group crate's API names it); assert the
   helper fails to decrypt messages sealed after the roll (MLS forward secrecy doing its job) and
   the index receives no rows for them, while pre-revocation rows remain indexed. The report states
   the honest asymmetry: revocation is forward-only; what the helper was shown, it was shown.
   Then implement.

3. **No-authority check, executable where possible.** One test: the helper's write surface into the
   system is the index only; it holds no route, key, or table that lets it alter group membership,
   governance state, or another member's records. Where this is only assertable by construction,
   say so in one sentence rather than overclaiming a proof.

4. **PASS/FALSIFY.** PASS: grant-index-serve loop closes and revocation forward-blindness is
   demonstrated with red-first evidence. FALSIFY: the croft-group crate cannot support a
   member-process consuming and later being removed without design decisions (if so, stop rule 7b:
   write the options to the backlog, mark PARTIAL, do not improvise group semantics).

---

## 5. Reporting

`alpha/experiments/RUN-14-SUMMARY.md` with: environment preflight; per-part table (part, red
commit, green commit, verdict, SPEC-DELTA ids); the predicted-vs-actual table for P-A1..A3; the two
declared stand-ins (recruiter fixture table, service-DID stub if used) with register rows; the
named non-goals (OAuth/DPoP client leg; AppView-provisioned scope key untouched); and one paragraph
mapping outcomes back to the evidence targets in section 1 so the proposed-changes staging entries
write themselves. Update the appview-validation README phase table (phases 8, 9) and add
helper-seam to `MASTER-INDEX.md`. If any part is BLOCKED, the summary says what unblocks it and
costs one line, not a plan.
