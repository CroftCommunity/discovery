# RUN-17: The tier proof — the group model demonstrated in a real way, end to end (v2)

`Status: runnable brief, 2026-07-17 (v2, same day: adds envelope-hash dedup goldens to P1, a
new P8 proving the delivery roles as separate co-hosted processes with membership-interval
backfill, and renumbers the tail). Companion to RUN-16 v2 (the model text; read its Section A
first — it is the specification under test). This run BUILDS AND RUNS the model: open-tier
self-registration collected live from the relay, the gated tier's two-sided membership facts
with causal revocation, device-key delegation verified against real DID documents, the sealed
steward group on the croft-group crates, catalogue reconstructability, the write-policy axis,
the blinded-roster variant, the delivery-roles rehearsal, and a scale rehearsal with measured
numbers. Executes in the discovery repo under house rules. TDD red-first per the standing
directive (2026-07-15): every part's acceptance criteria are committed as failing tests before
implementation; predictions about live wire behavior are written as constants before the first
live call; the summary shows red→green commits and predicted-vs-actual per part. Numbering:
assumes RUN-16; confirm against MASTER-INDEX and renumber consistently if taken.`

---

## 0. Owner pre-steps

1. Credentials, env only (never in code, fixtures, logs, or commits):
   `ATP_TEST_HANDLE` / `ATP_TEST_PASSWORD` (exists from RUN-14), and if available
   `ATP_TEST_HANDLE_2/_PASSWORD_2` and `_3` for genuinely multi-party live parts (steward,
   member, outsider). The run degrades gracefully without them (guardrail 4) but every extra
   live account upgrades a part from stand-in to live grade.

2. Nothing else. No money, no OVH, no R2. No real lexicons are published (guardrail 5).

## 1. Where it lands

`alpha/experiments/tier-proof/` — its own crate/workspace in the house experiment style
(README with goal/approach/result, self-contained; path-deps on croft-group crates permitted
for P6; the RUN-14 appview-validation service-auth verifier and the RUN-15 kit stub patterns
may be COPIED with provenance headers rather than imported, per the self-containment
convention). Registers: backlog + MASTER-INDEX rows; summary at
`alpha/experiments/RUN-17-SUMMARY.md`.

## 2. Guardrails

1. Branch `claude/tier-proof-run-17` from origin/main; one red commit then one green commit
   per part (`test(tier):` / `feat(tier):`); summary table shows order and grade per green
   (component / live / loopback) per the honesty contract; every stand-in gets a
   `SPEC-DELTA[run17-…]` tag at the site and a register row in the same commit.

2. Never edit the reviewed spec. Anything spec-facing goes to the staging doc + reviews log.

3. Prediction-first for every live wire interaction: predicted record propagation shape,
   Jetstream frame fields, DID-doc key encoding, PLC resolution responses — constants in the
   test file BEFORE the first live call; report CONFIRMED/DIVERGED per prediction.

4. Live-first with graded fallback. Parts marked LIVE must run against bsky.social + Jetstream
   when credentials and egress allow (RUN-14 proved this path). If a part needs more live
   accounts than provided, split it: the single-account live half runs live, the multi-party
   half runs with locally generated keypairs plus a stub DID resolver behind the SAME resolver
   interface, tagged `SPEC-DELTA[run17-multiparty | stand-in]`. Never silently downgrade;
   BLOCKED beats pretended.

5. Network hygiene. Experiment records use experiment-local NSIDs under the RUN-14 convention
   (clearly test-scoped collection names); do NOT publish lexicon schema records; do not
   create high-volume noise (each live part writes a handful of records); optionally clean up
   live records at the end but PRESERVE their identifiers in the summary so claims remain
   checkable.

6. Transport stand-ins. Browsers/WebTransport cannot be exercised in this environment; the
   web-native DS role serves plain HTTP here and the WebTransport claim stays a Phase-2/product
   concern (record as a named non-goal, not a delta). The iroh overlay is NOT required: the
   swarm-peer role runs over local sockets behind a transport trait
   (`SPEC-DELTA[run17-swarm-local | stand-in]`); if the iroh crate builds cleanly via the
   proxy within the time-box, a real two-peer iroh exchange upgrades the tag — attempt once,
   do not fight it.

7. Stop rules: ~30 min time-box per stuck build; owner decisions are not yours (Variant A/B is
   NOT decided here — the gated tier stays fork-agnostic behind the RUN-15 `GroupStore`
   shape; the interval-backfill widening dial is asserted as default-interval, not decided);
   finish the queue, report, stop.

