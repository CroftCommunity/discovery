# Drystone delivery layer: experiment plan, round 2

`Status: executable validation plan (round 2), methodology-bound, EXECUTED`

`Result: all 11 experiments CONFIRMED at real-lib fidelity (iroh-gossip 0.101.0, iroh-base 1.0.1, mls-rs 0.55.2), across crates e6 to e11. Both round-1 model-form debts (E3.3, E3.4) retired by R0.1/R0.2 against real mls-rs. No model-form debts remain. Full per-experiment source, raw output, and verdicts live in the separate round-2 results document (06-Drystone-Delivery-Experiment-Results-R2.md). Verdicts folded into design doc 01 §11.`

`Purpose: prove out the mechanisms round 1 did not reach, and retire the round-1 model-form debts at real-library fidelity`

`Companion to: 01-delivery-architecture.md (claims under test), 05-Drystone-Delivery-Experiments.md (round 1), 08-experiment-methodology.md (the fidelity rules this plan is bound by)`

`Builds on: round-1 workspace and state. Reuse the existing crates (e1-gossip, e2-planes, e3-rbsr, e4-push, e5-integration) and add round-2 crates alongside. Libraries to pin and print: iroh 1.0.1, iroh-gossip 0.101.0, iroh-base 1.0.1, mls-rs 0.55.2 (sync), mls-rs-crypto-rustcrypto 0.22.0.`

---

## How this round is bound by the methodology (read first)

Every experiment here MUST obey `08-experiment-methodology.md`. Restated as the gate for this round, because round 1 produced two model-form results that must not recur as silent stand-ins:

1. **State the fidelity rung in the verdict line.** `CONFIRMED (real-lib)`, `CONFIRMED (model-form: <stand-in named>)`, or `CONFIRMED (static)`. A bare `CONFIRMED` is inadmissible.

2. **Never substitute a stand-in for the exact component a claim is about.** The canonical forbidden move is XOR-as-MLS. This round, experiments that touch confidentiality, credential validation, or the seal MUST run against real `mls-rs`. Round 1's E3.3 (XOR for the seal) and E3.4 (hash-chain for the credential) are explicitly re-run here at Rung A, and a stand-in is not acceptable a second time.

3. **Pin and print exact resolved versions** in every result.

4. **Do not assert an API shape from memory.** Begin each experiment by reading the then-current crate docs or running `cargo doc` / inspecting types. Round 1 was nearly wrong about the gossip event surface until the real enum was read.

5. **A FALSIFIED result is a success.** Record it loudly, state the design consequence, name the branch it reshapes.

6. **Rung-B results generate a Rung-A follow-up** and do not retire a real-mechanism `[confirm]`.

For each experiment: claim under test (with the doc section it backs), hypothesis, method, fidelity rung intended, and the explicit pass/fail line to print.

---

## R0: Retire the round-1 model-form debts (highest priority)

These two close the only model-form gaps round 1 left open. They are the reason this round exists.

### R0.1: Entitlement boundary at real-library fidelity (retire E3.3)

**Claim under test:** sync moves content only within entitlement; a non-member device cannot read a group's messages (§6.2, the origin and trust boundaries).

**Round-1 debt:** E3.3 used an XOR cipher as the seal stand-in. That validated the *logic* (a wrong key yields garbage) but never exercised MLS. Methodology forbids leaving it there.

**Hypothesis:** a real `mls-rs` group of two members (Alice, Bob) seals an application message; Bob (member) decrypts via real `process_incoming_message`; a third device Carol that is **not** in the group, given the same sealed `MlsMessage` bytes, fails to decrypt (no key material / not a member), not "decrypts to garbage" but a real library-level failure.

**Method:**

- Build a real two-member group (reuse the e2-planes group-construction code: `create_group`, `generate_key_package_message`, `add_member`, `apply_pending_commit`, `join_group` from Welcome).

- Alice seals with `encrypt_application_message`. Bob decrypts: assert `ApplicationMessage` with correct plaintext.

- Construct Carol as a separate `mls-rs` client never added to the group. Feed Carol the same `MlsMessage` bytes via `from_bytes` then `process_incoming_message`. Assert this returns an **error** (or otherwise does not yield the plaintext), through the real library, not a hand-rolled key check.

**Fidelity rung intended:** A (real-lib). No stand-in for the seal.

**Print:** `R0.1 CONFIRMED (real-lib): member decrypts; non-member device rejected by mls-rs at <error/site>.` or `R0.1 FALSIFIED (real-lib): <what happened>.`

### R0.2: Credential / lineage-gated admission at real-library fidelity (retire E3.4)

**Claim under test:** admission to a group (and specifically the lineage-restricted device pool) is enforced member-side through the real credential-validation path, not a hand-rolled predicate (§6.4; §2.3 substrate note on credential validation at the AS).

