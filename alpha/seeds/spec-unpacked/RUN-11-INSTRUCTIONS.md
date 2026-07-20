# RUN-11 — Settle (riders + Map fixes), refresh the living docs, and build (L2a, the seam spike, transport continuity)

`Branch: fresh off main (RUN-09 + RUN-10 merged), e.g. run-11-settle-and-build. Parts 1/1b
markdown; Parts 2–4 code, each shipping independently (Part 4 is first-to-drop if the run runs
long). First run under the new site gate: run site/build.py locally after every doc edit batch —
the broken-ref gate is now part of this project's definition of green.`

## Run-wide rules

- I9 firewall holds; the parked list is unchanged (I9; X1; hot-N 500+; `[gates-release]` + BLAKE3;
  emitter integration — now formally deferred by decision; resolution-ACL design frontier).
- TDD for Parts 2–4 (RED-first, red → green evidenced); FIX/FINDING discipline; frozen-record rule
  with its ratified normalization exception; verbatim anchors; minimal diffs; both suites + clippy
  + the site build green at every commit boundary.
- Pre-compliance: new tests carry doc-comments; new/updated claims get EVIDENCE-MAP rows
  (index-not-source); the evidence parenthetical form is used for anything new.

## Part 1 — riders: the confirmed rulings (owner, 2026-07-15) + the Map fixes

1. **FND-T2 ratified.** Record in the settlement section: the RUN-08 §10.5 wording stands as
   reconciled (the footnote already names the over-the-wire authority distribution as the
   residual); the two senses of "emitted" may be split explicitly whenever that footnote is next
   touched, not before. No spec edit.
2. **FND-T3 confirmed.** The four inferred test §-mappings stand as owner-confirmed:
   `convergence.rs` → §7.3; `iroh_convergence.rs` → §6.10/§7.3 (loopback); `regress_free.rs` →
   §7.3; `dedup.rs` → §6.6.4. Record in settlement; doc-comments unchanged.
3. **FND-T4 applied, narrowly.** Reshape the existing inline evidence prose into the standard
   `(evidence: …, RUN-NN[, grade])` parenthetical in exactly the four claims where every component
   already exists: §7.2 R7, §7.3.2 (competing quorums), §7.6.2 (membership half), §8.2(e). No
   information added or dropped — a FINDING if any component turns out missing.
4. **FND-R10-1 applied.** §5.10's "how does the communal-namespace key rotate under churn" framing
   is reframed per the seam brief: a communal namespace has no shared whole-namespace secret to
   rotate; the question decomposes into per-subspace write authority and the fold-gated asset key.
   `Design` tag, seam brief cited (`beta/impl/drystone-design/group-principal-seam.md`).
5. **FND-R10-4 bannered.** One correction banner atop `alpha/Proofs/FROZEN-NOTICE.md` (body
   untouched): the "emits categories 1–6" line understates the folded core, which emits and
   re-proves categories 1–9 (66/0, RUN-08); the body below is the frozen original.
6. **FND-R10-5 applied.** In living docs, the parked design is referred to as the
   "resolution-ACL (croft-group L3)"; croft-group L2 (MLS) does not depend on it. Fix the label
   collision wherever living docs conflate the two (the L2-readiness brief's own usage is the
   model).
7. **Emitter decision recorded.** Annotate `EMITTER-INTEGRATION-BRIEF.md`: "Decided: Option C —
   defer to the `[gates-release]` pass (owner, 2026-07-15); Option B remains the fallback." The
   §10.5 residual line stays as-is.
8. **The Map fixes (audit catches).** In Part 2's back Map: (a) the §7.6 entry is duplicated
   (~lines 3019–3020) — remove the duplicate line; (b) the surviving copy still says "re-plant's
   …, message-continuity half open" — update to "…message-continuity half `Modeled` at loopback
   grade (RUN-09)". Run the site build after: the gate plus the anchor audit must stay clean.
9. Settlement section records 1–8; `part-2-changelog.md` entries for 3, 4, 8; reviews-log RUN-11
   entry at end of run covering all parts.

## Part 1b — the doc update: living-docs currency refresh (post-RUN-09/10 reality)

Living docs only (the frozen-record rule holds); every change provable against the RUN-09/10
summaries and the current registers:

1. **EXPERIMENT-BACKLOG execution order rewritten to the new queue.** Done items struck with
   landing runs (Vouch §2d; E12.2/E12.7 message facet; the M2 steady-state slice; the fan-out
   repeated-run; the traceability settlement). The forward order becomes: L2a (Part 2 of this
   run); the §2e seam spike (Part 3); message continuity over real transport (Part 4); the
   range-partitioned RBSR construction (open — Willow 3d-range vs Negentropy, a read-then-build);
   croft-group L2b+ per the readiness brief; then the parked list verbatim.
