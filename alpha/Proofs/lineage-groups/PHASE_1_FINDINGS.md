# Phase 1 Findings — Crypto/Protocol Feasibility (the thesis gate)

**Date:** 2026-06-14
**Library under test:** `openmls 0.8.1` + `openmls_rust_crypto 0.5.1` (verified
against the real crate source/behavior, not docs — per plan §6.4).
**Toolchain:** Rust 1.94.1, stable.
**Run:** `cargo test -p lineage-mls`

## Gate result: **GO** ✅

> The single most important question (plan §1.4, §7): *Can openmls express "pick
> a survivor epoch and re-key the other side, or mint a third" with
> post-compromise security intact?*

**Yes.** All four Phase 1 experiments pass against the real library. The
external-commit-builder API and the new-group/re-add path both exist and
compose, and post-compromise security (PCS) holds across removal. The reconnect
model is feasible on openmls 0.8.1. **Proceed to Phase 2.**

## Invariant / experiment results

| Exp  | What it proves | Invariant | Result |
|------|----------------|-----------|--------|
| E1.1 | Removed member cannot decrypt post-removal traffic | I4 (PCS) | **PASS** |
| E1.2 | External commit brings a B-member into A's epoch; both derive identical group secret | survivor re-key primitive (I4) | **PASS** |
| E1.3 | Fresh genesis yields a clean third epoch unrelated to both parents; all join converge | "mint a third" path | **PASS** |
| E1.4 | A revocation commit produced while a peer is offline still rekeys that peer correctly on later apply; removed member stays out | broker-carries-revocation | **PASS** |

Phase 0 gate (scaffold + reproducible trivial scenario) also green:
`cargo test -p lineage-sim` → `phase0_trivial_scenario_green`,
`phase0_trivial_scenario_is_logically_reproducible`.

## openmls API assumptions: confirmed vs. corrected

The thesis (§1.4, §6.1) assumed a "v0.7-era external-commit-builder API." Actual
findings against 0.8.1:

**Confirmed present and used:**
- `MlsGroup::external_commit_builder() … .build_group(provider, verifiable_group_info, credential) … .build(rand, crypto, signer, |_| true).finalize(provider)` → `(MlsGroup, CommitMessageBundle)`. This **is** the survivor re-key primitive (E1.2).
- `add_members` / `remove_members` with per-op epoch advance + welcome.
- `export_secret(crypto, label, context, len)` — used as the cross-view epoch-equality proof.
- `export_group_info` + `export_ratchet_tree` — the "survivor publishes its epoch" surface.

**Corrected assumptions (would have failed the thesis if taken on faith):**

1. **No high-level `reinit` with continuity.** openmls 0.8.1 exposes the
   `ReInitProposal` *primitive* but **no** group method that enacts a reinit
   while binding the new group to the old as a continuation. We therefore
   implement the thesis' "reinit / fresh-genesis" as **new group + re-add**
   (`Device::fresh_genesis`). The lineage binding ("both prior logs inherited as
   read-only ancestry") must live in the **Phase 2 governance layer**, not
   inside MLS. *We compose MLS with that layer; we do not extend MLS* — exactly
   the honesty boundary the thesis states (§1.4).

2. **`MlsMessageIn::into_verifiable_group_info` / `into_protocol_message` are
   `#[cfg(test-utils)]`-gated.** Production code must use `.extract()` and match
   `MlsMessageBodyIn`. Isolated in `lineage-mls`; would otherwise be a
   compile-time surprise downstream.

3. **Welcomes do not embed the ratchet tree by default.** A newcomer joining
   via welcome gets `MissingRatchetTree` unless the group is built with
   `use_ratchet_tree_extension(true)` (or the tree is shipped out-of-band). We
   enable the extension; revisit at scale in Phase 3 (tree size vs. welcome
   size tradeoff).

4. **`CannotDecryptOwnMessage`.** MLS forbids a sender decrypting its own
   application message — surfaced while writing the Phase 0 trivial scenario.
   Noted so the sim/transport layers never assume loopback delivery.

## Determinism / reproducibility honesty boundary

`openmls_rust_crypto::RustCrypto` seeds its internal `ChaCha20Rng` with
`from_entropy()` and exposes **no public seeded constructor** in 0.5.1.
Consequently **MLS key material / ciphertext is NOT bit-reproducible**.

We scope reproducibility to the **logic layer** (Lamport clock, seeded RNG for
op/partition ordering and survivor selection, lineage structure, decrypted
plaintext, member sets) — which is what the survivor-selection (I5) and
merge-order invariants actually depend on. Phase 0's reproducibility assertion
compares the *logical digest*, not bytes. If Phase 2 needs bit-reproducible MLS
state, it will require a custom `OpenMlsProvider` with a seedable RNG — flagged
as a Phase 2 task, not assumed solved.

## What this does and does not establish

**Establishes:** the reconnect model's crypto primitives are expressible on
openmls 0.8.1 with PCS intact; the survivor re-key and fresh-genesis paths work
end-to-end across a real `tls_codec` wire boundary.

**Does NOT establish:** anything about the Phase 2 data model (governance
fork/heal, deterministic survivor *selection*, conflict hard-stop), history
backfill provenance, transport behavior, scale, or security audit. Those are
Phase 2/3 and explicitly out of scope here (plan §7).

## Note of interest for Phase 2

openmls 0.8.1 ships a `fork_resolution` module and a `fork-resolution` feature
flag. Worth evaluating in Phase 2 — it may overlap with, or constrain, the
governance-tree fork/heal design. Not used in Phase 1.
