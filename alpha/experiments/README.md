# experiments

Code-forward, exploratory proof-of-value work for the Sovereign Commons / lineage-groups
effort. Experiments answer "does this work / what is actually true?" against real systems.

This corpus now lives inside `discovery` at `discovery/alpha/experiments/` (folded in 2026-07-13; the
standalone `experiments` repo is frozen and archived). Its two companion strands were folded into
`discovery/alpha/` the same way:

- **`Proofs`** (`alpha/Proofs/`) — durable proofs that verify invariants rigorously enough to become
  design principles. Equal but distinct from this corpus.

- **`discovery`** (the `alpha/` root and `beta/`) — the thinking/synthesis: thesis, research,
  principles, roadmap, narrative.

## Contents

```
alpha/
├── appview-validation/    (croftc PR #6) Rust: ingest → index → serve a minimal AT Protocol
│       AppView against the live public Jetstream, plus a live PDS write loop, a local proof
│       harness, AppView bootstrap (backfill→live-tail), crash recovery, trending/hydration.
│       Seven binaries. Findings: custom NSIDs propagate with no pre-registration; writes via
│       entryway, reads via the account's real PDS; firehose is collection-agnostic.
│
├── public-roundtrip/      (croftc PR #4) Rust: publish to a PDS, index back via Jetstream,
│       then the cryptographic CHAIN-OF-CUSTODY capstone (CAR export → signing key → signed
│       commit → MST inclusion: a record is provably part of a verified identity's signed
│       repo, zero trust in the PDS/relay) plus six validations (V1–V6) and moderation-label
│       signature verification (MODERATION.md). Takeaway: atproto gives cryptographic trust
│       for free, zero semantic trust — own your schema, threading, and policy.
│
├── encrypted-blob-share/  (croftc PR #5) Rust: encrypt → content-address → store → reference
│       → fetch → decrypt for large media in a private group over REAL iroh-blobs, with MLS
│       epoch rotation. Validates the large-binary media path (complements the small-state
│       CRDT path). Key tradeoff: encrypt-then-content-address loses cross-user dedup. Includes
│       a 760KB sample-photo.png test fixture. Cycode license findings waived as PoC.
│
├── android-p2p-app/       (croftc PR #7) Delta-Chat-inspired Android app over a Rust core
│       (UniFFI bindings) doing two-peer Automerge document sync over REAL iroh. Tier 1
│       verified (cargo test green incl. live two-peer sync); APK is toolchain-gated, not
│       code-gated — PATH_TO_APK.md documents the NDK/SDK steps. The mobile/UniFFI path the
│       dossier flagged (RustDesk/Mullvad/LibXMTP pattern). jniLibs .so built locally, not
│       committed.
│
├── automerge-partial-reconstruction/   Rust: verifies the partial-reconstruction / snapshot
│       invariant of the `automerge` crate — a node given only later-epoch changes with deps
│       withheld holds them INERT (no partial doc). Four scenarios PASS, matching JS 3.2.6.
│       Load-bearing for the late-joiner design. CAVEAT: built on 0.6.1, not the 0.7 ship
│       target (toolchain MSRV wall); 0.7 confirmation is an open gap. Imported from
│       discovery/chainvalidation.zip, not a PR — deliverable is REPORT.md.
│
├── croft-group/          Rust Cargo workspace: the Croft group/messaging "pond", on the same
│       functional-core/imperative-shell pattern as croft-app-phase0 (option C: per-pond cores
│       unified by a shared shell). `crates/group-core` (pure (model,intent)→(model,effects);
│       no I/O; WASM-clean; model/intent/effect/update/wire [serde_json, version-tagged] /
│       project/view; 10 tests) + `crates/croft-chat-cli` (the thin shell: holds the Transport
│       port + the deterministic InProcBus fake [DECISION 1], the perform_effect/apply/pump
│       runtime, render, and the `croft-chat demo` binary). Scope = the messaging happy-path
│       (join → send → receive → render over the in-proc bus), proven by a two-peer wiring test
│       + a binary smoke test. Extracted 2026-06-22 from iroh/ (where the chat CLI was a guest).
│       MLS/identity, fork/merge, governance, the real-iroh adapter, and shared-shell composition
│       are sequenced (L1–L6), not built. Plan:
│       croft-group/plans/2026-06-22-1-plan-croft-chat-cli-group-core.md.
│
├── croft-app-phase0/      (croftc PR #10) Rust Cargo workspace: Phase 0 of the Croft
        app/client layer (the Bluesky "pond"). Crux-style functional-core/imperative-shell —
        `crates/core` (pure (state,intent)→(state,effects); no I/O/async/clock; WASM-clean;
        20 acceptance tests A1–D2 + projection P1–P7), `crates/bluesky` (native post type +
        async port in the shell never the core [DECISION 1/2] + fixture-backed fake +
        real HTTP adapter), `crates/cli` (imperative-shell render of every view state),
        `crates/design` (tokens/primitives), `crates/shell` (layout/slots/pinning),
        `crates/web` (Leptos/WASM + ~20 PNG snapshots), `crates/desktop` (Tauri). Real
        recorded Bluesky fixtures (no fabrication). M6 (live Jacquard adapter) deferred.
        Imported at the user's direction (A8 IP/ownership decision); as-built
        BUILD-SPEC/design-philosophy kept verbatim (differ from the more-developed
        discovery/thinking/app copies — see PR-CONVERSATION.md + COHESION §23).
│
├── iroh/                 Rust Cargo workspace (the alt-drive substrate): the P2P transport
│       foundation the other spikes build on, all against REAL iroh. Relay/placement lab
│       (peer placement, hole-punch failure, relay cost), MLS-Welcome-over-iroh, faithful-sync,
│       blob transfer, the `meer` superpeer (Tier-0 blind mirror done, Tiers 1–2 sequenced), and
│       a conformance-vector harness. Extensive E-series + spike results in TEST-LOG.md; roadmap
│       in docs/roadmap.md; open items and next-session handoffs in RESUME-NEXT-SESSION.md.
│
├── local_storage_projection/   Rust: the mutation-vetted substrate — redb storage + append-only
│       governance fold + derived social-graph projection. A pure (state,intent)→(state,effects)
│       fold that is the sole membership/authority engine. Characterized end-to-end under lossy
│       delivery (hold/heal/dedup/observable), order-independent convergence, every §7.6.1 residue
│       shape hard-stopped without false-tripping, and k-of-n thresholds counted by lineage.
│       Substrate lib 97/0. Consumed by croft-chat as its substrate (path dep).
│
├── croft-chat/           Rust Cargo workspace: the integrated Drystone CLI — a ratatui two-pane
│       chat built around "the social graph is the substrate; chat is one tenant." social-graph-core
│       (protocol facade over local_storage_projection) + group-chat-core (the chat tenant) +
│       croft-chat (TUI + `serve` shell). Convergence proven locally (order-scrambling shared-dir)
│       and over REAL iroh-gossip (2- and 4-node `serve`, identical fingerprints); §7.6 contradiction
│       hard-stop surfaced as a blocking banner. Also holds the program's master next-experiments
│       ledger (Batteries 5–8). Plans + execution log in croft-chat/plans/.
│
├── mls-replant/          Rust standalone crate: Battery 7 Rung A — the "re-plant" (read the member
│       set from the governance chain, stamp a fresh MLS group over it, atomically repoint §7.6.2)
│       against REAL openmls 0.8.1. E12.1 + E12.3–E12.6 (dedup-not-fork keystone, drift reset, leaf
│       rotation, last-resort availability) + the M1 per-commit cost band (O(N) floor ↔ O(log N)
│       ceiling). The governance chain is out of scope here (that is the fold / Rung B). Suite 7/0.
│
└── replant-continuity/   Rust bridge crate: Battery 7 E12.7 Rung B — the ONLY place the fold and
        openmls meet (the fold stays openmls-free; mls-replant stays fold-free; the bridge depends on
        both by path). Proves the MLS-stamped member set is exactly the set the governance fold
        derives — across genesis, authorized adds, real removals, and rejected unauthorized changes —
        by driving the real DerivedFold::ingest path and comparing against a fresh stamp's crypto
        membership. Suite 3/0.
```

Most of the atproto/app spikes carry `PR-CONVERSATION.md` + `CODING-TRANSCRIPT.md`
(automerge-partial-reconstruction is the exception — see its README provenance note). The substrate
and re-plant line (`local_storage_projection`, `croft-chat`, `mls-replant`, `replant-continuity`)
instead carry `plans/` directories with dated plan + execution-log docs. Open work across all of
these is catalogued in `EXPERIMENT-BACKLOG.md` and sequenced in `MASTER-INDEX.md`; places where a
green result rests on a stand-in rather than the spec are registered in
`SPEC-DIVERGENCE-REGISTER.md` (grep the code for `SPEC-DELTA`).

## Provenance / exclusions

`appview-validation` (PR #6) and `public-roundtrip` (PR #4) were authored in
`croftc/SecurityPolicy` and imported here. SecurityPolicy plumbing was not imported.
Credentials were env-var-only in the original sessions; only `.env.example` templates are
present.

## Convention

Git identity: chasemp (`chase@owasp.org`, `github-personal` SSH host). Pin deps; commit
lockfiles; never commit credentials. Experiments are bounded and reproducible where possible.
