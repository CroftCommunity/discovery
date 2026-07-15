# Coding transcript — lineage-groups validation (PR #8)

source: AI coding session that produced this Rust validation, pasted verbatim 2026-06-15.

context: the session worked from the lineage-groups thesis (filed in
`discovery/thinking/thesis-lineage-groups.md` — identical brief, not duplicated here). This
log captures the execution narrative: environment assessment, grounding against the real
openmls 0.8.1 API, Phase 0 scaffold, the Phase 1 crypto-feasibility gate, and the CI
license-compliance gate that surfaced at the end. The PR subsequently grew Phase 2/2.5/2.6/3
findings (see the PHASE_*.md files in this directory).

---

> **can you execute on the experiments here?**

I'll start by assessing the environment — this plan depends heavily on Rust tooling and
network access to crates.io (openmls, automerge, iroh), so let me verify what's actually
available before committing to an approach.

> Ran 2 commands

Rust 1.94 is present. The 403 on crates.io homepage is typical (it blocks non-browser
requests) — the real question is whether cargo can reach the index/registry.

> Ran — Test whether cargo can resolve a real dependency from registry

Good — toolchain and registry both work (cargo resolved openmls v0.8.1; the homepage 403 is
just browser-gating). The plan assumed a v0.7-era API but current is 0.8.1, so I'll verify
the actual external-commit/reinit API against the real crate rather than the doc's
assumptions (the plan explicitly demands this).

Scope decision, faithful to the plan's sequencing (§6.2): execute Phase 0 (scaffold) +
Phase 1 (the crypto feasibility gate) in full, run E1.1–E1.4, and stop to report at the
Phase 1 go/no-go gate. Phases 2–3 are explicitly conditional on that gate.

> Ran 2 commands — ground Phase 1 in the real openmls 0.8 API

There's a `group/fork_resolution/` module and a fork-resolution feature — directly relevant.
[Read external commit builder usage test; reinit and add/remove usage; welcome-join +
keypackage build; checked whether the crypto provider's RNG can be seeded.]

The RNG field is private with no seeded constructor — so MLS-layer ops aren't bit-reproducible
via the stock provider (an honest Phase 0 finding; the logic layer will be deterministic).

> Versions confirmed (openmls 0.8.1, rust_crypto 0.5.1, traits 0.5.0, basic_credential 0.5.0).

[Built the workspace manifest + lineage-core deterministic primitives; confirmed provider
type, SignatureKeyPair, CommitMessageBundle, export_group_info, ProcessedMessageContent;
wrote lineage-mls.]

> Compile core and mls crates

`into_verifiable_group_info`/`into_protocol_message` are test-only gated — must use
`.extract()` and match the body. [Fixed the group-info accessor, RatchetTree import, borrow.]
Builds clean.

> Wrote the Phase 1 experiment tests (E1.1–E1.4), each named to match the plan.

E1.2 (the critical survivor primitive) passed. The three failures were the same root cause:
`join_from_welcome` needs the ratchet tree, which the default welcome doesn't embed.
[Enabled the ratchet-tree extension on group creation.]

**All four Phase 1 experiments pass against real openmls.** Then the Phase 0 trivial-scenario
test (with the reproducibility assertion) in lineage-sim — MLS forbids decrypting your own
message, so the one-device scenario doesn't self-deliver. Clippy clean, all tests pass.

> Wrote PHASE_1_FINDINGS, README, gitignore. Staged workspace (lockfile in, target/ out).
> Pushed claude/lineage-groups-validation-gkmgvj. Created PR #8.

### CI license gate (the open item handed back)

CI failure is Cycode license compliance — 6 violations (2 High, 4 Info) from the dependency
tree. Traced reverse dependencies:

- **hpke-rs + 3 sibling crates (MPL-2.0)** — the 2 High findings. Pulled by
  `openmls_rust_crypto`, the crypto provider the entire Phase 1 result depends on. HPKE is
  mandatory for MLS (RFC 9420), and `hpke-rs` is the HPKE implementation across the whole
  openmls ecosystem — both the rust-crypto and libcrux providers route through MPL-2.0 code.
  **There is no pure-permissive substitute.**

- **r-efi (×2)** — the 4 Info findings. Transitive build deps, dual MIT/Apache/LGPL, so a
  permissive license is selectable; effectively benign.

This is a license/compliance call (is MPL-2.0 acceptable for this use case?), not a code bug —
and the only "code" alternative would gut the dependency the experiment exists to validate.
I did not post a Cycode-ignore command or change deps, since that asserts a compliance
position on CroftC's behalf. **Open: license gate decision.**

---

## Note on scope

The PR as imported contains more than this transcript covers: PHASE_2_FINDINGS,
PHASE_2_5_FINDINGS, PHASE_2_6_FINDINGS, and PHASE_3_FINDINGS, plus the lineage-history
backfill tests, lineage-core adversarial/authority/governance tests, and lineage-iroh
end-to-end tests. The Phase 1 GO unlocked that downstream work. Read the PHASE_*.md files for
the per-phase invariant results.