**Round-1 debt:** E3.4 used a hash-chain predicate as the credential stand-in. It validated the gating *logic* and named the `mls-rs` hook surface but never exercised it.

**Hypothesis:** using `mls-rs`'s real identity-provider / credential-validation extension point, a custom validator that requires a lineage marker on the credential admits a leaf whose credential carries a valid marker and rejects one that does not, at `process_incoming_message` time for the Add (or at the validation hook the library actually invokes).

**Method:**

- Read the current `mls-rs` identity-provider / `IdentityProvider` (and any `CustomProposalRules` or credential-validation) surface from `cargo doc`; do not assume the trait shape from round-1 notes.

- Implement a minimal custom identity provider whose validation requires a named field/marker on the `BasicCredential` (standing in for the lineage proof *as data*, which is allowed: the data is a placeholder, but the validation path is real and is the thing under test).

- Add a leaf with a conforming credential: assert admission. Attempt to add a leaf with a non-conforming credential: assert the library rejects it through the validator.

**Fidelity rung intended:** A (real-lib) for the *validation path*. Note honestly: the lineage proof's cryptographic construction (descent from a rooting key) is a Part 2 §5.2 detail and may itself be a placeholder marker here; what must be real is that `mls-rs` invokes the validator and enforces its verdict. Tag precisely: `CONFIRMED (real-lib: mls-rs validation path; lineage proof is a placeholder marker, construction deferred to Part 2 §5.2)`.

**Print:** `R0.2 <verdict> (real-lib): conforming leaf admitted, non-conforming leaf rejected by mls-rs validator at <site>.`

---

## R1: Gap-aware history convergence (the session's central new mechanism)

These prove the unified detect-then-fill mechanism of §7.5, which round 1 did not test as a mechanism.

### R1.1: High-water mark defines the complete expected range

**Claim under test:** a single latest author-signed monotonic index defines the full expected range below it, so any unheld index in 1..mark is a nameable gap (§7.5 detection).

**Hypothesis:** given a set of records each carrying `(author_id, signed_index)`, a node holding a sparse subset can compute, per author, exactly which indices below the high-water mark it lacks, from the records alone, with no external coordination.

**Method:**

- Generate author A's records for indices 1..50, each a real signed record (use real Ed25519 signing over `(author_id, index, content_hash)`; do **not** stand in a non-signature here, the signature is part of what makes the index trustworthy).

- Give node N a subset (say it holds 1..10, 12, 15, 47). Compute high-water mark = 47, expected range = 1..47, derive the gap set {11, 13, 14, 16..46}.

- Assert the derived gap set exactly matches the withheld indices within 1..47, and that 48..50 are **not** claimed as gaps (correctly above the mark N has seen).

**Fidelity rung intended:** A for the signing/verification (real Ed25519); the record envelope is a model of the eventual MLS-carried index (named as such). Tag: `CONFIRMED (real-lib: Ed25519 signatures; record envelope models the MLS-carried index)`.

**Print:** `R1.1 <verdict>: high-water=47, derived gaps match withheld set, 48..50 correctly not claimed.`

### R1.2: A single fresh message re-establishes the range over a lossy path

**Claim under test:** even gossip's lossy, no-replay delivery sharpens detection, because one recent message updates the high-water mark (§7.5; ties to E1.2).

**Hypothesis:** a node that received an old, sparse subset and then receives exactly one fresh high-index message (over a path that replays nothing) immediately widens its known expected range to the new mark and names the new gaps.

**Method:**

- Node N holds A's 1..10. N's high-water mark is 10; no gaps.

- Deliver exactly one fresh record, A's index 30 (simulate the lossy path: deliver only this one, nothing between). Real signature on it.

- Assert N's high-water mark jumps to 30 and N now names 11..29 as gaps it previously had no signal for.