## 3. The queue

### P1 — Envelope, identity, and record shapes (component)

Define the experiment-local record shapes from RUN-16 §A.2–A.5: genesis (with BOTH policy
fields), self-registration, request, grant (R7 shape), leave (deletion), revocation (causal
cut), role grant/revoke, device-key attestation, and the message envelope (signature over
scope id + antecedents + payload, dag-cbor canonical). Red first, golden vectors:

- canonical encoding byte-stability; signature verify/reject; context-binding rejection (same
  payload replayed into a different scope or different antecedents MUST fail);

- IDENTITY goldens (§A.5): `H(envelope)` identical across two independent encodes; the same
  envelope arriving "twice" (two delivery paths simulated) dedups to one identity via
  `seen(H)`; the same payload from the same author under different antecedents yields TWO
  identities (correct: two messages); two different authors cannot share an identity (the
  signature is inside the hashed bytes — assert a re-signed copy changes the hash).

Then implement. This part is §A.5 made executable at the byte level.

### P2 — Open tier, LIVE (the one-signature tier)

Red first (predictions + assertions), then: write a genesis record (open/open) and a
self-registration record to the REAL PDS; ingest from REAL Jetstream; fold catalogue + roster;
verify the registration end to end (repo commit signature chain → DID doc key; the "own
signing proves they registered themselves" claim as a test); DELETE the registration live and
observe leave via the firehose; then the reconstructability proof: drop the entire index,
re-fold from backfill + tail, assert the catalogue and roster are byte-identical. PASS: all
live. FALSIFY: self-registration cannot be verified without any second party, or the rebuilt
catalogue differs.

### P3 — Write-policy axis (component; live where accounts allow)

A newsletter scope (open/single) and a forum scope (open/open) from the same machinery: red
matrix — non-author post into the newsletter rejected at fold and absent from serve; author
post served; the same author's post into the forum served; the catalogue displays both policy
fields. Additionally the VALIDATE-BEFORE-RELAY rule (§A.8) at component grade: a relay
function that re-emits only envelopes passing signature + roster + write-policy checks; an
invalid envelope injected into the stream is dropped and never re-emitted.

RECEPTION COMPLETENESS (§A.2), red first: newsletter envelopes are per-author CHAINED (each
carries the author's prior envelope in its antecedents; an unchained envelope from a `single`
writer fails validation); a reader handed the stream minus one MIDDLE envelope detects the
gap from the chain alone and repairs it via backfill from the DS/convergence store, ending
provably complete up to the newest envelope held; a reader whose TAIL is withheld reports
"complete as of <newest held>" and nothing stronger — the detector MUST NOT claim full
currency (the completeness-ahead limit encoded as an assertion about the detector's own
claim); delivery of the withheld tail via a second path (the P8 swarm route) is then detected
and folded, closing the loop the multimodal design promises.

### P4 — Gated tier: two-sided facts and causal revocation (LIVE with 2+ accounts, else split
per guardrail 4)

The heart of the run. Red first, then: steward-set genesis (threshold per a ratified rule;
with one steward account live, threshold-1 live and the multi-steward co-sign case against
local keypairs, tagged); a REQUEST record from the member account; a GRANT from the steward
account with the request hash among antecedents; roster fold shows membership FROM the grant's
causal position; messages verify (P1 envelope: signature vs DID doc + roster-at-position);
then REVOCATION with a causal cut: a message signed by the revoked member with antecedents
BEFORE the cut still verifies; one with antecedents AFTER the cut is rejected. Silence-is-not-
a-verdict as a test: an unanswered second request remains `pending` under fold at every later
fold point (no timeout-verdict code path exists to trigger). Leave-vs-revoke asymmetry: the
member deletes their claim → roster excludes them with no steward act. MEMBERSHIP INTERVALS
(§A.3): the fold emits the member's interval set (grant→cut), asserted against the
constructed history. Archive habit: folded ops written to a state table; a second folder
rebuilds the identical roster from the archive alone with signatures re-verified
(verifiable-not-trusted, executable).

### P5 — Device-key delegation, LIVE

Red first (predict the DID-doc verificationMethod encoding and the attestation record shape),
then: publish an attestation record to the real PDS delegating a locally generated device
signing key; the verifier accepts a P1 envelope signed by the device key by resolving the
REAL DID document plus fetching the attestation; DELETE the attestation live; the next
envelope from that device key is rejected; the key cache invalidates from the firehose event
rather than TTL (assert event-driven). PASS: delegation, verification, and
revocation-by-deletion all against the real network.