2. **MASTER-INDEX** pressing-edges narrative refreshed to the post-RUN-10 board: one Active
   divergence (`hermetic-gossip`, X1-gated); one open design call (I9); the parked release pass;
   the resolution-ACL frontier; everything else `Modeled`-or-better at stated grade or shaped in
   the backlog.
3. **open-threads.md** entries brought current where landed work resolves or reframes them: the
   §5.10 communal-namespace-rotation thread notes the seam brief's dissolution (per the Part 1
   reframe); the recovery thread notes the Tier-1 lock spike landed (RUN-08) with the trust tier
   still the open call; the §6.8.1 thread notes steady-state anti-entropy `Modeled` at loopback
   (RUN-09) with the range-partitioned form open.
4. The site build (gate + anchor audit) re-run after this part; clean is the ship condition.

## Part 2 — build L2a: the MLS-sealed happy-path frame

Execute the shaped backlog row (§3 L2a, per `CROFT-GROUP-L2-READINESS.md`) exactly as shaped.

1. Read first: the L2a backlog row and the readiness brief's §3; the croft-group plans it cites;
   the proven crates it names. **Reuse is a condition of considered compatibility**: L2a is built
   *on* `mls-replant`/`replant-continuity`/the welcome path, never re-implementing their
   mechanics.
2. RED first: the row's RED-able assertions land as failing tests before any implementation;
   red → green order evidenced in the summary.
3. Scope wall: L2a covers the mechanism half only (seal/unseal the happy-path frame over real MLS
   at loopback). If any assertion in the row turns out to require the authority half
   (revocation-over-the-wire, the trust tier) or the resolution-ACL: FINDING, stop that assertion,
   land the rest — do not improvise past the wall.
4. No wire/byte pinning; test-only serialization where comparison needs bytes; grade per A.9 with
   the when-in-doubt-`Modeled` rule and the rationale in the sentence.
5. Conditional edits on green: backlog L2a row → done (RUN-11); EVIDENCE-MAP row; the croft-group
   plan/readiness brief updated to point at the landed slice; changelog/MASTER-INDEX.

## Part 3 — the seam spike: execute backlog §2e as shaped

The group-principal seam brief (RUN-10) added the §2e row; this part executes it.

1. Read first: the §2e row and `beta/impl/drystone-design/group-principal-seam.md`; build exactly
   the row's shaped assertions, RED first.
2. Expected shape (defer to the row where they differ): the Group principal as a Meadowcap
   communal namespace identified by the genesis hash; personae as self-authorizing subspaces;
   capability issuance downstream of the folded Group Role, re-issued (never revoked-in-place)
   across a fold-driven authority change — asserting that a revoked persona's stale capability
   fails and a re-issued one succeeds, deterministically on both members.
3. Scope wall: all Drystone bindings stay `Design`-grade experiment code; no trust tier, no wire
   pinning, no MLS-internals changes. Any assertion in the row exceeding that: FINDING, stop it,
   land the rest.
4. Conditional edits on green: §2e row → done (RUN-11); EVIDENCE-MAP row; a one-line evidence note
   on the seam brief; changelog/MASTER-INDEX.

## Part 4 — message continuity over real transport (first-to-drop)

§7.6.2's message half is `Modeled` because the RUN-09 records were harness-delivered. This part
re-drives the same E12.2 assertions with the continuity records carried over **real iroh-gossip at
loopback** (the transport the other convergence tests already use), harness only injecting the
duplicate/drop faults.

1. RED: the existing four assertions re-asserted over transport delivery (reuse
   `e12_2_message_continuity.rs` shapes; new test file, transport-driven).
2. GREEN: the delivery plumbing only — no B1 wire pinning (test-only serialization rides inside
   the existing gossip frame payloads, commented as such).
3. Conditional edit on green: re-evaluate the §7.6.2 message-half grade per A.9 — if the ladder's
   `Verified` rung genuinely covers "real crypto/transport at loopback," upgrade with the
   parenthetical updated; when in doubt it stays `Modeled` with the rationale clause updated to
   drop "delivered by the harness" and name what still gates (the `[gates-release]` record
   encoding; real-NAT = X1). The summary states which way and why.
4. Ship-without rule: if the run runs long, record the RED tests as the shaped backlog row and
   drop this part cleanly.

## Output

`alpha/experiments/RUN-11-SUMMARY.md`: Part 1 per-rider status with before → after for the
one-line changes and the site-gate result; Part 1b the currency diff list; Parts 2–4 red → green
evidence, assertions landed vs FINDING-stopped or dropped-at-ship-rule, conditional edits applied
or withheld and why; the parked list restated; files changed.