**Fidelity rung intended:** A for signature/verification; B for the "lossy path" (in-process single-record delivery models gossip's behavior, named). Tag accordingly.

**Print:** `R1.2 <verdict>: one fresh signed record at 30 widened expected range 10 -> 30, named 11..29 as gaps.`

### R1.3: Fill is source-agnostic on validity, leak-ordered on choice

**Claim under test:** a self-verifying record is accepted regardless of which partner supplied it, so fill priority is driven by metadata leak (device-group, then members), never by source-as-validity (§7.5 fill; §3.4 Case-1 invariant).

**Hypothesis:** the same signed record, supplied by (a) the user's own second device and (b) a fellow member, verifies identically and is accepted identically; an *unsigned* or *wrong-signature* record from any source is rejected identically. Validity is a property of the record, not the source.

**Method:**

- One real signed record R for A's index 14.

- Present R to node N as if from device-sibling; assert accepted (verifies). Present the identical R as if from a member peer; assert accepted identically.

- Present a tampered R (mutated content, signature now invalid) from the most-trusted source (own device); assert **rejected** despite the trusted source.

- Assert the acceptance decision is a pure function of signature validity, independent of the source label.

**Fidelity rung intended:** A (real Ed25519 verification).

**Print:** `R1.3 <verdict>: identical record accepted from device and member; tampered record rejected even from own-device source; validity independent of source.`

---

## R2: Routing fabric larger than the entitlement group (the decoupling)

Proves §1.1: a non-member carrier routes and triggers without reading, and the entitlement boundary holds on the same overlay.

### R2.1: Non-member carrier routes and wakes, never reads (discard-the-blob P-gossip)

**Claim under test:** a fabric carrier that is not a group member receives the sealed blob via `Received`, can act on arrival (fire a wake), cannot read content, and the design needs no companion channel (§1.1, §4; the corrected E1.1 consequence).

**Hypothesis:** on a real iroh-gossip topic with three nodes where only two are MLS members, the third (carrier) receives the sealed `MlsMessage` bytes, fails to decrypt them with `mls-rs` (not a member), and can still observe arrival to trigger a wake. No second channel is used.

**Method:**

- Reuse e1-gossip's 3-node concurrent-join harness and e2-planes' real two-member MLS group. Nodes A and B are members; node C joins the gossip topic but is **not** added to the MLS group.

- A broadcasts the sealed bytes over gossip. Assert B receives and decrypts (real `mls-rs`); assert C receives the `Received` event (arrival observed) but `process_incoming_message` fails for C (no membership).

- Assert C can fire a wake signal (a bare function call / channel send) on arrival, and that the harness used only the one gossip topic, no companion ALPN.

**Fidelity rung intended:** A (real iroh-gossip + real mls-rs).

**Print:** `R2.1 <verdict> (real-lib): member B read; non-member carrier C observed arrival, could wake, could not decrypt; single channel only.`

### R2.2: Reconciliation partner need not be on the fabric (no transport gate)

**Claim under test:** a gap detected from a message arriving over the fabric is filled by reaching an entitled member over a *different* transport; the gate is membership, not fabric position (§7.5 flow; §1.1).

**Hypothesis:** node N detects a gap from a gossip-delivered high-water mark, then reconciles the missing record from a member reached over a **direct dial** (not the gossip topic), and the member supplying it need never have been on the gossip topic at all.

**Method:**

- N is on the gossip topic and receives a fresh high-index record (sets the mark, names a gap), reusing R1.2-style detection.

- The member M that holds the missing record is **not** subscribed to the gossip topic. N opens a direct iroh connection to M (separate from gossip) and reconciles the missing record (real signed record, real verification).

- Assert the gap is filled, that M was never a gossip participant, and that membership (not topic subscription) was the gate N applied before requesting.

**Fidelity rung intended:** A for transport (real iroh direct dial) and verification; B for the membership-gate check if the governance lookup is modeled rather than a full MLS view (name it).

**Print:** `R2.2 <verdict>: gap detected via fabric, filled via off-fabric direct dial from a non-topic member; gate was membership not transport.`

---

## R3: Device group as a real convergence backplane (encrypted content, stronger membership)

Proves §6.2 as corrected: the device group moves sealed bytes among its leaves exactly as any MLS group, leaves decrypt locally, it widens entitlement to no one (same owner), and the convergence invariants (trust-on-fold, dataplane-only) hold. It is a secondary convergence backplane with a lineage membership story, not a plaintext channel.

### R3.1: Two member-devices reconcile via the group; a non-member device cannot read

**Claim under test:** the device group moves sealed PrivateMessage bytes between co-entitled devices over a direct encrypted dial, each leaf decrypts locally, and a non-member device holds only undecryptable sealed bytes (§6.2 as corrected).

**Hypothesis:** two devices both leaves in group A reconcile their *sealed-record* sets over a direct iroh/TLS connection and converge; each decrypts locally via real `mls-rs`; a device not a leaf in A, given the same sealed bytes, cannot decrypt them (real library failure) and so gains no readable content.

**Method:**

- Two `mls-rs` clients D1 and D2, both members of group A (both processed the Welcome). Each holds a different subset of A's sealed records.

- Reconcile the *sealed records* (the bytes that move are ciphertext) over a direct iroh connection. Assert both converge to the union of sealed records, that the wire payload is ciphertext (assert the reconciled records are the sealed `MlsMessage` bytes, not cleartext), and that each then decrypts its newly-acquired records locally via real `process_incoming_message`.

- A third client D3, not a leaf in A, given the same sealed bytes: assert `process_incoming_message` fails (real `mls-rs`), so D3 gains nothing readable even if handed the ciphertext.

**Fidelity rung intended:** A (real mls-rs + real iroh).

**Print:** `R3.1 <verdict> (real-lib): two co-entitled devices converged sealed-record set over TLS dial, each decrypted locally; non-member device cannot decrypt the same bytes.`

### R3.2: Acceptance requires fold/verification, not the partner's word; governance never converges

**Claim under test:** a synced record is trusted only on self-verification into the hash structure, and governance state is never accepted as a peer's assertion (§6.2; §3.4 invariants).

**Hypothesis:** (a) a device rejects a tampered dataplane record from its own sibling (trust is on the fold, not the source); (b) a device ignores a *governance claim* offered over the device-group channel and reads governance only from its own MLS view.

**Method:**

- Sibling D2 offers D1 a tampered dataplane record (broken signature). Assert D1 rejects it despite the trusted-sibling source (reuses R1.3 logic, here in the device-group framing).

- D2 offers D1 a governance assertion over the same reconciliation channel (a synthetic "membership changed" record). Assert D1 does **not** mutate its governance/membership state from this channel, that any membership truth is taken from D1's own MLS processing, not the peer's assertion.

**Fidelity rung intended:** A for the signature rejection; B for the governance-isolation check if the governance view is modeled (name it). Tag precisely.

**Print:** `R3.2 <verdict>: tampered record rejected from trusted sibling; governance assertion over device-group channel ignored, membership read from own MLS view only.`

---

## R4: Exit finality and current-membership eligibility (a safety invariant untested in round 1)

Proves the §3.4 eligibility invariant and its §10 Part-1-§2.4 consequence.

### R4.1: A removed member cannot pull history after departure

**Claim under test:** the eligible convergence range is current-membership-only; a former member has no standing to request history after removal (§3.4 exit-final invariant).

**Hypothesis:** after a member M is removed from group A (real `mls-rs` Remove proposal + commit, epoch advances), a reconciliation request from M for post-or-pre-departure records is refused by a current member's gate, and the refusal is a function of M's current non-membership, while records M *already held* remain valid on their own signatures (validity is authorship-relative, not eligibility).

**Method:**

- Real group A with members Alice, Bob, M. Advance an epoch, author some records. Remove M (real Remove + commit); confirm M is no longer a current leaf in Alice's authoritative view.

- M requests reconciliation from Alice. Assert Alice's gate refuses (M is not a current member), and that the refusal does not depend on adjudicating which era M is asking about, it is simply current-membership-false.

- Separately assert a record M legitimately authored before removal still self-verifies (authorship-relative validity is intact; eligibility and validity are different axes).

**Fidelity rung intended:** A for the membership transition (real mls-rs Remove/commit); B for the gate predicate if modeled on top of the real membership view (name it).

**Print:** `R4.1 <verdict>: removed member refused by current-membership gate; pre-removal authored record still self-verifies; eligibility and validity separated.`

---

## R5: Concurrent-message ordering convergence under a real seal (strengthen E3.2)

Round 1's E3.2 confirmed clock-free ordering with a hash stand-in for content. Strengthen it to the real sealed-message hash.

### R5.1: Content-hash tiebreak over real MLS ciphertext converges identically

**Claim under test:** the concurrent-message display tiebreak is the content hash of the *real sealed message*, deterministic and identical across devices (§7.4).

**Hypothesis:** two devices each compute the SHA-256 of the same real `MlsMessage` wire bytes for a set of causally-concurrent messages and sort them into an identical order, with the `(monotonic_index, content_hash)` key, no wall-clock.

**Method:**

- Produce several real sealed application messages (real `mls-rs`). Assign a couple the same monotonic index (model concurrency). Compute SHA-256 over the actual wire bytes on two independent in-process "devices."

- Assert both devices derive the identical total order, including the content-hash tiebreak on the concurrent pair.

**Fidelity rung intended:** A (real mls-rs seal, real hash). This retires the E3.2 "hash of a stand-in" caveat.

**Print:** `R5.1 <verdict> (real-lib): identical order on both devices over real MLS ciphertext hashes; concurrent pair tiebroken identically.`

---

## Result-recording rule (applies to every experiment)

For each, record into the round-2 results document: claim + doc section; exact resolved versions; fidelity rung with stand-in named if any; the code or its location; raw output; the one-line verdict with rung; the design consequence; and for any Rung-B result, the Rung-A follow-up it generates. Promote/retire `[confirm]` items in `01 §11` accordingly: a Rung-A CONFIRMED retires a real-mechanism item, a FALSIFIED rewrites the branch, a Rung-B CONFIRMED leaves the real-mechanism item open with a follow-up.

Priority order if time-bound: **R0 first** (it retires standing methodology debt), then R1 (the central new mechanism), then R2/R3, then R4/R5.