### P6 — The steward group on croft-group, and the blinded roster (loopback)

Using the croft-group crates: a small real MLS steward group; deliberation sealed (loopback
transport per that crate's harness); a VERDICT (a grant matching P4's shape) emitted as a
public fact — sealed reasoning, public ruling, executable. Then the blind: a members-only
salt as steward state; roster entries published as hash(DID || salt) commitments;
assertions — an outsider with the full public chain cannot link a known DID to the roster; a
member holding their individual attestation proves their own membership; salt rotation
republishes commitments and old commitments stop matching. Removal from the steward group
demonstrates forward-blindness. Grade: loopback, tagged.

### P7 — Tier transition as re-plant (component)

An open-genesis scope gains a governed successor: a supersession record with the open genesis
in its lineage; assertions — the catalogue presents one continuous identity with the policy
change at a causal position; pre-transition self-registrations remain historically valid and
do not silently become gated-tier grants; the transition emits the plain-language DR-style
banner artifact (presence asserted, wording owner-editable).

### P8 — Delivery roles as separate processes, with interval backfill (component; the A.7
rehearsal)

Three PROCESSES on localhost, deliberately not one binary, co-hosted the way one VPS would
run them:

- the web-native DS (HTTP serve of backplane scopes, roster-gated; the RUN-15 stub pattern);

- a swarm-peer process exchanging envelopes over the transport trait (local sockets per
  guardrail 6; iroh upgrade attempted once);

- the history convergence node: receives envelope sets from BOTH the DS's store and the
  swarm peer, reconciles by envelope hash (set reconciliation; reuse or provenance-copy the
  RUN-12 RBSR construction if its shape fits within the time-box, else a straightforward
  hash-set diff tagged as the stand-in for RBSR), and serves backfill.

Red-first matrix: an envelope injected ONLY via the swarm path and another ONLY via the DS
path both appear exactly once in the converged set (dedup by `H(envelope)` across
transports); INTERVAL BACKFILL — a requester proving membership for interval [J, R) receives
exactly the envelopes with causal positions in that interval, is refused pre-J history, and
after revocation is refused new material while retaining nothing server-side to "unsee"
(assert the refusal is offering-side); a non-member's backfill request is refused outright;
for a SEALED scope the convergence node's store is ciphertext-only and the interval rule
gates offering with decryption bounded by harness-held keys (one assertion pair: offered vs
decryptable). Finally the co-hosting claim: all three processes run simultaneously under
distinct users/ports from one `make roles-up`, and killing any one leaves the other two
serving (failure isolation, executable).

### P9 — Scale rehearsal, measured (component; numbers are the deliverable)

Two measurements, predictions first: (a) backplane verification throughput — synthetic 100k
roster (~10 MB claim checked), envelope verifications/sec (signature + roster-at-position)
single-core, light-client proof size at 100k (log-N inclusion proof against a co-signed
roster head); (b) the sealed ceiling — the churn simulation: over the croft-group loopback
harness, drive membership churn at increasing N, record epoch-roll cost, commit processing
time, and concurrent-commit contradiction rate under the no-arbiter rule, and emit the
evidence-graded curve. Deliverable: MEASUREMENTS.md with both curves and the sentence each
supports; no boundary number is chosen (owner call).

### P10 — Summary and registers

`RUN-17-SUMMARY.md`: red→green table with grades; predicted-vs-actual for every live
prediction; SPEC-DELTA register rows; live record identifiers preserved; the measurements;
and one paragraph mapping each part to the RUN-16 model section it proves, so the corpus's
evidence discipline can cite `(evidence: …, RUN-17)` per claim. Backlog and MASTER-INDEX
rows; site gate green.

## 4. Explicit non-goals

No real lexicon publication, no production serving, no OVH/R2 (RUN-15's staged Phase 1.5/2),
no Variant A/B decision, no boundary-number decision (P9 informs, the owner decides), no
interval-widening decision (default asserted, dial owner-governed), no recovery/I9 work, no
OAuth client leg, no WebTransport-in-browser exercise (product/Phase-2 concern), no
requirement that real iroh builds (one attempt, then the tagged local-socket stand-in), no
cleanup of the reviewed spec's language (staged notes only). Each is one line in the
summary's non-goals so the run's edges stay crisp.
